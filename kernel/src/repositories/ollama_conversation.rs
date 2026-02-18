use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use uuid::Uuid;

use crate::{
    adapters::ollama_conversations::{
        CreateConversationHistory, CreateConversationPrompt, CreateConversationResponse,
        UpdateConversationHistory,
    },
    entities::{ollama_conversation_history, ollama_conversation_prompt, ollama_conversation_response},
    error::KernelError,
};

pub struct OllamaConversationRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait OllamaConversationRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create_history(
        &self,
        payload: &CreateConversationHistory,
    ) -> Result<ollama_conversation_history::Model, KernelError>;

    async fn find_history_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<ollama_conversation_history::Model>, KernelError>;

    async fn find_all_histories(
        &self,
    ) -> Result<Vec<ollama_conversation_history::Model>, KernelError>;

    async fn update_history(
        &self,
        id: &Uuid,
        payload: &UpdateConversationHistory,
    ) -> Result<ollama_conversation_history::Model, KernelError>;

    async fn delete_history(&self, id: &Uuid) -> Result<(), KernelError>;

    async fn add_prompt(
        &self,
        payload: &CreateConversationPrompt,
    ) -> Result<ollama_conversation_prompt::Model, KernelError>;

    async fn add_response(
        &self,
        payload: &CreateConversationResponse,
    ) -> Result<ollama_conversation_response::Model, KernelError>;
}

#[async_trait]
impl OllamaConversationRepositoryExt for OllamaConversationRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn create_history(
        &self,
        payload: &CreateConversationHistory,
    ) -> Result<ollama_conversation_history::Model, KernelError> {
        let active_model: ollama_conversation_history::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_history_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<ollama_conversation_history::Model>, KernelError> {
        ollama_conversation_history::Entity::find()
            .filter(ollama_conversation_history::Column::Identifier.eq(*id))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn find_all_histories(
        &self,
    ) -> Result<Vec<ollama_conversation_history::Model>, KernelError> {
        ollama_conversation_history::Entity::find()
            .all(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn update_history(
        &self,
        id: &Uuid,
        payload: &UpdateConversationHistory,
    ) -> Result<ollama_conversation_history::Model, KernelError> {
        let model = ollama_conversation_history::Entity::find()
            .filter(ollama_conversation_history::Column::Identifier.eq(*id))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?
            .ok_or_else(|| KernelError::DbOperationError("conversation history not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(bookmarked) = payload.bookmarked {
            active_model.bookmarked = Set(bookmarked);
        }

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn delete_history(&self, id: &Uuid) -> Result<(), KernelError> {
        ollama_conversation_history::Entity::delete_many()
            .filter(ollama_conversation_history::Column::Identifier.eq(*id))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn add_prompt(
        &self,
        payload: &CreateConversationPrompt,
    ) -> Result<ollama_conversation_prompt::Model, KernelError> {
        let active_model: ollama_conversation_prompt::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }

    async fn add_response(
        &self,
        payload: &CreateConversationResponse,
    ) -> Result<ollama_conversation_response::Model, KernelError> {
        let active_model: ollama_conversation_response::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| KernelError::DbOperationError(err.to_string()))
    }
}
