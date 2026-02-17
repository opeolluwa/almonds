use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{adapters::snippets::Snippet, entities::snippets, error::KernelError};

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
}
