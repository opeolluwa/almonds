use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    adapters::{meta::RequestMeta, recycle_bin::RecycleBinItemType},
    entities::recycle_bin,
    repositories::recycle_bin::RecycleBinRepositoryExt,
};

use crate::{adapters::recycle_bin::CreateRecycleBinEntry, errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn create_recycle_bin_entry(
    state: State<'_, AppState>,
    entry: CreateRecycleBinEntry,
    meta: Option<RequestMeta>,
) -> Result<recycle_bin::Model, AppError> {
    state
        .recycle_bin_repository
        .store(&entry.into(), &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_recycle_bin_entries(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recycle_bin_entry(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recycle_bin_entries_by_type(
    state: State<'_, AppState>,
    item_type: RecycleBinItemType,
    meta: Option<RequestMeta>,
) -> Result<Vec<recycle_bin::Model>, AppError> {
    state
        .recycle_bin_repository
        .find_by_item_type(&item_type, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn purge_recycle_bin_entry(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .recycle_bin_repository
        .purge(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn purge_all_recycle_bin_entries(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .recycle_bin_repository
        .purge_all(&meta)
        .await
        .map_err(Into::into)
}
