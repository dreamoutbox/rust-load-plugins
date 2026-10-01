//! `mysdk` — shared host API contract used by all plugin types.
//!
//! - Native (.so) plugins: depend on this crate and use [`api::HostApi`] +
//!   [`api::PluginMeta`].
//! - WASM plugins: use [`wasm_imports`] constants as a reference for the
//!   expected import module/function names (resolved at runtime by the host).
//! - Lua / Rhai plugins: host registers globals with the same names as
//!   [`wasm_imports`] for consistency.

pub mod api;
pub mod wasm_imports;
