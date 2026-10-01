#!/usr/bin/env bash
# build-lib.sh — build all plugin artifacts into ./target/libs/
#
# Outputs:
#   libmyshared.so   — native cdylib (dlopen POC)
#   libmystatic.a    — native staticlib (link-time POC)
#   mywasm.wasm      — WASM plugin (wasmtime + wasmer POC)
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

echo "==> Building mywasm (cdylib -> .wasm, target: wasm32-unknown-unknown) ..."
cargo build --release \
    --manifest-path "$REPO_ROOT/mywasm/Cargo.toml" \
    --target wasm32-unknown-unknown
# The wasm output retains the cdylib name but with a .wasm extension.
cp "$REPO_ROOT/mywasm/target/wasm32-unknown-unknown/release/mywasm.wasm" "$OUT_DIR/"

echo ""
echo "Libraries written to $OUT_DIR/"
ls -lh "$OUT_DIR/"
