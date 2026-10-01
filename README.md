# Rust Runtime Plugin Loading POC

Proof-of-concept exploring dynamic runtime plugin architectures in Rust, inspired by Minecraft server plugin systems (such as Bukkit, Spigot, and Paper).

The host loads plugins at runtime, exports host capabilities through a unified SDK (`mysdk`), and triggers plugin lifecycle hooks (`plugin_init`) with bidirectional communication.

---

## Plugin Approaches

| Approach | Engine / Tooling | Sandbox & Safety | Performance | Modding Ergonomics |
| :--- | :--- | :--- | :--- | :--- |
| **Native Shared Library (`.so`)** | `libloading` (dlopen) | None (segfault crashes host) | Native (zero overhead) | Moderate (strict C ABI, Rust ABI unstable) |
| **WebAssembly (WASM)** | `wasmtime` / `wasmer` | Strong (fault isolation, capability-based) | Near-native (JIT / AOT) | Good (multi-language: Rust, C, Go, Zig) |
| **Lua Scripting** | `mlua` (Lua 5.4) | Moderate (configurable sandbox) | Interpreted / JIT | High (popular, fast iteration, no compile step) |
| **Rhai Scripting** | `rhai` (pure Rust) | High (sandboxed by design, fuel limits) | Interpreted | High (Rust-native syntax, easy host binding) |
| **Static Library (`.a`)** | rustc / linker | N/A (compile-time baseline) | Native | None (requires host recompile) |

---

## Architectural Breakdown

### 1. Native Shared Library (`.so` / `.dll`)
- **Loader**: `src/load_dynamic.rs` using `libloading`.
- **Plugin**: `myshared/` compiled as `cdylib`.
- **Mechanism**: Loads symbol tables via `dlopen`. The host passes a `HostApi` vtable pointer (`*const HostApi`) into the plugin's `plugin_init`, enabling the plugin to call host logging and version APIs directly without link-time dependencies.
- **Tradeoffs**:
  - *Pros*: Maximum throughput, zero serialization, direct memory access.
  - *Cons*: Any panic or memory fault crashes the host process. Rust has no stable ABI; interfaces must use `extern "C"` or C-compatible FFI types.

### 2. WebAssembly (WASM)
- **Loaders**:
  - `src/load_wasm_wasmtime.rs` (Wasmtime runtime, AOT/JIT, production standard).
  - `src/load_wasm_wasmer.rs` (Wasmer runtime, alternative multi-engine runtime).
- **Plugin**: `mywasm/` compiled with target `wasm32-unknown-unknown`.
- **Mechanism**: Host imports (`host_log`, `host_get_version`) are declared under module `"host"` and bound at instantiation time. The host reads strings directly from the WASM instance's linear memory.
- **Tradeoffs**:
  - *Pros*: Complete memory isolation, cross-platform `.wasm` bytecode, crashes do not terminate the host.
  - *Cons*: Boundary crossing requires marshaling pointers and lengths across linear memory.

### 3. Embedded Scripting (Lua & Rhai)
- **Loaders**:
  - `src/load_lua_mlua.rs` via `mlua` (Lua 5.4 runtime).
  - `src/load_lua_rhai.rs` via `rhai` (embedded scripting engine).
- **Plugins**: `mylua/math_plugin.lua` and `mylua/math_plugin.rhai`.
- **Mechanism**: The host registers host functions (`host_log`, `host_get_version`) directly into the script execution environment. The host invokes `plugin_init()` and exposed math functions by name.
- **Tradeoffs**:
  - *Pros*: Instant reloading without compilation, accessible to non-systems programmers.
  - *Cons*: Dynamic typing errors at runtime, lower execution speed for heavy computation.

### 4. Static Linking (Reference Baseline)
- **Loader**: `src/load_static.rs`.
- **Plugin**: `mystatic/` compiled as `staticlib` (`.a`) and linked at build time via `build.rs`. Included to contrast compile-time symbol resolution against dynamic loading.

---

## Repository Structure

```
├── mysdk/                    # Shared FFI types, HostApi vtable, WASM import constants
├── myshared/                 # Dynamic C-ABI plugin (.so)
├── mystatic/                 # Static C-ABI library (.a)
├── mywasm/                   # WASM plugin (target: wasm32-unknown-unknown)
├── mylua/                    # Lua and Rhai script plugins
│   ├── math_plugin.lua
│   └── math_plugin.rhai
├── src/
│   ├── main.rs               # Entrypoint running all loader POCs
│   ├── extern_def.rs         # Shared FFI function signatures
│   ├── load_dynamic.rs       # libloading dynamic .so loader
│   ├── load_static.rs        # Compile-time static link loader
│   ├── load_wasm_wasmtime.rs # Wasmtime WASM loader
│   ├── load_wasm_wasmer.rs   # Wasmer WASM loader
│   ├── load_lua_mlua.rs      # mlua Lua loader
│   └── load_lua_rhai.rs      # Rhai script loader
├── build-lib.sh              # Compiles static, shared, and wasm artifacts
└── run.sh                    # Builds libraries and host, then executes all POCs
```

---

## Getting Started

### Prerequisites
- Rust toolchain (edition 2024 compatible)
- WASM target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Build & Run
Run all plugin loaders end-to-end:
```bash
./run.sh
```

Or build plugin artifacts manually:
```bash
./build-lib.sh
cargo run --release
```
