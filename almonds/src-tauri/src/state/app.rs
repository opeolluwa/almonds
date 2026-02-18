use crate::state::ollama::OllamaState;
use almond_kernel::{
    repositories::{
        snippets::{SnippetRepository, SnippetRepositoryExt},
        sync_queue::{SyncQueueRepository, SyncQueueRepositoryExt},
    },
    sea_orm::DatabaseConnection,
};
use std::sync::Arc;

pub struct AppState {
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
    pub ollama: OllamaState,
}

impl AppState {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_queue_repository = SyncQueueRepository::new(conn.clone());

        let ollama = OllamaState::new(conn.clone());

        AppState {
            snippet_repository,
            sync_queue_repository,
            ollama,
        }
    }
}
