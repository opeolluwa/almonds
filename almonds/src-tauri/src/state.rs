use std::sync::Arc;

use almond_kernel::{
    repositories::{snippets::SnippetRepository, sync_queue::SyncQueueRepository},
    sea_orm::DatabaseConnection,
};

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
}
