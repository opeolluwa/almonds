use axum::{routing::get, Router};

use crate::{
    handlers::country::{
        fetch_all_countries, fetch_countries_by_currency_code, fetch_country_by_identifier,
    },
    states::ServicesState,
};

pub(super) fn country_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", get(fetch_all_countries))
        .route("/{identifier}", get(fetch_country_by_identifier))
        .route(
            "/currency/{currency_code}",
            get(fetch_countries_by_currency_code),
        )
        .with_state(state)
}
