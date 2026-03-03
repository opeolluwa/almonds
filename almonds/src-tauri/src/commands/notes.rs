use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::notes::{CreateNote, UpdateNote},
    errors::AppError,
    state::app::AppState,
};
use almond_kernel::{
    adapters::meta::RequestMeta, entities::notes, repositories::notes::NotesRepositoryExt,
};

#[tauri::command]
pub async fn create_note(
    state: State<'_, AppState>,
    note: CreateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let created = state.notes_repository.create(&note.into(), &meta).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<notes::Model>, AppError> {
    state
        .notes_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state.notes_repository.delete(&identifier, &meta).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    note: UpdateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let updated = state
        .notes_repository
        .update(&identifier, &note.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .recently_added(&meta)
        .await
        .map_err(Into::into)
}
