use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    handlers::books::{
        create_new_book, delete_book, fetch_book, fetch_favorites, find_many_books, mark_favorite,
        unmark_favorite, update_book,
    },
    states::ServicesState,
};

pub(super) fn audio_book_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/", post(create_new_book))
        .route("/", get(find_many_books))
        .route("/{book_identifier}", put(update_book))
        .route("/{book_identifier}", get(fetch_book))
        .route("/{book_identifier}", delete(delete_book))
        .route("/{book_identifier}/favorite", post(mark_favorite))
        .route("/{book_identifier}/favorites", put(unmark_favorite))
        .route("/favorites", get(fetch_favorites))
        .with_state(state)
}
