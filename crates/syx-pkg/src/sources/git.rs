use std::path::Path;
use std::process::Command;

use syx_diagnostics::{Span, SyxResult};

use crate::lockfile::{try_load_lockfile, LockPackage, PackageSource};
use crate::manifest::{check_version_req, DependencySpec};
use crate::pkg_error_at;
use crate::sources::path::path_relative_to_project;
use crate::{install_locked, vendor_dir};

pub fn resolve_git_dependency(
    project_root: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<LockPackage> {
    let url = spec.resolve_git().ok_or_else(|| {
        pkg_error_at(
            format!("Git dependency '{dep_name}' has no git URL"),
            None,
            Span::DUMMY,
            "P031",
        )
    })?;

    // C-4a: locked install pins Git from syx.lock (symmetric with registry P014).
    if install_locked() {
        return resolve_git_dependency_locked(project_root, dep_name, spec, url);
    }

    // C-4a: manifest git rev is mandatory — never default to "main".
    let rev = spec.resolve_rev().ok_or_else(|| {
        pkg_error_at(
            format!("Git dependency '{dep_name}': git rev required (C-4a)"),
            None,
            Span::DUMMY,
            "P043",
        )
    })?;
    let req = spec.version_req()?.ok_or_else(|| {
        pkg_error_at(
            format!("Git dependency '{dep_name}' needs version"),
            None,
            Span::DUMMY,
            "P031",
        )
    })?;

    let vendor = vendor_dir(project_root);
    let dest = vendor.join(dep_name);
    std::fs::create_dir_all(&vendor).map_err(|e| {
        pkg_error_at(
            format!("Failed to create vendor dir: {e}"),
            None,
            Span::DUMMY,
            "P032",
        )
    })?;

    let resolved_rev = if dest.join(".git").exists() || dest.join("syx.toml").exists() {
        checkout_existing(&dest, rev)?
    } else {
        if dest.exists() {
            std::fs::remove_dir_all(&dest).ok();
        }
        clone_repo(url, &dest, rev)?
    };

    let _version = check_version_req(&dest, &req, dep_name)?;
    let rel_dir = path_relative_to_project(project_root, &dest);
    Ok(LockPackage {
        source: PackageSource::Git {
            git: url.to_string(),
            rev: resolved_rev,
            dir: rel_dir,
        },
    })
}

/// SYX_INSTALL_LOCKED: checkout the lock-pinned Git rev; do not re-resolve floating refs.
fn resolve_git_dependency_locked(
    project_root: &Path,
    dep_name: &str,
    spec: &DependencySpec,
    url: &str,
) -> SyxResult<LockPackage> {
    // Manifest still requires an explicit rev (fail-closed C-4a contract).
    let _manifest_rev = spec.resolve_rev().ok_or_else(|| {
        pkg_error_at(
            format!("Git dependency '{dep_name}': git rev required (C-4a)"),
            None,
            Span::DUMMY,
            "P043",
        )
    })?;
    let req = spec.version_req()?.ok_or_else(|| {
        pkg_error_at(
            format!("Git dependency '{dep_name}' needs version"),
            None,
            Span::DUMMY,
            "P031",
        )
    })?;

    let lock = try_load_lockfile(project_root).ok().flatten().ok_or_else(|| {
        pkg_error_at(
            "SYX_INSTALL_LOCKED requires an existing syx.lock",
            None,
            Span::DUMMY,
            "P014",
        )
    })?;
    let locked = lock.packages.get(dep_name).ok_or_else(|| {
        pkg_error_at(
            format!("SYX_INSTALL_LOCKED: package '{dep_name}' missing from syx.lock"),
            None,
            Span::DUMMY,
            "P014",
        )
    })?;

    let (lock_git, lock_rev, lock_dir) = match &locked.source {
        PackageSource::Git { git, rev, dir } => (git.clone(), rev.clone(), dir.clone()),
        _ => {
            return Err(pkg_error_at(
                format!("SYX_INSTALL_LOCKED: '{dep_name}' is not a git pin in syx.lock"),
                None,
                Span::DUMMY,
                "P014",
            ));
        }
    };

    if lock_git != url {
        return Err(pkg_error_at(
            format!(
                "SYX_INSTALL_LOCKED: git URL mismatch for '{dep_name}': lock={lock_git} manifest={url}"
            ),
            None,
            Span::DUMMY,
            "P014",
        ));
    }

    let dest = project_root.join(&lock_dir);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            pkg_error_at(
                format!("Failed to create vendor dir: {e}"),
                None,
                Span::DUMMY,
                "P032",
            )
        })?;
    }

    // Checkout the lock pin only — never re-resolve a floating branch from the manifest.
    // Checkout/clone failures under locked mode are P014 (pin unusable), not floating-resolve codes.
    let resolved_rev = {
        let result = if dest.join(".git").exists() || dest.join("syx.toml").exists() {
            checkout_existing(&dest, &lock_rev)
        } else {
            if dest.exists() {
                std::fs::remove_dir_all(&dest).ok();
            }
            clone_repo(&lock_git, &dest, &lock_rev)
        };
        result.map_err(|_| {
            pkg_error_at(
                format!(
                    "SYX_INSTALL_LOCKED: failed to checkout locked git rev '{lock_rev}' for '{dep_name}'"
                ),
                None,
                Span::DUMMY,
                "P014",
            )
        })?
    };

    if !revs_equal(&resolved_rev, &lock_rev) {
        return Err(pkg_error_at(
            format!(
                "SYX_INSTALL_LOCKED: git HEAD mismatch for '{dep_name}': HEAD={resolved_rev} lock={lock_rev}"
            ),
            None,
            Span::DUMMY,
            "P014",
        ));
    }

    let _version = check_version_req(&dest, &req, dep_name)?;
    Ok(LockPackage {
        source: PackageSource::Git {
            git: lock_git,
            rev: lock_rev,
            dir: lock_dir,
        },
    })
}

