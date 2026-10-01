# Rust Plugin System Architectures

> Context: this repo (`rust-load-so`) is a POC that uses `libloading` to
> `dlopen` a shared library at runtime and resolve symbols by name — the
> raw building-block of approach 1 below.
> The question: is that the right foundation for a real "Minecraft-style"
> plugin system, or is there something better?

---

## The Minecraft Model (what we're aiming for)

- Drop a `.jar` (or `.so`/`.dll`) into a folder.
- The host discovers and loads it at runtime — no recompile needed.
- Each plugin implements a known interface (`onLoad`, `onEnable`, `onDisable`, …).
- Plugins can be hotswapped or unloaded independently.
- Plugins can optionally call back into the host's API.

---

## Option 1 — Raw `dlopen` via `libloading` (current POC approach)

**How it works:**  
Load a `.so`/`.dll` at runtime, look up symbols by name as function pointers,
call them through an `unsafe` C-ABI boundary.

```rust
let lib = unsafe { Library::new("plugin.so") }?;
let init: Symbol<unsafe extern "C" fn()> = unsafe { lib.get(b"plugin_init\0") }?;
unsafe { init() };
```

**Pros**
- Zero external tooling. Works everywhere `dlopen` works.
- Full control over load/unload lifecycle.
- Natural for C-ABI plugins (interop with non-Rust code).

**Cons**
- Every symbol lookup is a stringly-typed runtime operation — no compile-time safety.
- Plugins and host must be compiled with the same Rust version / stdlib ABI
  (Rust has **no stable ABI**; mixing compiler versions is UB unless you stay
  on `extern "C"` boundaries throughout).
- You manually manage version negotiation, metadata discovery, and error reporting.
- Unloading a library while any reference to its code/data is still live is UB.

**When to use**  
Lowest-level interop; C/C++ plugins; when you need maximum ABI control.

---

## Option 2 — `libloading` + Trait Object over `extern "C"` factory

**How it works:**  
Export a single `extern "C"` factory function from each plugin that returns a
`Box<dyn Plugin>` (as a raw pointer). The host reconstructs the fat pointer.

```rust
// plugin crate
#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin))
}

// host
type Factory = unsafe extern "C" fn() -> *mut dyn Plugin;
let f: Symbol<Factory> = unsafe { lib.get(b"create_plugin\0") }?;
let plugin: Box<dyn Plugin> = unsafe { Box::from_raw(f()) };
```

**Pros**
- One entry point per plugin; no per-symbol lookup.
- Feels like idiomatic Rust trait dispatch.

**Cons**
- `dyn Trait` fat pointers embed a vtable whose layout is **not** guaranteed
  stable across compiler versions. Only safe when host and plugin are built
  with the **same toolchain version**.
- Still requires `unsafe` and careful lifetime discipline.

**When to use**  
Same-toolchain mono-repo plugins where ABI stability is guaranteed by your build system.

---

## Option 3 — `abi_stable` crate

**How it works:**  
`abi_stable` provides `RBox`, `RStr`, `RVec`, `DynTrait`, and
`StableAbi`-derive that lay out types with a guaranteed, documented ABI.
Plugins export a module root (`extern "C" fn get_module() -> &'static PluginMod`)
that contains pre-validated vtables.

```toml
[dependencies]
abi_stable = "0.11"
```

```rust
// shared interface crate
#[abi_stable::sabi_trait]
pub trait Plugin: Debug {
    fn on_enable(&self);
    fn on_disable(&self);
}

// host loads it
let m = unsafe { Library::new(path) }?;
let get_mod: Symbol<extern "C" fn() -> PluginMod_Ref> =
    unsafe { m.get(b"get_plugin_module\0") }?;
let plugin = get_mod().new()();
```

**Pros**
- Stable ABI: plugins compiled with different Rust versions interoperate
  safely (the crate itself enforces layout guarantees).
- Rich type support: strings, vecs, closures, trait objects — all ABI-stable.
- Version checking built in (prefix-compatible interface evolution).

**Cons**
- Significant API surface to learn; `sabi_trait` and `StableAbi` macros add compile complexity.
- Requires all public interface types to opt into `StableAbi` — third-party types need wrappers.
- Slower compile times.

**When to use**  
Production plugin systems where plugin authors compile independently and you
cannot guarantee toolchain alignment. The closest Rust equivalent to the JVM's
stable bytecode interface.

---

## Option 4 — WebAssembly (WASM) plugins via `wasmtime` or `wasmer`

**How it works:**  
Plugins are compiled to WASM (`wasm32-wasip1` or `wasm32-unknown-unknown`).
The host embeds a WASM runtime and instantiates plugin modules in a sandboxed
linear memory.

```toml
[dependencies]
wasmtime = "25"
# or: wasmer = "4"
```

```rust
let engine = Engine::default();
let module = Module::from_file(&engine, "plugin.wasm")?;
let mut store = Store::new(&engine, ());
let instance = Instance::new(&mut store, &module, &[])?;
let init = instance.get_typed_func::<(), ()>(&mut store, "plugin_init")?;
init.call(&mut store, ())?;
```

**Pros**
- **True sandboxing**: plugins cannot access host memory or syscalls unless
  explicitly granted via WASI or host imports. Critical for untrusted plugins.
- **100% ABI stable**: WASM is a bytecode standard — the runtime version and
  Rust version of the plugin are irrelevant.
- Cross-platform: the same `.wasm` file loads on Linux, macOS, Windows.
- Plugin crashes are contained; they don't take down the host.

