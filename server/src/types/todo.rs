use chrono::Utc;
use entities::sea_orm_active_enums::Priority;
use entities::todo::ActiveModel;
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
pub struct CreateTodoInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    pub description: Option<String>,

    /// ISO date string: YYYY-MM-DD
    pub due_date: Option<String>,

    /// One of: high, medium, low
    #[validate(length(min = 1, message = "Priority is required"))]
    pub priority: String,

    pub done: bool,

    /// Time string: HH:MM:SS
    pub due_time: Option<String>,

    pub workspace_identifier: Option<Uuid>,
}

impl Into<ActiveModel> for CreateTodoInput {
    fn into(self) -> entities::todo::ActiveModel {
        let priority = match self.priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            _ => Priority::Low,
        };

        let due_date = self
            .due_date
            .as_deref()
            .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let due_time = self
            .due_time
            .as_deref()
            .and_then(|t| chrono::NaiveTime::parse_from_str(t, "%H:%M:%S").ok());

        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            due_date: Set(due_date),
            priority: Set(priority),
            done: Set(self.done),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
            due_time: Set(due_time),
            workspace_identifier: Set(self.workspace_identifier),
        }
    }
}
