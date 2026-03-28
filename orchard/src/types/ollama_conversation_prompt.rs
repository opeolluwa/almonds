use chrono::Utc;
use entities::ollama_conversation_prompt::ActiveModel;
use sea_orm::ActiveValue::Set;
use seaography::{
    async_graphql::{self},
    CustomInputType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities;

#[derive(Debug, Serialize, Deserialize, Validate, CustomInputType)]
pub struct CreateOllamaConversationPromptInput {
    pub history_id: Uuid,

    #[validate(length(min = 1, message = "Content must not be empty"))]
    pub content: String,
}

impl Into<ActiveModel> for CreateOllamaConversationPromptInput {
    fn into(self) -> entities::ollama_conversation_prompt::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            history_id: Set(self.history_id),
            content: Set(self.content),
            created_at: Set(Utc::now().fixed_offset()),
        }
    }
}
