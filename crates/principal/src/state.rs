use axum::extract::FromRef;
use sqlx::PgPool;

use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub pool: PgPool,
}

impl FromRef<AppState> for config::Config {
    fn from_ref(app_state: &AppState) -> config::Config {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.pool.clone()
    }
}
