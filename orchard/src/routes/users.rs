use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{
    handlers::{
        auth::change_password,
        users::{
            add_backup_email, retrieve_information, toggle_2fa, toggle_biometrics, update_password,
            update_profile, update_profile_picture,
        },
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
        .route("/2fa", patch(toggle_2fa))
        .route("/biometrics", patch(toggle_biometrics))
        .route("/backup-email", post(add_backup_email))
        .with_state(state)
}
