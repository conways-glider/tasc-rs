use axum::{routing::get, Router};

use crate::state::AppState;

pub mod handlers;

pub fn get_app() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::root_get_handler).post(handlers::root_post_handler),
        )
        .route("/:user_id", get(handlers::root_get_by_id_handler))
}
