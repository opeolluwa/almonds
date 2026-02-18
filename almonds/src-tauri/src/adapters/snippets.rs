use almond_kernel::sea_orm::sea_query::prelude::{DateTime, Local};
use serde::Deserialize;

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
}

impl From<CreateSnippet> for almond_kernel::adapters::snippets::Snippet {
    fn from(snippet: CreateSnippet) -> Self {
        Self {
            title: snippet.title,
            language: snippet.language,
            code: snippet.code,
            description: snippet.description,
            is_pinned: false,
            created_at: Local::now().into(),
            updated_at: Local::now().into(),
        }
    }
}
