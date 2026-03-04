use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    adapters::meta::RequestMeta, entities::snippets, repositories::snippets::SnippetRepositoryExt,
};

use crate::{
    adapters::snippets::{CreateSnippet, UpdateSnippet},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_snippet(
    state: State<'_, AppState>,
    snippet: CreateSnippet,
    meta: Option<RequestMeta>,
) -> Result<snippets::Model, AppError> {
    let created = state
        .snippet_repository
        .create(&snippet.into(), &meta)
        .await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_snippets(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state.snippet_repository.delete(&identifier, &meta).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    snippet: UpdateSnippet,
    meta: Option<RequestMeta>,
) -> Result<snippets::Model, AppError> {
    let updated = state
        .snippet_repository
        .update(&identifier, &snippet.into(), &meta)
        .await?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_snippet(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<snippets::Model>, AppError> {
    let snippets = state.snippet_repository.recently_added(&meta).await?;
    Ok(snippets)
}
