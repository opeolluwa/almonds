use graphql_client::GraphQLQuery;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = ".graphql/schema.graphql",
    query_path = ".graphql/sync_queue_query.graphql"
)]
pub struct SyncQueueView;
