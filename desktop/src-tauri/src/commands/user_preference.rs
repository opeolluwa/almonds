use almond_kernel::{
    adapters::meta::RequestMeta,
    entities::user_preference,
    repositories::user_preference::UserPreferenceRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::user_preference::{CreateUserPreference, UpdateUserPreference},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn get_user_preference(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Option<user_preference::Model>, AppError> {
    state
        .user_preference_repository
        .get(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_user_preference(
    state: State<'_, AppState>,
    preference: CreateUserPreference,
    meta: Option<RequestMeta>,
) -> Result<user_preference::Model, AppError> {
    let created = state
        .user_preference_repository
        .create(&preference.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn update_user_preference(
    state: State<'_, AppState>,
    identifier: Uuid,
    preference: UpdateUserPreference,
    meta: Option<RequestMeta>,
) -> Result<user_preference::Model, AppError> {
    let updated = state
        .user_preference_repository
        .update(&identifier, &preference.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn duplicate_user_preference(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .user_preference_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn transfer_user_preference(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .user_preference_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}
