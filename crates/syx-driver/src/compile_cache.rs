//! SYX driver incremental compile cache face.
//! Extracted in S2-I228 (zero semantics).

use crate::cache_fingerprint::{
    cache_version_tag, dependency_hashes_from_graph, dependency_hashes_still_valid,
    dependency_tree_digest, hash_source, module_hash,
};
use crate::driver_options::project_cache_dir;
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
use std::sync::atomic::{AtomicU64, Ordering};
use syx_codegen::CompiledProgram;
use syx_diagnostics::{SourceFile, SyxResult};
use syx_pkg::{load_manifest, load_resolved_packages};

/// Census nail (S3-I49): dedicated prelude snapshot loads (disk).
pub(crate) static PRELUDE_SNAPSHOT_HIT: AtomicU64 = AtomicU64::new(0);

/// Census nail (S3-I54): embedded prelude snapshot loads (`include_bytes` fallback).
pub(crate) static PRELUDE_EMBED_HIT: AtomicU64 = AtomicU64::new(0);

/// Census nail: rkyv VM-image / process-hydrated prelude loads.
pub(crate) static PRELUDE_RKYV_HIT: AtomicU64 = AtomicU64::new(0);

/// S3-I54: checked-in PreludeSnapshot bincode (gen via `scripts/gen_prelude_snapshot.sh`).
pub(crate) static EMBEDDED_PRELUDE_SNAPSHOT: &[u8] =
    include_bytes!("../assets/prelude_snapshot.bin");

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
    // Explicit cache API: always allow disk prelude/entry artifacts.
    let mut opts = DriverOptions {
        incremental_cache: true,
        ..DriverOptions::default()
    };
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
    // S3-I53: roundtrip requires FunctionUnit/CompiledProgram without skip_serializing_if.
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

/// S3-I49: dedicated prelude/stdlib graph snapshot (≠ entry module `.bin`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreludeSnapshot {
    pub cache_version_tag: String,
    pub order: Vec<PathBuf>,
    pub modules: Vec<CachedModuleUnit>,
    /// Precomputed extension methods so interpret path skips re-parsing prelude sources.
    #[serde(default)]
    pub extension_methods: Vec<(String, String)>,
    /// Precomputed exports: `(module_path, [(name, type_wire), ...])`.
    #[serde(default)]
    pub module_exports: Vec<(String, Vec<(String, String)>)>,
}

/// Disk path for prelude snapshot under `project_cache_dir` (S3-I49).
/// Does not create directories — store path must `create_dir_all` when writing.
pub(crate) fn prelude_cache_path(root: &Path) -> PathBuf {
    let tag = cache_version_tag();
    let digest = hash_source(&tag);
    project_cache_dir(root).join(format!("prelude_{digest}.bin"))
}

/// Apply a deserialized [`PreludeSnapshot`] into `graph` when tag matches.
fn apply_prelude_snapshot(graph: &mut ModuleGraph, snap: PreludeSnapshot) -> bool {
    if snap.cache_version_tag != cache_version_tag() || snap.modules.is_empty() {
        return false;
    }
    if !snap.extension_methods.is_empty() {
        graph.stdlib_extension_methods = snap
            .extension_methods
            .into_iter()
            .map(|(a, b)| (SmolStr::new(a), SmolStr::new(b)))
            .collect();
    }
    if !snap.module_exports.is_empty() {
        use crate::stdlib_fastpath::decode_type_id;
        graph.stdlib_exports = snap
            .module_exports
            .into_iter()
            .map(|(path, exports)| {
                (
                    PathBuf::from(path),
                    exports
                        .into_iter()
                        .map(|(n, tw)| (SmolStr::new(n), decode_type_id(&tw)))
                        .collect(),
                )
            })
            .collect();
    }
    for cached in snap.modules {
        let path = cached.path.clone();
        if graph.modules.contains_key(&path) {
            continue;
        }
        let unit: ModuleUnit = cached.into();
        graph.modules.insert(path.clone(), unit);
    }
    for key in snap.order {
        if graph.modules.contains_key(&key) && !graph.order.contains(&key) {
            graph.order.push(key);
        }
    }
    // Ensure any modules missing from order still appear (defensive).
    let missing: Vec<PathBuf> = graph
        .modules
        .keys()
        .filter(|k| !graph.order.contains(k))
        .cloned()
        .collect();
    graph.order.extend(missing);
    true
}

/// Disk path for the slim rkyv VM image (same tag digest as bincode prelude).
pub(crate) fn prelude_rkyv_cache_path(root: &Path) -> PathBuf {
    let tag = cache_version_tag();
    let digest = hash_source(&tag);
    project_cache_dir(root).join(format!("prelude_{digest}.rkyv"))
}

