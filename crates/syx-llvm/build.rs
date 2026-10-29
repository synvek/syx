//! Validate local LLVM (SYX_LLVM_PREFIX / sibling ../llvm) for the `llvm` feature.
//!
//! llvm-sys is wired to the sibling install via `.cargo/config.toml` (internal).
//! When `SYX_LLVM_PREFIX` points elsewhere, use `./scripts/with-syx-llvm cargo 鈥.

fn main() {
    println!("cargo:rerun-if-env-changed=SYX_LLVM_PREFIX");
    println!("cargo:rerun-if-changed=build.rs");

    if !cfg!(feature = "llvm") {
        return;
    }

    let prefix = syx_llvm_prefix::require_llvm_prefix("syx-llvm");
    // inkwell `llvm21-1` expects LLVM 21.x
    syx_llvm_prefix::expect_llvm_major(&prefix, 21);
    println!(
        "cargo:rustc-env=SYX_RESOLVED_LLVM_PREFIX={}",
        prefix.display()
    );
    println!(
        "cargo:warning=syx-llvm: using LLVM at {} ({})",
        prefix.display(),
        syx_llvm_prefix::llvm_version(&prefix).unwrap_or_else(|| "?".into())
    );
    // Prefer -lzstd over absolute Homebrew paths baked into some llvm-config builds.
    println!("cargo:rustc-link-lib=zstd");
}
