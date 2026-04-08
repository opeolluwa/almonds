use chrono::Utc;
use entities::notes::ActiveModel;
use sea_orm::ActiveValue::Set;
use seaography::{
    async_graphql::{self},
    CustomInputType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities;

#[derive(Debug, Serialize, Deserialize, Validate, CustomInputType)]
pub struct CreateNoteInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Content must not be empty"))]
    pub content: String,

    /// JSON array string of category identifiers
    pub categories: Option<String>,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateNoteInput {
    fn into(self) -> entities::notes::ActiveModel {
        let categories = self
            .categories
            .as_deref()
            .and_then(|c| serde_json::from_str(c).ok());

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
