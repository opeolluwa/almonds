use std::sync::{Arc, OnceLock};
use std::time::Duration;

use almond_kernel::{
    adapters::{
        meta::RequestMeta,
        todo::{CreateTodo, TodoPriority},
        workspace::CreateWorkspace,
    },
    migration::{Migrator, MigratorTrait},
    repositories::{
        todo::{TodoRepository, TodoRepositoryExt},
        workspace::{WorkspaceRepository, WorkspaceRepositoryExt},
    },
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use uuid::Uuid;

static DB: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();
static WORKSPACE_ID: OnceLock<Uuid> = OnceLock::new();

pub struct TodoItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub done: bool,
}

fn db() -> Arc<DatabaseConnection> {
    DB.get().expect("kernel not initialized — call initKernel first").clone()
}

fn meta() -> Option<RequestMeta> {
    Some(RequestMeta {
        workspace_identifier: *WORKSPACE_ID.get().expect("workspace not set"),
    })
}

/// Initialise the SQLite database at `db_path` (absolute filesystem path),
/// run any pending migrations, and ensure a default workspace exists.
/// Safe to call multiple times — subsequent calls are no-ops.
pub async fn init_kernel(db_path: String) -> Result<(), String> {
    if DB.get().is_some() {
        return Ok(());
    }

    let db_url = format!("sqlite://{}?mode=rwc", db_path);
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    let conn = Database::connect(opt)
        .await
        .map_err(|e| format!("db connect: {e}"))?;

    Migrator::up(&conn, None)
        .await
        .map_err(|e| format!("migration: {e}"))?;

    let conn = Arc::new(conn);
    let workspace_repo = WorkspaceRepository::new(conn.clone());

    let workspaces = workspace_repo
        .list_workspaces()
        .await
        .map_err(|e| format!("list workspaces: {e}"))?;

    let workspace_id = if workspaces.is_empty() {
        let ws = workspace_repo
            .create_workspace(CreateWorkspace {
                name: "Default".into(),
                description: "Default workspace".into(),
            })
            .await
            .map_err(|e| format!("create workspace: {e}"))?;
        ws.identifier
    } else {
        workspaces[0].identifier
    };

    // Ignore errors — another isolate may have raced us.
    let _ = DB.set(conn);
    let _ = WORKSPACE_ID.set(workspace_id);

    Ok(())
}

pub async fn create_todo(
    title: String,
    description: Option<String>,
    priority: String,
) -> Result<TodoItem, String> {
    let prio = match priority.as_str() {
        "high" => TodoPriority::High,
        "low" => TodoPriority::Low,
        _ => TodoPriority::Medium,
    };

    let model = TodoRepository::new(db())
        .create_todo(
            &CreateTodo {
                title,
                description,
                due_date: None,
                priority: prio,
            },
            &meta(),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(TodoItem {
        id: model.identifier.to_string(),
        title: model.title,
        description: model.description,
        priority: model.priority,
        done: model.done,
    })
}

pub async fn list_todos() -> Result<Vec<TodoItem>, String> {
    let models = TodoRepository::new(db())
        .find_all(&meta())
        .await
        .map_err(|e| e.to_string())?;

    Ok(models
        .into_iter()
        .map(|m| TodoItem {
            id: m.identifier.to_string(),
            title: m.title,
            description: m.description,
            priority: m.priority,
            done: m.done,
        })
        .collect())
}

pub async fn mark_todo_done(id: String, done: bool) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    TodoRepository::new(db())
        .mark_done(&uuid, done, &meta())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_todo(id: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    TodoRepository::new(db())
        .delete(&uuid, &meta())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
