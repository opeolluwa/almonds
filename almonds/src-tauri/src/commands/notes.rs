use tauri::State;
use uuid::Uuid;

use almond_kernel::{entities::notes, repositories::notes::NotesRepositoryExt};

use crate::{
    adapters::notes::{CreateNote, UpdateNote},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_note(
    state: State<'_, AppState>,
    note: CreateNote,
) -> Result<notes::Model, AppError> {
    let created = state.notes_repository.create(&note.into()).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_note(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<notes::Model>, AppError> {
    state
        .notes_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_notes(state: State<'_, AppState>) -> Result<Vec<notes::Model>, AppError> {
    state.notes_repository.find_all().await.map_err(Into::into)
}

#[tauri::command]
pub async fn delete_note(state: State<'_, AppState>, identifier: Uuid) -> Result<(), AppError> {
    state.notes_repository.delete(&identifier).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    note: UpdateNote,
) -> Result<notes::Model, AppError> {
    let updated = state
        .notes_repository
        .update(&identifier, &note.into())
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_notes(
    state: State<'_, AppState>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .recently_added()
        .await
        .map_err(Into::into)
}
