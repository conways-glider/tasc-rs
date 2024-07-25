#!/usr/bin/env bash

set -eou pipefail

docker compose -f ./crates/principal/docker-compose.yaml down -v
