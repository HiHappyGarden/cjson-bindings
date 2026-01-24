#!/usr/bin/env bash
set -euo pipefail

# Simple helper to build cJSON locally and run `cargo test` for `cjson-rs` linking against
# the built cJSON artifacts. This mirrors the steps used in the project CI/local workflow.

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_HOST="$ROOT/build-host"
CJSON_DIR_DEFAULT="$BUILD_HOST/cJSON"
CJSON_DIR="${CJSON_DIR:-$CJSON_DIR_DEFAULT}"
CJSON_BUILD="$CJSON_DIR/build"

echo "Using cJSON dir: $CJSON_DIR"

if [ ! -d "$CJSON_DIR" ]; then
  echo "Cloning cJSON into $CJSON_DIR..."
  git clone --depth 1 --branch v1.7.19 https://github.com/DaveGamble/cJSON.git "$CJSON_DIR"
fi

echo "Configuring cJSON (build dir: $CJSON_BUILD)..."
cmake -S "$CJSON_DIR" -B "$CJSON_BUILD" -DBUILD_SHARED_AND_STATIC_LIBS=OFF -DENABLE_CJSON_UTILS=ON -DENABLE_CJSON_TEST=OFF

echo "Building cJSON..."
cmake --build "$CJSON_BUILD" -- -j

# Decide linking mode: dynamic by default, set STATIC_LINK=1 to use static linking
if [ "${STATIC_LINK:-0}" = "1" ]; then
  echo "Using static linking for cjson"
  export RUSTFLAGS="-L native=$CJSON_BUILD -l static=cjson -l static=cjson_utils ${RUSTFLAGS:-}"
else
  echo "Using dynamic linking for cjson"
  export LD_LIBRARY_PATH="$CJSON_BUILD:${LD_LIBRARY_PATH:-}"
  export RUSTFLAGS="-L native=$CJSON_BUILD -l cjson -l cjson_utils ${RUSTFLAGS:-}"
fi

echo "Running cargo test for cjson-rs..."
cd "$ROOT/cjson-rs"
cargo test --lib

echo "Done."