/// Compare git revs: trim; for hex OIDs compare case-insensitively; allow prefix match
/// when one side is a shortened OID of the other.
fn revs_equal(a: &str, b: &str) -> bool {
    let a = a.trim();
    let b = b.trim();
    if a == b {
        return true;
    }
    let a_hex = is_hex_oid(a);
    let b_hex = is_hex_oid(b);
    if a_hex && b_hex {
        let al = a.to_ascii_lowercase();
        let bl = b.to_ascii_lowercase();
        return al == bl || al.starts_with(&bl) || bl.starts_with(&al);
    }
    false
}

fn is_hex_oid(s: &str) -> bool {
    let len = s.len();
    (4..=40).contains(&len) && s.bytes().all(|c| c.is_ascii_hexdigit())
}

fn clone_repo(url: &str, dest: &Path, rev: &str) -> SyxResult<String> {
    let status = Command::new("git")
        .args([
            "clone",
            "-q",
            "--depth",
            "1",
            url,
            &dest.to_string_lossy(),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| {
            pkg_error_at(
                format!("Failed to run git clone: {e}"),
                None,
                Span::DUMMY,
                "P033",
            )
        })?;
    if !status.success() {
        return Err(pkg_error_at(
            format!("git clone failed for {url}"),
            None,
            Span::DUMMY,
            "P033",
        ));
    }
    checkout_rev(dest, rev)
}

fn checkout_existing(dest: &Path, rev: &str) -> SyxResult<String> {
    if dest.join(".git").exists() {
        checkout_rev(dest, rev)
    } else {
        Ok(rev.to_string())
    }
}

fn checkout_rev(dest: &Path, rev: &str) -> SyxResult<String> {
    let _ = Command::new("git")
        .args([
            "-C",
            &dest.to_string_lossy(),
            "fetch",
            "-q",
            "--depth",
            "1",
            "origin",
            rev,
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    let status = Command::new("git")
        .args([
            "-C",
            &dest.to_string_lossy(),
            "checkout",
            "-q",
            rev,
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| {
            pkg_error_at(
                format!("Failed to run git checkout: {e}"),
                None,
                Span::DUMMY,
                "P034",
            )
        })?;
    if !status.success() {
        return Err(pkg_error_at(
            format!("git checkout '{rev}' failed in {}", dest.display()),
            None,
            Span::DUMMY,
            "P034",
        ));
    }

    let output = Command::new("git")
        .args(["-C", &dest.to_string_lossy(), "rev-parse", "HEAD"])
        .output()
        .map_err(|e| {
            pkg_error_at(
                format!("Failed to run git rev-parse: {e}"),
                None,
                Span::DUMMY,
                "P035",
            )
        })?;
    if !output.status.success() {
        return Ok(rev.to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
