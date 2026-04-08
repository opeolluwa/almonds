use almond_kernel::{
    adapters::recycle_bin::{CreateRecycleBinEntry},
    repositories::recycle_bin::RecycleBinRepositoryExt,
};

use almond_kernel::enums::ItemType as RecycleBinItemType;

use crate::error::{make_meta, parse_uuid};
use crate::state::app_state;

fn parse_item_type(s: &str) -> Result<RecycleBinItemType, String> {
    match s {
        "todo" => Ok(RecycleBinItemType::Todo),
        "note" => Ok(RecycleBinItemType::Note),
        "reminder" => Ok(RecycleBinItemType::Reminder),
        "snippet" => Ok(RecycleBinItemType::Snippet),
        "bookmark" => Ok(RecycleBinItemType::Bookmark),
       // "workspace" => Ok(RecycleBinItemType::Workspace),
        other => Err(format!("unknown recycle bin item type: '{other}'")),
    }
}

/// `payload` — JSON-serialized copy of the deleted record.
#[flutter_rust_bridge::frb]
pub async fn create_recycle_bin_entry(
    item_id: String,
    item_type: String,
    payload: String,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let entry = CreateRecycleBinEntry {
        item_id: parse_uuid(&item_id).map_err(|e| e.to_string())?,
        item_type: parse_item_type(&item_type)?,
        payload: serde_json::from_str(&payload)
            .map_err(|e| format!("invalid payload JSON: {e}"))?,
        workspace_identifier: None,
    };

    let result = app_state()
        .recycle_bin
        .store(&entry, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_all_recycle_bin_entries(
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let entries = app_state()
        .recycle_bin
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&entries).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_recycle_bin_entry(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let entry = app_state()
        .recycle_bin
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    entry
        .map(|e| serde_json::to_string(&e).map_err(|e| e.to_string()))
        .transpose()
}

/// `item_type` — one of: `"todo"`, `"note"`, `"reminder"`, `"snippet"`,
/// `"bookmark"`, `"workspace"`.
#[flutter_rust_bridge::frb]
pub async fn get_recycle_bin_entries_by_type(
    item_type: String,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let kind = parse_item_type(&item_type)?;

    let entries = app_state()
        .recycle_bin
        .find_by_item_type(&kind, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&entries).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn purge_recycle_bin_entry(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .recycle_bin
        .purge(&id, &meta)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn purge_all_recycle_bin_entries(
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .recycle_bin
        .purge_all(&meta)
        .await
        .map_err(|e| e.to_string())
}
