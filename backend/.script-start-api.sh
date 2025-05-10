#!/bin/bash
set -e

# SAM API ローカル起動スクリプト
echo "Starting SAM local API on port 8080 with --network=halal..."

sam local start-api \
  --port 8080 \
  --env-vars env.local.json \
  --docker-network halal_default