#!/usr/bin/env bash
# run.sh — build the main binary then run all plugin POCs.
#
# Assumes build-lib.sh has already been run at least once.
# Re-runs it automatically if any artifact is missing.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$REPO_ROOT/target/libs"

# Auto-build native/wasm libs if they don't exist yet.
if [[ ! -f "$OUT_DIR/libmyshared.so" || \
      ! -f "$OUT_DIR/libmystatic.a"  || \
      ! -f "$OUT_DIR/mywasm.wasm"    ]]; then
    echo "==> Artifacts not found, running build-lib.sh first ..."
    bash "$REPO_ROOT/build-lib.sh"
fi

echo "==> Building main binary (links mystatic.a at compile time) ..."
# MYSTATIC_DIR tells build.rs where to find libmystatic.a.
MYSTATIC_DIR="$OUT_DIR" cargo build --release

echo ""
echo "==> Running POC ..."
echo ""

# Each env var overrides the default path inside the binary.
MYSHARED_PATH="$OUT_DIR/libmyshared.so" \
MYWASM_PATH="$OUT_DIR/mywasm.wasm" \
    "$REPO_ROOT/target/release/rust-load-so"
