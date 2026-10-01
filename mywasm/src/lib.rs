//! WASM plugin — same math surface as myshared but compiled to .wasm.
//!
//! Build target: wasm32-unknown-unknown  (no WASI needed; pure functions only)
//! The host (wasmtime or wasmer) loads the .wasm binary and calls these
//! functions by name through the runtime's typed-function API.
//!
//! No `extern "C"` linkage is needed: WASM exports are already name-stable
//! and ABI-stable across any compiler version.

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Integer division. Returns 0 on divide-by-zero — the WASM sandbox
/// already catches traps, but we avoid one for clarity.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
