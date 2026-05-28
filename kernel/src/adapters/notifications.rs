use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::NotificationType;
use crate::entities::{self, notifications::ActiveModel};
#[cfg(feature = "sqlite")]
use crate::enums::NotificationType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotification {
    pub title: String,
    pub body: String,
    pub notification_type: NotificationType,
    pub workspace_identifier: Option<Uuid>,
    pub is_read: bool,
}

#[cfg(feature = "postgres")]
impl Into<entities::notifications::ActiveModel> for CreateNotification {
    fn into(self) -> entities::notifications::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            body: Set(self.body),
            notification_type: Set(self.notification_type),
            is_read: Set(self.is_read),
            workspace_identifier: Set(self.workspace_identifier),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[cfg(feature = "sqlite")]
impl Into<entities::notifications::ActiveModel> for CreateNotification {
    fn into(self) -> entities::notifications::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            body: Set(self.body),
            notification_type: Set(self.notification_type.to_string()),
            is_read: Set(self.is_read),
            workspace_identifier: Set(self.workspace_identifier),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}
