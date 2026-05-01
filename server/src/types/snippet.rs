use almond_kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncSnippetInput")]
pub struct SyncSnippetInput {
    pub identifier: Uuid,
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl From<SyncSnippetInput> for entities::snippets::Model {
    fn from(val: SyncSnippetInput) -> Self {
        entities::snippets::Model {
            identifier: val.identifier,
            title: val.title,
            language: val.language,
            code: val.code,
            description: val.description,
            is_pinned: val.is_pinned,
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            workspace_identifier: val.workspace_identifier,
        }
    }
}
