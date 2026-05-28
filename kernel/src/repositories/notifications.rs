use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder,
};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::NotificationType;
#[cfg(feature = "sqlite")]
use crate::enums::NotificationType;
use crate::{
    adapters::{meta::RequestMeta, notifications::CreateNotification},
    entities::notifications,
    error::KernelError,
    utils::extract_req_meta,
};

#[derive(Debug, Clone)]
pub struct NotificationRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait NotificationRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateNotification,
        meta: &Option<RequestMeta>,
    ) -> Result<notifications::Model, KernelError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<notifications::Model>, KernelError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notifications::Model>, KernelError>;

    async fn find_by_type(
        &self,
        notification_type: &NotificationType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notifications::Model>, KernelError>;

    async fn mark_as_read(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<notifications::Model, KernelError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), KernelError>;
}

#[async_trait]
impl NotificationRepositoryExt for NotificationRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create(
        &self,
        payload: &CreateNotification,
        meta: &Option<RequestMeta>,
    ) -> Result<notifications::Model, KernelError> {
        let mut active_model: notifications::ActiveModel = payload.to_owned().into();

        let meta = extract_req_meta(meta)?;
        active_model.workspace_identifier = Set(Some(meta.workspace_identifier));

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<notifications::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        notifications::Entity::find()
            .filter(notifications::Column::Identifier.eq(*identifier))
            .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notifications::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        notifications::Entity::find()
            .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(notifications::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_by_type(
        &self,
        notification_type: &NotificationType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<notifications::Model>, KernelError> {
        let meta = extract_req_meta(meta)?;

        #[cfg(feature = "sqlite")]
        {
            notifications::Entity::find()
                .filter(notifications::Column::NotificationType.eq(notification_type.to_string()))
                .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .order_by_desc(notifications::Column::CreatedAt)
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }
        #[cfg(feature = "postgres")]
        {
            notifications::Entity::find()
                .filter(notifications::Column::NotificationType.eq(notification_type.to_owned()))
                .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
                .order_by_desc(notifications::Column::CreatedAt)
                .all(self.conn.as_ref())
                .await
                .map_err(|err| KernelError::DbOperationError(err.to_string()))
        }
    }

    async fn mark_as_read(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<notifications::Model, KernelError> {
        let meta = extract_req_meta(meta)?;

        let model = notifications::Entity::find()
            .filter(notifications::Column::Identifier.eq(*identifier))
            .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::NotificationNotFound(identifier.to_string()))?;

        let mut active_model = model.into_active_model();

        active_model.is_read = Set(true);
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

        notifications::Entity::delete_many()
            .filter(notifications::Column::Identifier.eq(*identifier))
            .filter(notifications::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}
