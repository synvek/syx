fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| {
        format!("{manifest_dir}/../../target")
    });
    let archive = std::path::Path::new(&target_dir)
        .join(&profile)
        .join("deps");
    println!("cargo:rustc-env=SYX_RT_STATIC_LIB={}", archive.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/abi.rs");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/link.rs");
    let _ = out_dir;
    let _ = target;
}
