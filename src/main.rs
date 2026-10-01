use libloading::{Library, Symbol};
use std::path::Path;

// Symbols baked in at link time from mystatic.a.
// build.rs wires up the search path; if MYSTATIC_DIR was not set the linker
// will error at build time — intentionally, so the binary always works correctly.
unsafe extern "C" {
    fn static_add(a: i64, b: i64) -> i64;
    fn static_subtract(a: i64, b: i64) -> i64;
    fn static_multiply(a: i64, b: i64) -> i64;
    fn static_divide(a: i64, b: i64) -> i64;
}

// Signature shared by every exported symbol in myshared.
type MathFn = unsafe extern "C" fn(i64, i64) -> i64;

fn demo_dynamic(so_path: &Path) {
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

fn demo_static() {
    println!("\n=== Static (.a) — symbols resolved at link time ===");
    // SAFETY: these are valid C-ABI functions linked into this binary.
    unsafe {
        println!("static_add(10, 3)      = {}", static_add(10, 3));
        println!("static_subtract(10, 3) = {}", static_subtract(10, 3));
        println!("static_multiply(10, 3) = {}", static_multiply(10, 3));
        println!("static_divide(10, 3)   = {}", static_divide(10, 3));
        println!(
            "static_divide(10, 0)   = {} (zero-safe)",
            static_divide(10, 0)
        );
    }
    println!("-> no load/unload; code is baked into the binary.");
}

fn main() {
    println!("Rust dynamic/static library POC");
    println!("================================");

    // Accept an explicit override; fall back to a .so beside the binary.
    let so_path = std::env::var("MYSHARED_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let exe = std::env::current_exe().expect("cannot resolve exe path");
            exe.parent()
                .expect("exe has no parent dir")
                .join("libmyshared.so")
        });

    demo_dynamic(&so_path);
    demo_static();

    println!("\nDone.");
}
