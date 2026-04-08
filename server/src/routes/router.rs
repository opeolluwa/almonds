use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Router};
use sea_orm::DatabaseConnection;

use crate::{
    response::ApiResponseBuilder,
    routes::{
        app::public_routes,
        auth::authentication_routes, /* notification::notification_routes*/
        users::user_routes,
        // wait_list::wait_list_routes,
    },
    services::{
        authentication_service::AuthenticationService, /*notification_service::NotificationService,*/
        root_service::RootService,
        user_service::UserService,
        // wait_list_service::WaitListService,
    },
    states::ServicesState,
};

pub fn load_routes(db_conn: &Arc<DatabaseConnection>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(db_conn),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(db_conn),
        // notification_service: NotificationService::init(db_conn),
        // country_service: CountryService::init(db_conn),
        // wait_list_service: WaitListService::init(db_conn),
    };

    Router::new()
        .merge(public_routes())
        .nest("/auth", authentication_routes(state.clone()))
        // .nest("/user", user_routes(state.clone()))
        // .nest("/notifications", notification_routes(state.clone()))
        // .nest("/countries", country_routes(state.clone()))
        // .nest("/wait-list", wait_list_routes(state.clone()))
        .fallback(async || {
            ApiResponseBuilder::<()>::new()
                .message(
                    "the resource you're looking does not exist or it has been permanently moved",
                )
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        })
}
