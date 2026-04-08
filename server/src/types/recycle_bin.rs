use chrono::Utc;
use entities::recycle_bin::ActiveModel;
use entities::sea_orm_active_enums::ItemType;
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
pub struct CreateRecycleBinItemInput {
    pub item_id: Uuid,

    /// One of: todo, note, reminder, snippet, bookmark
    #[validate(length(min = 1, message = "item_type is required"))]
    pub item_type: String,

    #[validate(length(min = 1, message = "Payload must not be empty"))]
    pub payload: String,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateRecycleBinItemInput {
    fn into(self) -> entities::recycle_bin::ActiveModel {
        let item_type = match self.item_type.to_lowercase().as_str() {
            "note" => ItemType::Note,
            "reminder" => ItemType::Reminder,
            "snippet" => ItemType::Snippet,
            "bookmark" => ItemType::Bookmark,
            _ => ItemType::Todo,
        };

        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            item_id: Set(self.item_id),
            item_type: Set(item_type),
            payload: Set(self.payload),
            deleted_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}
