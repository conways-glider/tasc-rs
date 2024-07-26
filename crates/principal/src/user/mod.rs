use axum::{routing::get, Router};

use crate::state::AppState;

pub mod handler;
mod postgres;

pub fn get_app() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handler::root_get_handler).post(handler::root_post_handler),
        )
        .route("/:user_id", get(handler::root_get_by_id_handler))
}
