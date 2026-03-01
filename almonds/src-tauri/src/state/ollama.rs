use std::sync::Arc;

use almond_kernel::{
    repositories::ollama_conversation::{
        OllamaConversationRepository, OllamaConversationRepositoryExt,
    },
    sea_orm::DatabaseConnection,
};

#[allow(dead_code)]
pub struct OllamaState {
    pub ollama_repository: OllamaConversationRepository,
}

impl OllamaState {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        OllamaState {
            ollama_repository: OllamaConversationRepository::new(conn),
        }
    }
}
