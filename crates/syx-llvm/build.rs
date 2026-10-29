//! Validate local LLVM (SYX_LLVM_PREFIX / sibling ../llvm) for the `llvm` feature.
//!
//! llvm-sys is wired to the sibling install via `.cargo/config.toml` (internal).
//! When `SYX_LLVM_PREFIX` points elsewhere, use `./scripts/with-syx-llvm cargo …`.

use std::path::PathBuf;
use std::process::Command;

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

    // Sibling / GitHub LLVM prefixes often ship libc++.{a,dylib} under lib/ with
    // @rpath. llvm-sys adds -L$prefix/lib -lc++, so ld picks that copy and leaves
    // operator new/delete / __cxa_* undefined. Force the Xcode SDK .tbd instead
    // (same failure mode as linking with only -L../llvm/lib -lc++).
    if std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("macos") {
        link_macos_system_libcxx();
    }

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

fn link_macos_system_libcxx() {
    let sdk = macos_sdk_path();
    let mut linked = false;
    if let Some(sdk) = sdk.as_ref() {
        for name in ["libc++.tbd", "libc++abi.tbd"] {
            let tbd = sdk.join("usr/lib").join(name);
            if tbd.is_file() {
                println!("cargo:rustc-link-arg={}", tbd.display());
                linked = true;
            }
        }
    }
    if !linked {
        // Older macOS / non-Xcode CLT: file may live in the dyld shared cache.
        println!("cargo:rustc-link-arg=-lc++");
        println!("cargo:rustc-link-arg=-lc++abi");
        println!(
            "cargo:warning=syx-llvm: macOS SDK libc++.tbd not found{}; relying on -lc++",
            sdk.as_ref()
                .map(|p| format!(" under {}", p.display()))
                .unwrap_or_default()
        );
    }
}

fn macos_sdk_path() -> Option<PathBuf> {
    let out = Command::new("xcrun").args(["--show-sdk-path"]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    if path.is_empty() {
        return None;
    }
    let p = PathBuf::from(&path);
    p.is_dir().then_some(p)
}
