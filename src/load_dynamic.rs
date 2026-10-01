use libloading::{Library, Symbol};
use std::path::Path;

use crate::extern_def::MathFn;

pub fn demo_dynamic(so_path: &Path) {
    println!("\n=== Dynamic (.so) — runtime load/unload via libloading ===");
    println!("path : {}", so_path.display());

    // SAFETY: we supply the correct path and know the exported symbol signatures.
    let lib = unsafe { Library::new(so_path) }
        .unwrap_or_else(|e| panic!("dlopen failed for {}: {e}", so_path.display()));

    // Look up a symbol by name, call it, and return the result.
    let call = |name: &[u8], a: i64, b: i64| -> i64 {
        // SAFETY: `Symbol` lifetime is tied to `lib`; `lib` outlives this closure.
        let f: Symbol<MathFn> =
            unsafe { lib.get(name) }.unwrap_or_else(|e| panic!("symbol {:?} not found: {e}", name));
        // SAFETY: function pointer is valid while the library is loaded.
        unsafe { f(a, b) }
    };

    println!("add(10, 3)      = {}", call(b"add\0", 10, 3));
    println!("subtract(10, 3) = {}", call(b"subtract\0", 10, 3));
    println!("multiply(10, 3) = {}", call(b"multiply\0", 10, 3));
    println!("divide(10, 3)   = {}", call(b"divide\0", 10, 3));
    println!("divide(10, 0)   = {} (zero-safe)", call(b"divide\0", 10, 0));

    // Explicit drop == dlclose(); the .so is unmapped here.
    drop(lib);
    println!("-> library unloaded (drop).");
}
