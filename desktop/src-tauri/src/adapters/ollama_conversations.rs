use almond_kernel::sea_orm::sea_query::prelude::Local;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationHistory {
    #[serde(default)]
    pub bookmarked: bool,
}

impl From<CreateConversationHistory>
    for almond_kernel::adapters::ollama_conversations::CreateConversationHistory
{
    fn from(h: CreateConversationHistory) -> Self {
        Self {
            bookmarked: h.bookmarked,
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationHistory {
    pub bookmarked: Option<bool>,
}

impl From<UpdateConversationHistory>
    for almond_kernel::adapters::ollama_conversations::UpdateConversationHistory
{
    fn from(h: UpdateConversationHistory) -> Self {
        Self {
            bookmarked: h.bookmarked,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationPrompt {
    pub history_id: Uuid,
    pub content: String,
}

impl From<CreateConversationPrompt>
    for almond_kernel::adapters::ollama_conversations::CreateConversationPrompt
{
    fn from(p: CreateConversationPrompt) -> Self {
        Self {
            history_id: p.history_id,
            content: p.content,
            created_at: Local::now().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationResponse {
    pub history_id: Uuid,
    pub content: String,
}

impl From<CreateConversationResponse>
    for almond_kernel::adapters::ollama_conversations::CreateConversationResponse
{
    fn from(r: CreateConversationResponse) -> Self {
        Self {
            history_id: r.history_id,
            content: r.content,
            created_at: Local::now().into(),
        }
    }
}
