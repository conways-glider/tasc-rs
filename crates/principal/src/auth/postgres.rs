use anyhow::Context;
use sqlx::PgPool;
use tasc_rs_common::user::{User};
use tracing::{error, info, instrument};

#[instrument(skip(pool))]
pub async fn get_user_by_username(
    pool: PgPool,
    username: String,
) -> Result<Option<User>, anyhow::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_optional(&pool)
        .await
        .context("could not get user by username");

    // log results
    match result {
        Ok(_) => {
            info!("got user by username");
        }
        Err(ref err) => {
            error!(%err, "could not get user by username");
        }
    };
    result
}
