//! Lua plugin loading POC via **mlua** (Lua 5.4, statically vendored).
//!
//! The plugin is `mylua/math_plugin.lua`.  The host creates a Lua VM,
//! loads the script file, then retrieves and calls each global function.
//! The VM is dropped at the end of the function — equivalent to "unloading"
//! the plugin.

use std::path::Path;

pub fn demo_mlua(script_path: &Path) {
    use mlua::prelude::*;

    println!("\n=== Lua plugin — mlua (Lua 5.4) ===");
    println!("path : {}", script_path.display());

    // One Lua VM per plugin load.  `Lua::new()` is cheap; state is isolated.
    let lua = Lua::new();

    let script = std::fs::read_to_string(script_path)
        .unwrap_or_else(|e| panic!("mlua: cannot read {:?}: {e}", script_path));

    // Execute the script to populate globals with the plugin's functions.
    lua.load(&script)
        .exec()
        .expect("mlua: script execution failed");

    // Helper: retrieve a global Lua function and call it with two i64 args.
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
