use almond_kernel::{
    adapters::meta::RequestMeta,
    entities::reminder,
    repositories::reminder::ReminderRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::reminder::{CreateReminder, UpdateReminder},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_reminder(
    state: State<'_, AppState>,
    reminder: CreateReminder,
    meta: Option<RequestMeta>,
) -> Result<reminder::Model, AppError> {
    let created = state
        .reminder_repository
        .create(&reminder.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_reminders(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    reminder: UpdateReminder,
    meta: Option<RequestMeta>,
) -> Result<reminder::Model, AppError> {
    let updated = state
        .reminder_repository
        .update(&identifier, &reminder.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state.reminder_repository.delete(&identifier, &meta).await?;
    Ok(())
}

#[tauri::command]
pub async fn duplicate_reminder(
    state: State<'_, AppState>,
    record_identifier: &Uuid,
    previous_workspace_identifier: &Uuid,
    target_workspace_identifier: &Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .reminder_repository
        .duplicate_record(
            record_identifier,
            previous_workspace_identifier,
            target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn transfer_reminder(
    state: State<'_, AppState>,
    record_identifier: &Uuid,
    previous_workspace_identifier: &Uuid,
    target_workspace_identifier: &Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .reminder_repository
        .transfer_record(
            record_identifier,
            previous_workspace_identifier,
            target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}
