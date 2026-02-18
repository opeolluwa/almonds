use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::snippets::{Snippet, UpdateSnippet},
    entities::snippets,
    error::KernelError,
};

pub struct SnippetRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait SnippetRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(&self, payload: &Snippet) -> Result<snippets::Model, KernelError>;

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<snippets::Model>, KernelError>;

    async fn find_all(&self) -> Result<Vec<snippets::Model>, KernelError>;

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError>;

    async fn recently_added(&self) -> Result<Vec<snippets::Model>, KernelError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
    ) -> Result<snippets::Model, KernelError>;

}

#[async_trait]
impl SnippetRepositoryExt for SnippetRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(&self, payload: &Snippet) -> Result<snippets::Model, KernelError> {
        let active_model: snippets::ActiveModel = payload.to_owned().into();

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(&self, identifier: &Uuid) -> Result<Option<snippets::Model>, KernelError> {
        snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<snippets::Model>, KernelError> {
        snippets::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete(&self, identifier: &Uuid) -> Result<(), KernelError> {
        snippets::Entity::delete_many()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn recently_added(&self) -> Result<Vec<snippets::Model>, KernelError> {
        snippets::Entity::find()
            .limit(10)
            .order_by_asc(snippets::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
    ) -> Result<snippets::Model, KernelError> {
        let model = snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("snippet not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(Some(title.clone()));
        }
        if let Some(language) = &payload.language {
            active_model.language = Set(Some(language.clone()));
        }
        if let Some(code) = &payload.code {
            active_model.code = Set(code.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
        }
        if let Some(is_pinned) = payload.is_pinned {
            active_model.is_pinned = Set(is_pinned);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
