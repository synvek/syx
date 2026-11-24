use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use semver::Version;
use syx_diagnostics::{Span, SyxResult};

use crate::lockfile::{LockPackage, SyxLockfile};
use crate::manifest::{
    default_registry_url, load_manifest, registry_url_chain, DependencySourceKind, DependencySpec,
    SyxManifest,
};
use crate::pkg_error_at;
use crate::sources::git::resolve_git_dependency;
use crate::sources::path::resolve_path_dependency;
use crate::sources::registry::resolve_registry_dependency;

#[derive(Debug, Clone, Default)]
pub struct ResolvedGraph {
    pub packages: IndexMap<String, LockPackage>,
}

struct QueueItem {
    name: String,
    spec: DependencySpec,
    resolve_base: PathBuf,
}

pub fn resolve_dependencies(project_root: &Path) -> SyxResult<ResolvedGraph> {
    let manifest = load_manifest(project_root)?;
    let mut graph = ResolvedGraph::default();
    let mut queue: VecDeque<QueueItem> = VecDeque::new();

    for (name, spec) in &manifest.dependencies {
        queue.push_back(QueueItem {
            name: name.clone(),
            spec: spec.clone(),
            resolve_base: project_root.to_path_buf(),
        });
    }

    while let Some(item) = queue.pop_front() {
        if let Some(existing) = graph.packages.get(&item.name) {
            if let Some(req) = item.spec.version_req()? {
                let existing_version = existing.resolved_version();
                if let Ok(v) = Version::parse(&existing_version) {
                    if !req.matches(&v) {
                        return Err(pkg_error_at(format!(
                                "Semver conflict for '{0}': already resolved to {existing_version}, \
                             new requirement {req} is incompatible",
                                item.name
                            ), None, Span::DUMMY, "R001"));
                    }
                }
            }
            continue;
        }

        let lock_pkg = resolve_one(
            project_root,
            &item.resolve_base,
            &item.name,
            &item.spec,
            &manifest,
        )?;
        let pkg_root = project_root.join(lock_pkg.root_relative());

        if pkg_root.join("syx.toml").exists() {
            if let Ok(sub_manifest) = load_manifest(&pkg_root) {
                // Wave C W676 / C-01 first slice: always enqueue transitive edges so
                // VersionReq is re-checked against already-resolved packages (R001).
                // Do not skip when `graph.packages` already contains `sub_name`.
                for (sub_name, sub_spec) in &sub_manifest.dependencies {
                    queue.push_back(QueueItem {
                        name: sub_name.clone(),
                        spec: sub_spec.clone(),
                        resolve_base: pkg_root.clone(),
                    });
                }
            }
        }

        graph.packages.insert(item.name, lock_pkg);
    }

    Ok(graph)
}

fn resolve_one(
    project_root: &Path,
    resolve_base: &Path,
    dep_name: &str,
    spec: &DependencySpec,
    root_manifest: &SyxManifest,
) -> SyxResult<LockPackage> {
    match spec.source_kind() {
        DependencySourceKind::Path => {
            resolve_path_dependency(project_root, resolve_base, dep_name, spec)
        }
        DependencySourceKind::Git => resolve_git_dependency(project_root, dep_name, spec),
        DependencySourceKind::Registry => {
            resolve_registry_dependency(project_root, dep_name, spec, root_manifest)
        }
    }
}

/// C-4d: default locks carry dependency reproducibility only — no CDN/farm/billing
/// env snapshots. Infra fields stay on `SyxLockfile` for old locks + enforce_*; runtime
/// CDN/farm continue to read process env directly.
pub fn graph_to_lockfile(graph: &ResolvedGraph, manifest: &SyxManifest) -> SyxLockfile {
    let mut tarball_sha256 = IndexMap::new();
    for (name, pkg) in &graph.packages {
        if let crate::lockfile::PackageSource::Registry { checksum, .. } = &pkg.source {
            tarball_sha256.insert(name.clone(), checksum.clone());
        }
    }
    let resolved_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());
    let registry = default_registry_url(manifest);
    let registry_mirrors: Vec<String> = registry_url_chain(manifest)
        .into_iter()
        .filter(|url| url != &registry)
        .collect();
    SyxLockfile {
        version: 16,
        syx_version: "60.0".into(),
        syx_version_floor: "59.0".into(),
        resolved_at,
        registry,
        registry_mirrors,
        publisher_key_id: std::env::var("SYX_PUBLISHER_KEY_ID").unwrap_or_default(),
        signature_policy: crate::lockfile::SignaturePolicy::Mandatory,
        tarball_sha256,
        packages: graph.packages.clone(),
        plugins: vec![],
        // C-4d: leave CDN/farm/billing/marketplace infra at Default (empty).
        ..SyxLockfile::default()
    }
}
