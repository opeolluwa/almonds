use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

pub use crate::types::EntitySyncResult;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = ".graphql/sync_queue_schema.graphql",
    query_path = ".graphql/sync_queue_query.graphql"
        response_derives = "Debug,Serialize,Deserialize",
        normalization = "rust"
)]
pub struct SyncQueueView;

pub type DataQueue = Vec<crate::entities::sync_queue::Model>;
pub type SyncQueueItemIdentifier = String;

#[derive(Debug, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "sync_result.ts")]
pub struct SyncResult {
    pub sync_queue_item: Vec<SyncQueueItemIdentifier>,
    pub failed_items: Vec<SyncQueueItemIdentifier>,
    pub new_records: DataQueue,
}
