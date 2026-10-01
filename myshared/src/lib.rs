/// Dynamically-loaded shared library (.so).
///
/// All public symbols use `#[unsafe(no_mangle)]` + `extern "C"` so the
/// loader can resolve them by name at runtime without Rust name mangling.
/// Edition 2024 requires the `unsafe(...)` wrapper on `no_mangle`.

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Integer division. Returns 0 on divide-by-zero rather than panicking,
/// so the host process stays alive even on bad input.
#[unsafe(no_mangle)]
pub extern "C" fn divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
