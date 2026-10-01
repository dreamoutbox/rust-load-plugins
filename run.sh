#!/usr/bin/env bash
# run.sh — build the main binary (with static lib linked in) then run it.
#
# Assumes build-lib.sh has already been run at least once.
# Re-runs it automatically if the libs are missing.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$REPO_ROOT/target/libs"

# Auto-build libs if they don't exist yet.
if [[ ! -f "$OUT_DIR/libmyshared.so" || ! -f "$OUT_DIR/libmystatic.a" ]]; then
    echo "==> Libraries not found, running build-lib.sh first ..."
    bash "$REPO_ROOT/build-lib.sh"
fi

echo "==> Building main binary (links mystatic.a at compile time) ..."
# MYSTATIC_DIR tells build.rs where to find libmystatic.a.
MYSTATIC_DIR="$OUT_DIR" cargo build --release

echo ""
echo "==> Running POC ..."
echo ""
# MYSHARED_PATH tells main.rs where to dlopen libmyshared.so at runtime.
MYSHARED_PATH="$OUT_DIR/libmyshared.so" \
    "$REPO_ROOT/target/release/rust-load-so"
