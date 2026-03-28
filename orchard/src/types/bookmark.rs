use chrono::Utc;
use entities::bookmark::ActiveModel;
use entities::sea_orm_active_enums::Tag;
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
pub struct CreateBookmarkInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(url(message = "URL must be a valid URL"))]
    pub url: String,

    /// One of: development, inspiration, design, research
    #[validate(length(min = 1, message = "Tag is required"))]
    pub tag: String,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateBookmarkInput {
    fn into(self) -> entities::bookmark::ActiveModel {
        let tag = match self.tag.to_lowercase().as_str() {
            "inspiration" => Tag::Inspiration,
            "design" => Tag::Design,
            "research" => Tag::Research,
            _ => Tag::Development,
        };

        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            url: Set(self.url),
            tag: Set(tag),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}
