// math_plugin.rhai — loaded by rhai POC.
// host_log(level, msg) and host_get_version() are registered by the host
// before this script is compiled.

// Called once by the host after compiling the AST.
fn plugin_init() {
    host_log(0, "math_plugin.rhai: plugin initialized");
    host_log(0, "math_plugin.rhai: host is " + host_get_version());
}

fn add(a, b) { a + b }

fn subtract(a, b) { a - b }

fn multiply(a, b) { a * b }

// Returns 0 on divide-by-zero.
fn divide(a, b) {
    if b == 0 { return 0; }
    a / b
}
