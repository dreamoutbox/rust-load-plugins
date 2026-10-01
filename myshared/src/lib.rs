//! Native shared library plugin (.so).
//!
//! Exports:
//!   plugin_meta()          — identity (name + version)
//!   plugin_init(api)       — lifecycle init; receives host vtable, calls back
//!   add/subtract/multiply/divide — the actual plugin functionality

use std::ffi::{CStr, CString};
use mysdk::api::{HostApi, LogLevel, PluginMeta};

// ── Plugin lifecycle ──────────────────────────────────────────────────────────

/// Returns plugin identity. Called by the host before `plugin_init`.
#[unsafe(no_mangle)]
pub extern "C" fn plugin_meta() -> PluginMeta {
    PluginMeta {
        name:    c"myshared-math".as_ptr(),
        version: c"1.0.0".as_ptr(),
    }
}

/// Called once by the host after loading.  Receives the host vtable and uses
/// it to log and query the host version — plugin→host callbacks.
///
/// SAFETY: `api` must be a valid pointer for at least the duration of this call.
#[unsafe(no_mangle)]
pub extern "C" fn plugin_init(api: *const HostApi) {
    let api = unsafe { &*api };

    (api.log)(LogLevel::Info, c"myshared math plugin: init called".as_ptr());

    // Call back to get the host version, then log it.
    let version_ptr = (api.get_version)();
    let version = unsafe { CStr::from_ptr(version_ptr) }.to_str().unwrap_or("?");
    // CString keeps the allocation alive for the duration of the log call.
    let msg = CString::new(format!("myshared math plugin: host is {version}")).unwrap();
    (api.log)(LogLevel::Info, msg.as_ptr());
}

// ── Plugin functionality ──────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i64, b: i64) -> i64 { a + b }

#[unsafe(no_mangle)]
pub extern "C" fn subtract(a: i64, b: i64) -> i64 { a - b }

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i64, b: i64) -> i64 { a * b }

/// Returns 0 on divide-by-zero so the host process stays alive even on bad input.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
