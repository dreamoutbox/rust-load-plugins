//! Scripting plugin loading via **Rhai**.
//!
//! The host registers `host_log` and `host_get_version` as native functions
//! on the engine before compiling the script.  The plugin calls them from
//! `plugin_init()`.

use std::path::Path;

pub fn demo_rhai(script_path: &Path) {
    use rhai::{Engine, Scope};

    println!("\n=== Scripting plugin — Rhai ===");
    println!("path : {}", script_path.display());

    let mut engine = Engine::new();

    // Register host API as Rhai native functions.
    engine.register_fn("host_log", |level: i64, msg: &str| {
        let lvl = match level {
            1 => "WARN",
            2 => "ERROR",
            _ => "INFO",
        };
        println!("[rhai][{lvl}] {msg}");
    });
    engine.register_fn("host_get_version", || "rust-load-so/1.0");

    // Compile — errors are caught here, not at call time.
    let ast = engine
        .compile_file(script_path.to_path_buf())
        .unwrap_or_else(|e| panic!("rhai: compile failed for {:?}: {e}", script_path));

    let mut scope = Scope::new();

    // Call plugin lifecycle init — Rhai script calls back via registered fns.
    engine
        .call_fn::<()>(&mut scope, &ast, "plugin_init", ())
        .expect("rhai: plugin_init failed");

    // Call plugin functionality.
    let call = |name: &str, a: i64, b: i64| -> i64 {
        engine
            .call_fn::<i64>(&mut scope.clone(), &ast, name, (a, b))
            .unwrap_or_else(|e| panic!("rhai: '{name}' call failed: {e}"))
    };

    println!("add(10, 3)      = {}", call("add", 10, 3));
    println!("subtract(10, 3) = {}", call("subtract", 10, 3));
    println!("multiply(10, 3) = {}", call("multiply", 10, 3));
    println!("divide(10, 3)   = {}", call("divide", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call("divide", 10, 0));
    println!("-> Rhai engine + AST dropped (plugin unloaded).");
}
