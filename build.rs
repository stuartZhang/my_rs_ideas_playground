use cc::Build;
use std::path::Path;
fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let native_src = project_root.join("native");
    Build::new()
        .file(native_src.join("closure_callback.c"))
        .compile("closure_callback");
    println!("cargo:rustc-link-lib=closure_callback");
}
