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
    let people = crate::db::users::get_users(pool).await?;
    Ok(Json(people))
}

#[instrument(skip(pool))]
pub async fn root_get_by_id_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let person = crate::db::users::get_user_by_id(pool, user_id).await?;
    let response = match person {
        Some(person) => Json(person).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    };
    Ok(response)
}

#[instrument(skip(pool, new_person))]
pub async fn root_post_handler(
    State(pool): State<PgPool>,
    Json(new_person): Json<NewUser>,
) -> Result<impl IntoResponse, AppError> {
    // Get any existing user with the same username
    let existing_user =
        crate::db::users::get_user_by_username(pool.clone(), new_person.username.clone()).await?;

    match existing_user {
        Some(user) => {
            let message = format!("user {} already exists", user.username);
            Ok((StatusCode::BAD_REQUEST, message).into_response())
        }
        None => {
            let person = crate::db::users::add_user(pool, &new_person).await?;
            let response = (StatusCode::CREATED, Json(person)).into_response();
            Ok(response)
        }
    }
}
