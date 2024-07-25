use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sqlx::PgPool;
use tasc_rs_common::user::{NewUser, User};
use tracing::instrument;

use crate::error::AppError;

#[instrument(skip(pool))]
pub async fn root_get_handler(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, AppError> {
    let people = super::postgres::get_users(pool).await?;
    Ok(Json(people))
}

#[instrument(skip(pool))]
pub async fn root_get_by_id_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let person = super::postgres::get_user_by_id(pool, user_id).await?;
    let response = match person {
        Some(person) => Json(person).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    };
    Ok(response)
}

#[instrument(skip(pool))]
pub async fn root_post_handler(
    State(pool): State<PgPool>,
    Json(new_person): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let person = super::postgres::add_user(pool, &new_person).await?;
    let response = (StatusCode::CREATED, Json(person));
    Ok(response)
}
