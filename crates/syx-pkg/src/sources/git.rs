use std::path::{Path, PathBuf};
use std::process::Command;

use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::lockfile::{LockPackage, PackageSource};
use crate::manifest::{check_version_req, DependencySpec};
use crate::sources::path::path_relative_to_project;
use crate::vendor_dir;

pub fn resolve_git_dependency(
    project_root: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<LockPackage> {
    let url = spec.resolve_git().ok_or_else(|| {
        SyxDiagnostic::error(format!("Git dependency '{dep_name}' has no git URL"))
    })?;
    let rev = spec.resolve_rev().unwrap_or("main");
    let req = spec
        .version_req()?
        .ok_or_else(|| SyxDiagnostic::error(format!("Git dependency '{dep_name}' needs version")))?;

    let vendor = vendor_dir(project_root);
    let dest = vendor.join(dep_name);
    std::fs::create_dir_all(&vendor).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to create vendor dir: {e}"))
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

fn clone_repo(url: &str, dest: &Path, rev: &str) -> SyxResult<String> {
    let status = Command::new("git")
        .args(["clone", "--depth", "1", url, &dest.to_string_lossy()])
        .status()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to run git clone: {e}")))?;
    if !status.success() {
        return Err(SyxDiagnostic::error(format!(
            "git clone failed for {url}"
        )));
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
    let fetch = Command::new("git")
        .args(["-C", &dest.to_string_lossy(), "fetch", "--depth", "1", "origin", rev])
        .status();
    let _ = fetch;

    let status = Command::new("git")
        .args(["-C", &dest.to_string_lossy(), "checkout", rev])
        .status()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to run git checkout: {e}")))?;
    if !status.success() {
        return Err(SyxDiagnostic::error(format!(
            "git checkout '{rev}' failed in {}",
            dest.display()
        )));
    }

    let output = Command::new("git")
        .args(["-C", &dest.to_string_lossy(), "rev-parse", "HEAD"])
        .output()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to run git rev-parse: {e}")))?;
    if !output.status.success() {
        return Ok(rev.to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
