use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::ItemType;
#[cfg(feature = "sqlite")]
use crate::enums::ItemType;
#[cfg(feature = "sync_engine")]
use crate::types::EntitySyncResult;
use crate::{
    adapters::{meta::RequestMeta, recycle_bin::CreateRecycleBinEntry},
    entities::{recycle_bin, sync_queue},
    error::KernelError,
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct RecycleBinRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait RecycleBinRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, KernelError>;

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn purge(&self, identifier: &Uuid, meta: &Option<RequestMeta>)
    -> Result<(), KernelError>;

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<(), KernelError>;

    async fn extract_unsynced(&self) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError>;

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl RecycleBinRepositoryExt for RecycleBinRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, KernelError> {
        let mut active_model: recycle_bin::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        } else {
            return Err(KernelError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        #[cfg(feature = "sqlite")]
        {
            recycle_bin::Entity::find()
                .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .filter(recycle_bin::Column::ItemType.eq(item_type.to_string()))
                .order_by_desc(recycle_bin::Column::DeletedAt)
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }

        #[cfg(feature = "postgres")]
        {
            recycle_bin::Entity::find()
                .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .filter(recycle_bin::Column::ItemType.eq(item_type.to_owned()))
                .order_by_desc(recycle_bin::Column::DeletedAt)
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }
    }

    async fn purge(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn extract_unsynced(&self) -> Result<Vec<recycle_bin::Model>, KernelError> {
        let queue_entries = sync_queue::Entity::find()
            .filter(sync_queue::Column::TableName.eq("recycle_bin"))
            .limit(25)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        let identifiers = queue_entries
            .iter()
            .map(|entry| {
                Uuid::parse_str(&entry.record_identifier)
                    .map_err(|err| KernelError::DbOperationError(err.to_string()))
            })
            .collect::<Result<Vec<Uuid>, KernelError>>()?;

        if identifiers.is_empty() {
            return Ok(Vec::new());
        }

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("recycle_bin"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError> {
        let mut sync_results: Vec<EntitySyncResult> = Vec::new();
        for chunk in models.chunks(20) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|model| {
                    let conn = self.conn.clone();
                    let model = model.clone();
                    async move {
                        let identifier = model.identifier.to_string();
                        let op_result: Result<(), KernelError> = async {
                            let exists = recycle_bin::Entity::find()
                                .filter(recycle_bin::Column::Identifier.eq(model.identifier))
                                .one(conn.as_ref())
                                .await
                                .map_err(|err| KernelError::DbOperationError(err.to_string()))?
                                .is_some();

                            let active_model = model.into_active_model();

                            if exists {
                                active_model.update(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            } else {
                                active_model.insert(conn.as_ref()).await.map_err(|err| {
                                    KernelError::DbOperationError(err.to_string())
                                })?;
                            }
                            Ok(())
                        }
                        .await;
                        EntitySyncResult {
                            identifier,
                            success: op_result.is_ok(),
                            error_message: op_result.err().map(|e| e.to_string()),
                        }
                    }
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;
            sync_results.extend(chunk_results);
        }
        Ok(sync_results)
    }
}
