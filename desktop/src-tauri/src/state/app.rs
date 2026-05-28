use std::sync::Arc;

use almond_kernel::{
    repositories::{
        bookmarks::BookmarkRepository, notes::NotesRepository, prelude::*,
        recycle_bin::RecycleBinRepository, reminder::ReminderRepository,
        snippets::SnippetRepository, sync_queue::SyncQueueRepository, todo::TodoRepository,
        user_preferences::UserPreferencesRepository,
        user_preferences::UserPreferencesRepositoryExt, workspace::WorkspaceRepository,
        workspace_preferences::WorkspacePreferenceRepository,
    },
    sea_orm::DatabaseConnection,
};

#[allow(unused)]
pub struct AppState {
    pub bookmark_repository: BookmarkRepository,
    pub notes_repository: NotesRepository,
    pub recycle_bin_repository: RecycleBinRepository,
    pub reminder_repository: ReminderRepository,
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
    pub todo_repository: TodoRepository,
    pub user_preference_repository: UserPreferencesRepository,
    pub workspace_preference_repository: WorkspacePreferenceRepository,
    pub workspace_repository: WorkspaceRepository,
}

impl AppState {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Self {
        let bookmark_repository = BookmarkRepository::new(conn.clone());
        let notes_repository = NotesRepository::new(conn.clone());
        let recycle_bin_repository = RecycleBinRepository::new(conn.clone());
        let reminder_repository = ReminderRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_queue_repository = SyncQueueRepository::new(conn.clone());
        let todo_repository = TodoRepository::new(conn.clone());
        let user_preference_repository = UserPreferencesRepository::new(conn.clone());
        let workspace_repository = WorkspaceRepository::new(conn.clone());
        let workspace_preference_repository = WorkspacePreferenceRepository::new(conn.clone());

        AppState {
            bookmark_repository,
            notes_repository,
            recycle_bin_repository,
            reminder_repository,
            snippet_repository,
            sync_queue_repository,
            todo_repository,
            user_preference_repository,
            workspace_repository,
            workspace_preference_repository,
        }
    }
}
