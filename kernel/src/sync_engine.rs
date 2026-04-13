use async_trait::async_trait;
use futures::{StreamExt, stream};
use tokio::time::{Duration, sleep};

const MAX_RETRIES: usize = 3;
const CONCURRENCY_LIMIT: usize = 10;

#[cfg(feature = "sync_engine")]
use graphql_client::GraphQLQuery;
use reqwest::{
    Client, Url,
    header::{HeaderMap, HeaderValue},
};

use crate::error::KernelError;
pub struct SyncEngine {
    graphql_client: reqwest::Client,
    api_url: String,
    api_key: String,
    resource_path: String,
}

#[cfg(feature = "sync_engine")]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = ".graphql/sync_queue_schema.graphql",
    query_path = ".graphql/sync_queue_query.graphql"
        response_derives = "Debug",
        normalization = "rust"
)]
pub struct SyncQueueView;

pub type DataQueue = Vec<crate::entities::sync_queue::Model>;

#[async_trait]
pub trait SyncEngineTrait {
    async fn new(api_url: &str, api_key: &str, resource_path: &str) -> Result<Self, KernelError>
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
}

#[async_trait]
impl SyncEngineTrait for SyncEngine {
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
    async fn new(api_url: &str, api_key: &str, resource_path: &str) -> Result<Self, KernelError> {
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

        // check failures
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

    async fn handle_up_sync(
        &self,
        item: crate::entities::sync_queue::Model,
    ) -> Result<(), KernelError> {
        dbg!("Handling up sync for item: {:?}", item);

        Ok(())
    }

    async fn handle_down_sync(
        &self,
        items: Vec<crate::entities::sync_queue::Model>,
    ) -> Result<(), KernelError> {
        dbg!("Handling down sync for items: {:?}", items);

        Ok(())
    }

    async fn down_sync(&self, data_queue: DataQueue) -> Result<(), KernelError> {
        unimplemented!()
    }
}
