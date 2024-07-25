use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    username: String,

    #[serde(skip_serializing)]
    password: String,
}

#[instrument(skip(pool, login_user))]
pub async fn login(
    State(pool): State<PgPool>,
    Json(login_user): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let person = super::postgres::get_user_by_username(pool, login_user.username).await?;
    match person {
        Some(person) => {
            let result = person.validate_password(login_user.password);
            match result {
                Ok(_) => Ok(StatusCode::OK.into_response()),
                Err(_) => Ok((StatusCode::UNAUTHORIZED, "could not login").into_response()),
            }
        }
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}
