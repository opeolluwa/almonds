use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::ItemType;
#[cfg(feature = "sqlite")]
use crate::enums::ItemType;
use crate::{
    adapters::{meta::RequestMeta, recycle_bin::CreateRecycleBinEntry},
    entities::recycle_bin,
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

    async fn upsert_many(&self, models: Vec<recycle_bin::Model>) -> Result<(), KernelError>;
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

    async fn upsert_many(&self, models: Vec<recycle_bin::Model>) -> Result<(), KernelError> {
        for chunk in models.chunks(20) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|model| {
                    let conn = self.conn.clone();
                    let model = model.clone();
                    async move {
                        let exists = recycle_bin::Entity::find()
                            .filter(recycle_bin::Column::Identifier.eq(model.identifier))
                            .one(conn.as_ref())
                            .await
                            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
                            .is_some();

                        let active_model = model.into_active_model();

                        if exists {
                            active_model
                                .update(conn.as_ref())
                                .await
                                .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
                        } else {
                            active_model
                                .insert(conn.as_ref())
                                .await
                                .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
                        }
                        Ok::<(), KernelError>(())
                    }
                })
                .collect();

            futures::future::try_join_all(futures).await?;
        }
        Ok(())
    }
}
