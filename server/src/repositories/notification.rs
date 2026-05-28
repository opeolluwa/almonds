use std::sync::Arc;

use almond_kernel::entities::notifications;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{notification::CreateNotification, pagination::PaginationParams},
    dto::{common::RowCount, notifications::PaginatedNotification},
    errors::database_error::DatabaseError,
    repositories::base::Repository,
};

#[derive(Clone)]
pub struct NotificationRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for NotificationRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait NotificationRepositoryExt {
    async fn create(
        &self,
        notification: &CreateNotification,
    ) -> Result<notifications::Model, DatabaseError>;

    async fn mark_read(
        &self,
        notification_identifier: &Uuid,
    ) -> Result<(), DatabaseError>;

    async fn fetch_all(
        &self,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError>;

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model>;

    async fn count_unread(&self) -> Result<RowCount, DatabaseError>;

    #[allow(dead_code)]
    async fn get_latest_unread_notifications(
        &self,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError>;
}

impl NotificationRepositoryExt for NotificationRepository {
    async fn create(
        &self,
        notification: &CreateNotification,
    ) -> Result<notifications::Model, DatabaseError> {
        let active: notifications::ActiveModel = notification.to_owned().into();
        let model = active.insert(self.db_conn.as_ref()).await?;
        Ok(model)
    }

    async fn fetch_all(
        &self,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError> {
        let notifications = notifications::Entity::find()
            .offset(((pagination.page.unwrap_or(1) - 1) * pagination.per_page.unwrap_or(10)) as u64)
            .limit(pagination.per_page.unwrap_or(10) as u64)
            .all(self.db_conn.as_ref())
            .await?;

        let total_count = notifications::Entity::find()
            .count(self.db_conn.as_ref())
            .await?;

        Ok(PaginatedNotification {
            notifications,
            total: total_count,
        })
    }

    async fn mark_read(
        &self,
        notification_identifier: &Uuid,
    ) -> Result<(), DatabaseError> {
        let Some(notification) = self.fetch_one(notification_identifier).await else {
            return Err(DatabaseError::RecordNotFound);
        };

        let mut active = notification.into_active_model();
        active.is_read = Set(true);
        active.update(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model> {
        notifications::Entity::find_by_id(notification_identifier.to_owned())
            .one(self.db_conn.as_ref())
            .await
            .ok()?
    }

    async fn count_unread(&self) -> Result<RowCount, DatabaseError> {
        let count = notifications::Entity::find()
            .filter(notifications::Column::IsRead.eq(false))
            .count(self.db_conn.as_ref())
            .await?;

        Ok(RowCount { count })
    }

    async fn get_latest_unread_notifications(
        &self,
        _pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError> {
        todo!()
    }
}
