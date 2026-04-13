#![allow(clippy::all, warnings)]
pub struct SyncQueueView;
pub mod sync_queue_view {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "SyncQueueView";
    pub const QUERY: &str = "query SyncQueueView {\n  syncQueue {\n    nodes {\n      identifier\n      tableName\n      recordIdentifier\n      createdAt\n      operation\n    }\n  }\n}\n";
    use super::*;
    use ::serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    #[serde(crate = ":: serde")]
    pub struct Variables;
    #[derive(Deserialize)]
    #[serde(crate = ":: serde")]
    pub struct ResponseData {
        #[serde(rename = "syncQueue")]
        pub sync_queue: SyncQueueViewSyncQueue,
    }
    #[derive(Deserialize)]
    #[serde(crate = ":: serde")]
    pub struct SyncQueueViewSyncQueue {
        pub nodes: Vec<SyncQueueViewSyncQueueNodes>,
    }
    #[derive(Deserialize)]
    #[serde(crate = ":: serde")]
    pub struct SyncQueueViewSyncQueueNodes {
        pub identifier: String,
        #[serde(rename = "tableName")]
        pub table_name: String,
        #[serde(rename = "recordIdentifier")]
        pub record_identifier: String,
        #[serde(rename = "createdAt")]
        pub created_at: String,
        pub operation: String,
    }
}
impl graphql_client::GraphQLQuery for SyncQueueView {
    type Variables = sync_queue_view::Variables;
    type ResponseData = sync_queue_view::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sync_queue_view::QUERY,
            operation_name: sync_queue_view::OPERATION_NAME,
        }
    }
}
