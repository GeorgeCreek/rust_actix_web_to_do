#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:8001}"
EMAIL="${EMAIL:-test@gmail.com}"
PASSWORD="${PASSWORD:-password}"

echo "Creating user ${EMAIL}..."
curl -sS -X POST "${BASE_URL}/api/v1/users/create" \
  -H "Content-Type: application/json" \
  -d "{\"email\": \"${EMAIL}\", \"password\": \"${PASSWORD}\"}"
echo

echo "Logging in..."
TOKEN=$(curl -sS -u "${EMAIL}:${PASSWORD}" \
  -X GET "${BASE_URL}/api/v1/auth/login" | tr -d '"')
echo "TOKEN=${TOKEN}"
