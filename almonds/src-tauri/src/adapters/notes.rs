use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub categories: Option<Vec<String>>,
}

impl From<CreateNote> for almond_kernel::adapters::notes::CreateNote {
    fn from(note: CreateNote) -> Self {
        Self {
            title: note.title,
            content: note.content,
            categories: note.categories,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<String>>,
}

impl From<UpdateNote> for almond_kernel::adapters::notes::UpdateNote {
    fn from(note: UpdateNote) -> Self {
        Self {
            title: note.title,
            content: note.content,
            categories: note.categories,
        }
    }
}
