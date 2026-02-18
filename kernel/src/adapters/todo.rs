use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::entities::{self, todo::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TodoPriority {
    High,
    Medium,
    Low,
}

impl fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoPriority::High => write!(f, "high"),
            TodoPriority::Medium => write!(f, "medium"),
            TodoPriority::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
    pub priority: TodoPriority,
}

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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
}
