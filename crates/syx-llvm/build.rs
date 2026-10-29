//! Validate local LLVM (SYX_LLVM_PREFIX / sibling ../llvm) for the `llvm` feature.
//!
//! llvm-sys is wired to the sibling install via `.cargo/config.toml` (internal).
//! When `SYX_LLVM_PREFIX` points elsewhere, use `./scripts/with-syx-llvm cargo …`.

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

    // Official Windows LLVM embeds rpmalloc. Keep its malloc when the final
    // link mixes object files; complements `.cargo/config.toml` `+crt-static`
    // (still required for the whole crate graph — cannot be set from here).
    //
    // llvm-config --system-libs only names libxml2s; static libxml2 also needs
    // zlib/iconv/lzma if those .lib files were placed under prefix/lib (CI and
    // local Windows setup copy them next to libxml2s.lib). bcrypt is a Windows
    // SDK lib required by libxml2 >= 2.12.
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        println!("cargo:rustc-link-arg=/INCLUDE:malloc");
        println!("cargo:rustc-link-lib=bcrypt");
        let lib = prefix.join("lib");
        // vcpkg `x64-windows-static` may ship zlib as `zs.lib`.
        for name in ["zlib", "zs", "libiconv", "iconv", "charset", "lzma", "liblzma"] {
            if lib.join(format!("{name}.lib")).is_file() {
                println!("cargo:rustc-link-lib={name}");
            }
        }
    }
}
