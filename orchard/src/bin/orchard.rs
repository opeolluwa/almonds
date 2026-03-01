use almond_kernel::kernel;

use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use orchard_lib::{errors::AppError, shutdown::shutdown_signal};
use seaography::{async_graphql, lazy_static::lazy_static};
use std::{
    env,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

lazy_static! {
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
}

async fn graphql_handler(State(schema): State<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let kernel = kernel::Kernel::new(&*DATABASE_URL).await?;

    kernel.run_migrations().await?;

    let db = kernel.connection().to_owned();

    let schema = orchard_lib::query_root::schema(db, *DEPTH_LIMIT, *COMPLEXITY_LIMIT)
        .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let app = Router::new()
        .route(&*ENDPOINT, get(graphql_playground).post(graphql_handler))
        .layer(cors)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(25),
        ))
        .with_state(schema);

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8000));
    tracing::info!("Visit GraphQL Playground at http://{}", ip_address);

    axum::serve(TcpListener::bind(ip_address).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))?;

    Ok(())
}
