use std::sync::Arc;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::{
    adapters::{notification::CreateNotification, pagination::PaginationParams},
    dto::{common::RowCount, notifications::PaginatedNotification},
    entities::notifications,
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
        user_identifier: &Uuid,
        notification_identifier: &Uuid,
    ) -> Result<(), DatabaseError>;

    async fn fetch_all(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError>;

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model>;

    async fn count_unread(&self, user_identifier: &Uuid) -> Result<RowCount, DatabaseError>;

    #[allow(dead_code)]
    async fn get_latest_unread_notifications(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError>;
}

impl NotificationRepositoryExt for NotificationRepository {
    async fn create(
        &self,
        notification: &CreateNotification,
    ) -> Result<notifications::Model, DatabaseError> {
        let notification = notifications::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            subject: Set(notification.subject.to_owned()),
            body: Set(notification.description.to_owned()),
            user_identifier: Set(Some(notification.user_identifier)),
            ..Default::default()
        };

        let notification: notifications::Model = notification.insert(self.db_conn.as_ref()).await?;

        Ok(notification)
    }

    async fn fetch_all(
        &self,
        user_identifier: &Uuid,
        pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError> {
        let notifications = notifications::Entity::find()
            .filter(notifications::Column::UserIdentifier.eq(user_identifier.to_owned()))
            .offset(((pagination.page.unwrap_or(1) - 1) * pagination.per_page.unwrap_or(10)) as u64)
            .limit(pagination.per_page.unwrap_or(10) as u64)
            .all(self.db_conn.as_ref())
            .await?;

        let total_count = notifications::Entity::find()
            .filter(notifications::Column::UserIdentifier.eq(user_identifier.to_owned()))
            .count(self.db_conn.as_ref())
            .await?;

        let _total_pages =
            (total_count as f64 / pagination.per_page.unwrap_or(10) as f64).ceil() as i64;

        let paginated_response = PaginatedNotification {
            notifications,
            total: total_count,
        };

        Ok(paginated_response)
    }

    async fn mark_read(
        &self,
        _user_identifier: &Uuid,
        notification_identifier: &Uuid,
    ) -> Result<(), DatabaseError> {
        let Some(notification) = self.fetch_one(notification_identifier).await else {
            return Err(DatabaseError::RecordNotFound);
        };

        let mut notification: notifications::ActiveModel = notification.into();
        notification.is_read = Set(Some(true));
        notification.update(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model> {
        let notification = notifications::Entity::find_by_id(notification_identifier.to_owned())
            .one(self.db_conn.as_ref())
            .await
            .ok()??;

        Some(notification)
    }

    async fn count_unread(&self, user_identifier: &Uuid) -> Result<RowCount, DatabaseError> {
        let count = notifications::Entity::find()
            .filter(notifications::Column::UserIdentifier.eq(user_identifier.to_owned()))
            .filter(notifications::Column::IsRead.eq(false))
            .count(self.db_conn.as_ref())
            .await?;

        Ok(RowCount { count })
    }

    async fn get_latest_unread_notifications(
        &self,
        _user_identifier: &Uuid,
        _pagination: &PaginationParams,
    ) -> Result<PaginatedNotification, DatabaseError> {
        todo!()
    }
}
