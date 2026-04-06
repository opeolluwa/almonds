use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::sea_orm_active_enums::Tag;
use crate::entities::{self, bookmark::ActiveModel};

#[cfg(feature = "sqlite")]
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Development => write!(f, "development"),
            Tag::Inspiration => write!(f, "inspiration"),
            Tag::Design => write!(f, "design"),
            Tag::Research => write!(f, "research"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmark {
    pub title: String,
    pub url: String,
    pub tag: Tag,
}

#[cfg(feature = "postgres")]
impl Into<entities::bookmark::ActiveModel> for CreateBookmark {
    fn into(self) -> entities::bookmark::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            url: Set(self.url),
            tag: Set(self.tag),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}

#[cfg(feature = "sqlite")]
impl Into<entities::bookmark::ActiveModel> for CreateBookmark {
    fn into(self) -> entities::bookmark::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            url: Set(self.url),
            tag: Set(self.tag.to_string()),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookmark {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tag: Option<Tag>,
}
