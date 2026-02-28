use almond_kernel::sea_orm::sea_query::prelude::Local;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl From<CreateSnippet> for almond_kernel::adapters::snippets::CreateSnippet {
    fn from(snippet: CreateSnippet) -> Self {
        Self {
            title: snippet.title,
            language: snippet.language,
            code: snippet.code,
            description: snippet.description,
            is_pinned: false,
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
            workspace_identifier: snippet.workspace_identifier,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
}

impl From<UpdateSnippet> for almond_kernel::adapters::snippets::UpdateSnippet {
    fn from(snippet: UpdateSnippet) -> Self {
        Self {
            title: snippet.title,
            language: snippet.language,
            code: snippet.code,
            description: snippet.description,
            is_pinned: None,
        }
    }
}
