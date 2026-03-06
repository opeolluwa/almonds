use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::auth::{
        create_account, forgotten_password, login, onboard_user, request_refresh_token,
        set_new_password, verify_account, verify_reset_otp,
    },
    states::ServicesState,
};

pub(super) fn authentication_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/signup", post(create_account))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/reset-password", post(set_new_password))
        .route("/verify-account", post(verify_account))
        .route("/refresh-token", get(request_refresh_token))
        .route("/onboard", post(onboard_user))
        .route("/verify", post(verify_reset_otp))
        .with_state(state)
}
