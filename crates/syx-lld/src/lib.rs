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

fn prefix_lld_binary() -> Option<PathBuf> {
    if let Some(p) = option_env!("SYX_LLD_BIN") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }
    if let Some(prefix) = resolved_llvm_prefix() {
        return syx_llvm_prefix::resolve_lld_binary(prefix);
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

/// Whether this build can use prefix LLD (in-process or bin).
pub fn available() -> bool {
    #[cfg(syx_lld_inprocess)]
    {
        return true;
    }
    cached_lld_bin().is_some()
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
