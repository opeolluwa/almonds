use almond_kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyncNoteInput {
    pub identifier: Uuid,
    pub title: String,
    pub content: String,
    pub categories: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl From<SyncNoteInput> for entities::notes::Model {
    fn from(val: SyncNoteInput) -> Self {
        entities::notes::Model {
            identifier: val.identifier,
            title: val.title,
            content: val.content,
            // categories: val.categories,
            categories: None,
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            workspace_identifier: val.workspace_identifier,
        }
    }
}
