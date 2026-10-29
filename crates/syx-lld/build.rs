//! Build script: statically embed LLD from SYX_LLVM_PREFIX / sibling ../llvm.
//!
//! Unix: stage *symlinks to `.a` only* so `-lc++` from llvm-sys resolves to the
//! system libc++ (not LLVM's `@rpath` dylib).
//!
//! Windows MSVC: official LLVM ships `lld*.lib` / `LLVM*.lib` (not `liblld*.a`).
//! Stage/copy those into OUT_DIR and emit `static=` link lines so LLD is inside
//! `syx.exe` (single-file SDK goal; no runtime `bin/lld-link` dependency).

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
        "syx-lld: cannot embed LLD from {} (need liblld*.a / lld*.lib + Driver.h). \
         Use a full LLVM install at ../llvm or SYX_LLVM_PREFIX.",
        prefix.display()
    );
}

fn target_is_windows() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("windows")
}

fn find_lld_common(lib: &Path) -> Option<PathBuf> {
    for name in ["liblldCommon.a", "lldCommon.lib"] {
        let p = lib.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn try_inprocess_lld(prefix: &Path) -> bool {
    let lib = prefix.join("lib");
    let inc = prefix.join("include");
    let driver = inc.join("lld/Common/Driver.h");
    if !driver.is_file() {
        println!("cargo:warning=syx-lld: missing {}", driver.display());
        return false;
    }
    if find_lld_common(&lib).is_none() {
        println!(
            "cargo:warning=syx-lld: no liblldCommon.a / lldCommon.lib under {}",
            lib.display()
        );
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
    // Also search the real LLVM lib dir for any deps not staged (e.g. libxml2s).
    println!("cargo:rustc-link-search=native={}", lib.display());
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

/// LLD component archives: (unix `lib*.a`, msvc `*.lib`).
const LLD_ARCHIVES: &[(&str, &str)] = &[
    ("liblldMachO.a", "lldMachO.lib"),
    ("liblldELF.a", "lldELF.lib"),
    ("liblldCOFF.a", "lldCOFF.lib"),
    ("liblldMinGW.a", "lldMinGW.lib"),
    ("liblldWasm.a", "lldWasm.lib"),
    ("liblldCommon.a", "lldCommon.lib"),
];

fn resolve_archive(lib: &Path, unix_name: &str, msvc_name: &str) -> Option<PathBuf> {
    let unix = lib.join(unix_name);
    if unix.is_file() {
        return Some(unix);
    }
    let msvc = lib.join(msvc_name);
    if msvc.is_file() {
        return Some(msvc);
    }
    None
}

/// Stage LLD (+ LLVM) static archives into OUT_DIR for a private `-L`.
fn stage_static_archives(prefix: &Path) -> Result<PathBuf, String> {
    let out = PathBuf::from(std::env::var("OUT_DIR").map_err(|e| e.to_string())?);
    let staging = out.join("lld_static_libs");
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging).map_err(|e| e.to_string())?;

    let lib = prefix.join("lib");
    let windows = target_is_windows();
    for &(unix_name, msvc_name) in LLD_ARCHIVES {
        let src = resolve_archive(&lib, unix_name, msvc_name)
            .ok_or_else(|| format!("missing {unix_name} / {msvc_name}"))?;
        // Destination name must match what rustc `static=lldFoo` looks up.
        let dst_name = if windows {
            msvc_name
        } else {
            unix_name
        };
        symlink_archive(&src, &staging.join(dst_name))?;
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
        if file.starts_with("libLLVM")
            || file.starts_with("libPolly")
            || file.starts_with("LLVM")
            || file.starts_with("Polly")
        {
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
                    continue;
                }
                // Windows llvm-config prints absolute `…\LLVMFoo.lib` paths.
                let path = Path::new(tok);
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem.starts_with("LLVM") || stem.starts_with("Polly") {
                        println!("cargo:rustc-link-lib=static={stem}");
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
                    if tok.starts_with('/') || Path::new(tok).is_absolute() {
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
                    continue;
                }
                // Windows: `psapi.lib shell32.lib … libxml2s.lib`
                if let Some(name) = tok.strip_suffix(".lib") {
                    // Match llvm-sys MSVC policy: system import libs as dylib=;
                    // libxml2s is a static archive next to LLVM.
                    if name.eq_ignore_ascii_case("libxml2s")
                        || name.eq_ignore_ascii_case("zstd")
                        || name.eq_ignore_ascii_case("zlib")
                        || name.eq_ignore_ascii_case("liblzma")
                        || name.eq_ignore_ascii_case("lzma")
                        || name.eq_ignore_ascii_case("libiconv")
                        || name.eq_ignore_ascii_case("iconv")
                    {
                        println!("cargo:rustc-link-lib=static={name}");
                    } else {
                        println!("cargo:rustc-link-lib=dylib={name}");
                    }
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
    if target_is_windows() {
        // Official Windows LLVM is /MT + rpmalloc; keep malloc from the archive.
        println!("cargo:rustc-link-arg=/INCLUDE:malloc");
        println!("cargo:rustc-link-lib=bcrypt");
        // C++ EH/RTTI pieces used by LLD when the final link is driven by rustc.
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
        // Static libxml2 (from llvm-config --system-libs) needs these when present
        // under prefix/lib (CI copies them from vcpkg; see release.yml).
        let lib = prefix.join("lib");
        // vcpkg `x64-windows-static` names zlib `zs.lib`; CI/README may use `zlib.lib`.
        for name in ["zlib", "zs", "libiconv", "iconv", "charset", "lzma", "liblzma"] {
            if lib.join(format!("{name}.lib")).is_file() {
                println!("cargo:rustc-link-lib=static={name}");
            }
        }
    }
}
