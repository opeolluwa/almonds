use axum::{Router, routing::get};

use crate::{controllers::user::retrieve_information, states::services_state::ServicesState};

pub(super) fn user_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .with_state(state)
}
