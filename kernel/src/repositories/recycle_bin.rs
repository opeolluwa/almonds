use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::{
    adapters::recycle_bin::{CreateRecycleBinEntry, RecycleBinItemType},
    entities::recycle_bin,
    error::KernelError,
};

pub struct RecycleBinRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait RecycleBinRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
    ) -> Result<recycle_bin::Model, KernelError>;

    async fn find_all(&self) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<recycle_bin::Model>, KernelError>;

    async fn find_by_item_type(
        &self,
        item_type: &RecycleBinItemType,
    ) -> Result<Vec<recycle_bin::Model>, KernelError>;

    async fn purge(&self, identifier: &Uuid) -> Result<(), KernelError>;

    async fn purge_all(&self) -> Result<(), KernelError>;
}

#[async_trait]
impl RecycleBinRepositoryExt for RecycleBinRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
    ) -> Result<recycle_bin::Model, KernelError> {
        let active_model: recycle_bin::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<recycle_bin::Model>, KernelError> {
        recycle_bin::Entity::find()
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
    ) -> Result<Option<recycle_bin::Model>, KernelError> {
        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_item_type(
        &self,
        item_type: &RecycleBinItemType,
    ) -> Result<Vec<recycle_bin::Model>, KernelError> {
        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::ItemType.eq(item_type.to_string()))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn purge(&self, identifier: &Uuid) -> Result<(), KernelError> {
        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn purge_all(&self) -> Result<(), KernelError> {
        recycle_bin::Entity::delete_many()
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}
