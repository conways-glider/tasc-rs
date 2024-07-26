use anyhow::Context;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tasc_rs_common::user::AuthUser;
use tracing::instrument;

use crate::{error::AppError, state::AppState};

const AUTH_COOKIE_NAME: &str = "tasc-rs-auth-token";

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    username: String,

    #[serde(skip_serializing)]
    password: String,
}

#[instrument(skip(app_state, login_user))]
pub async fn login(
    State(app_state): State<AppState>,
    Json(login_user): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = crate::db::users::get_user_by_username(app_state.pool, login_user.username).await?;
    match user {
        Some(user) => {
            let result = user.validate_password(login_user.password);
            match result {
                Ok(_) => {
                    let auth_user = AuthUser::new_auth_user(user)?;
                    let jwt_secret = app_state.config.auth.jwt_secret.as_bytes();
                    let token = auth_user
                        .encode(jwt_secret)
                        .context("could not make login token")?;

                    let auth_token_value = format!("{}={}", AUTH_COOKIE_NAME, token);

                    let response_headers = [(axum::http::header::SET_COOKIE, auth_token_value)];
                    Ok((StatusCode::OK, response_headers).into_response())
                }
                Err(_) => Ok(StatusCode::UNAUTHORIZED.into_response()),
            }
        }
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}
