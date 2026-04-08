use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    response::Response,
};
use uuid::Uuid;

use crate::{
    adapters::{
        jwt::Claims,
        pagination::{PaginatedResponse, PaginationParams},
    },
    dto::common::RowCount,
    entities::notifications,
    errors::service_error::ServiceError,
    response::ApiResponse,
    services::notification_service::{NotificationService, NotificationServiceExt},
};

pub async fn listen_for_new_notifications(
    State(notification_service): State<NotificationService>,
    // claims: Claims,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| {
        let service = notification_service.clone();
        async move { service.handle_web_socket_connection(socket).await }
    })
}

pub async fn fetch_notifications(
    State(notification_service): State<NotificationService>,
    claims: Claims,
    Query(pagination): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<Vec<notifications::Model>>>, ServiceError> {
    let notifications = notification_service
        .fetch_notifications(&claims, &pagination)
        .await?;

    Ok(ApiResponse::builder()
        .data(notifications)
        .message("fetch notifications")
        .build())
}

pub async fn count_unread(
    State(notification_service): State<NotificationService>,
    claims: Claims,
) -> Result<ApiResponse<RowCount>, ServiceError> {
    let resp = notification_service.count_unread(&claims).await?;

    Ok(ApiResponse::builder().data(resp).build())
}

pub async fn mark_read(
    State(notification_service): State<NotificationService>,
    claims: Claims,
    Path(notification_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    notification_service
        .mark_read(&claims, &notification_identifier)
        .await?;

    Ok(ApiResponse::builder().message("notification read").build())
}
