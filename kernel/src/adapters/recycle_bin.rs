use std::fmt;

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, recycle_bin::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecycleBinItemType {
    Todo,
    Note,
    Reminder,
    Snippet,
    Bookmark,
}

impl fmt::Display for RecycleBinItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecycleBinItemType::Todo => write!(f, "todo"),
            RecycleBinItemType::Note => write!(f, "note"),
            RecycleBinItemType::Reminder => write!(f, "reminder"),
            RecycleBinItemType::Snippet => write!(f, "snippet"),
            RecycleBinItemType::Bookmark => write!(f, "bookmark"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecycleBinEntry {
    pub item_id: Uuid,
    pub item_type: RecycleBinItemType,
    pub payload: String,
}

impl Into<entities::recycle_bin::ActiveModel> for CreateRecycleBinEntry {
    fn into(self) -> entities::recycle_bin::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            item_id: Set(self.item_id),
            item_type: Set(self.item_type.to_string()),
            payload: Set(self.payload),
            deleted_at: Set(Utc::now().fixed_offset()),
        }
    }
}
