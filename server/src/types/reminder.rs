use chrono::Utc;
use entities::reminder::ActiveModel;
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
pub struct CreateReminderInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    pub description: Option<String>,

    pub recurring: bool,

    pub recurrence_rule: Option<String>,

    pub alarm_sound: Option<String>,

    /// RFC3339 datetime string
    #[validate(length(min = 1, message = "remind_at is required"))]
    pub remind_at: String,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateReminderInput {
    fn into(self) -> entities::reminder::ActiveModel {
        let remind_at = chrono::DateTime::parse_from_rfc3339(&self.remind_at)
            .unwrap_or_else(|_| Utc::now().fixed_offset());

        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            recurring: Set(self.recurring),
            recurrence_rule: Set(self.recurrence_rule),
            alarm_sound: Set(self.alarm_sound),
            remind_at: Set(remind_at),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}
