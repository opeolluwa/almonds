use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, notes::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub categories: Option<Vec<String>>,
    pub workspace_identifier: Option<Uuid>,
}

impl Into<entities::notes::ActiveModel> for CreateNote {
    fn into(self) -> entities::notes::ActiveModel {
        let categories = serde_json::json!(self.categories.unwrap_or_default());
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            content: Set(self.content),
            categories: Set(categories),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub categories: Option<Vec<String>>,
}
