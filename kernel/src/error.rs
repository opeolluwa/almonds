use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum KernelError {
    #[error("failed to connect to database due to {0}")]
    DbConnectError(String),

    #[error("database operation failed: {0}")]
    DbOperationError(String),

    #[error("Failed to extract env: {0}")]
    EnvError(String),

    #[error("Workspace does not exist: {0}")]
    WorkspaceNotFound(String),

    #[error("Bookmark does not exist: {0}")]
    BookmarkNotFound(String),

    #[error("Note does not exist: {0}")]
    NotesNotFound(String),

    #[error("Todo does not exist: {0}")]
    TodoNotFound(String),

    #[error("Snippet does not exist: {0}")]
    SnippetNotFound(String),

    #[error("Reminder does not exists ")]
    ReminderNotFound(String),
}
