use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

pub mod handler;
mod postgres;

pub fn get_app() -> Router<Pool<Postgres>> {
    Router::new().route("/login", post(handler::login))
}
