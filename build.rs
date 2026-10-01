/// build.rs — tells cargo where the pre-built mystatic.a lives.
///
/// MYSTATIC_DIR env var (set by build-lib.sh / run.sh) points at the
/// directory containing libmystatic.a.  If unset, we skip static linking
/// so a plain `cargo build` without the libs still compiles (it will just
/// panic at runtime if the static path is exercised).
fn main() {
    if let Ok(dir) = std::env::var("MYSTATIC_DIR") {
        println!("cargo:rustc-link-search=native={dir}");
        println!("cargo:rustc-link-lib=static=mystatic");
    }
    // Re-run if the env var or the library itself changes.
    println!("cargo:rerun-if-env-changed=MYSTATIC_DIR");
}
