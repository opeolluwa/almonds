use almond_kernel::{
    adapters::todo::{CreateTodo, TodoPriority, UpdateTodo},
    repositories::todo::TodoRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use chrono::NaiveDate;

use crate::error::{make_meta, parse_uuid, AppError};
use crate::state::app_state;

fn parse_priority(p: &str) -> TodoPriority {
    match p {
        "high" => TodoPriority::High,
        "low" => TodoPriority::Low,
        _ => TodoPriority::Medium,
    }
}

#[flutter_rust_bridge::frb]
pub async fn create_todo(
    title: String,
    description: Option<String>,
    priority: String,
    due_date: Option<String>,
    workspace_identifier: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let ws_id = workspace_identifier
        .as_deref()
        .map(parse_uuid)
        .transpose()
        .map_err(|e: AppError| e.to_string())?;

    let date = due_date
        .as_deref()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| e.to_string()))
        .transpose()?;

    let payload = CreateTodo {
        title,
        description,
        priority: parse_priority(&priority),
        due_date: date,
    };

    let todo = app_state()
        .todos
        .create_todo(&payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todo).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn get_todo(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<Option<String>, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let todo = app_state()
        .todos
        .find_by_id(&id, &meta)
        .await
        .map_err(|e| e.to_string())?;

    todo.map(|t| serde_json::to_string(&t).map_err(|e| e.to_string()))
        .transpose()
}

#[flutter_rust_bridge::frb]
pub async fn get_all_todos(meta_workspace_id: Option<String>) -> Result<String, String> {
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    let todos = app_state()
        .todos
        .find_all(&meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todos).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn update_todo(
    identifier: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    due_date: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let date = due_date
        .as_deref()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| e.to_string()))
        .transpose()?;

    let payload = UpdateTodo {
        title,
        description,
    };

    let todo = app_state()
        .todos
        .update(&id, &payload, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todo).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn delete_todo(
    identifier: String,
    meta_workspace_id: Option<String>,
) -> Result<(), String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;
    app_state()
        .todos
        .delete(&id, &meta)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn mark_todo_done(
    identifier: String,
    done: bool,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let todo = app_state()
        .todos
        .mark_done(&id, done, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todo).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn change_todo_priority(
    identifier: String,
    priority: String,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let todo = app_state()
        .todos
        .change_priority(&id, &parse_priority(&priority), &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todo).map_err(|e| e.to_string())
}

/// `due_date` — ISO 8601 date string (`"YYYY-MM-DD"`), or `None` to clear.
#[flutter_rust_bridge::frb]
pub async fn update_todo_due_date(
    identifier: String,
    due_date: Option<String>,
    meta_workspace_id: Option<String>,
) -> Result<String, String> {
    let id = parse_uuid(&identifier).map_err(|e| e.to_string())?;
    let meta = make_meta(meta_workspace_id).map_err(|e| e.to_string())?;

    let date = due_date
        .as_deref()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| e.to_string()))
        .transpose()?;

    let todo = app_state()
        .todos
        .update_due_date(&id, date, &meta)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&todo).map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn duplicate_todo(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .todos
        .duplicate_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}

#[flutter_rust_bridge::frb]
pub async fn transfer_todo(
    record_identifier: String,
    previous_workspace_identifier: String,
    target_workspace_identifier: String,
) -> Result<(), String> {
    let record = parse_uuid(&record_identifier).map_err(|e| e.to_string())?;
    let prev = parse_uuid(&previous_workspace_identifier).map_err(|e| e.to_string())?;
    let target = parse_uuid(&target_workspace_identifier).map_err(|e| e.to_string())?;

    app_state()
        .todos
        .transfer_record(&record, &prev, &target)
        .await
        .map_err(|e| e.to_string())
}
