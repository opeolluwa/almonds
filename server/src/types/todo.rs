use almond_kernel::entities;
use almond_kernel::entities::sea_orm_active_enums::Priority;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncTodoInput")]
pub struct SyncTodoInput {
    pub identifier: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub priority: Priority,
    pub done: bool,
    pub created_at: String,
    pub updated_at: String,
    pub due_time: Option<String>,
    pub workspace_identifier: Option<Uuid>,
}

impl From<SyncTodoInput> for entities::todo::Model {
    fn from(val: SyncTodoInput) -> Self {
        entities::todo::Model {
            identifier: val.identifier,
            title: val.title,
            description: val.description,
            due_date: val.due_date.map(|d| d.parse().unwrap()),
            priority: val.priority,
            done: val.done,
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            due_time: val.due_time.map(|t| t.parse().unwrap()),
            workspace_identifier: val.workspace_identifier,
        }
    }
}
