use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    entities::snippets,
    repositories::snippets::{SnippetRepository, SnippetRepositoryExt},
};

use crate::{adapters::snippets::CreateSnippet, errors::AppError, state::AppState};

#[tauri::command]
pub async fn create_snippet(
    state: State<'_, AppState>,
    snippet: CreateSnippet,
) -> Result<snippets::Model, AppError> {
    let repo = SnippetRepository::new(state.conn.clone());
    let created = repo.create(&snippet.into()).await?;
    Ok(created)
}

#[tauri::command]
pub async fn get_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<snippets::Model>, AppError> {
    let repo = SnippetRepository::new(state.conn.clone());
    repo.find_by_id(&identifier).await.map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_snippets(
    state: State<'_, AppState>,
) -> Result<Vec<snippets::Model>, AppError> {
    let repo = SnippetRepository::new(state.conn.clone());
    repo.find_all().await.map_err(Into::into)
}

#[tauri::command]
pub async fn delete_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<(), AppError> {
    let repo = SnippetRepository::new(state.conn.clone());
    repo.delete(&identifier).await?;
    Ok(())
}
