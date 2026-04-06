use std::fmt;

use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::sea_orm_active_enums::ItemType;
use crate::entities::{self, recycle_bin::ActiveModel};

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Todo => write!(f, "todo"),
            ItemType::Note => write!(f, "note"),
            ItemType::Reminder => write!(f, "reminder"),
            ItemType::Snippet => write!(f, "snippet"),
            ItemType::Bookmark => write!(f, "bookmark"),
            // ItemType::Workspace => write!(f, "workspace"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecycleBinEntry {
    pub item_id: Uuid,
    pub item_type: ItemType,
    pub payload: String,
    pub workspace_identifier: Option<Uuid>,
}

#[cfg(feature = "postgres")]
impl Into<entities::recycle_bin::ActiveModel> for CreateRecycleBinEntry {
    fn into(self) -> entities::recycle_bin::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            item_id: Set(self.item_id),
            item_type: Set(self.item_type),
            payload: Set(self.payload),
            workspace_identifier: Set(self.workspace_identifier),
            deleted_at: Set(Utc::now().fixed_offset()),
        }
    }
}
#[cfg(feature = "sqlite")]
impl Into<entities::recycle_bin::ActiveModel> for CreateRecycleBinEntry {
    fn into(self) -> entities::recycle_bin::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            item_id: Set(self.item_id),
            item_type: Set(self.item_type.to_string()),
            payload: Set(self.payload),
            workspace_identifier: Set(self.workspace_identifier),
            deleted_at: Set(Utc::now().fixed_offset()),
        }
    }
}
