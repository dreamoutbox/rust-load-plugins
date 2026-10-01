//! WASM plugin loading via **wasmer**.
//!
//! Uses `FunctionEnv` to give host import callbacks access to WASM linear
//! memory.  The chicken-and-egg problem (memory only exists after instantiation,
//! but we need imports before instantiation) is solved by initialising the env
//! with `Option<Memory>` and filling it in right after `Instance::new`.

use mysdk::wasm_imports;
use std::path::Path;

/// State shared between host import closures and accessible after instantiation.
struct HostEnv {
    memory: Option<wasmer::Memory>,
}

fn level_str(level: i32) -> &'static str {
    match level {
        1 => "WARN",
        2 => "ERROR",
        _ => "INFO",
    }
}

pub fn demo_wasmer(wasm_path: &Path) {
    use wasmer::{
        Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, TypedFunction, imports,
    };

    println!("\n=== WASM plugin — wasmer ===");
    println!("path : {}", wasm_path.display());

    let wasm_bytes = std::fs::read(wasm_path)
        .unwrap_or_else(|e| panic!("wasmer: cannot read {:?}: {e}", wasm_path));

    let mut store = Store::default();
    let module =
        Module::new(&store, &wasm_bytes).unwrap_or_else(|e| panic!("wasmer: compile failed: {e}"));

    // Env holds the memory handle; starts empty, filled after instantiation.
    let host_env = FunctionEnv::new(&mut store, HostEnv { memory: None });

    // host_log: read a UTF-8 slice from WASM memory and print it.
    let log_fn = Function::new_typed_with_env(
        &mut store,
        &host_env,
        |env: FunctionEnvMut<HostEnv>, level: i32, ptr: i32, len: i32| {
            let memory = env
                .data()
                .memory
                .clone()
                .expect("wasmer: memory not yet initialised");
            let view = memory.view(&env);
            let mut buf = vec![0u8; len as usize];
            view.read(ptr as u64, &mut buf)
                .expect("wasmer: memory read failed");
            let msg = std::str::from_utf8(&buf).unwrap_or("<invalid utf8>");
            println!("[wasm][{}] {msg}", level_str(level));
        },
    );

    // host_get_version: returns 100 (= version 1.0).
    let get_version_fn = Function::new_typed_with_env(
        &mut store,
        &host_env,
        |_env: FunctionEnvMut<HostEnv>| -> i32 { 100 },
    );

    let import_object = imports! {
        wasm_imports::MODULE => {
            wasm_imports::FN_LOG         => log_fn,
            wasm_imports::FN_GET_VERSION => get_version_fn,
        }
    };

    let instance = Instance::new(&mut store, &module, &import_object)
        .expect("wasmer: failed to instantiate module");

    // Wire up memory now that the instance exists.
    let memory = instance
        .exports
        .get_memory("memory")
        .expect("wasmer: plugin has no 'memory' export")
        .clone();
    host_env.as_mut(&mut store).memory = Some(memory);

    // Call plugin lifecycle init — triggers the callbacks above.
    let init: TypedFunction<(), ()> = instance
        .exports
        .get_typed_function(&store, wasm_imports::FN_PLUGIN_INIT)
        .expect("wasmer: plugin_init not found");
    init.call(&mut store).expect("wasmer: plugin_init trapped");

    // Call plugin functionality.
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
