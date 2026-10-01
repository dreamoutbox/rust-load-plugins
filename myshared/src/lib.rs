//! Native shared library plugin (.so).
//!
//! Exports:
//!   plugin_meta()          — identity (name + version)
//!   plugin_init(api)       — lifecycle init; receives host vtable, calls back
//!   add/subtract/multiply/divide — the actual plugin functionality

mod host;

use mysdk::api::{HostApi, PluginMeta};
use host::{host_get_version, host_log, make_meta};

// ── Plugin lifecycle ──────────────────────────────────────────────────────────

/// Returns plugin identity. Called by the host before `plugin_init`.
#[unsafe(no_mangle)]
pub extern "C" fn plugin_meta() -> PluginMeta {
    make_meta(c"myshared-math", c"1.0.0")
}

/// Called once by the host after loading.  Receives the host vtable and uses
/// it to log and query the host version — plugin→host callbacks.
#[unsafe(no_mangle)]
pub extern "C" fn plugin_init(api: *const HostApi) {
    host::init(api);

    let msg = "myshared math plugin: init called";
    host_log(msg);

    let ver = host_get_version();
    let msg2 = format!("myshared math plugin: host is {ver}");
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

/// Returns 0 on divide-by-zero so the host process stays alive even on bad input.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
