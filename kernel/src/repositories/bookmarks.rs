use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::bookmarks::{BookmarkTag, CreateBookmark, UpdateBookmark},
    entities::bookmark,
    error::KernelError,
};

pub struct BookmarkRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait BookmarkRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(&self, payload: &CreateBookmark) -> Result<bookmark::Model, KernelError>;

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<bookmark::Model>, KernelError>;

    async fn find_all(&self) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn find_by_tag(&self, tag: &BookmarkTag) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn recently_added(&self) -> Result<Vec<bookmark::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateBookmark,
    ) -> Result<bookmark::Model, KernelError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError>;
}

#[async_trait]
impl BookmarkRepositoryExt for BookmarkRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(&self, payload: &CreateBookmark) -> Result<bookmark::Model, KernelError> {
        let active_model: bookmark::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<bookmark::Model>, KernelError> {
        bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<bookmark::Model>, KernelError> {
        bookmark::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_tag(&self, tag: &BookmarkTag) -> Result<Vec<bookmark::Model>, KernelError> {
        bookmark::Entity::find()
            .filter(bookmark::Column::Tag.eq(tag.to_string()))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn recently_added(&self) -> Result<Vec<bookmark::Model>, KernelError> {
        bookmark::Entity::find()
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
    ) -> Result<bookmark::Model, KernelError> {
        let model = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
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

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError> {
        bookmark::Entity::delete_many()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}
