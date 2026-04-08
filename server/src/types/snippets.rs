use chrono::Utc;
use entities::snippets::ActiveModel;
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
pub struct CreateSnippetInput {
    pub title: Option<String>,

    pub language: Option<String>,

    #[validate(length(min = 1, message = "Code must not be empty"))]
    pub code: String,

    pub description: Option<String>,

    pub is_pinned: bool,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateSnippetInput {
    fn into(self) -> entities::snippets::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            language: Set(self.language),
            code: Set(self.code),
            description: Set(self.description),
            is_pinned: Set(self.is_pinned),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}
