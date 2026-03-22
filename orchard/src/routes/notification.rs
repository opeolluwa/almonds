use axum::{
    routing::{get, patch},
    Router,
};

use crate::{
    handlers::notification::{
        count_unread, fetch_notifications, listen_for_new_notifications, mark_read,
    },
    states::ServicesState,
};

pub(super) fn notification_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", get(fetch_notifications))
        .route("/listen", get(listen_for_new_notifications))
        .route("/unread", get(count_unread))
        .route("/{notification_identifier}", patch(mark_read))
        .with_state(state)
}
