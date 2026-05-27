use almond_kernel::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncWorkspacePreferenceInput")]
pub struct SyncWorkspacePreferenceInput {
    pub identifier: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub workspace_identifier: Option<Uuid>,
}

impl TryFrom<SyncWorkspacePreferenceInput> for entities::workspace_preferences::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncWorkspacePreferenceInput) -> Result<Self, Self::Error> {
        Ok(entities::workspace_preferences::Model {
            identifier: val.identifier,
            first_name: val.first_name,
            last_name: val.last_name,
            created_at: val
                .created_at
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updated_at
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            workspace_identifier: val.workspace_identifier,
        })
    }
}
