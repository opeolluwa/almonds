use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{
        bookmarks::{BookmarkTag, CreateBookmark, UpdateBookmark},
        meta::RequestMeta,
        recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
    },
    entities::bookmark,
    error::KernelError,
    repositories::recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct BookmarkRepository {
    conn: Arc<DatabaseConnection>,
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
        tag: &BookmarkTag,
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
}

#[async_trait]
impl BookmarkRepositoryExt for BookmarkRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(
        &self,
        payload: &CreateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, KernelError> {
        let mut active_model: bookmark::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        };
        //TODO: activate
        // else {
        //     return Err(KernelError::DbOperationError(
        //         "workspace identifier is required".into(),
        //     ));
        // };

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
        tag: &BookmarkTag,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::Tag.eq(tag.to_string()))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
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
        if let Some(tag) = &payload.tag {
            active_model.tag = Set(tag.to_string());
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
            .store(&CreateRecycleBinEntry {
                item_id: model.identifier,
                item_type: RecycleBinItemType::Bookmark,
                workspace_identifier: model.workspace_identifier,
                payload,
            })
            .await?;

        bookmark::Entity::delete_many()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}
