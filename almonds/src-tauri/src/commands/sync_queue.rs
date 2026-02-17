use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    entities::sync_queue,
    repositories::sync_queue::{SyncQueueRepository, SyncQueueRepositoryExt},
};

use crate::{adapters::sync_queue::AddSyncQueueEntry, errors::AppError, state::AppState};

#[tauri::command]
pub async fn add_sync_queue_entry(
    state: State<'_, AppState>,
    entry: AddSyncQueueEntry,
) -> Result<(), AppError> {
    let repo = SyncQueueRepository::new(state.conn.clone());
    repo.push(&entry.into()).await?;
    Ok(())
}

#[tauri::command]
pub async fn remove_sync_queue_entry(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<(), AppError> {
    let repo = SyncQueueRepository::new(state.conn.clone());
    repo.pop(&identifier).await?;
    Ok(())
}

#[tauri::command]
pub async fn count_sync_queue_entries(state: State<'_, AppState>) -> Result<i32, AppError> {
    let repo = SyncQueueRepository::new(state.conn.clone());
    repo.len().await.map_err(Into::into)
}

#[tauri::command]
pub async fn run_sync(state: State<'_, AppState>) -> Result<Vec<sync_queue::Model>, AppError> {
    let repo = SyncQueueRepository::new(state.conn.clone());
    let entries = repo.entries().await?;
    Ok(entries)
}
