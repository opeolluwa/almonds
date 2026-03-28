use chrono::Utc;
use entities::ollama_conversation_history::ActiveModel;
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
pub struct CreateOllamaConversationHistoryInput {
    pub bookmarked: bool,
}

impl Into<ActiveModel> for CreateOllamaConversationHistoryInput {
    fn into(self) -> entities::ollama_conversation_history::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            bookmarked: Set(self.bookmarked),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}
