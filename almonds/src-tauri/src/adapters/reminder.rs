use almond_kernel::sea_orm::prelude::DateTimeWithTimeZone;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReminder {
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub recurring: bool,
    pub recurrence_rule: Option<String>,
    pub alarm_sound: Option<String>,
    pub remind_at: DateTimeWithTimeZone,
}

impl From<CreateReminder> for almond_kernel::adapters::reminder::CreateReminder {
    fn from(r: CreateReminder) -> Self {
        Self {
            title: r.title,
            description: r.description,
            recurring: r.recurring,
            recurrence_rule: r.recurrence_rule,
            alarm_sound: r.alarm_sound,
            remind_at: r.remind_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReminder {
    pub title: Option<String>,
    pub description: Option<String>,
    pub recurring: Option<bool>,
    pub recurrence_rule: Option<String>,
    pub alarm_sound: Option<String>,
    pub remind_at: Option<DateTimeWithTimeZone>,
}

impl From<UpdateReminder> for almond_kernel::adapters::reminder::UpdateReminder {
    fn from(r: UpdateReminder) -> Self {
        Self {
            title: r.title,
            description: r.description,
            recurring: r.recurring,
            recurrence_rule: r.recurrence_rule,
            alarm_sound: r.alarm_sound,
            remind_at: r.remind_at,
        }
    }
}
