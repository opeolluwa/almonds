
use graphql_client::GraphQLQuery;
use async_trait::async_trait;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = ".graphql/sync_queqe_schema.graphql",
    query_path = ".graphql/sync_queue_query.graphql"
        response_derives = "Debug",
        normalization = "rust"
)]
pub struct SyncQueueView;


// pub struct SyncEngine {
//     graphql_client: GraphQLClient,
// }


#[async_trait]
pub trait SyncEngineTrait {
    // async fn upsert (&self) -> Result<()>;
    // async fn downsync(&self) -> Result<()>;
}
