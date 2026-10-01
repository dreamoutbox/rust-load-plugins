//! Host API types shared by native (.so) plugins and the host binary.
//!
//! These types are `repr(C)` so they survive the `dlopen` ABI boundary.
//! WASM plugins do NOT use these directly — see [`crate::wasm_imports`].

use core::ffi::c_char;

/// Severity level for [`HostApi::log`].
///
/// `repr(u8)` ensures it passes over the `extern "C"` boundary as a single
/// byte regardless of Rust's internal enum layout.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info = 0,
    Warn = 1,
    Error = 2,
}

impl LogLevel {
    pub fn from_raw(v: u8) -> Self {
        match v {
            1 => Self::Warn,
            2 => Self::Error,
            _ => Self::Info,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

/// ABI-stable vtable the host passes to every native plugin at `plugin_init`.
///
/// Analogous to Minecraft's `Server` instance given to `JavaPlugin.onEnable()`.
/// All fields are `extern "C"` function pointers — the only thing that is
/// guaranteed safe across a `dlopen` boundary in Rust.
#[repr(C)]
pub struct HostApi {
    /// Emit a structured log line.
    /// `msg` must be a valid, nul-terminated UTF-8 C string.
    pub log: extern "C" fn(level: LogLevel, msg: *const c_char),

    /// Return the host's version as a static, nul-terminated ASCII string.
    /// The returned pointer is valid for the entire process lifetime.
    pub get_version: extern "C" fn() -> *const c_char,
}

// SAFETY: HostApi only contains fn pointers, which are Send + Sync.
unsafe impl Send for HostApi {}
unsafe impl Sync for HostApi {}

/// Identity metadata every native plugin must export via `plugin_meta()`.
///
/// Both pointers must point to `'static` nul-terminated string literals.
#[repr(C)]
pub struct PluginMeta {
    pub name: *const c_char,
    pub version: *const c_char,
}

// SAFETY: both fields are pointers to static string literals — never mutated.
unsafe impl Send for PluginMeta {}
unsafe impl Sync for PluginMeta {}
