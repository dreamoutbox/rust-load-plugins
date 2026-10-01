// math_plugin.rhai — loaded by rhai POC.
// Rhai scripts define functions that the host engine calls by name.

fn add(a, b) { a + b }

fn subtract(a, b) { a - b }

fn multiply(a, b) { a * b }

// Returns 0 on divide-by-zero.
fn divide(a, b) {
    if b == 0 { return 0; }
    a / b
}
