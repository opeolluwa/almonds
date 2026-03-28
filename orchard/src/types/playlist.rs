use chrono::Utc;
use entities::playlists::ActiveModel;
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
pub struct CreatePlaylistInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    pub description: Option<String>,

    pub user_identifier: Uuid,
}

impl Into<ActiveModel> for CreatePlaylistInput {
    fn into(self) -> entities::playlists::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(None),
            user_identifier: Set(self.user_identifier),
        }
    }
}
