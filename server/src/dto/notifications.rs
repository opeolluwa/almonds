use std::fmt;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub identifier: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub is_read: bool,
}

impl Notification {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            title: title.to_string(),
            body: body.to_string(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}\n{}\nCreated: {} | Updated: {} | Read: {}",
            self.identifier,
            self.title,
            self.body,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "N/A".into()),
            self.is_read
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedNotification {
    pub notifications: Vec<almond_kernel::entities::notifications::Model>,
    pub total: u64,
}
