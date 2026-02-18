use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{adapters::sync_queue::SyncQueueEntry, entities::sync_queue, error::KernelError};

pub struct SyncQueueRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait SyncQueueRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn push(&self, payload: &SyncQueueEntry) -> Result<(), KernelError>;

    async fn pop(&self, entry_identifier: &Uuid) -> Result<(), KernelError>;

    async fn len(&self) -> Result<i32, KernelError>;

    async fn entries(&self) -> Result<Vec<sync_queue::Model>, KernelError>;
}

#[async_trait]
impl SyncQueueRepositoryExt for SyncQueueRepository {
     fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn push(&self, payload: &SyncQueueEntry) -> Result<(), KernelError> {
        let active_model: sync_queue::ActiveModel = payload.to_owned().into();

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn pop(&self, entry_identifier: &Uuid) -> Result<(), KernelError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::Identifier.eq(*entry_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn len(&self) -> Result<i32, KernelError> {
        let count = sync_queue::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .len();
        Ok(count as i32)
    }

    async fn entries(&self) -> Result<Vec<sync_queue::Model>, KernelError> {
        sync_queue::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
