# Rust Plugin System Architectures & Tradeoffs

## Overview

When designing a plugin system in Rust (similar to Minecraft's Bukkit/Spigot model), loading raw shared libraries (`.so`, `.dll`) via `libloading` is rarely optimal. This document outlines why raw dynamic libraries present challenges in Rust and evaluates the standard alternatives.

---

## 1. Limitations of Raw Dynamic Libraries (`libloading`)

- **Zero Fault Isolation:** Plugins execute inside the host's process memory. An uncaught panic, invalid pointer, or segmentation fault in any plugin immediately terminates the entire host server.
- **Unstable Rust ABI:** Rust does not guarantee ABI stability across compiler versions, build targets, or compiler flags. Direct Rust-to-Rust dynamic calls without C-ABI abstractions (`#[repr(C)]`) cause undefined behavior if compiler versions drift.
- **Allocator Mismatch:** If a plugin allocates memory and the host frees it (or vice versa), allocator corruption can occur unless both sides coordinate using a shared allocator or C-standard `malloc`/`free`.
- **Platform Coupling:** Every plugin must be compiled separately for each OS and architecture (`.so`, `.dll`, `.dylib`).
- **Unloading Hazards:** Calling `dlclose` while a background thread, static variable, or function pointer remains active from that library results in use-after-free crashes.

---

## 2. Alternatives

### A. WebAssembly (Wasm) — *Recommended for Minecraft-like Systems*

Plugins compile to WebAssembly (`.wasm`) bytecode and run within an embedded runtime like **`wasmtime`** or **`extism`**.

- **How it works:** Host defines a capability-based API (functions exposed to Wasm); guest exports lifecycle and event hooks (e.g. `on_enable`, `on_player_action`).
- **Advantages:**
    - **Crash isolation:** Faults are trapped inside the sandbox; the host server stays up.
    - **Portability:** A single `.wasm` binary runs across Linux, macOS, and Windows.
    - **Polyglot:** Plugins can be authored in Rust, C, Go, Zig, or AssemblyScript.
    - **Safe lifecycle:** Dropping or reloading instances at runtime is deterministic and memory-safe.
    - **Capability security:** Sandboxing restricts access to filesystem and networking unless granted by host.
- **Tradeoffs:** Slight CPU overhead compared to native code (~1.2x–2x), boundary serialization cost.
- **Ecosystem:** `wasmtime`, `extism`, `wasmer`.

### B. Embedded Scripting (Rhai / Lua)

Embed a lightweight scripting engine directly into the Rust binary.

- **How it works:** Host exposes game state and APIs; scripts run from a `plugins/` directory as plain text files.
- **Advantages:**
    - Zero compilation step for plugin developers.
    - Hot-reloading scripts during development is trivial.
    - Memory-safe by design without native crash risks.
- **Tradeoffs:** Lower throughput on computationally intensive tasks.
- **Ecosystem:**
    - `rhai`: 100% safe Rust, intuitive syntax, no C toolchain dependencies.
    - `mlua`: High performance via Lua 5.4 or LuaJIT.

### C. Stable FFI Native Libraries (`abi_stable`)

For domains requiring maximum possible performance with native execution.

- **How it works:** Provides Rust-like types (`RString`, `RVec`, trait objects) that have stable C-ABI layouts across dynamic libraries.
- **Advantages:** 100% native speed; richer API ergonomics than raw C pointers.
- **Tradeoffs:** Still shares process memory (no crash protection), plugins must be built per-platform.
- **Ecosystem:** `abi_stable`.

---

## 3. Comparison Matrix

| Criteria | Raw `.so` (`libloading`) | WebAssembly (`wasmtime`) | Scripting (`rhai`) | `abi_stable` |
| --- | --- | --- | --- | --- |
| **Fault Isolation** | None (crashes host) | Sandbox (trapped) | High (memory-safe) | None (crashes host) |
| **Performance** | Native (100%) | Near-Native (~80–90%) | Interpreted (~10–30%) | Native (100%) |
| **Cross-Platform** | Per-OS build required | Universal `.wasm` | Universal text scripts | Per-OS build required |
| **Author Experience** | Difficult (unsafe FFI) | Moderate (compile to Wasm) | Simple (plain scripts) | Moderate |
| **Safe Unloading** | High risk | Safe & native | Safe & native | High risk |

---

## 4. Decision Guide

- **Minecraft / Game Server Plugin System:** Choose **WebAssembly (`wasmtime` / `extism`)** for sandboxing, portability, and crash resilience.
- **Game Rules, Quests, or Admin Scripts:** Choose **Embedded Scripting (`rhai`)** for rapid authoring without toolchain friction.
- **Low-Latency Heavy Compute (Audio DSP, Physics Engines):** Choose **`abi_stable`** or raw shared libraries.
