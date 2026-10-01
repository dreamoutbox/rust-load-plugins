//! WASM plugin — same math surface as myshared but compiled to .wasm.

mod host;

use host::{host_get_version, host_log};

// ── Plugin lifecycle ──────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn plugin_init() {
    let msg = "mywasm math plugin: init called";
    host_log(msg);

    let ver = host_get_version();
    let msg2 = format!("mywasm math plugin: host version {}", ver);
    host_log(msg2);
}

// ── Plugin functionality ──────────────────────────────────────────────────────

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

/// Returns 0 on divide-by-zero.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
