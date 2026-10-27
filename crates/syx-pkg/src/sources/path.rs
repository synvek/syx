use std::path::{Path, PathBuf};

use semver::Version;
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::lockfile::{LockPackage, PackageSource};
use crate::manifest::{check_version_req, load_manifest, DependencySpec};

pub fn resolve_path_dependency(
    project_root: &Path,
    resolve_base: &Path,
    dep_name: &str,
    spec: &DependencySpec,
) -> SyxResult<LockPackage> {
    let path_str = spec.resolve_path().ok_or_else(|| {
        SyxDiagnostic::error(format!("Path dependency '{dep_name}' has no path"))
    })?;
    let abs = resolve_base.join(path_str);
    let canonical = abs.canonicalize().unwrap_or(abs);
    if !canonical.exists() {
        return Err(SyxDiagnostic::error(format!(
            "Path dependency '{dep_name}' not found at {}",
            canonical.display()
        )));
    }
    let req = spec
        .version_req()?
        .ok_or_else(|| SyxDiagnostic::error(format!("Path dependency '{dep_name}' needs version")))?;
    let version = check_version_req(&canonical, &req, dep_name)?;
    let rel = path_relative_to_project(project_root, &canonical);
    Ok(LockPackage {
        source: PackageSource::Path {
            version: version.to_string(),
            path: rel,
        },
    })
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

pub fn package_manifest_at(root: &Path) -> SyxResult<(String, Version)> {
    let manifest = load_manifest(root)?;
    let version = Version::parse(&manifest.version).map_err(|e| {
        SyxDiagnostic::error(format!("Invalid version in {}: {e}", root.display()))
    })?;
    Ok((manifest.name, version))
}
