use axum::{
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    handlers::{
        auth::change_password,
        users::{retrieve_information, update_password, update_profile, update_profile_picture},
    },
    states::ServicesState,
};

pub(super) fn user_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/profile", get(retrieve_information))
        .route("/profile", put(update_profile))
        .route("/avatar", post(update_profile_picture))
        .route("/password", put(update_password))
        .route("/password", patch(change_password))
        .with_state(state)
}
