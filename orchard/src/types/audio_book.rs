use chrono::Utc;
use entities::audio_books::ActiveModel;
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
pub struct CreateAudioBookInput {
    pub user_identifier: Uuid,

    #[validate(length(min = 1, message = "Source path must not be empty"))]
    pub src: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    pub playlist_identifier: Option<Uuid>,

    pub starred: bool,
}

impl Into<ActiveModel> for CreateAudioBookInput {
    fn into(self) -> entities::audio_books::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            user_identifier: Set(self.user_identifier),
            src: Set(self.src),
            name: Set(self.name),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(None),
            last_played: Set(None),
            playlist_identifier: Set(self.playlist_identifier),
            starred: Set(self.starred),
        }
    }
}
