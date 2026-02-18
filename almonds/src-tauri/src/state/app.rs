use std::sync::Arc;

use almond_kernel::{
    repositories::prelude::*,
    repositories::{
        notes::NotesRepository, snippets::SnippetRepository, sync_queue::SyncQueueRepository,
        todo::TodoRepository,
    },
    sea_orm::DatabaseConnection,
};

use crate::state::ollama::OllamaState;

pub struct AppState {
    pub notes_repository: NotesRepository,
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
    pub todo_repository: TodoRepository,
    pub ollama: OllamaState,
}

impl AppState {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        let notes_repository = NotesRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_queue_repository = SyncQueueRepository::new(conn.clone());
        let todo_repository = TodoRepository::new(conn.clone());

        let ollama = OllamaState::new(conn.clone());

        AppState {
            notes_repository,
            snippet_repository,
            sync_queue_repository,
            todo_repository,
            ollama,
        }
    }
}
