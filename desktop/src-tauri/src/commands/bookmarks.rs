use almond_kernel::{
    adapters::meta::RequestMeta,
    entities::bookmark,
    enums::Tag as BookmarkTag,
    repositories::bookmarks::BookmarkRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::bookmarks::{CreateBookmark, UpdateBookmark},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_bookmark(
    state: State<'_, AppState>,
    bookmark: CreateBookmark,
    meta: Option<RequestMeta>,
) -> Result<bookmark::Model, AppError> {
    state
        .bookmark_repository
        .create(&bookmark.into(), &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_bookmark(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_bookmarks(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_bookmarks_by_tag(
    state: State<'_, AppState>,
    tag: String,
    meta: Option<RequestMeta>,
) -> Result<Vec<bookmark::Model>, AppError> {
    let tag = match tag.as_str() {
        "development" => BookmarkTag::Development,
        "inspiration" => BookmarkTag::Inspiration,
        "design" => BookmarkTag::Design,
        _ => BookmarkTag::Research,
    };
    state
        .bookmark_repository
        .find_by_tag(&tag, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recently_added_bookmarks(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .recently_added(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_bookmark(
    state: State<'_, AppState>,
    identifier: Uuid,
    bookmark: UpdateBookmark,
    meta: Option<RequestMeta>,
) -> Result<bookmark::Model, AppError> {
    state
        .bookmark_repository
        .update(&identifier, &bookmark.into(), &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_bookmark(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .bookmark_repository
        .delete(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn duplicate_bookmark(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .bookmark_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn transfer_bookmark(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .bookmark_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}
