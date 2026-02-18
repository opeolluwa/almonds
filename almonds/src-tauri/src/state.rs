use almond_kernel::repositories::{snippets::SnippetRepository, sync_queue::SyncQueueRepository};

pub struct AppState {
    pub snippet_repository: SnippetRepository,
    pub sync_queue_repository: SyncQueueRepository,
}
