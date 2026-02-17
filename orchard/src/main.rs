#![warn(unused_extern_crates)]

use uralium_lib::{errors, routes, shared};

use errors::app_error::AppError;
use routes::router::load_routes;
use shared::extract_env::extract_env;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::Path,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = extract_env::<String>("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    log::info!("Database initialized");

    let migrator = Migrator::new(Path::new("migrations"))
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;
    migrator
        .run(&pool)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    let app = load_routes(pool);
    let port = extract_env::<u16>("PORT")?;
    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    log::info!("Application listening on http://{}", ip_address);

    let listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    Ok(())
}

