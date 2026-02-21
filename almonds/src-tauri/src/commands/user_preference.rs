use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    entities::user_preference,
    repositories::user_preference::UserPreferenceRepositoryExt,
};

use crate::{
    adapters::user_preference::{CreateUserPreference, UpdateUserPreference},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn get_user_preference(
    state: State<'_, AppState>,
) -> Result<Option<user_preference::Model>, AppError> {
    state
        .user_preference_repository
        .get()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_user_preference(
    state: State<'_, AppState>,
    preference: CreateUserPreference,
) -> Result<user_preference::Model, AppError> {
    let created = state
        .user_preference_repository
        .create(&preference.into())
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn update_user_preference(
    state: State<'_, AppState>,
    identifier: Uuid,
    preference: UpdateUserPreference,
) -> Result<user_preference::Model, AppError> {
    let updated = state
        .user_preference_repository
        .update(&identifier, &preference.into())
        .await?;
    Ok(updated)
}
