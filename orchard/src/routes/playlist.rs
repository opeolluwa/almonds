use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::playlist::{
        create_playlist, delete_playlist, list_playlists, retrieve_playlist, update_playlist,
    },
    states::ServicesState,
};

pub(super) fn playlist_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(create_playlist))
        .route("/", get(list_playlists))
        .route("/{playlist_identifier}", get(retrieve_playlist))
        .route("/{playlist_identifier}", put(update_playlist))
        .route("/{playlist_identifier}", delete(delete_playlist))
        .with_state(state)
}
