use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::notes::{CreateNote, UpdateNote},
    entities::notes,
    error::KernelError,
};

pub struct NotesRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait NotesRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(&self, payload: &CreateNote) -> Result<notes::Model, KernelError>;

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<notes::Model>, KernelError>;

    async fn find_all(&self) -> Result<Vec<notes::Model>, KernelError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError>;

    async fn recently_added(&self) -> Result<Vec<notes::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateNote,
    ) -> Result<notes::Model, KernelError>;
}

#[async_trait]
impl NotesRepositoryExt for NotesRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(&self, payload: &CreateNote) -> Result<notes::Model, KernelError> {
        let active_model: notes::ActiveModel = payload.to_owned().into();

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<notes::Model>, KernelError> {
        notes::Entity::find()
            .filter(notes::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<notes::Model>, KernelError> {
        notes::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError> {
        notes::Entity::delete_many()
            .filter(notes::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn recently_added(&self) -> Result<Vec<notes::Model>, KernelError> {
        notes::Entity::find()
            .limit(10)
            .order_by_asc(notes::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateNote,
    ) -> Result<notes::Model, KernelError> {
        let model = notes::Entity::find()
            .filter(notes::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("note not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(content) = &payload.content {
            active_model.content = Set(content.clone());
        }
        if let Some(categories) = &payload.categories {
            active_model.categories = Set(serde_json::json!(categories));
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
