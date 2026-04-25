use almond_kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncWorkspaceInput {
    pub identifier: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_default: bool,
    pub is_hidden: bool,
    pub is_secured: bool,
    pub password_hash: Option<String>,
}

impl From<SyncWorkspaceInput> for entities::workspaces::Model {
    fn from(val: SyncWorkspaceInput) -> Self {
        entities::workspaces::Model {
            identifier: val.identifier,
            name: val.name,
            description: val.description,
            created_at: val.created_at.parse().unwrap(),
            updated_at: val.updated_at.parse().unwrap(),
            is_default: val.is_default,
            is_hidden: val.is_hidden,
            is_secured: val.is_secured,
            password_hash: val.password_hash,
        }
    }
}
