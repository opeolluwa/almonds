use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, user_preference::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPreference {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Into<entities::user_preference::ActiveModel> for CreateUserPreference {
    fn into(self) -> entities::user_preference::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            email: Set(self.email),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPreference {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
