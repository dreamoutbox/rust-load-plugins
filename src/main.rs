mod extern_def;
mod load_dynamic;
mod load_lua_mlua;
mod load_lua_rhai;
mod load_static;
mod load_wasm_wasmer;
mod load_wasm_wasmtime;

use std::path::PathBuf;

/// Resolve an env-var path, falling back to `fallback` relative to the exe dir.
fn resolve_path(env_var: &str, fallback: &str) -> PathBuf {
    std::env::var(env_var)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let exe = std::env::current_exe().expect("cannot resolve exe path");
            exe.parent().expect("exe has no parent dir").join(fallback)
        })
}

fn main() {
    println!("Rust plugin POC — dynamic / static / wasm / scripting");
    println!("======================================================");

    let so_path = resolve_path("MYSHARED_PATH", "libmyshared.so");
    let wasm_path = resolve_path("MYWASM_PATH", "mywasm.wasm");
    let lua_path = resolve_path("MYLUA_PATH", "math_plugin.lua");
    let rhai_path = resolve_path("MYRHAI_PATH", "math_plugin.rhai");

    // 1. Native .so via libloading / dlopen
    load_dynamic::demo_dynamic(&so_path);

    // 2. Baked-in static lib symbols
    load_static::demo_static();

    // 3. WASM plugin — wasmtime
    load_wasm_wasmtime::demo_wasmtime(&wasm_path);

    // 4. WASM plugin — wasmer
    load_wasm_wasmer::demo_wasmer(&wasm_path);

    // 5. Lua plugin — mlua (Lua 5.4)
    load_lua_mlua::demo_mlua(&lua_path);

    // 6. Scripting plugin — Rhai
    load_lua_rhai::demo_rhai(&rhai_path);

    println!("\nDone.");
}
