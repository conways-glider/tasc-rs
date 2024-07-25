use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

pub mod handler;
mod postgres;

pub fn get_app() -> Router<Pool<Postgres>> {
    Router::new()
        .route(
            "/",
            get(handler::root_get_handler).post(handler::root_post_handler),
        )
        .route("/:user_id", get(handler::root_get_by_id_handler))
}
