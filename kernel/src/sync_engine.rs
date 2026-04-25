use graphql_client::GraphQLQuery;
use sea_orm::DatabaseConnection;

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

pub struct SyncResult {
    pub sync_queue_item: Vec<SyncQueueItemIdentifier>,
    pub failed_items: Vec<SyncQueueItemIdentifier>,
    pub new_records: DataQueue,
}
