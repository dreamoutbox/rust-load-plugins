//! WASM plugin — same math surface as myshared but compiled to .wasm.
//!
//! Host imports (from "host" module per mysdk::wasm_imports):
//!   host_log(level: i32, msg_ptr: i32, msg_len: i32)
//!   host_get_version() -> i32   (major * 100 + minor)
//!
//! The host resolves these at instantiation time via its linker / import_object.
//! There is no link-time check — missing imports fail at runtime.

// Declare the functions the host must provide under the "host" WASM module.
#[link(wasm_import_module = "host")]
unsafe extern "C" {
    // level: 0=INFO, 1=WARN, 2=ERROR
    // msg_ptr / msg_len: UTF-8 byte slice in WASM linear memory
    fn host_log(level: i32, msg_ptr: i32, msg_len: i32);

    // Returns host version as integer: major * 100 + minor
    fn host_get_version() -> i32;
}

// ── Plugin lifecycle ──────────────────────────────────────────────────────────

/// Called by the host after instantiation.  Uses host imports to call back.
#[unsafe(no_mangle)]
pub extern "C" fn plugin_init() {
    let msg = b"mywasm math plugin: init called";
    unsafe { host_log(0, msg.as_ptr() as i32, msg.len() as i32) };

    let ver = unsafe { host_get_version() };
    let msg2 = format!("mywasm math plugin: host version {}.{}", ver / 100, ver % 100);
    let bytes = msg2.as_bytes();
    unsafe { host_log(0, bytes.as_ptr() as i32, bytes.len() as i32) };
}

// ── Plugin functionality ──────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i64, b: i64) -> i64 { a + b }

#[unsafe(no_mangle)]
pub extern "C" fn subtract(a: i64, b: i64) -> i64 { a - b }

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i64, b: i64) -> i64 { a * b }

/// Returns 0 on divide-by-zero.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
