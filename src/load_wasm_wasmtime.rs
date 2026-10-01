//! WASM plugin loading POC — demonstrates two runtimes side by side.
//!
//! Both runtimes load the same `mywasm.wasm` binary and call the same
//! exported functions.  The wasm file is built by `build-lib.sh` and
//! placed at `target/libs/mywasm.wasm`.

use std::path::Path;

// ── wasmtime ─────────────────────────────────────────────────────────────────

/// Loads `wasm_path` with **wasmtime** and exercises the four math exports.
pub fn demo_wasmtime(wasm_path: &Path) {
    use wasmtime::{Engine, Instance, Module, Store};

    println!("\n=== WASM plugin — wasmtime ===");
    println!("path : {}", wasm_path.display());

    let engine = Engine::default();
    let module = Module::from_file(&engine, wasm_path)
        .unwrap_or_else(|e| panic!("wasmtime: failed to load {:?}: {e}", wasm_path));

    let mut store: Store<()> = Store::new(&engine, ());
    let instance =
        Instance::new(&mut store, &module, &[]).expect("wasmtime: failed to instantiate module");

    // Resolve each export as a typed function and call it.
    // The WASM ABI for i64 params/return maps directly to Rust i64.
    let mut call = |name: &str, a: i64, b: i64| -> i64 {
        let f = instance
            .get_typed_func::<(i64, i64), i64>(&mut store, name)
            .unwrap_or_else(|e| panic!("wasmtime: export '{name}' not found: {e}"));
        f.call(&mut store, (a, b))
            .unwrap_or_else(|e| panic!("wasmtime: '{name}' trapped: {e}"))
    };

    println!("add(10, 3)      = {}", call("add", 10, 3));
    println!("subtract(10, 3) = {}", call("subtract", 10, 3));
    println!("multiply(10, 3) = {}", call("multiply", 10, 3));
    println!("divide(10, 3)   = {}", call("divide", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call("divide", 10, 0));
    println!("-> module unloaded (store + module dropped at end of function).");
}
