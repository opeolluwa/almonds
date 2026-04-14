use almond_kernel::{
    adapters::meta::RequestMeta,
    entities::todo,
    enums::Priority as TodoPriority,
    repositories::todo::TodoRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::todo::{CreateTodo, UpdateTodo},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_todo(
    state: State<'_, AppState>,
    todo: CreateTodo,
    meta: Option<RequestMeta>,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .create_todo(&todo.into(), &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_todo(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<todo::Model>, AppError> {
    state
        .todo_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_todos(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<todo::Model>, AppError> {
    state
        .todo_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, AppState>,
    identifier: Uuid,
    todo: UpdateTodo,
    meta: Option<RequestMeta>,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .update(&identifier, &todo.into(), &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_todo(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .todo_repository
        .delete(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn mark_todo_done(
    state: State<'_, AppState>,
    identifier: Uuid,
    done: bool,
    meta: Option<RequestMeta>,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .mark_done(&identifier, done, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn change_todo_priority(
    state: State<'_, AppState>,
    identifier: Uuid,
    priority: String,
    meta: Option<RequestMeta>,
) -> Result<todo::Model, AppError> {
    let priority = match priority.as_str() {
        "high" => TodoPriority::High,
        "low" => TodoPriority::Low,
        _ => TodoPriority::Medium,
    };
    state
        .todo_repository
        .change_priority(&identifier, &priority, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_todo_due_date(
    state: State<'_, AppState>,
    identifier: Uuid,
    due_date: Option<String>,
    meta: Option<RequestMeta>,
) -> Result<todo::Model, AppError> {
    use chrono::NaiveDate;

    let date = due_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    state
        .todo_repository
        .update_due_date(&identifier, date, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn transfer_todo(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .todo_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn duplicate_todo(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .todo_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(Into::into)
}