**Cons**
- Plugins cannot directly share heap pointers with the host (everything crosses
  a linear-memory boundary via copy or offset arithmetic).
- Higher call overhead than a native function pointer.
- Complex host<->plugin APIs (structs, callbacks) require a component model or
  serialisation layer (e.g. `wit-bindgen`, `extism`).
- Plugin binary size is larger than a native `.so`.

**When to use**  
Untrusted third-party plugins; cross-platform runtimes; any system where plugin
isolation is a hard requirement (VS Code extensions, game mods from strangers).

---

## Option 5 — `extism` (WASM plugin framework)

**How it works:**  
`extism` is an opinionated layer on top of `wasmtime` that gives you a
batteries-included plugin host + SDK. Plugins expose typed functions via a PDK
(plugin development kit); the host calls them with JSON/MessagePack payloads.

```toml
[dependencies]
extism = "1"
```

```rust
let plugin = Plugin::new(wasm_bytes, [], true)?;
let result: Vec<u8> = plugin.call("greet", b"world")?;
```

**Pros**
- Minimal boilerplate; the host<->plugin protocol is already solved.
- PDKs exist for Rust, Go, JS, Python, etc. — plugins in any language.
- Built-in manifest, versioning, and permission system.

**Cons**
- Opinionated: the call model is request/response with byte payloads.
  Not suitable for tight event-loop callbacks.
- Still carries WASM overhead.

**When to use**  
You want WASM sandboxing but don't want to design the ABI from scratch.
Great for CLI tools, serverless-style plugins, API gateways.

---

## Option 6 — In-process scripting (Lua / Rhai / Rune)

**How it works:**  
Embed a scripting language runtime. Plugin authors write scripts, not compiled
binaries. The host exposes a Rust API to the script engine.

| Engine | Crate | Language |
|--------|-------|----------|
| Lua    | `mlua` | Lua 5.4 / LuaJIT |
| Rhai   | `rhai` | Rhai (Rust-native) |
| Rune   | `rune` | Rune (async, Rust-native) |

```rust
// mlua example
let lua = Lua::new();
lua.globals().set("host_api", my_api_table)?;
lua.load(script_source).exec()?;
```

**Pros**
- No compilation step for plugin authors; hot-reload is trivial (reload the script file).
- Easy to sandbox (control what globals are exposed).
- Familiar to mod communities (Lua is the de-facto modding language).

**Cons**
- Dynamic typing; runtime errors instead of compile-time errors.
- Performance ceiling lower than native code (though LuaJIT is fast).
- Feature set limited to what the host exposes.

**When to use**  
Game modding (WoW addons, Factorio mods); config-driven extensibility;
when the plugin author audience is not a systems programmer.

---

## Decision Matrix

| Criterion | `libloading` raw | Trait factory | `abi_stable` | WASM (`wasmtime`) | `extism` | Scripting (Lua/Rhai) |
|-----------|:---:|:---:|:---:|:---:|:---:|:---:|
| ABI stable across toolchains | No | No | Yes | Yes | Yes | Yes |
| Sandboxing / isolation | No | No | No | Yes | Yes | Partial |
| Low call overhead | Yes | Yes | Yes | Medium | Medium | Medium |
| Complex types across boundary | Unsafe | Unsafe | Yes | Via wit-bindgen | Via serialise | Via host API |
| Plugin authors need Rust | Yes | Yes | Yes | Any lang | Any lang | No |
| Hot-reload | Possible* | Possible* | Possible* | Yes | Yes | Yes |
| Host implementation effort | Low | Low | Medium | Medium | Low | Medium |

\* Hot-reload with native `.so` requires careful `drop` sequencing and cannot
safely reload if any reference to old code exists. In practice this means
leaking the old library handle or using a generation/epoch scheme.

---

## Recommendation by scenario

**Scenario A — trusted plugins, same team, mono-repo:**  
Option 1 or 2 (`libloading` raw / trait factory). Simple and low-overhead.
The current POC is already the right foundation.

**Scenario B — third-party plugin authors, Rust ecosystem:**  
Option 3 (`abi_stable`). Only approach giving a stable Rust-native ABI across
independent compiler builds without giving up type safety.

**Scenario C — untrusted plugins / multi-language / mod community:**  
Option 4 or 5 (WASM / extism). Sandboxing is mandatory when you can't vet
plugin code. Modern engines trend this direction.

**Scenario D — modders are not programmers:**  
Option 6 (Lua via `mlua`). Lowest barrier; widest modding tradition.

---

## What the POC demonstrates

`src/main.rs` shows the raw mechanism underlying all native plugin systems:
`dlopen` -> symbol lookup -> call through function pointer. Everything above
option 1 is built on top of this same OS primitive, but adds ABI contracts,
vtable stability, or a sandboxed runtime on top.

### Concrete next steps to graduate to a real plugin system

1. Define a `Plugin` trait in a shared `plugin-api` crate.
2. Export a single `extern "C" fn create_plugin() -> *mut dyn Plugin` from each plugin `.so`.
3. Build a `PluginManager` that:
   - Iterates a `plugins/` directory.
   - Loads each `.so` via `libloading`.
   - Calls the factory, stores `Box<dyn Plugin>` alongside the `Library` handle
     (the handle must outlive the plugin object or you get UB).
4. If plugin authors compile independently, migrate the interface crate to
   `abi_stable` or switch the loading layer to WASM.
