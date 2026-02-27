use std::fmt;

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, bookmark::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BookmarkTag {
    Development,
    Inspiration,
    Design,
    Research,
}

impl fmt::Display for BookmarkTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookmarkTag::Development => write!(f, "development"),
            BookmarkTag::Inspiration => write!(f, "inspiration"),
            BookmarkTag::Design => write!(f, "design"),
            BookmarkTag::Research => write!(f, "research"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmark {
    pub title: String,
    pub url: String,
    pub tag: BookmarkTag,
    pub workspace_identifier: Option<Uuid>,
}

impl Into<entities::bookmark::ActiveModel> for CreateBookmark {
    fn into(self) -> entities::bookmark::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            url: Set(self.url),
            tag: Set(self.tag.to_string()),
            workspace_identifier: Set(self.workspace_identifier),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookmark {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tag: Option<BookmarkTag>,
}
