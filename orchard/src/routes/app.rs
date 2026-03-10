use axum::{routing::get, Router};

use crate::handlers::app::health_check;

pub(super) fn public_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
