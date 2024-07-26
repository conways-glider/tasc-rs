use axum::{routing::post, Router};

use crate::state::AppState;

pub mod handler;
mod postgres;

pub fn get_app() -> Router<AppState> {
    Router::new().route("/login", post(handler::login))
}
