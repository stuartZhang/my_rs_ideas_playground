#![allow(missing_docs)]
/// 添加文档说明
fn main() {
    #[cfg(feature = "cc")]
    {
        use ::cc::Build;
        use ::std::path::Path;
        let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let native_src = project_root.join("native");
        Build::new()
            .file(native_src.join("closure_callback.c"))
            .compile("closure_callback");
        println!("cargo:rustc-link-lib=closure_callback");
    }
}
