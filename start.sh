#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG="${CONFIG:-$ROOT_DIR/rustpress.toml}"
HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-5177}"

cd "$ROOT_DIR"

echo "Starting RustPress docs at http://$HOST:$PORT/"
exec cargo run -p rustpress-cli -- dev \
  --config "$CONFIG" \
  --host "$HOST" \
  --port "$PORT"
