use axum::{Router, routing::post};

use crate::{handlers::wait_list::join_wait_list, states::ServicesState};

pub(super) fn wait_list_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/join", post(join_wait_list))
        .with_state(state)
}
