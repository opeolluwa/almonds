use almond_kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncReminderInput {
    pub identifier: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub recurring: bool,
    pub recurrence_rule: Option<String>,
    pub alarm_sound: Option<String>,
    pub remind_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl From<SyncReminderInput> for entities::reminder::Model {
    fn from(val: SyncReminderInput) -> Self {
        entities::reminder::Model {
            identifier: val.identifier,
            title: val.title,
            description: val.description,
            recurring: val.recurring,
            recurrence_rule: val.recurrence_rule,
            alarm_sound: val.alarm_sound,
            remind_at: val.remind_at.parse().unwrap(),
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            workspace_identifier: val.workspace_identifier,
        }
    }
}
