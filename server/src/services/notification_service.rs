use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket},
    response::Response,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::{
        jwt::Claims,
        notification::CreateNotification,
        pagination::{PaginatedResponse, PaginationParams},
    },
    dto::{common::RowCount, notifications::Notification},
    entities::notifications,
    errors::{app_error::AppError, service_error::ServiceError},
    repositories::{
        base::Repository,
        notification::{NotificationRepository, NotificationRepositoryExt},
    },
};

#[derive(Clone)]
pub struct NotificationService {
    repository: NotificationRepository,
}

impl NotificationService {
    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            repository: NotificationRepository::init(db_conn),
        }
    }
    pub async fn handle_web_socket_connection(&self, socket: WebSocket) {
        // let (mut outgoing, mut incoming) = socket.split();

        // // send messages to connected client
        // tokio::spawn(async move {
        //     if let Err(err) = outgoing.send(Message::Text("sample message".into())).await {
        //         log::error!("{err}");
        //     }
        // });

        // // get message from worker
        // tokio::spawn(async move {
        //     while let Some(message) = incoming.next().await {
        //         // send the message
        //         log::info!("got incoming message {message:#?}");
        //     }
        // });

        unimplemented!()
    }

    //push message to client
    pub async fn notify() {}
}

#[allow(dead_code)]
pub(crate) trait NotificationServiceExt {
    async fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> Result<Uuid, ServiceError>;

    async fn listen_for_new_notifications(&self) -> Response;

    async fn get_latest_unread_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> Result<Vec<Notification>, ServiceError>;

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model>;

    async fn fetch_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<Vec<notifications::Model>>, ServiceError>;

    async fn count_unread(&self, claims: &Claims) -> Result<RowCount, ServiceError>;

    async fn mark_read(
        &self,
        claims: &Claims,
        notification_identifier: &Uuid,
    ) -> Result<(), ServiceError>;
}

impl NotificationServiceExt for NotificationService {
    async fn create_new_notification(
        &self,
        request: &CreateNotification,
    ) -> Result<Uuid, ServiceError> {
        // let notification = self.repository.create(request).await?;

        // let api_key = extract_env::<String>("ABLY_API_KEY").map_err(|err| {
        //     log::error!("{}", err);
        //     AppError::OperationFailed(err.to_string())
        // })?;

        // let Some(user_identifier) = notification.user_identifier else {
        //     return Err(ServiceError::OperationFailed);
        // };

        // let ably_client = AblyClient::new(&api_key);
        // let notification_channel = format!("notify:{user_identifier}");

        // log::info!("dispatching real time notification");
        // let notification_payload =
        //     Notification::new("Conversion completed", "file converted successfully");
        // let _ = ably_client
        //     .publish_message(&notification_channel, notification_payload)
        //     .await;

        // Ok(notification.identifier)

        unimplemented!()
    }
    async fn listen_for_new_notifications(&self) -> Response {
        todo!()
        // self.handle_web_socket_connection(socket).await
    }

    async fn get_latest_unread_notifications(
        &self,
        _claims: &Claims,
        _pagination: &PaginationParams,
    ) -> Result<Vec<Notification>, ServiceError> {
        todo!()
    }

    async fn fetch_one(&self, notification_identifier: &Uuid) -> Option<notifications::Model> {
        self.repository.fetch_one(notification_identifier).await
    }

    async fn fetch_notifications(
        &self,
        claims: &Claims,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<Vec<notifications::Model>>, ServiceError> {
        let records = self
            .repository
            .fetch_all(&claims.user_identifier, pagination)
            .await?;

        let paginated_result = PaginatedResponse {
            records: records.notifications,
            page: pagination.page(),
            per_page: pagination.per_page(),
            total_count: records.total as u64,
            total_pages: records.total as u32 / pagination.per_page(),
        };
        Ok(paginated_result)
    }

    async fn count_unread(&self, claims: &Claims) -> Result<RowCount, ServiceError> {
        let result = self
            .repository
            .count_unread(&claims.user_identifier)
            .await?;

        Ok(result)
    }

    async fn mark_read(
        &self,
        claims: &Claims,
        notification_identifier: &Uuid,
    ) -> Result<(), ServiceError> {
        self.repository
            .mark_read(&claims.user_identifier, notification_identifier)
            .await?;
        Ok(())
    }
}
