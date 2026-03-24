use std::sync::Arc;

use almond_kernel::{
    repositories::ollama_conversation::{
        OllamaConversationRepository, OllamaConversationRepositoryExt,
    },
    sea_orm::DatabaseConnection,
};
use ollama_rs::Ollama;

#[allow(dead_code)]
pub struct OllamaState {
    pub ollama_repository: OllamaConversationRepository,
    pub ollama_client: Option<Ollama>,
    pub ollama_models: Vec<String>,
    pub ollama_active_model: Option<String>,
}

impl OllamaState {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Self {
        let ollama = Ollama::default();

        let (client, models, active_model) = match ollama.list_local_models().await {
            Ok(local_models) => {
                let models: Vec<String> = local_models.into_iter().map(|m| m.name).collect();

                let active = models.first().cloned();

                (Some(ollama), models, active)
            }
            Err(e) => {
                log::warn!("Ollama not available: {:?}", e);

                (None, vec![], None)
            }
        };

        OllamaState {
            ollama_repository: OllamaConversationRepository::new(conn),
            ollama_client: client,
            ollama_models: models,
            ollama_active_model: active_model,
        }
    }
}
