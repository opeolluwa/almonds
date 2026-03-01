use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, reminder::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReminder {
    pub title: String,
    pub description: Option<String>,
    pub recurring: bool,
    pub recurrence_rule: Option<String>,
    pub alarm_sound: Option<String>,
    pub remind_at: DateTimeWithTimeZone,
    pub workspace_identifier: Option<Uuid>,
}

impl Into<entities::reminder::ActiveModel> for CreateReminder {
    fn into(self) -> entities::reminder::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            recurring: Set(self.recurring),
            recurrence_rule: Set(self.recurrence_rule),
            alarm_sound: Set(self.alarm_sound),
            remind_at: Set(self.remind_at),
            created_at: Set(Local::now().fixed_offset()),
            updated_at: Set(Local::now().fixed_offset()),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReminder {
    pub title: Option<String>,
    pub description: Option<String>,
    pub recurring: Option<bool>,
    pub recurrence_rule: Option<String>,
    pub alarm_sound: Option<String>,
    pub remind_at: Option<DateTimeWithTimeZone>,
}
