use lunar::{
    adapters::{meta::RequestMeta, notifications::CreateNotification},
    entities::notifications,
    entities::sea_orm_active_enums::NotificationType,
    repositories::notifications::NotificationRepositoryExt,
};
use tauri::State;
use uuid::Uuid;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn create_notification(
    state: State<'_, AppState>,
    notification: CreateNotification,
    meta: Option<RequestMeta>,
) -> Result<notifications::Model, AppError> {
    state
        .notification_repository
        .create(&notification, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_notification(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<notifications::Model>, AppError> {
    state
        .notification_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_notifications(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notifications::Model>, AppError> {
    state
        .notification_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_notifications_by_type(
    state: State<'_, AppState>,
    notification_type: NotificationType,
    meta: Option<RequestMeta>,
) -> Result<Vec<notifications::Model>, AppError> {
    state
        .notification_repository
        .find_by_type(&notification_type, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn mark_notification_as_read(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<notifications::Model, AppError> {
    state
        .notification_repository
        .mark_as_read(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_notification(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .notification_repository
        .delete(&identifier, &meta)
        .await
        .map_err(Into::into)
}
