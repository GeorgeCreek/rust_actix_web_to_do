#!/usr/bin/env bash
# scripts/run_tests.sh
set -euo pipefail

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd "$SCRIPTPATH/.."

if [ -d "./logs" ]; then
  echo "Removing existing log directory: ./logs"
  rm -rf "./logs"
fi
mkdir logs

STARTED_POSTGRES=0

postgres_ready() {
  # Prefer the known container name (may pre-exist outside this compose project).
  if docker ps --format '{{.Names}}' | grep -qx 'to-do-postgres'; then
    docker exec to-do-postgres pg_isready -U username -d to_do >/dev/null 2>&1
    return $?
  fi
  # Fall back to compose service if present.
  docker compose exec -T postgres pg_isready -U username -d to_do >/dev/null 2>&1
}

ensure_postgres() {
  if postgres_ready; then
    echo "Using existing PostgreSQL (to-do-postgres)."
    return 0
  fi

  echo "Starting PostgreSQL via docker compose..."
  # Only start postgres so we don't recreate elasticsearch/kibana/cache.
  if ! docker compose up -d postgres; then
    echo "docker compose failed (likely a name conflict). Trying existing container..."
    if docker ps -a --format '{{.Names}}' | grep -qx 'to-do-postgres'; then
      docker start to-do-postgres >/dev/null
    else
      echo "No usable PostgreSQL container found." >&2
      exit 1
    fi
  else
    STARTED_POSTGRES=1
  fi

  echo "Waiting for PostgreSQL to be ready..."
  until postgres_ready; do
    echo -n "."
    sleep 1
  done
  echo
  echo "PostgreSQL is ready."
}

# Load env if present (optional for unit tests that don't need DB).
if [ -f .env ]; then
  set -a
  # shellcheck disable=SC1091
  source .env
  set +a
fi
# DAL tests expect TO_DO_DB_URL (from nanoservices/to_do/.env or ingress/.env).
if [ -z "${TO_DO_DB_URL:-}" ]; then
  if [ -f nanoservices/to_do/.env ]; then
    set -a
    # shellcheck disable=SC1091
    source nanoservices/to_do/.env
    set +a
  elif [ -f ingress/.env ]; then
    set -a
    # shellcheck disable=SC1091
    source ingress/.env
    set +a
  fi
fi
if [ -z "${TO_DO_DB_URL:-}" ]; then
  export TO_DO_DB_URL="${DATABASE_URL:-postgres://username:password@localhost:5432/to_do}"
fi

cargo test -p to-do-core > ./logs/to-do-core.log
cargo test -p to-do-actix-server > ./logs/to-do-actix-server.log

ensure_postgres

export TO_DO_MAX_CONNECTIONS=1
cargo test -p to-do-dal --features sqlx-postgres \
  -- --test-threads=1 > ./logs/to-do-dal.log

# Only stop postgres if this script started it; never tear down the shared stack.
if [ "$STARTED_POSTGRES" -eq 1 ]; then
  docker compose stop postgres >/dev/null || true
fi

echo "Tests completed."
echo "Logs saved to ./logs"
