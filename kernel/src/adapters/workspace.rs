use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, workspaces::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct CreateWorkspace {
    pub name: String,
    pub description: String,
}

impl Into<entities::workspaces::ActiveModel> for CreateWorkspace {
    fn into(self) -> entities::workspaces::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            created_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}
