// Symbols baked in at link time from mystatic.a.
// build.rs wires up the search path; if MYSTATIC_DIR was not set the linker
// will error at build time — intentionally, so the binary always works correctly.
unsafe extern "C" {
    pub(crate) fn static_add(a: i64, b: i64) -> i64;
    pub(crate) fn static_subtract(a: i64, b: i64) -> i64;
    pub(crate) fn static_multiply(a: i64, b: i64) -> i64;
    pub(crate) fn static_divide(a: i64, b: i64) -> i64;
}

// Signature shared by every exported symbol in myshared.
pub type MathFn = unsafe extern "C" fn(i64, i64) -> i64;
