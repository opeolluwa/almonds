use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    adapters::recycle_bin::RecycleBinItemType,
    entities::recycle_bin,
    repositories::recycle_bin::RecycleBinRepositoryExt,
};

use crate::{
    adapters::recycle_bin::CreateRecycleBinEntry,
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_recycle_bin_entry(
    state: State<'_, AppState>,
    entry: CreateRecycleBinEntry,
) -> Result<recycle_bin::Model, AppError> {
    state
        .recycle_bin_repository
        .store(&entry.into())
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_recycle_bin_entries(
    state: State<'_, AppState>,
) -> Result<Vec<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_all()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recycle_bin_entry(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recycle_bin_entries_by_type(
    state: State<'_, AppState>,
    item_type: RecycleBinItemType,
) -> Result<Vec<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_by_item_type(&item_type)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn purge_recycle_bin_entry(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<(), AppError> {
    state
        .recycle_bin_repository
        .purge(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn purge_all_recycle_bin_entries(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    state
        .recycle_bin_repository
        .purge_all()
        .await
        .map_err(Into::into)
}
