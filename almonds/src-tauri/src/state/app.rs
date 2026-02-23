use std::sync::Arc;

use almond_kernel::{
    repositories::prelude::*,
    repositories::{
        bookmarks::BookmarkRepository, notes::NotesRepository, recycle_bin::RecycleBinRepository,
        reminder::ReminderRepository, snippets::SnippetRepository, sync_queue::SyncQueueRepository,
        todo::TodoRepository, user_preference::UserPreferenceRepository,
    },
    sea_orm::DatabaseConnection,
};

use crate::state::ollama::OllamaState;

pub struct AppState {
    pub bookmark_repository: BookmarkRepository,
    pub notes_repository: NotesRepository,
    pub recycle_bin_repository: RecycleBinRepository,
    pub reminder_repository: ReminderRepository,
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
    pub todo_repository: TodoRepository,
    pub user_preference_repository: UserPreferenceRepository,
    pub ollama: OllamaState,
}

impl AppState {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        let bookmark_repository = BookmarkRepository::new(conn.clone());
        let notes_repository = NotesRepository::new(conn.clone());
        let recycle_bin_repository = RecycleBinRepository::new(conn.clone());
        let reminder_repository = ReminderRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_queue_repository = SyncQueueRepository::new(conn.clone());
        let todo_repository = TodoRepository::new(conn.clone());
        let user_preference_repository = UserPreferenceRepository::new(conn.clone());
        let ollama = OllamaState::new(conn.clone());

        AppState {
            bookmark_repository,
            notes_repository,
            recycle_bin_repository,
            reminder_repository,
            snippet_repository,
            sync_queue_repository,
            todo_repository,
            user_preference_repository,
            ollama,
        }
    }
}
