use tauri::State;
use uuid::Uuid;

use almond_kernel::{
    adapters::todo::TodoPriority, entities::todo, repositories::todo::TodoRepositoryExt,
};

use crate::{
    adapters::todo::{CreateTodo, UpdateTodo},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn create_todo(
    state: State<'_, AppState>,
    todo: CreateTodo,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .create_todo(&todo.into())
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_todo(
    state: State<'_, AppState>,
    identifier: Uuid,
) -> Result<Option<todo::Model>, AppError> {
    state
        .todo_repository
        .find_by_id(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_todos(state: State<'_, AppState>) -> Result<Vec<todo::Model>, AppError> {
    state.todo_repository.find_all().await.map_err(Into::into)
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, AppState>,
    identifier: Uuid,
    todo: UpdateTodo,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .update(&identifier, &todo.into())
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_todo(state: State<'_, AppState>, identifier: Uuid) -> Result<(), AppError> {
    state
        .todo_repository
        .delete(&identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn mark_todo_done(
    state: State<'_, AppState>,
    identifier: Uuid,
    done: bool,
) -> Result<todo::Model, AppError> {
    state
        .todo_repository
        .mark_done(&identifier, done)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn change_todo_priority(
    state: State<'_, AppState>,
    identifier: Uuid,
    priority: String,
) -> Result<todo::Model, AppError> {
    let priority = match priority.as_str() {
        "high" => TodoPriority::High,
        "low" => TodoPriority::Low,
        _ => TodoPriority::Medium,
    };
    state
        .todo_repository
        .change_priority(&identifier, &priority)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_todo_due_date(
    state: State<'_, AppState>,
    identifier: Uuid,
    due_date: Option<String>,
) -> Result<todo::Model, AppError> {
    use chrono::NaiveDate;

    let date = due_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    state
        .todo_repository
        .update_due_date(&identifier, date)
        .await
        .map_err(Into::into)
}
