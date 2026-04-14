use async_trait::async_trait;
use futures::{StreamExt, stream};
use graphql_client::GraphQLQuery;
use reqwest::{
    Client, Url,
    header::{HeaderMap, HeaderValue},
};
use sea_orm::{DatabaseConnection, EntityTrait, FromQueryResult, Statement};
use tokio::time::{Duration, sleep};

use crate::error::KernelError;

const MAX_RETRIES: usize = 3;
const CONCURRENCY_LIMIT: usize = 10;

pub struct SyncEngine {
    db: DatabaseConnection,
    graphql_client: reqwest::Client,
    api_url: String,
    api_key: String,
    resource_path: String,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = ".graphql/sync_queue_schema.graphql",
    query_path = ".graphql/sync_queue_query.graphql"
        response_derives = "Debug,Serialize,Deserialize",
        normalization = "rust"
)]
pub struct SyncQueueView;

pub type DataQueue = Vec<crate::entities::sync_queue::Model>;

#[async_trait]
pub trait SyncEngineTrait {
    async fn new(
        db: DatabaseConnection,
        api_url: &str,
        api_key: &str,
        resource_path: &str,
    ) -> Result<Self, KernelError>
    where
        Self: Sized;

    async fn up_sync(&self, data_queue: DataQueue) -> Result<(), KernelError>;

    async fn down_sync(&self, data_queue: DataQueue) -> Result<(), KernelError>;

    async fn upload_one(&self, item: crate::entities::sync_queue::Model)
    -> Result<(), KernelError>;

    async fn upload_with_retry(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError>;

    async fn handle_up_sync(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError>;

    async fn handle_down_sync(
        &self,
        items: Vec<crate::entities::sync_queue::Model>,
    ) -> Result<(), KernelError>;

    async fn remove_from_queue(&self, identifier: uuid::Uuid) -> Result<(), KernelError>;
}

#[async_trait]
impl SyncEngineTrait for SyncEngine {
    async fn new(
        db: DatabaseConnection,
        api_url: &str,
        api_key: &str,
        resource_path: &str,
    ) -> Result<Self, KernelError> {
        let client = Client::builder();
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(api_key).map_err(|e| KernelError::EnvError(e.to_string()))?,
        );

        let base_url = Url::parse(api_url).map_err(|e| KernelError::EnvError(e.to_string()))?;
        let graphql_url = base_url
            .join(resource_path)
            .map_err(|e| KernelError::EnvError(e.to_string()))?;

        let graphql_client = client
            .default_headers(headers)
            .build()
            .map_err(|e| KernelError::EnvError(e.to_string()))?;

        Ok(Self {
            db,
            graphql_client,
            api_url: graphql_url.to_string(),
            api_key: api_key.to_string(),
            resource_path: resource_path.to_string(),
        })
    }

    async fn up_sync(&self, data_queue: DataQueue) -> Result<(), KernelError> {
        let results = stream::iter(data_queue.into_iter())
            .map(|item| async move { self.upload_with_retry(item).await })
            .buffer_unordered(CONCURRENCY_LIMIT)
            .collect::<Vec<_>>()
            .await;

        let mut errors = vec![];

        for res in results {
            if let Err(e) = res {
                errors.push(e);
            }
        }

        if !errors.is_empty() {
            return Err(KernelError::EnvError(format!(
                "{} uploads failed",
                errors.len()
            )));
        }

        Ok(())
    }

    async fn down_sync(&self, data_queue: DataQueue) -> Result<(), KernelError> {
        unimplemented!()
    }

    async fn upload_one(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError> {
        let res = self
            .graphql_client
            .post(&self.api_url)
            .json(&item)
            .send()
            .await
            .map_err(|e| KernelError::EnvError(e.to_string()))?;

        if !res.status().is_success() {
            return Err(KernelError::EnvError(format!(
                "Request failed: {}",
                res.status()
            )));
        }

        Ok(())
    }

    async fn upload_with_retry(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError> {
        let mut attempts = 0;

        loop {
            match self.upload_one(item.clone()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    attempts += 1;

                    if attempts >= MAX_RETRIES {
                        return Err(e);
                    }

                    let delay = Duration::from_secs(2_u64.pow(attempts as u32));
                    sleep(delay).await;
                }
            }
        }
    }

    /// Find or create the record on the remote. If it already exists, compare
    /// `updated_at` timestamps and upload only when the local copy is newer.
    async fn handle_up_sync(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError> {
        #[derive(Debug, FromQueryResult)]
        struct RecordTimestamp {
            updated_at: String,
        }

        // Fetch the local record's updated_at via raw SQL (table_name is dynamic).
        let backend = self.db.get_database_backend();
        let sql = format!(
            "SELECT updated_at FROM \"{}\" WHERE identifier = ?",
            item.table_name
        );
        let stmt =
            Statement::from_sql_and_values(backend, &sql, [item.record_identifier.clone().into()]);

        let local_record = RecordTimestamp::find_by_statement(stmt)
            .one(&self.db)
            .await
            .map_err(|e| KernelError::DbOperationError(e.to_string()))?;

        let Some(local_record) = local_record else {
            // Record no longer exists locally; nothing to upload.
            return Ok(());
        };

        let local_updated_at = local_record.updated_at;

        // Check whether the record already exists on the remote.
        let remote_url = format!("{}/{}", self.api_url, item.record_identifier);
        let resp = self
            .graphql_client
            .get(&remote_url)
            .send()
            .await
            .map_err(|e| KernelError::EnvError(e.to_string()))?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            // Record does not exist remotely — create it.
            self.upload_one(item.clone()).await?;
            self.remove_from_queue(item.identifier).await?;
            return Ok(());
        }

        if !resp.status().is_success() {
            return Err(KernelError::EnvError(format!(
                "Failed to fetch remote record: {}",
                resp.status()
            )));
        }

        // Record exists remotely. Compare timestamps and upload only if local is newer.
        let remote: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| KernelError::EnvError(e.to_string()))?;

        let remote_updated_at = remote["updatedAt"].as_str().unwrap_or("").to_string();

        if local_updated_at >= remote_updated_at {
            self.upload_one(item.clone()).await?;
        }

        // Remove from queue regardless — either uploaded or remote was already newer.
        self.remove_from_queue(item.identifier).await?;

        Ok(())
    }

    async fn handle_down_sync(
        &self,
        items: Vec<crate::entities::sync_queue::Model>,
    ) -> Result<(), KernelError> {
        dbg!("Handling down sync for items: {:?}", items);

        Ok(())
    }

    async fn remove_from_queue(&self, identifier: uuid::Uuid) -> Result<(), KernelError> {
        crate::entities::sync_queue::Entity::delete_by_id(identifier)
            .exec(&self.db)
            .await
            .map_err(|e| KernelError::DbOperationError(e.to_string()))?;
        Ok(())
    }
}
