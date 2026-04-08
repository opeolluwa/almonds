use chrono::Utc;
use entities::workspaces::ActiveModel;
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
pub struct CreateWorkspaceInput {
    #[validate(length(
        min = 3,
        max = 100,
        message = "Workspace name must be between 3 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 5,
        max = 500,
        message = "Description must be between 5 and 500 characters"
    ))]
    pub description: String,
}

impl Into<ActiveModel> for CreateWorkspaceInput {
    fn into(self) -> entities::workspaces::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}
