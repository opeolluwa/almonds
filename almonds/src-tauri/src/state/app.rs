use std::sync::Arc;

use almond_kernel::{
    repositories::{
        notes::{NotesRepository, NotesRepositoryExt},
        snippets::{SnippetRepository, SnippetRepositoryExt},
        sync_queue::{SyncQueueRepository, SyncQueueRepositoryExt},
    },
    sea_orm::DatabaseConnection,
};

use crate::state::ollama::OllamaState;

pub struct AppState {
    pub notes_repository: NotesRepository,
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
    pub ollama: OllamaState,
}

impl AppState {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        let notes_repository = NotesRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_queue_repository = SyncQueueRepository::new(conn.clone());

        let ollama = OllamaState::new(conn.clone());

        AppState {
            notes_repository,
            snippet_repository,
            sync_queue_repository,
            ollama,
        }
    }
}
