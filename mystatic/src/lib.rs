/// Static library (.a) linked at compile time.
///
/// Functions share the same signatures as myshared so the demo can
/// call identical logic through both paths.
/// Edition 2024 requires the `unsafe(...)` wrapper on `no_mangle`.

#[unsafe(no_mangle)]
pub extern "C" fn static_add(a: i64, b: i64) -> i64 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn static_subtract(a: i64, b: i64) -> i64 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn static_multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Integer division. Returns 0 on divide-by-zero.
#[unsafe(no_mangle)]
pub extern "C" fn static_divide(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a / b }
}
