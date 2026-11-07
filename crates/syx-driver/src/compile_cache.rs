//! SYX driver incremental compile cache face.
//! Extracted in S2-I228 (zero semantics).

use crate::cache_fingerprint::{
    cache_version_tag, dependency_hashes_from_graph, dependency_hashes_still_valid,
    dependency_tree_digest, hash_source, module_hash,
};
use crate::farm_tcp::{
    cache_remote_push_enabled, fetch_cache_remote, incremental_cache_path,
    purge_stale_incremental_caches, push_cache_remote_nodes, record_cache_remote_fetch,
};
use crate::module_ref::ModuleRef;
use crate::{
    compile_module_recursive, ensure_project_lock, DriverOptions, ModuleGraph, ModuleUnit,
};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::path::{Path, PathBuf};
use syx_codegen::CompiledProgram;
use syx_diagnostics::{SourceFile, SyxResult};
use syx_pkg::{load_manifest, load_resolved_packages};

pub fn compile_with_cache(path: &Path, root: &Path) -> SyxResult<ModuleUnit> {
    purge_stale_incremental_caches(root);
    let cache = incremental_cache_path(root, path);
    let current_source = std::fs::read_to_string(path).unwrap_or_default();
    if cache.exists() {
        if let Ok(bytes) = std::fs::read(&cache) {
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    return Ok(cached.into());
                }
            }
        }
    } else if let Some(remote) = std::env::var("SYX_CACHE_REMOTE")
        .ok()
        .filter(|s| !s.is_empty())
    {
        let key = module_hash(path, root, &[]);
        let url = format!("{}/{}", remote.trim_end_matches('/'), format!("{key}.bin"));
        if let Ok(bytes) = fetch_cache_remote(&url) {
            record_cache_remote_fetch(&url);
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    if let Some(parent) = cache.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = std::fs::write(&cache, &bytes);
                    return Ok(cached.into());
                }
            }
        }
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = DriverOptions::default();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, &opts)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    if let Some(parent_dir) = cache.parent() {
        let _ = std::fs::create_dir_all(parent_dir);
    }
    if let Ok(bytes) = bincode_encode(&unit, &graph) {
        let _ = std::fs::write(&cache, &bytes);
        if cache_remote_push_enabled() {
            if let Some(remote) = std::env::var("SYX_CACHE_REMOTE")
                .ok()
                .filter(|s| !s.is_empty())
            {
                let key = format!("{}.bin", module_hash(path, root, &[]));
                push_cache_remote_nodes(&remote, &key, &bytes);
            }
        }
    }
    Ok(unit)
}

pub(crate) fn bincode_encode(unit: &ModuleUnit, graph: &ModuleGraph) -> Result<Vec<u8>, String> {
    let cached = CachedModuleUnit::from_unit(unit, graph);
    bincode::serialize(&cached).map_err(|e| e.to_string())
}

fn bincode_decode(bytes: &[u8]) -> Result<ModuleUnit, String> {
    let cached: CachedModuleUnit = bincode::deserialize(bytes).map_err(|e| e.to_string())?;
    Ok(cached.into())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CachedModuleUnit {
    pub path: PathBuf,
    pub source_name: String,
    pub source_contents: String,
    pub source_hash: String,
    pub imported_modules: Vec<String>,
    pub dependency_hashes: Vec<(String, String)>,
    #[serde(default)]
    pub dependency_tree_hash: String,
    /// When set, matches `cache_version_tag()` and skips per-dep stdlib re-hash on load.
    #[serde(default)]
    pub cache_version_tag: String,
    pub program: CompiledProgram,
}

impl CachedModuleUnit {
    fn from_unit(unit: &ModuleUnit, graph: &ModuleGraph) -> Self {
        Self {
            path: unit.path.clone(),
            source_name: unit.source.name.clone(),
            source_contents: unit.source.contents.to_string(),
            source_hash: hash_source(&unit.source.contents),
            imported_modules: unit
                .imported_modules
                .iter()
                .map(|m| m.to_string())
                .collect(),
            dependency_hashes: {
                let deps = dependency_hashes_from_graph(graph, &unit.path);
                deps
            },
            dependency_tree_hash: {
                let deps = dependency_hashes_from_graph(graph, &unit.path);
                dependency_tree_digest(&deps)
            },
            cache_version_tag: cache_version_tag(),
            program: unit.program.clone(),
        }
    }
}

/// Fast cache hit check: source unchanged and toolchain/stdlib tag matches (skips per-dep stdlib re-hash).
pub(crate) fn cache_entry_fresh(cached: &CachedModuleUnit, current_source: &str) -> bool {
    if cached.source_hash != hash_source(current_source) {
        return false;
    }
    let tag = cache_version_tag();
    if !cached.cache_version_tag.is_empty() && cached.cache_version_tag == tag {
        return true;
    }
    dependency_hashes_still_valid(&cached.dependency_hashes)
}

impl From<&ModuleUnit> for CachedModuleUnit {
    fn from(unit: &ModuleUnit) -> Self {
        Self {
            path: unit.path.clone(),
            source_name: unit.source.name.clone(),
            source_contents: unit.source.contents.to_string(),
            source_hash: hash_source(&unit.source.contents),
            imported_modules: unit
                .imported_modules
                .iter()
                .map(|m| m.to_string())
                .collect(),
            dependency_hashes: Vec::new(),
            dependency_tree_hash: String::new(),
            cache_version_tag: cache_version_tag(),
            program: unit.program.clone(),
        }
    }
}

impl From<CachedModuleUnit> for ModuleUnit {
    fn from(cached: CachedModuleUnit) -> Self {
        Self {
            path: cached.path,
            source: SourceFile::new(cached.source_name, cached.source_contents),
            imported_modules: cached
                .imported_modules
                .into_iter()
                .map(SmolStr::new)
                .collect(),
            program: cached.program,
        }
    }
}
