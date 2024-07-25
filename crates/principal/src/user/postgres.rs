use anyhow::Context;
use sqlx::PgPool;
use tasc_rs_common::user::{NewUser, User};
use tracing::{error, info, instrument};

#[instrument(skip(pool))]
pub async fn get_users(pool: PgPool) -> Result<Vec<User>, anyhow::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .context("could not get people");

    // log results
    match result {
        Ok(_) => {
            info!("got people");
        }
        Err(ref err) => {
            error!(%err, "could not get people");
        }
    };
    result
}

#[instrument(skip(pool))]
pub async fn get_user_by_id(
    pool: PgPool,
    user_id: uuid::Uuid,
) -> Result<Option<User>, anyhow::Error> {
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&pool)
        .await
        .context("could not get user by id");

    // log results
    match result {
        Ok(_) => {
            info!("got user by id");
        }
        Err(ref err) => {
            error!(%err, "could not get user by id");
        }
    };
    result
}

#[instrument(skip(pool))]
pub async fn add_user(pool: PgPool, new_user: &NewUser) -> Result<User, anyhow::Error> {
    let uuid = new_user.get_uuid();

    let passhash = new_user.get_pass_hash()?;

    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, name, passhash) VALUES ($1, $2, $3, $4) RETURNING *",
        &uuid,
        &new_user.username,
        &new_user.name,
        passhash
    )
    .fetch_one(&pool)
    .await
    .context("could not add user");

    // log results
    match result {
        Ok(user) => {
            info!(
                id = user.id.to_string(),
                username = user.username,
                "created user"
            );
            Ok(user)
        }
        Err(err) => {
            error!(%err, "could not create user");
            Err(err)
        }
    }
}
