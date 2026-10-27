//! Resolve the SYX LLVM install prefix (user-facing: no versioned env vars).
//!
//! Order: `SYX_LLVM_PREFIX` → workspace sibling `../llvm` (must contain `bin/llvm-config`).

use std::path::{Path, PathBuf};
use std::process::Command;

/// Environment variable for an explicit LLVM install root (bin/, lib/, include/).
pub const SYX_LLVM_PREFIX_ENV: &str = "SYX_LLVM_PREFIX";

/// Resolve LLVM prefix. Returns `None` if neither env nor sibling is valid.
pub fn resolve_llvm_prefix() -> Option<PathBuf> {
    if let Ok(p) = std::env::var(SYX_LLVM_PREFIX_ENV) {
        let path = PathBuf::from(p);
        if is_valid_prefix(&path) {
            return Some(canonicalize_best_effort(&path));
        }
    }
    if let Some(sib) = sibling_llvm() {
        if is_valid_prefix(&sib) {
            return Some(canonicalize_best_effort(&sib));
        }
    }
    None
}

/// Like [`resolve_llvm_prefix`], but panics with a clear build-time message.
pub fn require_llvm_prefix(context: &str) -> PathBuf {
    resolve_llvm_prefix().unwrap_or_else(|| {
        panic!(
            "{context}: LLVM prefix not found. Place a full LLVM install at \
             <workspace>/../llvm or set {SYX_LLVM_PREFIX_ENV} to that prefix \
             (must contain bin/llvm-config)."
        )
    })
}

pub fn is_valid_prefix(prefix: &Path) -> bool {
    prefix.join("bin").join("llvm-config").is_file()
        || prefix.join("bin").join("llvm-config.exe").is_file()
}

pub fn llvm_config_path(prefix: &Path) -> PathBuf {
    let unix = prefix.join("bin").join("llvm-config");
    if unix.is_file() {
        return unix;
    }
    prefix.join("bin").join("llvm-config.exe")
}

/// Run `llvm-config --version` (e.g. `"21.1.6"`).
pub fn llvm_version(prefix: &Path) -> Option<String> {
    let cfg = llvm_config_path(prefix);
    let out = Command::new(&cfg).arg("--version").output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Major.minor from `llvm-config --version` (e.g. `(21, 1)`).
pub fn llvm_version_major_minor(prefix: &Path) -> Option<(u32, u32)> {
    let v = llvm_version(prefix)?;
    let mut parts = v.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor))
}

/// Ensure the prefix matches what the current inkwell / llvm-sys feature expects.
pub fn expect_llvm_major(prefix: &Path, expected_major: u32) {
    match llvm_version_major_minor(prefix) {
        Some((major, _)) if major == expected_major => {}
        Some((major, minor)) => panic!(
            "SYX LLVM at {} is {major}.{minor}, but this build expects LLVM {expected_major}.x \
             (inkwell feature). Use a matching install or set {SYX_LLVM_PREFIX_ENV}.",
            prefix.display()
        ),
        None => panic!(
            "failed to read LLVM version via {} --version",
            llvm_config_path(prefix).display()
        ),
    }
}

/// Path to `lld` / `ld64.lld` under the prefix, if present.
pub fn resolve_lld_binary(prefix: &Path) -> Option<PathBuf> {
    let bin = prefix.join("bin");
    for name in ["ld64.lld", "ld.lld", "lld", "lld-link", "lld-link.exe", "lld.exe"] {
        let p = bin.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn sibling_llvm() -> Option<PathBuf> {
    // CARGO_MANIFEST_DIR is crates/<name>; workspace root is ../..
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR")?);
    let workspace = manifest.parent()?.parent()?;
    Some(workspace.join("..").join("llvm"))
}

fn canonicalize_best_effort(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_name_has_no_version_digits() {
        assert!(!SYX_LLVM_PREFIX_ENV.chars().any(|c| c.is_ascii_digit()));
    }
}
