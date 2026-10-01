#!/usr/bin/env bash
# build-lib.sh — build myshared (.so) and mystatic (.a) independently.
#
# Outputs land in ./target/libs/ so run.sh and the main binary know
# exactly where to find them without any install step.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$REPO_ROOT/target/libs"

mkdir -p "$OUT_DIR"

echo "==> Building myshared (cdylib -> .so) ..."
cargo build --release --manifest-path "$REPO_ROOT/myshared/Cargo.toml"
cp "$REPO_ROOT/myshared/target/release/libmyshared.so" "$OUT_DIR/"

echo "==> Building mystatic (staticlib -> .a) ..."
cargo build --release --manifest-path "$REPO_ROOT/mystatic/Cargo.toml"
cp "$REPO_ROOT/mystatic/target/release/libmystatic.a" "$OUT_DIR/"

echo ""
echo "Libraries written to $OUT_DIR/"
ls -lh "$OUT_DIR/"
