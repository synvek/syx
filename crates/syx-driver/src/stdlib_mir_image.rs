//! Embedded / disk stdlib MIR fragment for wasm/native (`stdlib_mir=true`).
//! Parallel to [`crate::rkyv_image`] (interpret bytecode), but stores lowered
//! [`MirFunction`] bodies so AOT Call targets stay module functions.

use crate::cache_fingerprint::{cache_version_tag, hash_source};
use crate::driver_options::project_cache_dir;
use crate::is_stdlib_module_path;
use crate::ModuleGraph;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use syx_mir::MirFunction;

/// Gen via `scripts/gen_prelude_snapshot.sh` / `.ps1`. Empty until regenerated.
pub(crate) static EMBEDDED_STDLIB_MIR: &[u8] =
    include_bytes!("../assets/prelude_stdlib_mir.bin");

pub(crate) static STDLIB_MIR_EMBED_HIT: AtomicU64 = AtomicU64::new(0);
pub(crate) static STDLIB_MIR_DISK_HIT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StdlibMirImage {
    pub cache_version_tag: String,
    pub modules_key: u64,
    /// Fingerprint of monomorphization instantiations used when lowering `functions`.
    #[serde(default)]
    pub mono_key: u64,
    /// Sorted `<stdlib>/…` paths covered by `functions` (prelude-only for embed).
    pub module_paths: Vec<String>,
    pub functions: Vec<MirFunction>,
}

pub(crate) fn stdlib_mir_disk_path(root: &Path) -> PathBuf {
    let tag = cache_version_tag();
    let digest = hash_source(&tag);
    project_cache_dir(root).join(format!("stdlib_mir_{digest}.bin"))
}

pub(crate) fn modules_key_from_paths_and_sources<'a, I>(items: I) -> u64
where
    I: IntoIterator<Item = (&'a Path, &'a str)>,
{
    let mut rows: Vec<(String, u64)> = items
        .into_iter()
        .map(|(p, src)| {
            let mut h = DefaultHasher::new();
            src.hash(&mut h);
            (p.to_string_lossy().into_owned(), h.finish())
        })
        .collect();
    rows.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut h = DefaultHasher::new();
    rows.hash(&mut h);
    h.finish()
}

pub(crate) fn stdlib_modules_key(graph: &ModuleGraph) -> u64 {
    modules_key_from_paths_and_sources(graph.modules.iter().filter_map(|(p, u)| {
        if is_stdlib_module_path(p) {
            Some((p.as_path(), u.source.contents.as_ref()))
        } else {
            None
        }
    }))
}

pub(crate) fn stdlib_module_paths(graph: &ModuleGraph) -> Vec<String> {
    let mut paths: Vec<String> = graph
        .modules
        .keys()
        .filter(|p| is_stdlib_module_path(p))
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    paths.sort();
    paths
}

pub(crate) fn build_image(
    graph: &ModuleGraph,
    functions: Vec<MirFunction>,
    mono_key: u64,
) -> Option<StdlibMirImage> {
    if functions.is_empty() {
        return None;
    }
    Some(StdlibMirImage {
        cache_version_tag: cache_version_tag(),
        modules_key: stdlib_modules_key(graph),
        mono_key,
        module_paths: stdlib_module_paths(graph),
        functions,
    })
}

pub(crate) fn write_image(path: &Path, image: &StdlibMirImage) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let bytes = bincode::serialize(image).map_err(|e| e.to_string())?;
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

pub(crate) fn load_image_bytes(bytes: &[u8]) -> Option<StdlibMirImage> {
    if bytes.is_empty() {
        return None;
    }
    let image: StdlibMirImage = bincode::deserialize(bytes).ok()?;
    if image.cache_version_tag != cache_version_tag() || image.functions.is_empty() {
        return None;
    }
    Some(image)
}

pub(crate) fn try_load_disk(
    root: &Path,
    modules_key: u64,
    mono_key: u64,
) -> Option<StdlibMirImage> {
    let path = stdlib_mir_disk_path(root);
    let bytes = std::fs::read(&path).ok()?;
    let image = load_image_bytes(&bytes)?;
    if image.modules_key != modules_key || image.mono_key != mono_key {
        return None;
    }
    STDLIB_MIR_DISK_HIT.fetch_add(1, Ordering::Relaxed);
    Some(image)
}

/// Prelude-only embed (caller bumps [`STDLIB_MIR_EMBED_HIT`] when applied).
pub(crate) fn try_load_embed() -> Option<StdlibMirImage> {
    load_image_bytes(EMBEDDED_STDLIB_MIR)
}

pub(crate) fn note_embed_hit() {
    STDLIB_MIR_EMBED_HIT.fetch_add(1, Ordering::Relaxed);
}

/// True when every path in `image` is present in `graph` stdlib set.
pub(crate) fn image_paths_subset_of_graph(image: &StdlibMirImage, graph: &ModuleGraph) -> bool {
    image.module_paths.iter().all(|p| {
        let pb = PathBuf::from(p);
        graph.modules.contains_key(&pb) && is_stdlib_module_path(&pb)
    })
}

pub(crate) fn store_disk(root: &Path, image: &StdlibMirImage) {
    let path = stdlib_mir_disk_path(root);
    let _ = write_image(&path, image);
}
