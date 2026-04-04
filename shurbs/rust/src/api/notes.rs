use almond_kernel::{
    adapters::notes::{CreateNote, UpdateNote},
    repositories::notes::NotesRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};

use crate::error::{make_meta, parse_uuid, AppError};
use crate::state::app_state;

// ── Create ────────────────────────────────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub async fn create_note(
    title: String,
    content: String,
    categories: Option<Vec<String>>,
    workspace_identifier: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let ws_id = workspace_identifier
        .as_deref()
        .map(parse_uuid)
        .transpose()
        .map_err(|e: AppError| e.to_string())?;

    let payload = CreateNote {
        title,
        content,
        categories,
        workspace_identifier: ws_id,
    };
    let note = app_state()
        .notes
        .create(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&note).map_err(|e| e.to_string())
}

// ── Read ──────────────────────────────────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub async fn get_note(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let note = app_state()
        .notes
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    note.map(|n| serde_json::to_string(&n).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn get_all_notes(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let notes = app_state()
        .notes
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&notes).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_recently_added_notes(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let notes = app_state()
        .notes
        .recently_added(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&notes).map_err(|e| e.to_string())
}

// ── Update ────────────────────────────────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub async fn update_note(
    identifier: String,
    title: Option<String>,
    content: Option<String>,
    categories: Option<Vec<String>>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let payload = UpdateNote {
        title,
        content,
        categories,
    };

    let note = app_state()
        .notes
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&note).map_err(|e| e.to_string())
}

// ── Delete ────────────────────────────────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub async fn delete_note(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .notes
        .delete(&id, &meta)
        .await
        .map_err(|e| e.to_string())
}

// ── Workspace operations ──────────────────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub async fn duplicate_note(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .notes
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_note(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .notes
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
