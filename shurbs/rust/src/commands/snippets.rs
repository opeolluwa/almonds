use almond_kernel::{
    adapters::snippets::{CreateSnippet, UpdateSnippet},
    repositories::snippets::SnippetRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use chrono::DateTime;

use crate::error::{make_meta, parse_uuid, AppError};
use crate::state::app_state;

#[flutter_rust_bridge::frb]
pub async fn create_snippet(
    code: String,
    title: Option<String>,
    language: Option<String>,
    description: Option<String>,
    is_pinned: bool,
    workspace_identifier: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let ws_id = workspace_identifier
        .as_deref()
        .map(parse_uuid)
        .transpose()
        .map_err(|e: AppError| e.to_string())?;

    let now = DateTime::parse_from_rfc3339(&chrono::Utc::now().to_rfc3339()).unwrap();
    let payload = CreateSnippet {
        title,
        language,
        code,
        description,
        is_pinned,
        created_at: now,
        updated_at: now,
        workspace_identifier: ws_id,
    };

    let snippet = app_state()
        .snippets
        .create(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&snippet).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_snippet(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let snippet = app_state()
        .snippets
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    snippet
        .map(|s| serde_json::to_string(&s).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn get_all_snippets(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let snippets = app_state()
        .snippets
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&snippets).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_recently_added_snippets(
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let snippets = app_state()
        .snippets
        .recently_added(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&snippets).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_snippet(
    identifier: String,
    title: Option<String>,
    language: Option<String>,
    code: Option<String>,
    description: Option<String>,
    is_pinned: Option<bool>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let payload = UpdateSnippet { title, language, code, description, is_pinned };

    let snippet = app_state()
        .snippets
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&snippet).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn delete_snippet(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state().snippets.delete(&id, &meta).await.map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn duplicate_snippet(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .snippets
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_snippet(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .snippets
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
