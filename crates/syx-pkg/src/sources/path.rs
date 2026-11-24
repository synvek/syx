use std::path::{Path, PathBuf};

use semver::Version;
use sha2::{Digest, Sha256};
use syx_diagnostics::{Span, SyxResult};

use crate::lockfile::{try_load_lockfile, LockPackage, PackageSource};
use crate::manifest::{check_version_req, load_manifest, DependencySpec};
use crate::pkg_error_at;
use crate::install_locked;

/// Fixed exclude names for path-tree content hashing (C-4e). No ignore engine.
const PATH_HASH_EXCLUDE: &[&str] = &[".git", "target", ".DS_Store"];

pub fn resolve_path_dependency(
    project_root: &Path,
    resolve_base: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<LockPackage> {
    if install_locked() {
        return resolve_path_dependency_locked(project_root, resolve_base, dep_name, spec);
    }

    let (canonical, version, rel) =
        resolve_path_location(project_root, resolve_base, dep_name, spec)?;
    let checksum = path_tree_checksum(&canonical)?;
    Ok(LockPackage {
        source: PackageSource::Path {
            version: version.to_string(),
            path: rel,
            checksum,
        },
    })
}

/// C-4e: Locked install pins path content from syx.lock (symmetric with git/registry P014).
fn resolve_path_dependency_locked(
    project_root: &Path,
    resolve_base: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<LockPackage> {
    let (canonical, _version, rel) =
        resolve_path_location(project_root, resolve_base, dep_name, spec)?;

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

    let (lock_version, lock_path, lock_checksum) = match &locked.source {
        PackageSource::Path {
            version,
            path,
            checksum,
        } => (version.clone(), path.clone(), checksum.clone()),
        _ => {
            return Err(pkg_error_at(
                format!("SYX_INSTALL_LOCKED: '{dep_name}' is not a path pin in syx.lock"),
                None,
                Span::DUMMY,
                "P014",
            ));
        }
    };

    if lock_checksum.is_empty() {
        return Err(pkg_error_at(
            format!(
                "SYX_INSTALL_LOCKED: path package '{dep_name}' missing checksum in syx.lock (C-4e)"
            ),
            None,
            Span::DUMMY,
            "P014",
        ));
    }

    if lock_path != rel {
        return Err(pkg_error_at(
            format!(
                "SYX_INSTALL_LOCKED: path mismatch for '{dep_name}': lock={lock_path} resolved={rel} (C-4e)"
            ),
            None,
            Span::DUMMY,
            "P014",
        ));
    }

    let actual = path_tree_checksum(&canonical)?;
    if actual != lock_checksum {
        return Err(pkg_error_at(
            format!(
                "SYX_INSTALL_LOCKED: path content hash mismatch for '{dep_name}': lock={lock_checksum} actual={actual} (C-4e)"
            ),
            None,
            Span::DUMMY,
            "P014",
        ));
    }

    Ok(LockPackage {
        source: PackageSource::Path {
            version: lock_version,
            path: lock_path,
            checksum: lock_checksum,
        },
    })
}

fn resolve_path_location(
    project_root: &Path,
    resolve_base: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<(PathBuf, Version, String)> {
    let path_str = spec.resolve_path().ok_or_else(|| {
        pkg_error_at(
            format!("Path dependency '{dep_name}' has no path"),
            None,
            Span::DUMMY,
            "P040",
        )
    })?;
    let abs = resolve_base.join(path_str);
    let canonical = abs.canonicalize().unwrap_or(abs);
    if !canonical.exists() {
        return Err(pkg_error_at(
            format!(
                "Path dependency '{dep_name}' not found at {}",
                canonical.display()
            ),
            None,
            Span::DUMMY,
            "P040",
        ));
    }
    let req = spec.version_req()?.ok_or_else(|| {
        pkg_error_at(
            format!("Path dependency '{dep_name}' needs version"),
            None,
            Span::DUMMY,
            "P040",
        )
    })?;
    let version = check_version_req(&canonical, &req, dep_name)?;
    let rel = path_relative_to_project(project_root, &canonical);
    Ok((canonical, version, rel))
}

/// Deterministic directory content hash for path dependencies (C-4e).
/// Walks files in relative-path lexicographic order; hashes `rel_path + NUL + bytes`.
/// Excludes only `.git`, `target`, and `.DS_Store` (by name, at any depth).
pub fn path_tree_checksum(root: &Path) -> SyxResult<String> {
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_hashable_files(root, root, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut hasher = Sha256::new();
    for (rel, abs) in &files {
        hasher.update(rel.as_bytes());
        hasher.update([0u8]);
        let bytes = std::fs::read(abs).map_err(|e| {
            pkg_error_at(
                format!("Failed to read path dep file {}: {e}", abs.display()),
                None,
                Span::DUMMY,
                "P040",
            )
        })?;
        hasher.update(&bytes);
    }
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn collect_hashable_files(
    root: &Path,
    dir: &Path,
    out: &mut Vec<(String, PathBuf)>,
) -> SyxResult<()> {
    let entries = std::fs::read_dir(dir).map_err(|e| {
        pkg_error_at(
            format!("Failed to read path dep dir {}: {e}", dir.display()),
            None,
            Span::DUMMY,
            "P040",
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| {
            pkg_error_at(
                format!("Failed to read path dep entry under {}: {e}", dir.display()),
                None,
                Span::DUMMY,
                "P040",
            )
        })?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if PATH_HASH_EXCLUDE.iter().any(|x| *x == name_str.as_ref()) {
            continue;
        }
        let path = entry.path();
        let ft = entry.file_type().map_err(|e| {
            pkg_error_at(
                format!("Failed to stat {}: {e}", path.display()),
                None,
                Span::DUMMY,
                "P040",
            )
        })?;
        if ft.is_dir() {
            collect_hashable_files(root, &path, out)?;
        } else if ft.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, path));
        }
    }
    Ok(())
}

pub fn path_relative_to_project(project_root: &Path, target: &Path) -> String {
    let project = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let target = target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf());
    pathdiff::diff_paths(&target, &project)
        .unwrap_or(target)
        .to_string_lossy()
        .into_owned()
}

/// Simple relative path without extra crate if pathdiff not available - use manual
pub mod pathdiff {
    use std::path::{Component, Path, PathBuf};

    pub fn diff_paths(path: &Path, base: &Path) -> Option<PathBuf> {
        let path = path.components().collect::<Vec<_>>();
        let base = base.components().collect::<Vec<_>>();
        let mut i = 0;
        while i < path.len() && i < base.len() && path[i] == base[i] {
            i += 1;
        }
        let mut result = PathBuf::new();
        for _ in i..base.len() {
            result.push("..");
        }
        for component in &path[i..] {
            match component {
                Component::Normal(c) => result.push(c),
                Component::CurDir => {}
                Component::ParentDir => result.push(".."),
                _ => return None,
            }
        }
        if result.as_os_str().is_empty() {
            Some(PathBuf::from("."))
        } else {
            Some(result)
        }
    }
}

#[allow(dead_code)] // path-source manifest probe retained for pkg deepen
pub fn package_manifest_at(root: &Path) -> SyxResult<(String, Version)> {
    let manifest = load_manifest(root)?;
    let version = Version::parse(&manifest.version).map_err(|e| {
        pkg_error_at(
            format!("Invalid version in {}: {e}", root.display()),
            None,
            Span::DUMMY,
            "P041",
        )
    })?;
    Ok((manifest.name, version))
}
