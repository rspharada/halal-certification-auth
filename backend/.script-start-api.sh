#!/bin/bash
set -e

NETWORK_NAME="halmark_network"
PORT=8080
ENV_FILE="env.local.json"

# ネットワークが存在しない場合に作成
if ! docker network inspect "$NETWORK_NAME" > /dev/null 2>&1; then
  echo "Docker network '$NETWORK_NAME' not found. Creating..."
  docker network create "$NETWORK_NAME"
else
  echo "Docker network '$NETWORK_NAME' already exists."
fi

# SAM ローカル API 起動
echo "Starting SAM local API on port $PORT with --network=$NETWORK_NAME..."

sam local start-api \
  --port "$PORT" \
  --env-vars "$ENV_FILE" \
  --docker-network "$NETWORK_NAME"