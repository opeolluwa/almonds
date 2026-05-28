use almond_kernel::{
    adapters::meta::RequestMeta,
    entities::workspace_preferences,
    repositories::workspace_preferences::WorkspacePreferenceRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::workspace_preference::{CreateUserPreference, UpdateUserPreference},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn get_workspace_preference(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Option<workspace_preferences::Model>, AppError> {
    state
        .workspace_preference_repository
        .get(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_workspace_preference(
    state: State<'_, AppState>,
    preference: CreateUserPreference,
    meta: Option<RequestMeta>,
) -> Result<workspace_preferences::Model, AppError> {
    let created = state
        .workspace_preference_repository
        .create(&preference.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn update_workspace_preference(
    state: State<'_, AppState>,
    identifier: Uuid,
    preference: UpdateUserPreference,
    meta: Option<RequestMeta>,
) -> Result<workspace_preferences::Model, AppError> {
    let updated = state
        .workspace_preference_repository
        .update(&identifier, &preference.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn duplicate_workspace_preference(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .workspace_preference_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn transfer_workspace_preference(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .workspace_preference_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_unsynced_workspace_preferences(
    state: State<'_, AppState>,
) -> Result<Vec<workspace_preferences::Model>, AppError> {
    state
        .workspace_preference_repository
        .extract_unsynced()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_synced_workspace_preferences(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .workspace_preference_repository
        .clear_synced(identifiers)
        .await
        .map_err(Into::into)
}
