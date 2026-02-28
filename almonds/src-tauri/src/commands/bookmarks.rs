use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    adapters::bookmarks::BookmarkTag, entities::bookmark,
    repositories::bookmarks::BookmarkRepositoryExt,
};

use crate::{
    adapters::bookmarks::{CreateBookmark, UpdateBookmark},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_bookmark(
    state: State<'_, AppState>,
    bookmark: CreateBookmark,
) -> Result<bookmark::Model, AppError> {
    state
        .bookmark_repository
        .create(&bookmark.into())
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_bookmark(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_bookmarks(
    state: State<'_, AppState>,
) -> Result<Vec<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .find_all()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_bookmarks_by_tag(
    state: State<'_, AppState>,
    tag: String,
) -> Result<Vec<bookmark::Model>, AppError> {
    let tag = match tag.as_str() {
        "development" => BookmarkTag::Development,
        "inspiration" => BookmarkTag::Inspiration,
        "design" => BookmarkTag::Design,
        _ => BookmarkTag::Research,
    };
    state
        .bookmark_repository
        .find_by_tag(&tag)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_recently_added_bookmarks(
    state: State<'_, AppState>,
) -> Result<Vec<bookmark::Model>, AppError> {
    state
        .bookmark_repository
        .recently_added()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_bookmark(
    state: State<'_, AppState>,
    identifier: Uuid,
    bookmark: UpdateBookmark,
) -> Result<bookmark::Model, AppError> {
    state
        .bookmark_repository
        .update(&identifier, &bookmark.into())
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_bookmark(state: State<'_, AppState>, identifier: Uuid) -> Result<(), AppError> {
    state
        .bookmark_repository
        .delete(&identifier)
        .await
        .map_err(Into::into)
}
