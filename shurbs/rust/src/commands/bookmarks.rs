use almond_kernel::{
    adapters::{
        bookmarks::{BookmarkTag, CreateBookmark, UpdateBookmark},
        meta::RequestMeta,
    },
    repositories::bookmarks::BookmarkRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};

use crate::error::{make_meta, parse_uuid, AppError};
use crate::state::app_state;

fn parse_tag(tag: &str) -> BookmarkTag {
    match tag {
        "development" => BookmarkTag::Development,
        "inspiration" => BookmarkTag::Inspiration,
        "design" => BookmarkTag::Design,
        _ => BookmarkTag::Research,
    }
}

#[flutter_rust_bridge::frb]
pub async fn create_bookmark(
    title: String,
    url: String,
    tag: String,
    workspace_identifier: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let ws_id = workspace_identifier
        .as_deref()
        .map(parse_uuid)
        .transpose()
        .map_err(|e: AppError| e.to_string())?;

    let payload = CreateBookmark { title, url, tag: parse_tag(&tag).to_string(), workspace_identifier: ws_id };
    let bookmark = app_state()
        .bookmarks
        .create(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&bookmark).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_bookmark(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let bookmark = app_state()
        .bookmarks
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    bookmark
        .map(|b| serde_json::to_string(&b).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn get_all_bookmarks(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let bookmarks = app_state()
        .bookmarks
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&bookmarks).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_bookmarks_by_tag(
    tag: String,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let bookmarks = app_state()
        .bookmarks
        .find_by_tag(&parse_tag(&tag), &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&bookmarks).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_recently_added_bookmarks(
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let bookmarks = app_state()
        .bookmarks
        .recently_added(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&bookmarks).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_bookmark(
    identifier: String,
    title: Option<String>,
    url: Option<String>,
    tag: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let payload = UpdateBookmark { title, url, tag: tag.as_deref().map(parse_tag).map(|t| t.to_string()) };

    let bookmark = app_state()
        .bookmarks
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&bookmark).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn delete_bookmark(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state().bookmarks.delete(&id, &meta).await.map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn duplicate_bookmark(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .bookmarks
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_bookmark(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .bookmarks
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
