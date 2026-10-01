// ── wasmer ────────────────────────────────────────────────────────────────────

use std::path::Path;

/// Loads `wasm_path` with **wasmer** and exercises the four math exports.
pub fn demo_wasmer(wasm_path: &Path) {
    use wasmer::{Instance, Module, Store, TypedFunction, imports};

    println!("\n=== WASM plugin — wasmer ===");
    println!("path : {}", wasm_path.display());

    let wasm_bytes = std::fs::read(wasm_path)
        .unwrap_or_else(|e| panic!("wasmer: cannot read {:?}: {e}", wasm_path));

    let mut store = Store::default();
    let module =
        Module::new(&store, &wasm_bytes).unwrap_or_else(|e| panic!("wasmer: compile failed: {e}"));

    // No host imports needed; the plugin is self-contained.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)
        .expect("wasmer: failed to instantiate module");

    let mut call = |name: &str, a: i64, b: i64| -> i64 {
        let f: TypedFunction<(i64, i64), i64> = instance
            .exports
            .get_typed_function(&store, name)
            .unwrap_or_else(|e| panic!("wasmer: export '{name}' not found: {e}"));

        f.call(&mut store, a, b)
            .unwrap_or_else(|e| panic!("wasmer: '{name}' trapped: {e}"))
    };

    println!("add(10, 3)      = {}", call("add", 10, 3));
    println!("subtract(10, 3) = {}", call("subtract", 10, 3));
    println!("multiply(10, 3) = {}", call("multiply", 10, 3));
    println!("divide(10, 3)   = {}", call("divide", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call("divide", 10, 0));
    println!("-> module unloaded (store + module dropped at end of function).");
}
