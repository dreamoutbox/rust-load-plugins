use crate::extern_def::{static_add, static_divide, static_multiply, static_subtract};
pub fn demo_static() {
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
