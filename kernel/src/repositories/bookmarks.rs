use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::{ItemType, Tag};
#[cfg(feature = "sqlite")]
use crate::enums::{ItemType, Tag};
#[cfg(feature = "sync_engine")]
use crate::types::EntitySyncResult;
use crate::{
    adapters::{
        bookmarks::{CreateBookmark, UpdateBookmark},
        meta::RequestMeta,
        recycle_bin::CreateRecycleBinEntry,
    },
    entities::{bookmark, sync_queue},
    error::KernelError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
        workspace::WorkspaceRepository,
        workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
    },
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct BookmarkRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait BookmarkRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<bookmark::Model>, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn find_by_tag(
        &self,
        tag: &Tag,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;

    async fn exists(&self, identifier: &Uuid) -> Result<bool, KernelError>;

    async fn extract_unsynced(&self) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError>;

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<bookmark::Model>,
    ) -> Result<Vec<EntitySyncResult>, KernelError>;
}

#[async_trait]
impl BookmarkRepositoryExt for BookmarkRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, KernelError> {
        let mut active_model: bookmark::ActiveModel = payload.to_owned().into();

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

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<bookmark::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_tag(
        &self,
        tag: &Tag,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        #[cfg(feature = "sqlite")]
        {
            bookmark::Entity::find()
                .filter(bookmark::Column::Tag.eq(tag.to_string()))
                .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }
        #[cfg(feature = "postgres")]
        {
            bookmark::Entity::find()
                .filter(bookmark::Column::Tag.eq(tag.to_owned()))
                .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .limit(10)
            .order_by_desc(bookmark::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("bookmark not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(url) = &payload.url {
            active_model.url = Set(url.clone());
        }

        #[cfg(feature = "sqlite")]
        {
            if let Some(tag) = &payload.tag {
                active_model.tag = Set(tag.to_string());
            }
        }
        #[cfg(feature = "postgres")]
        {
            if let Some(tag) = &payload.tag {
                active_model.tag = Set(tag.to_owned());
            }
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("bookmark not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: ItemType::Bookmark,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        bookmark::Entity::delete_many()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn exists(&self, identifier: &Uuid) -> Result<bool, KernelError> {
        let result = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
            .ok();

        Ok(result.is_some())
    }

    async fn extract_unsynced(&self) -> Result<Vec<bookmark::Model>, KernelError> {
        let queue_entries = sync_queue::Entity::find()
            // .filter(sync_queue::Column::TableName.eq("bookmarks"))
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

        bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), KernelError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("bookmark"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    #[cfg(feature = "sync_engine")]
    async fn upsert_many(
        &self,
        models: Vec<bookmark::Model>,
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
                            let exists = bookmark::Entity::find()
                                .filter(bookmark::Column::Identifier.eq(model.identifier))
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

#[async_trait::async_trait]
impl TransferRecord for BookmarkRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(KernelError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(KernelError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        if !self
            .record_exists_in_workspace(record_identifier, previous_workspace_identifier)
            .await?
        {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        }

        let Some(record) = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
#[async_trait::async_trait]
impl RecordExistInWorkspace for BookmarkRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, KernelError> {
        let record = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}

#[async_trait::async_trait]
impl DuplicateRecord for BookmarkRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(KernelError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(KernelError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        let Some(record) = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
        else {
            return Err(KernelError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut new_record = record.into_active_model();

        new_record.identifier = Set(Uuid::new_v4());
        new_record.workspace_identifier = Set(Some(*target_workspace_identifier));
        new_record.created_at = Set(Utc::now().fixed_offset());
        new_record.updated_at = Set(Utc::now().fixed_offset());

        new_record
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
