
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "sqlite")]
use crate::enums::Priority;
#[cfg(feature = "postgres")]
use crate::entities::sea_orm_active_enums::Priority;
use crate::entities::{self, todo::ActiveModel};



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
    // pub due_time: Option<Time>,//TODO
    pub priority: Priority,
}

#[cfg(feature = "postgres")]
impl Into<entities::todo::ActiveModel> for CreateTodo {
    fn into(self) -> entities::todo::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            due_date: Set(self.due_date),
            priority: Set(self.priority),
            done: Set(false),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}

#[cfg(feature = "sqlite")]
impl Into<entities::todo::ActiveModel> for CreateTodo {
    fn into(self) -> entities::todo::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            due_date: Set(self.due_date),
            priority: Set(self.priority.to_string()),
            done: Set(false),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
}
