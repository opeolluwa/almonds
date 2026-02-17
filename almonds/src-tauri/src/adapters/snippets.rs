use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CreateSnippet> for almond_kernel::adapters::snippets::Snippet {
    fn from(snippet: CreateSnippet) -> Self {
        Self {
            title: snippet.title,
            language: snippet.language,
            code: snippet.code,
            description: snippet.description,
            is_pinned: snippet.is_pinned,
            created_at: snippet.created_at.parse().expect("invalid created_at"),
            updated_at: snippet.updated_at.parse().expect("invalid updated_at"),
        }
    }
}
