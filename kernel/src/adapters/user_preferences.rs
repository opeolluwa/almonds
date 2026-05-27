use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, user_preferences::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPreferences {
    pub master_first_name: String,
    pub master_last_name: String,
    pub master_email: String,
}

impl Into<entities::user_preferences::ActiveModel> for CreateUserPreferences {
    fn into(self) -> entities::user_preferences::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            master_first_name: Set(self.master_first_name),
            master_last_name: Set(self.master_last_name),
            master_email: Set(self.master_email),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPreferences {
    pub master_first_name: Option<String>,
    pub master_last_name: Option<String>,
    pub master_email: Option<String>,
}
