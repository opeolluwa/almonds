use axum::http::{Method, header};
use tower_http::cors::{Any, CorsLayer};

use tokio::signal;
use tracing::info;

use crate::config::AppConfig;

pub fn create_cors_layer(config: &AppConfig) -> CorsLayer {
    if config.environment == "production" {
        CorsLayer::new()
            .allow_origin(config.allowed_origins.clone())
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = signal::ctrl_c().await {
            log::warn!("Failed to install Ctrl+C handler  due to {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        
        signal::unix::signal(signal::unix::SignalKind::terminate())
        
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received");
}
