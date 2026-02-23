use tauri::State;
use uuid::Uuid;

use almond_kernel::{entities::reminder, repositories::reminder::ReminderRepositoryExt};

use crate::{
    adapters::reminder::{CreateReminder, UpdateReminder},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_reminder(
    state: State<'_, AppState>,
    reminder: CreateReminder,
) -> Result<reminder::Model, AppError> {
    let created = state.reminder_repository.create(&reminder.into()).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_reminders(
    state: State<'_, AppState>,
) -> Result<Vec<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_all()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    reminder: UpdateReminder,
) -> Result<reminder::Model, AppError> {
    let updated = state
        .reminder_repository
        .update(&identifier, &reminder.into())
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_reminder(state: State<'_, AppState>, identifier: Uuid) -> Result<(), AppError> {
    state.reminder_repository.delete(&identifier).await?;
    Ok(())
}
