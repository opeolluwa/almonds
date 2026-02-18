use tauri::State;
use uuid::Uuid;

use almond_kernel::{entities::snippets, repositories::snippets::SnippetRepositoryExt};

use crate::{adapters::snippets::CreateSnippet, errors::AppError, state::AppState};

#[tauri::command]
pub async fn create_snippet(
    state: State<'_, AppState>,
    snippet: CreateSnippet,
) -> Result<snippets::Model, AppError> {
    let created = state.snippet_repository.create(&snippet.into()).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_snippets(
    state: State<'_, AppState>,
) -> Result<Vec<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_all()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_snippet(state: State<'_, AppState>, identifier: Uuid) -> Result<(), AppError> {
    state.snippet_repository.delete(&identifier).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_recently_added_snippet(
    state: State<'_, AppState>,
) -> Result<Vec<snippets::Model>, AppError> {
    let snippets = state.snippet_repository.recently_added().await?;
    Ok(snippets)
}
