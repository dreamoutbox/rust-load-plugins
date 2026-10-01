//! Scripting plugin loading POC via **Rhai** (pure-Rust embedded engine).
//!
//! The plugin is `mylua/math_plugin.rhai`.  Rhai uses an AST-based approach:
//! the script is compiled into an AST once and functions are called by name
//! through the engine.  No C dependencies — fully self-contained.

use std::path::Path;

pub fn demo_rhai(script_path: &Path) {
    use rhai::{Engine, Scope};

    println!("\n=== Scripting plugin — Rhai ===");
    println!("path : {}", script_path.display());

    let engine = Engine::new();

    // Compile the script to an AST.  Errors here are caught at "load time",
    // not at call time — a key advantage over Lua's load-then-exec model.
    let ast = engine
        .compile_file(script_path.to_path_buf())
        .unwrap_or_else(|e| panic!("rhai: compile failed for {:?}: {e}", script_path));

    let scope = Scope::new();

    // Call a Rhai function defined in the AST by name.
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

    // Suppress unused-variable warning; scope intentionally empty for POC.
    let _ = scope;
}
