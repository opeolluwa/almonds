use std::sync::Arc;

use almond_kernel::{
    kernel::DataEngine,
    repositories::{
        bookmarks::BookmarkRepository, notes::NotesRepository, prelude::*,
        recycle_bin::RecycleBinRepository, reminder::ReminderRepository,
        snippets::SnippetRepository, sync_queue::SyncQueueRepository, todo::TodoRepository,
        user_preference::UserPreferenceRepository, workspace::WorkspaceRepository,
    },
    sea_orm::DatabaseConnection,
};
use once_cell::sync::OnceCell;

// ── Global singleton ──────────────────────────────────────────────────────────

static APP_STATE: OnceCell<AppState> = OnceCell::new();

pub fn app_state() -> &'static AppState {
    APP_STATE
        .get()
        .expect("kernel not initialized — call init_kernel() first")
}

// ── Initialiser (called from Flutter on app start) ────────────────────────────

/// Connect to the database, run migrations, and initialise all repositories.
/// `database_url` examples:
///   - SQLite : `"sqlite:///data/user/0/app.shurbs/databases/shurbs.db?mode=rwc"`
///   - In-memory: `"sqlite::memory:"`
pub async fn init_data_engine(database_url: String) -> Result<(), String> {
    if APP_STATE.get().is_some() {
        return Ok(()); // already initialised — idempotent
    }

    let data_engine = DataEngine::new(&database_url)
        .await
        .map_err(|e| e.to_string())?;

    data_engine.run_migrations().await.map_err(|e| e.to_string())?;

    let conn = Arc::new(data_engine.connection().clone());
    let state = AppState::new(conn).await;

    APP_STATE.set(state).ok();
    Ok(())
}

// ── App state ─────────────────────────────────────────────────────────────────

pub struct AppState {
    pub notes: NotesRepository,
    pub bookmarks: BookmarkRepository,
    pub todos: TodoRepository,
    pub reminders: ReminderRepository,
    pub snippets: SnippetRepository,
    pub recycle_bin: RecycleBinRepository,
    pub workspaces: WorkspaceRepository,
    pub user_preferences: UserPreferenceRepository,
    pub sync_queue: SyncQueueRepository,
}

impl AppState {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Self {
        AppState {
            notes: NotesRepository::new(conn.clone()),
            bookmarks: BookmarkRepository::new(conn.clone()),
            todos: TodoRepository::new(conn.clone()),
            reminders: ReminderRepository::new(conn.clone()),
            snippets: SnippetRepository::new(conn.clone()),
            recycle_bin: RecycleBinRepository::new(conn.clone()),
            workspaces: WorkspaceRepository::new(conn.clone()),
            user_preferences: UserPreferenceRepository::new(conn.clone()),
            sync_queue: SyncQueueRepository::new(conn.clone()),
        }
    }
}
