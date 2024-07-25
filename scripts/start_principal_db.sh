#!/usr/bin/env bash

set -eou pipefail

docker compose -f ./crates/principal/docker-compose.yaml up -d

export DATABASE_URL=postgres://postgres:password@localhost:5432
cargo sqlx migrate run --source crates/principal/migrations
