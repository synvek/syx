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
    println!("cargo:rerun-if-changed=src/user_entry_weak_stubs.c");

    // Linux (rust-lld) / macOS may retain `syx_rt_start` when building the
    // staticlib or host tools; weak C stubs satisfy the undefined refs.
    // Real AOT objects supply strong `syx_user_*` and override these.
    //
    // Windows `atexit` stub is *not* compiled into syx_rt here: it would
    // collide with libcmt when linking the syx CLI under +crt-static. Native
    // AOT injects it at link time via `link.rs` instead.
    if !target.contains("windows") {
        cc::Build::new()
            .file("src/user_entry_weak_stubs.c")
            .compile("syx_user_entry_weak_stubs");
    }

    let _ = out_dir;
}
