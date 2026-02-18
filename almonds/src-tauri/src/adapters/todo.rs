use serde::Deserialize;

use almond_kernel::adapters::todo::TodoPriority;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub priority: String,
}

impl From<CreateTodo> for almond_kernel::adapters::todo::CreateTodo {
    fn from(t: CreateTodo) -> Self {
        use chrono::NaiveDate;

        let due_date = t
            .due_date
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let priority = match t.priority.as_str() {
            "high" => TodoPriority::High,
            "low" => TodoPriority::Low,
            _ => TodoPriority::Medium,
        };

        Self {
            title: t.title,
            description: t.description,
            due_date,
            priority,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl From<UpdateTodo> for almond_kernel::adapters::todo::UpdateTodo {
    fn from(t: UpdateTodo) -> Self {
        Self {
            title: t.title,
            description: t.description,
        }
    }
}
