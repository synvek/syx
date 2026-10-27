use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use semver::Version;
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::lockfile::{LockPackage, SyxLockfile};
use crate::manifest::{
    default_registry_url, load_manifest, registry_url_chain, DependencySourceKind, DependencySpec,
    SyxManifest,
};
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
                        return Err(SyxDiagnostic::error(format!(
                            "Semver conflict for '{0}': already resolved to {existing_version}, \
                             new requirement {req} is incompatible",
                            item.name
                        ))
                        .with_code("R001"));
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
    let artifact_cdn_base = std::env::var("SYX_REGISTRY_CDN")
        .ok()
        .filter(|value| !value.is_empty());
    let artifact_cdn_regions: Vec<String> = crate::registry_cdn_phase8_regions()
        .iter()
        .map(|s| s.to_string())
        .collect();
    let resolved_region = artifact_cdn_regions
        .first()
        .cloned()
        .unwrap_or_default();
    let cache_remote_nodes: Vec<String> = std::env::var("SYX_CACHE_REMOTE_NODES")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty())
                .collect()
        })
        .filter(|v: &Vec<String>| !v.is_empty())
        .unwrap_or_else(|| {
            std::env::var("SYX_CACHE_REMOTE")
                .ok()
                .filter(|s| !s.is_empty())
                .into_iter()
                .collect()
        });
    let farm_queue_url = std::env::var("SYX_BUILD_FARM_QUEUE")
        .ok()
        .filter(|s| !s.is_empty());
    let marketplace_cdn_base = std::env::var("SYX_MARKETPLACE_CDN")
        .ok()
        .filter(|s| !s.is_empty());
    let plugin_unload_epoch = std::env::var("SYX_PLUGIN_UNLOAD_EPOCH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let publisher_key_rotation = std::env::var("SYX_PUBLISHER_KEY_ROTATION")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let farm_worker_id = std::env::var("SYX_FARM_WORKER_ID")
        .ok()
        .filter(|s| !s.is_empty());
    SyxLockfile {
        version: 16,
        syx_version: "60.0".into(),
        syx_version_floor: "59.0".into(),
        resolved_at,
        registry,
        registry_mirrors,
        artifact_cdn_base,
        artifact_cdn_regions,
        resolved_region,
        cache_remote_nodes,
        plugin_unload_epoch,
        marketplace_cdn_base,
        farm_queue_url,
        publisher_key_rotation,
        farm_worker_id,
        plugin_bundle_sha256: IndexMap::new(),
        tenant_id: None,
        rotation_cron_spec: None,
        farm_lease_ttl_secs: 300,
        farm_worker_pool: Vec::new(),
        hsm_key_ref: std::env::var("SYX_HSM_KEY_REF").ok().filter(|s| !s.is_empty()),
        billing_meter_id: std::env::var("SYX_BILLING_METER_ID").ok().filter(|s| !s.is_empty()),
        hsm_rotation_cron_job: std::env::var("SYX_HSM_ROTATION_CRON_JOB")
            .ok()
            .filter(|s| !s.is_empty()),
        billing_settlement_id: std::env::var("SYX_BILLING_SETTLEMENT_ID")
            .ok()
            .filter(|s| !s.is_empty()),
        marketplace_plugin_epoch: std::env::var("SYX_MARKETPLACE_PLUGIN_EPOCH")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        marketplace_publish_epoch: std::env::var("SYX_MARKETPLACE_PUBLISH_EPOCH")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        extension_signing_key_id: std::env::var("SYX_EXTENSION_SIGNING_KEY_ID")
            .ok()
            .filter(|s| !s.is_empty()),
        publisher_key_id: std::env::var("SYX_PUBLISHER_KEY_ID").unwrap_or_default(),
        signature_policy: crate::lockfile::SignaturePolicy::Mandatory,
        tarball_sha256,
        packages: graph.packages.clone(),
        plugins: vec![],
    }
}
