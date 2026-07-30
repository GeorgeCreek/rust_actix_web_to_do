#!/usr/bin/env bash
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../../../

# Reuse existing Postgres if already running; otherwise start compose
if ! docker ps --format '{{.Names}}' | grep -qx 'to-do-postgres'; then
  docker compose build
  docker compose up -d
fi
sleep 1
export $(cat ./nanoservices/to_do/.env | xargs)

cargo build \
--manifest-path ./nanoservices/to_do/networking/actix_server/Cargo.toml \
--features auth-http \
--release \
--no-default-features

cargo build \
--manifest-path ./nanoservices/auth/networking/actix_server/Cargo.toml \
--release

cargo run \
--manifest-path ./nanoservices/to_do/networking/actix_server/Cargo.toml \
--features auth-http \
--release --no-default-features &
TO_DO_PID=$!

cargo run \
--manifest-path ./nanoservices/auth/networking/actix_server/Cargo.toml \
--release &
AUTH_PID=$!

# Wait until both servers accept connections
for i in $(seq 1 30); do
  if curl -sf -o /dev/null http://127.0.0.1:8081/api/v1/auth/login || \
     curl -sf -o /dev/null http://127.0.0.1:8090/api/v1/get/all; then
    :
  fi
  if (echo >/dev/tcp/127.0.0.1/8081) 2>/dev/null && \
     (echo >/dev/tcp/127.0.0.1/8090) 2>/dev/null; then
    break
  fi
  sleep 1
done

curl -X POST http://127.0.0.1:8081/api/v1/users/create \
-H "Content-Type: application/json" \
-d '{
  "email": "test3@hotmail.com",
  "password": "password3"
}'

token=$(curl \
-u test3@hotmail.com:password3 \
-X GET http://127.0.0.1:8081/api/v1/auth/login)
token=$(echo "$token" | tr -d '\r\n' | sed 's/^"//' | sed 's/"$//')

response=$(curl -s -X POST http://127.0.0.1:8090/api/v1/create \
-H "Content-Type: application/json" \
-H "token: $token" \
-d '{
  "title": "run",
  "status": "DONE"
}')

sleep 1
echo "$response"
sleep 2

kill $TO_DO_PID
kill $AUTH_PID
# Do not tear down an externally managed Postgres container
