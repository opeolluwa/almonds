use almond_kernel::{
    adapters::workspace::{CreateWorkspace, UpdateWorkspace},
    repositories::workspace::WorkspaceRepositoryExt,
};

use crate::error::{make_meta, parse_uuid};
use crate::state::app_state;

#[flutter_rust_bridge::frb]
pub async fn create_workspace(name: String, description: String) -> Result<String, String> {
    let payload = CreateWorkspace { name, description };
    let workspace = app_state()
        .workspaces
        .create_workspace(payload)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&workspace).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn list_workspaces() -> Result<String, String> {
    let workspaces = app_state()
        .workspaces
        .list_workspaces()
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&workspaces).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_workspace_by_id(id: String) -> Result<String, String> {
    let uuid = parse_uuid(&id).map_err(|e| e.to_string())?;
    let workspace = app_state()
        .workspaces
        .get_workspace_by_id(uuid)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&workspace).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_workspace(
    identifier: String,
    name: Option<String>,
    description: Option<String>,
    is_default: Option<bool>,
    is_hidden: Option<bool>,
) -> Result<String, String> {
    let uuid = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let payload = UpdateWorkspace {
        name,
        description,
        is_default,
        is_hidden,
    };

    let workspace = app_state()
        .workspaces
        .update_workspace(&uuid, payload)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&workspace).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn delete_workspace(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let uuid = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .workspaces
        .delete_workspace(&uuid, &meta)
        .await
        .map_err(|e| e.to_string())
}
