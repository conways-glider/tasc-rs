#!/usr/bin/env bash

set -eou pipefail

# ./start_principal_db.sh
source $(dirname $0)/start_principal_db.sh

export RUST_LOG="tasc_rs_principal=debug,tower_http=debug,axum::rejection=trace"
cargo run --bin tasc-rs-principal -- -c ./crates/principal/local_config.toml