/// Load prelude snapshot into an empty-ish graph when tag matches.
/// Prefer rkyv VM image / process hydrate, then bincode disk, then embed.
/// When `allow_disk` is false, skip disk probe (embed only) — no mkdir side effects.
/// Disk hit bumps [`PRELUDE_SNAPSHOT_HIT`]; embed fallback bumps [`PRELUDE_EMBED_HIT`] (S3-I54).
pub(crate) fn load_prelude_snapshot(
    graph: &mut ModuleGraph,
    root: &Path,
    allow_disk: bool,
) -> bool {
    // `compile_module_recursive` calls ensure_prelude for every new import.
    // If this graph already carries stdlib units (prelude and/or prior imports like
    // Math), treat as hit — never replace/wipe (hydrate OnceLock must not clobber).
    if graph
        .modules
        .keys()
        .any(|p| crate::is_stdlib_module_path(p))
    {
        return true;
    }
    let disk_rkyv = if allow_disk {
        Some(prelude_rkyv_cache_path(root))
    } else {
        None
    };
    if crate::rkyv_image::try_load_rkyv_into_graph(graph, disk_rkyv.as_deref()) {
        PRELUDE_RKYV_HIT.fetch_add(1, Ordering::Relaxed);
        return true;
    }
    if allow_disk {
        let path = prelude_cache_path(root);
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(snap) = bincode::deserialize::<PreludeSnapshot>(&bytes) {
                if apply_prelude_snapshot(graph, snap) {
                    PRELUDE_SNAPSHOT_HIT.fetch_add(1, Ordering::Relaxed);
                    crate::rkyv_image::store_hydrated_graph(graph.clone());
                    return true;
                }
            }
        }
    }
    // S3-I54: empty-cache cold path — try embedded PreludeSnapshot bytes.
    if !EMBEDDED_PRELUDE_SNAPSHOT.is_empty() {
        if let Ok(snap) = bincode::deserialize::<PreludeSnapshot>(EMBEDDED_PRELUDE_SNAPSHOT) {
            if apply_prelude_snapshot(graph, snap) {
                PRELUDE_EMBED_HIT.fetch_add(1, Ordering::Relaxed);
                crate::rkyv_image::store_hydrated_graph(graph.clone());
                return true;
            }
        }
    }
    false
}

/// Persist current graph prelude modules (`<stdlib>/…`) after a successful ensure.
/// No-op when `allow_disk` is false.
///
/// Dedupes function bodies across modules (ensure historically accumulated O(n²)
/// merges into each unit) and drops per-module `host_fn_names` copies — interpret
/// fills the global table from `all_host_fn_names()` at use sites.
pub(crate) fn store_prelude_snapshot(graph: &ModuleGraph, root: &Path, allow_disk: bool) {
    // Always keep a process-local hydrate after a cold ensure (daemon / multi-run).
    crate::rkyv_image::store_hydrated_graph(graph.clone());
    if !allow_disk {
        return;
    }
    let tag = cache_version_tag();
    let mut order = Vec::new();
    let mut modules = Vec::new();
    let mut seen_fns = std::collections::HashSet::<String>::new();
    let extension_methods: Vec<(String, String)> = {
        let set = if graph.stdlib_extension_methods.is_empty() {
            crate::collect_stdlib_extension_methods(graph)
        } else {
            graph.stdlib_extension_methods.clone()
        };
        set.into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect()
    };
    let mut module_exports: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for key in &graph.order {
        let Some(unit) = graph.modules.get(key) else {
            continue;
        };
        let is_stdlib = key.to_string_lossy().starts_with("<stdlib>/");
        if !is_stdlib {
            continue;
        }
        order.push(key.clone());
        let mut cached = CachedModuleUnit::from(unit);
        cached
            .program
            .functions
            .retain(|f| seen_fns.insert(f.name.to_string()));
        // Shared HostFn name table — avoid 34× copies in the embed.
        cached.program.host_fn_names.clear();
        modules.push(cached);
        let exports = if let Some(cached_ex) = graph.stdlib_exports.get(key) {
            cached_ex.clone()
        } else {
            crate::export_symbols(&unit.source)
        };
        module_exports.push((
            key.to_string_lossy().into_owned(),
            exports
                .into_iter()
                .map(|(n, ty)| {
                    (
                        n.to_string(),
                        crate::stdlib_fastpath::encode_type_id(&ty),
                    )
                })
                .collect(),
        ));
    }
    if modules.is_empty() {
        return;
    }
    let snap = PreludeSnapshot {
        cache_version_tag: tag,
        order,
        modules,
        extension_methods,
        module_exports,
    };
    let path = prelude_cache_path(root);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(bytes) = bincode::serialize(&snap) {
        let _ = std::fs::write(path, bytes);
    }
    if let Some(image) = crate::rkyv_image::build_rkyv_image_from_graph(graph) {
        let rkyv_path = prelude_rkyv_cache_path(root);
        let _ = crate::rkyv_image::write_rkyv_image(&rkyv_path, &image);
    }
}
