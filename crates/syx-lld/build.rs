//! Build script: statically embed LLD from SYX_LLVM_PREFIX / sibling ../llvm.
//!
//! Uses a private link-search directory of *symlinks to `.a` only* so `-lc++`
//! from llvm-sys resolves to the system libc++ (not LLVM's `@rpath` dylib).

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=SYX_LLVM_PREFIX");
    println!("cargo:rerun-if-changed=cpp/syx_lld_bridge.cpp");
    println!("cargo:rustc-check-cfg=cfg(syx_lld_inprocess)");

    let prefix = syx_llvm_prefix::require_llvm_prefix("syx-lld");
    syx_llvm_prefix::expect_llvm_major(&prefix, 21);

    println!(
        "cargo:rustc-env=SYX_RESOLVED_LLVM_PREFIX={}",
        prefix.display()
    );

    if try_inprocess_lld(&prefix) {
        println!("cargo:rustc-cfg=syx_lld_inprocess");
        println!("cargo:rustc-env=SYX_LLD_MODE=inprocess");
        println!(
            "cargo:warning=syx-lld: statically embedding LLD from {}",
            prefix.display()
        );
        return;
    }

    if let Some(lld) = syx_llvm_prefix::resolve_lld_binary(&prefix) {
        println!("cargo:rustc-env=SYX_LLD_BIN={}", lld.display());
        println!("cargo:rustc-env=SYX_LLD_MODE=prefix-bin");
        println!(
            "cargo:warning=syx-lld: static LLD link failed; falling back to {}",
            lld.display()
        );
        return;
    }

    println!("cargo:rustc-env=SYX_LLD_MODE=missing");
    panic!(
        "syx-lld: cannot embed LLD from {} (need liblld*.a + Driver.h). \
         Use a full LLVM install at ../llvm or SYX_LLVM_PREFIX.",
        prefix.display()
    );
}

fn try_inprocess_lld(prefix: &Path) -> bool {
    let lib = prefix.join("lib");
    let inc = prefix.join("include");
    let driver = inc.join("lld/Common/Driver.h");
    if !driver.is_file() {
        println!("cargo:warning=syx-lld: missing {}", driver.display());
        return false;
    }
    if !lib.join("liblldCommon.a").is_file() {
        println!("cargo:warning=syx-lld: no liblldCommon.a under {}", lib.display());
        return false;
    }

    let bridge = Path::new("cpp/syx_lld_bridge.cpp");
    if !bridge.is_file() {
        return false;
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file(bridge)
        .std("c++17")
        .define("SYX_LLD_HAS_DRIVER", None)
        .include(&inc)
        .flag_if_supported("-fno-exceptions")
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-funwind-tables");
    apply_llvm_cxxflags(&mut build, prefix);

    let staging = match stage_static_archives(prefix) {
        Ok(p) => p,
        Err(e) => {
            println!("cargo:warning=syx-lld: staging archives failed ({e})");
            return false;
        }
    };
    println!("cargo:rustc-link-search=native={}", staging.display());
    emit_static_libs(prefix);

    match build.try_compile("syx_lld_bridge") {
        Ok(()) => true,
        Err(e) => {
            println!("cargo:warning=syx-lld: bridge compile failed ({e})");
            false
        }
    }
}

fn apply_llvm_cxxflags(build: &mut cc::Build, prefix: &Path) {
    let cfg = syx_llvm_prefix::llvm_config_path(prefix);
    if let Ok(out) = std::process::Command::new(&cfg).arg("--cxxflags").output() {
        if out.status.success() {
            for tok in String::from_utf8_lossy(&out.stdout).split_whitespace() {
                if tok.starts_with("-I")
                    || tok == "-fno-exceptions"
                    || tok == "-fno-rtti"
                    || tok == "-funwind-tables"
                    || tok.starts_with("-std=")
                    || tok.starts_with("-D")
                {
                    build.flag(tok);
                }
            }
        }
    }
}

/// Symlink only `.a` archives into OUT_DIR so `-L` never sees LLVM's libc++.dylib.
fn stage_static_archives(prefix: &Path) -> Result<PathBuf, String> {
    let out = PathBuf::from(std::env::var("OUT_DIR").map_err(|e| e.to_string())?);
    let staging = out.join("lld_static_libs");
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging).map_err(|e| e.to_string())?;

    let lib = prefix.join("lib");
    for name in [
        "liblldMachO.a",
        "liblldELF.a",
        "liblldCOFF.a",
        "liblldMinGW.a",
        "liblldWasm.a",
        "liblldCommon.a",
    ] {
        symlink_archive(&lib.join(name), &staging.join(name))?;
    }

    let cfg = syx_llvm_prefix::llvm_config_path(prefix);
    let out = std::process::Command::new(&cfg)
        .args(["--link-static", "--libfiles"])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("llvm-config --libfiles failed".into());
    }
    for tok in String::from_utf8_lossy(&out.stdout).split_whitespace() {
        let src = PathBuf::from(tok);
        let file = src
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if file.starts_with("libLLVM") || file.starts_with("libPolly") {
            symlink_archive(&src, &staging.join(&file))?;
        }
    }
    Ok(staging)
}

fn symlink_archive(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.is_file() {
        return Err(format!("missing {}", src.display()));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let _ = fs::remove_file(dst);
        symlink(src, dst).map_err(|e| format!("symlink {} -> {}: {e}", src.display(), dst.display()))
    }
    #[cfg(not(unix))]
    {
        fs::copy(src, dst).map(|_| ()).map_err(|e| e.to_string())
    }
}

fn emit_static_libs(prefix: &Path) {
    for name in [
        "lldMachO",
        "lldELF",
        "lldCOFF",
        "lldMinGW",
        "lldWasm",
        "lldCommon",
    ] {
        println!("cargo:rustc-link-lib=static={name}");
    }

    let cfg = syx_llvm_prefix::llvm_config_path(prefix);
    if let Ok(out) = std::process::Command::new(&cfg)
        .args(["--link-static", "--libs"])
        .output()
    {
        if out.status.success() {
            for tok in String::from_utf8_lossy(&out.stdout).split_whitespace() {
                if let Some(name) = tok.strip_prefix("-l") {
                    if name.starts_with("LLVM") || name.starts_with("Polly") {
                        println!("cargo:rustc-link-lib=static={name}");
                    }
                }
            }
        }
    }

    if let Ok(out) = std::process::Command::new(&cfg)
        .args(["--link-static", "--system-libs"])
        .output()
    {
        if out.status.success() {
            for tok in String::from_utf8_lossy(&out.stdout).split_whitespace() {
                if tok.contains("zstd") {
                    if tok.starts_with('/') {
                        if let Some(parent) = Path::new(tok).parent() {
                            println!("cargo:rustc-link-search=native={}", parent.display());
                        }
                    }
                    println!("cargo:rustc-link-lib=static=zstd");
                    continue;
                }
                if let Some(name) = tok.strip_prefix("-l") {
                    if name == "c++" || name == "c++abi" || name == "stdc++" {
                        continue;
                    }
                    println!("cargo:rustc-link-lib={name}");
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Explicit system libc++ (same as ../llvm/bin/lld).
        println!("cargo:rustc-link-arg=/usr/lib/libc++.1.dylib");
        println!("cargo:rustc-link-lib=dylib=xml2");
        println!("cargo:rustc-link-lib=dylib=z");
        println!("cargo:rustc-link-lib=dylib=ncurses");
    }
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=pthread");
    }
}
