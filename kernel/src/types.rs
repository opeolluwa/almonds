use async_graphql::SimpleObject;
use seaography::CustomOutputType;
use seaography::async_graphql;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CustomOutputType, SimpleObject)]
#[serde(rename_all = "camelCase")]
#[graphql(rename_fields = "camelCase")]
pub struct EntitySyncResult {
    pub identifier: String,
    pub success: bool,
    pub error_message: Option<String>,
}
