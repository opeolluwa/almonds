use almond_kernel::{
    adapters::workspace::CreateWorkspace, repositories::workspace::WorkspaceRepositoryExt,
};
use tauri::State;

use crate::{errors::AppError, state::app::AppState};

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, AppState>,
    workspace: CreateWorkspace,
) -> Result<(), AppError> {
    state
        .workspace_repository
        .create_workspace(workspace)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn list_workspaces(
    state: State<'_, AppState>,
) -> Result<Vec<almond_kernel::entities::workspaces::Model>, AppError> {
    let workspaces = state.workspace_repository.list_workspaces().await?;
    Ok(workspaces)
}

#[tauri::command]
pub async fn get_workspace_by_id(
    state: State<'_, AppState>,
    id: String,
) -> Result<almond_kernel::entities::workspaces::Model, AppError> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| AppError::Io(format!("Invalid UUID string: {}", id)))?;
    let workspace = state.workspace_repository.get_workspace_by_id(uuid).await?;
    Ok(workspace)
}
