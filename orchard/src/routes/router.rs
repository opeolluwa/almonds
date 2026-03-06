use std::sync::Arc;

use axum::{Router, http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;

use crate::{
    response::ApiResponseBuilder,
    routes::{
        app::public_routes, auth::authentication_routes, book::audio_book_routes,
        country::country_routes, notification::notification_routes, playlist::playlist_routes,
        users::user_routes, wait_list::wait_list_routes,
    },
    services::{
        audio_book_service::AudioBooksService, authentication_service::AuthenticationService,
        country_service::CountryService, notification_service::NotificationService,
        playlist_service::PlaylistService, root_service::RootService, user_service::UserService,
        wait_list_service::WaitListService,
    },
    states::ServicesState,
};

pub fn load_routes(db_conn: &Arc<DatabaseConnection>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(db_conn),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(db_conn),
        playlist_service: PlaylistService::init(db_conn),
        audio_book_service: AudioBooksService::init(db_conn),
        notification_service: NotificationService::init(db_conn),
        country_service: CountryService::init(db_conn),
        wait_list_service: WaitListService::init(db_conn),
    };

    Router::new()
        .merge(public_routes())
        .nest("/auth", authentication_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/books", audio_book_routes(state.clone()))
        .nest("/playlist", playlist_routes(state.clone()))
        .nest("/notifications", notification_routes(state.clone()))
        .nest("/countries", country_routes(state.clone()))
        .nest("/wait-list", wait_list_routes(state.clone()))
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
