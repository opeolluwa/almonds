use crate::{
    handlers::country::{fetch_all_countries, fetch_country_by_identifier},
    states::ServicesState,
};
use axum::{Router, routing::get};

pub(super) fn country_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", get(fetch_all_countries))
        .route("/{identifier}", get(fetch_country_by_identifier))
        .with_state(state)
}
