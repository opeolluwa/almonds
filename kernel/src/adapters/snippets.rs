use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, snippets::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub title: Option<String>,
    pub language: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub is_pinned: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl Into<entities::snippets::ActiveModel> for Snippet {
    fn into(self) -> entities::snippets::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            language: Set(self.language),
            code: Set(self.code),
            description: Set(self.description),
            is_pinned: Set(self.is_pinned),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
        }
    }
}
