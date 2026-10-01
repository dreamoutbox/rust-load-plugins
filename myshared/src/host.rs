//! Safe wrapper around host SDK vtable and FFI structures.

use std::ffi::{CStr, CString};
use std::sync::OnceLock;
use mysdk::api::{HostApi, LogLevel, PluginMeta};

static HOST_API: OnceLock<&'static HostApi> = OnceLock::new();

/// Construct plugin metadata safely without raw pointer manipulation in callers.
pub const fn make_meta(name: &'static CStr, version: &'static CStr) -> PluginMeta {
    PluginMeta {
        name: name.as_ptr(),
        version: version.as_ptr(),
    }
}

/// Initialize the host API reference.
pub fn init(api: *const HostApi) {
    if !api.is_null() {
        // SAFETY: caller guarantees the pointer points to a valid HostApi vtable.
        let api_ref = unsafe { &*api };
        let _ = HOST_API.set(api_ref);
    }
}

/// Send an info log message to the host.
pub fn host_log(msg: impl AsRef<str>) {
    host_log_level(LogLevel::Info, msg);
}

/// Send a log message with a specific level to the host.
pub fn host_log_level(level: LogLevel, msg: impl AsRef<str>) {
    if let Some(api) = HOST_API.get() {
        if let Ok(c_msg) = CString::new(msg.as_ref()) {
            (api.log)(level, c_msg.as_ptr());
        }
    }
}

/// Query the host version string.
pub fn host_get_version() -> &'static str {
    HOST_API
        .get()
        .map(|api| {
            let ptr = (api.get_version)();
            if ptr.is_null() {
                "unknown"
            } else {
                // SAFETY: HostApi implementation guarantees get_version returns a valid nul-terminated C string.
                unsafe { CStr::from_ptr(ptr) }
                    .to_str()
                    .unwrap_or("unknown")
            }
        })
        .unwrap_or("uninitialized")
}
