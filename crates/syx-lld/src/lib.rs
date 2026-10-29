//! LLD driver for SYX `full-native` (sibling LLVM / `SYX_LLVM_PREFIX`).
//!
//! Default: **in-process** — `liblld*.a` statically linked into `syx` via
//! `lld::lldMain` (no `../llvm/bin/lld` required at runtime).
//!
//! Platform SDK libraries remain host-provided.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

#[cfg(syx_lld_inprocess)]
mod inprocess {
    use std::ffi::CString;
    use std::os::raw::c_char;

    extern "C" {
        fn syx_lld_link(argc: i32, argv: *const *const c_char) -> i32;
    }

    pub fn run(args: &[String]) -> Result<(), String> {
        let mut full = Vec::with_capacity(args.len() + 1);
        full.push("lld".to_string());
        full.extend(args.iter().cloned());
        let c_strs: Vec<CString> = full
            .iter()
            .map(|a| CString::new(a.as_str()).map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;
        let ptrs: Vec<*const c_char> = c_strs.iter().map(|s| s.as_ptr()).collect();
        let rc = unsafe { syx_lld_link(ptrs.len() as i32, ptrs.as_ptr()) };
        if rc == 0 {
            Ok(())
        } else {
            Err(format!("in-process LLD failed with exit code {rc}"))
        }
    }
}

/// How LLD was wired at build time: `inprocess`, `prefix-bin`, or `missing`.
pub fn lld_mode() -> &'static str {
    option_env!("SYX_LLD_MODE").unwrap_or("missing")
}

/// Resolved LLVM prefix baked at build time (if any).
pub fn resolved_llvm_prefix() -> Option<&'static Path> {
    option_env!("SYX_RESOLVED_LLVM_PREFIX").map(Path::new)
}

/// LLD next to the running `syx` (SDK layout: `bin/syx` + `bin/lld-link`).
///
/// Windows CI often embeds LLD as **prefix-bin** with an absolute Actions path
/// (`D:\a\…\lld-link.exe`). That path is dead after packaging; ship `lld-link`
/// beside `syx` and resolve it here instead.
fn install_layout_lld_binary() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let bin_dir = exe.parent()?;
    for name in [
        "lld-link.exe",
        "lld-link",
        "lld.exe",
        "lld",
        "ld.lld",
        "ld64.lld",
    ] {
        let p = bin_dir.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn prefix_lld_binary() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SYX_LLD") {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }
    // Prefer install-relative LLD so packaged SDKs work off the CI machine.
    if let Some(p) = install_layout_lld_binary() {
        return Some(p);
    }
    if let Some(p) = option_env!("SYX_LLD_BIN") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }
    if let Some(prefix) = resolved_llvm_prefix() {
        if let Some(p) = syx_llvm_prefix::resolve_lld_binary(prefix) {
            return Some(p);
        }
    }
    syx_llvm_prefix::resolve_llvm_prefix().and_then(|p| syx_llvm_prefix::resolve_lld_binary(&p))
}

fn cached_lld_bin() -> Option<&'static PathBuf> {
    static CELL: OnceLock<Option<PathBuf>> = OnceLock::new();
    CELL.get_or_init(prefix_lld_binary).as_ref()
}

/// Prefix argv with multi-flavor driver flags for LLD.
pub fn flavor_prefix_args() -> Vec<String> {
    let mut args = Vec::new();
    #[cfg(target_os = "macos")]
    {
        args.push("-flavor".into());
        args.push("darwin".into());
        #[cfg(target_arch = "aarch64")]
        {
            args.push("-arch".into());
            args.push("arm64".into());
        }
        #[cfg(target_arch = "x86_64")]
        {
            args.push("-arch".into());
            args.push("x86_64".into());
        }
        let sdk_ver = macos_sdk_version();
        args.push("-platform_version".into());
        args.push("macos".into());
        args.push(sdk_ver.clone());
        args.push(sdk_ver);
    }
    #[cfg(target_os = "linux")]
    {
        args.push("-flavor".into());
        args.push("gnu".into());
    }
    #[cfg(target_os = "windows")]
    {
        args.push("-flavor".into());
        args.push("link".into());
    }
    args
}

#[cfg(target_os = "macos")]
fn macos_sdk_version() -> String {
    if let Ok(output) = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-version"])
        .output()
    {
        if output.status.success() {
            let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !v.is_empty() {
                return v;
            }
        }
    }
    "14.0".into()
}

/// Run LLD with the given program arguments (excluding argv0).
pub fn run_lld(link_args: &[OsString]) -> io::Result<()> {
    // Fail fast: LLVM 21 ld64.lld cannot parse MacOSX26+/27 TBD `arm64e.x1-*`.
    // Attempting the link burns CPU/RAM on thousands of unresolved CRT symbols.
    #[cfg(target_os = "macos")]
    if macos_sdk_unsupported_by_llvm21_lld() {
        return Err(io::Error::other(
            "macOS SDK TBD uses arm64e.x1 (or newer) arches unsupported by LLVM 21 LLD; \
             use system cc (default) or a newer LLVM LLD",
        ));
    }

    let mut argv: Vec<String> = flavor_prefix_args();
    for a in link_args {
        argv.push(a.to_string_lossy().into_owned());
    }

    #[cfg(syx_lld_inprocess)]
    {
        match inprocess::run(&argv) {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("syx-lld: in-process LLD failed ({e}); trying prefix bin/lld");
            }
        }
    }

    let prog = cached_lld_bin().cloned().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "LLD not available (rebuild with full-native; need liblld or bin/lld under SYX_LLVM_PREFIX / ../llvm)",
        )
    })?;

    let mut cmd = Command::new(&prog);
    cmd.args(&argv);
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "LLD failed ({prog}): {status}",
            prog = prog.display()
        )))
    }
}

/// True when the active macOS SDK ships TBD target tags LLVM 21 ld64.lld rejects.
#[cfg(target_os = "macos")]
fn macos_sdk_unsupported_by_llvm21_lld() -> bool {
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        let output = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .ok();
        let Some(output) = output else {
            return false;
        };
        if !output.status.success() {
            return false;
        }
        let sdk = String::from_utf8_lossy(&output.stdout);
        let sdk = sdk.trim();
        if sdk.is_empty() {
            return false;
        }
        let tbd = Path::new(sdk).join("usr/lib/libSystem.tbd");
        let Ok(text) = std::fs::read_to_string(tbd) else {
            return false;
        };
        text.contains("arm64e.x1-") || text.contains("arm64e.x2-")
    })
}

/// Whether this build can use LLD (in-process and/or install/prefix bin).
pub fn available() -> bool {
    if cfg!(syx_lld_inprocess) {
        true
    } else {
        cached_lld_bin().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_is_set() {
        let m = lld_mode();
        assert!(
            m == "inprocess" || m == "prefix-bin" || m == "missing",
            "unexpected mode {m}"
        );
    }
}
