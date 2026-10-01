//! Lua plugin loading via **mlua** (Lua 5.4).
//!
//! The host registers `host_log` and `host_get_version` as Lua globals before
//! loading the script, so the plugin can call them during `plugin_init()` and
//! from any function.

use std::path::Path;

pub fn demo_mlua(script_path: &Path) {
    use mlua::prelude::*;

    println!("\n=== Lua plugin — mlua (Lua 5.4) ===");
    println!("path : {}", script_path.display());

    let lua = Lua::new();

    // Register host API as Lua globals — plugin calls these back into the host.
    lua.globals()
        .set(
            "host_log",
            lua.create_function(|_, (level, msg): (i64, String)| {
                let lvl = match level {
                    1 => "WARN",
                    2 => "ERROR",
                    _ => "INFO",
                };
                println!("[lua][{lvl}] {msg}");
                Ok(())
            })
            .unwrap(),
        )
        .unwrap();

    lua.globals()
        .set(
            "host_get_version",
            lua.create_function(|_, ()| Ok("rust-load-plugins/1.0".to_string()))
                .unwrap(),
        )
        .unwrap();

    // Load and execute the script — defines plugin functions as globals.
    let script = std::fs::read_to_string(script_path)
        .unwrap_or_else(|e| panic!("mlua: cannot read {:?}: {e}", script_path));
    lua.load(&script).exec().expect("mlua: script load failed");

    // Call plugin lifecycle init — Lua calls back via host_log/host_get_version.
    let init: LuaFunction = lua
        .globals()
        .get("plugin_init")
        .expect("plugin_init not found");
    init.call::<()>(()).expect("mlua: plugin_init failed");

    // Call plugin functionality.
    let call = |name: &str, a: i64, b: i64| -> i64 {
        let f: LuaFunction = lua
            .globals()
            .get(name)
            .unwrap_or_else(|e| panic!("mlua: global '{name}' not found: {e}"));
        f.call::<i64>((a, b))
            .unwrap_or_else(|e| panic!("mlua: '{name}' call failed: {e}"))
    };

    println!("add(10, 3)      = {}", call("add", 10, 3));
    println!("subtract(10, 3) = {}", call("subtract", 10, 3));
    println!("multiply(10, 3) = {}", call("multiply", 10, 3));
    println!("divide(10, 3)   = {}", call("divide", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call("divide", 10, 0));
    println!("-> Lua VM dropped (plugin unloaded).");
}
