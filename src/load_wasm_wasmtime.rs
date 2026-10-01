//! WASM plugin loading via **wasmtime**.
//!
//! Uses wasmtime's `Linker` to provide host imports before instantiation,
//! then calls `plugin_init` so the plugin can call back into the host.

use mysdk::wasm_imports;
use std::path::Path;

pub fn demo_wasmtime(wasm_path: &Path) {
    use wasmtime::{Caller, Engine, Linker, Module, Store};

    println!("\n=== WASM plugin — wasmtime ===");
    println!("path : {}", wasm_path.display());

    let engine = Engine::default();
    let module = Module::from_file(&engine, wasm_path)
        .unwrap_or_else(|e| panic!("wasmtime: failed to load {:?}: {e}", wasm_path));

    let mut store: Store<()> = Store::new(&engine, ());

    // Register host functions the plugin imports under the "host" module.
    let mut linker: Linker<()> = Linker::new(&engine);

    linker
        .func_wrap(
            wasm_imports::MODULE,
            wasm_imports::FN_LOG,
            |mut caller: Caller<'_, ()>, level: i32, ptr: i32, len: i32| {
                let mem = caller
                    .get_export("memory")
                    .and_then(|e| e.into_memory())
                    .expect("wasmtime: plugin has no 'memory' export");
                let data = mem.data(&caller);
                let msg = data
                    .get(ptr as usize..(ptr as usize + len as usize))
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .unwrap_or("<invalid>");
                let lvl = match level {
                    1 => "WARN",
                    2 => "ERROR",
                    _ => "INFO",
                };
                println!("[wasm][{lvl}] {msg}");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            wasm_imports::MODULE,
            wasm_imports::FN_GET_VERSION,
            |_: Caller<'_, ()>| -> i32 { 100 }, // 1.00
        )
        .unwrap();

    let instance = linker
        .instantiate(&mut store, &module)
        .expect("wasmtime: failed to instantiate module");

    // Call plugin lifecycle init — plugin calls back via host imports above.
    instance
        .get_typed_func::<(), ()>(&mut store, wasm_imports::FN_PLUGIN_INIT)
        .expect("wasmtime: plugin_init not found")
        .call(&mut store, ())
        .expect("wasmtime: plugin_init trapped");

    // Call plugin functionality.
    let mut call = |name: &str, a: i64, b: i64| -> i64 {
        instance
            .get_typed_func::<(i64, i64), i64>(&mut store, name)
            .unwrap_or_else(|e| panic!("wasmtime: export '{name}' not found: {e}"))
            .call(&mut store, (a, b))
            .unwrap_or_else(|e| panic!("wasmtime: '{name}' trapped: {e}"))
    };

    println!("add(10, 3)      = {}", call("add", 10, 3));
    println!("subtract(10, 3) = {}", call("subtract", 10, 3));
    println!("multiply(10, 3) = {}", call("multiply", 10, 3));
    println!("divide(10, 3)   = {}", call("divide", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call("divide", 10, 0));
    println!("-> module unloaded (store + module dropped at end of function).");
}
