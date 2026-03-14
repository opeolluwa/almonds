use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use almond_kernel::{error::KernelError, kernel};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header, Method, StatusCode},
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use orchard_lib::{
    config::AppConfig, errors::app_error::AppError, routes::router::load_routes,
    shutdown::shutdown_signal, states::GraphQlState,
};
use seaography::async_graphql;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

use orchard_migration::{Migrator, MigratorTrait};

#[axum::debug_handler]
async fn graphql_playground(
    State(GraphQlState { endpoint, .. }): State<GraphQlState>,
) -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&endpoint)))
}

#[axum::debug_handler]
async fn graphql_handler(
    State(GraphQlState { schema, .. }): State<GraphQlState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let app_config = AppConfig::from_env()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let cors = if app_config.environment == "production" {
        CorsLayer::new()
            .allow_origin(app_config.allowed_origins.clone())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let kernel = kernel::Kernel::new("postgres://orchard:orchard@postgres/orchard").await?;

    kernel.run_migrations().await?;

    let db = kernel.connection().to_owned();
    let db_conn = Arc::new(db.clone());

    Migrator::up(&db, None)
        .await
        .map_err(|e| KernelError::DbConnectError(e.to_string()))?;

    let schema = orchard_lib::query_root::schema(db, Some(100), app_config.complexity_limit)
        .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let graphql_state = GraphQlState {
        schema,
        endpoint: app_config.endpoint.clone(),
    };

    let http_routes = load_routes(&db_conn);

    let graphql_router = Router::new()
        .route(
            &app_config.endpoint,
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(graphql_state);

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(cors)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(25),
        ));

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port));
    tracing::info!(
        "Visit GraphQL Playground at http://{}{}",
        ip_address,
        app_config.endpoint
    );
    tracing::info!("Service health check at http://{}/health", ip_address,);
    axum::serve(TcpListener::bind(ip_address).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|err| AppError::InternalError(err.to_string()))?;

    Ok(())
}
