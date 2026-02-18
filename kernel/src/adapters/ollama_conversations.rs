use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{
    self, ollama_conversation_history::ActiveModel as HistoryActiveModel,
    ollama_conversation_prompt::ActiveModel as PromptActiveModel,
    ollama_conversation_response::ActiveModel as ResponseActiveModel,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateConversationHistory {
    pub bookmarked: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl Into<entities::ollama_conversation_history::ActiveModel> for CreateConversationHistory {
    fn into(self) -> entities::ollama_conversation_history::ActiveModel {
        HistoryActiveModel {
            identifier: Set(Uuid::new_v4()),
            bookmarked: Set(self.bookmarked),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConversationHistory {
    pub bookmarked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateConversationPrompt {
    pub history_id: Uuid,
    pub content: String,
    pub created_at: DateTimeWithTimeZone,
}

impl Into<entities::ollama_conversation_prompt::ActiveModel> for CreateConversationPrompt {
    fn into(self) -> entities::ollama_conversation_prompt::ActiveModel {
        PromptActiveModel {
            identifier: Set(Uuid::new_v4()),
            history_id: Set(self.history_id),
            content: Set(self.content),
            created_at: Set(self.created_at),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateConversationResponse {
    pub history_id: Uuid,
    pub content: String,
    pub created_at: DateTimeWithTimeZone,
}

impl Into<entities::ollama_conversation_response::ActiveModel> for CreateConversationResponse {
    fn into(self) -> entities::ollama_conversation_response::ActiveModel {
        ResponseActiveModel {
            identifier: Set(Uuid::new_v4()),
            history_id: Set(self.history_id),
            content: Set(self.content),
            created_at: Set(self.created_at),
        }
    }
}
