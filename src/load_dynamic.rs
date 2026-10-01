//! Native .so plugin loading via libloading.
//!
//! Demonstrates the full plugin lifecycle:
//!   1. dlopen the library
//!   2. Call `plugin_meta()` — read plugin identity
//!   3. Call `plugin_init(&HOST_API)` — plugin calls back into the host
//!   4. Call plugin functionality (math functions)
//!   5. drop(lib) — dlclose

use crate::extern_def::MathFn;
use libloading::{Library, Symbol};
use mysdk::api::{HostApi, LogLevel, PluginMeta};
use std::ffi::{CStr, c_char};

// ── Host API implementation ───────────────────────────────────────────────────

// These are the actual host functions the plugin is allowed to call back into.
// They are passed as a `HostApi` vtable — plugins never link against the host
// binary directly.

extern "C" fn host_log(level: LogLevel, msg: *const c_char) {
    // SAFETY: plugin guarantees msg is a valid nul-terminated UTF-8 string.
    let s = unsafe { CStr::from_ptr(msg) }
        .to_str()
        .unwrap_or("<invalid utf8>");
    println!("[host][{}] {s}", level.as_str());
}

extern "C" fn host_get_version() -> *const c_char {
    // 'static literal — safe to return a raw pointer.
    c"rust-load-plugins/1.0".as_ptr()
}

static HOST_API: HostApi = HostApi {
    log: host_log,
    get_version: host_get_version,
};

// ── Demo ──────────────────────────────────────────────────────────────────────

pub fn demo_dynamic(so_path: &std::path::Path) {
    println!("\n=== Dynamic (.so) — libloading / dlopen ===");
    println!("path : {}", so_path.display());

    // SAFETY: we supply the correct path and know the exported symbol signatures.
    let lib = unsafe { Library::new(so_path) }
        .unwrap_or_else(|e| panic!("dlopen failed for {}: {e}", so_path.display()));

    // 1. Read plugin identity.
    type MetaFn = extern "C" fn() -> PluginMeta;
    let meta: PluginMeta = {
        let f: Symbol<MetaFn> =
            unsafe { lib.get(b"plugin_meta\0") }.expect("plugin_meta symbol not found");
        f()
    };
    let name = unsafe { CStr::from_ptr(meta.name) }.to_str().unwrap_or("?");
    let version = unsafe { CStr::from_ptr(meta.version) }
        .to_str()
        .unwrap_or("?");
    println!("[plugin] {name} v{version}");

    // 2. Initialize plugin — it will call back into HOST_API.
    type InitFn = extern "C" fn(api: *const HostApi);
    let init: Symbol<InitFn> =
        unsafe { lib.get(b"plugin_init\0") }.expect("plugin_init symbol not found");
    init(&raw const HOST_API);

    // 3. Call plugin functionality.
    let call = |name: &[u8], a: i64, b: i64| -> i64 {
        // SAFETY: Symbol lifetime is tied to `lib`; `lib` outlives this closure.
        let f: Symbol<MathFn> =
            unsafe { lib.get(name) }.unwrap_or_else(|e| panic!("symbol {:?} not found: {e}", name));
        // SAFETY: function pointer valid while library is loaded.
        unsafe { f(a, b) }
    };

    println!("add(10, 3)      = {}", call(b"add\0", 10, 3));
    println!("subtract(10, 3) = {}", call(b"subtract\0", 10, 3));
    println!("multiply(10, 3) = {}", call(b"multiply\0", 10, 3));
    println!("divide(10, 3)   = {}", call(b"divide\0", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call(b"divide\0", 10, 0));

    drop(lib);
    println!("-> library unloaded (drop).");
}
