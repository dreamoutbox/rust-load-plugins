//! Names for host-provided imports in WASM plugins.
//!
//! The host registers these under the `MODULE` import module when it
//! instantiates a `.wasm` plugin.  WASM plugin authors declare them as:
//!
//! ```rust,no_run
//! #[link(wasm_import_module = "host")]
//! unsafe extern "C" {
//!     // (level: i32, msg_ptr: i32, msg_len: i32) -> ()
//!     // level: 0=INFO 1=WARN 2=ERROR
//!     // msg_ptr/msg_len: UTF-8 slice in WASM linear memory
//!     fn host_log(level: i32, msg_ptr: i32, msg_len: i32);
//!
//!     // () -> i32   (major * 100 + minor)
//!     fn host_get_version() -> i32;
//! }
//! ```

/// WASM import module all host functions are registered under.
pub const MODULE: &str = "host";

/// Log a message. Signature: `(level: i32, msg_ptr: i32, msg_len: i32)`
pub const FN_LOG: &str = "host_log";

/// Get host version as integer (major * 100 + minor). Signature: `() -> i32`
pub const FN_GET_VERSION: &str = "host_get_version";

/// Plugin lifecycle init called by host after instantiation. Signature: `()`
pub const FN_PLUGIN_INIT: &str = "plugin_init";
