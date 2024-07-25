#![forbid(unsafe_code)]

use std::{path::PathBuf, time::Duration};

use anyhow::{anyhow, Context};
use axum::{extract::MatchedPath, http::Request, Router};
use clap::{arg, command, value_parser};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing::{debug, error, info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod error;
mod user;

const DEFAULT_LOG_CONFIG: &str = "tasc_rs_principal=info,tower_http=info,axum::rejection=trace";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                DEFAULT_LOG_CONFIG.into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // get config path
    let default_config_path = config::get_default_config_file();
    let config_path = matches
        .get_one::<PathBuf>("config")
        .or(default_config_path.as_ref())
        .context("could not get config path")?;

    // load config
    let config = config::get_config(config_path)?;
    debug!(config=?config, "loaded config");

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        // .max_lifetime(None)
        // .idle_timeout(None)
        .connect(&config.database.url)
        .await
        .context("could not connect to database")?;

    // handle migrations
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => {
            info!("successfully ran db migrations");
            Ok(())
        }
        Err(err) => {
            error!(?err, "could not run db migrations");
            Err(anyhow!("could not run db migrations"))
        }
    }?;

    // build our application with a route
    let app = Router::new()
        .nest("/auth", auth::get_app())
        .nest("/users", user::get_app())
        .nest_service("/tasks", ServeDir::new("tasks"))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                )
            }),
        )
        .layer(CompressionLayer::new())
        .with_state(pool);

    // run it
    let listener = TcpListener::bind(config.route())
        .await
        .context("could not bind listener")?;
    let local_address = listener
        .local_addr()
        .context("could not get local address")?;

    info!("listening on {}", local_address);

    axum::serve(listener, app)
        .await
        .context("could not start server")?;
    Ok(())
}
