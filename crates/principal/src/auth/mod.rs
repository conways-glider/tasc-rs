use axum::{routing::post, Router};

use crate::state::AppState;

pub mod handlers;

pub fn get_app() -> Router<AppState> {
    Router::new().route("/login", post(handlers::login))
}
