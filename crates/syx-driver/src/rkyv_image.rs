//! Rkyv-backed stdlib VM image (instructions stored as bincode blobs).
//! Faster / mmap-friendly alternative to fat `PreludeSnapshot` bincode for interpret path.

use crate::cache_fingerprint::cache_version_tag;
use crate::stdlib_fastpath::{decode_type_id, encode_type_id};
use crate::{export_symbols, is_stdlib_module_path, ModuleGraph, ModuleUnit};
use indexmap::IndexMap;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use smol_str::SmolStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use syx_codegen::{CompiledProgram, FunctionUnit};
use syx_diagnostics::SourceFile;
use syx_semantic::TypeId;

/// Embedded rkyv image (gen via `scripts/gen_prelude_snapshot.sh` / `.ps1`).
pub(crate) static EMBEDDED_RKYV_IMAGE: &[u8] =
    include_bytes!("../assets/prelude_vm_image.rkyv");

#[derive(Debug, Clone, Archive, RkyvSerialize, RkyvDeserialize)]
pub(crate) struct RkyvFn {
    pub name: String,
    pub param_count: u32,
    pub local_count: u32,
    /// `bincode` of `Vec<Instruction>` (avoids Archive on the huge Instruction enum).
    pub instructions: Vec<u8>,
    pub has_begin_try: bool,
    pub can_stack_bind: bool,
}

#[derive(Debug, Clone, Archive, RkyvSerialize, RkyvDeserialize)]
pub(crate) struct RkyvStdlibImage {
    pub cache_version_tag: String,
    pub functions: Vec<RkyvFn>,
    pub extension_methods: Vec<(String, String)>,
    pub module_exports: Vec<(String, Vec<(String, String)>)>,
}

/// Process-wide hydrated graph template (skip re-deserialize / re-rkyv within one process).
static HYDRATED: OnceLock<(String, Arc<ModuleGraph>)> = OnceLock::new();

pub(crate) fn clear_hydrated_for_tests() {
    // OnceLock cannot clear; tests use unique processes. Placeholder for API symmetry.
}

/// Return a cloned ModuleGraph from the process cache when the tag matches.
pub(crate) fn take_hydrated_graph() -> Option<ModuleGraph> {
    let tag = cache_version_tag();
    let (t, g) = HYDRATED.get()?;
    if t != &tag {
        return None;
    }
    Some((**g).clone())
}

pub(crate) fn store_hydrated_graph(graph: ModuleGraph) {
    let tag = cache_version_tag();
    let _ = HYDRATED.set((tag, Arc::new(graph)));
}

pub(crate) fn build_rkyv_image_from_graph(graph: &ModuleGraph) -> Option<RkyvStdlibImage> {
    let tag = cache_version_tag();
    let mut seen = std::collections::HashSet::<String>::new();
    let mut functions = Vec::new();
    for (path, unit) in &graph.modules {
        if !is_stdlib_module_path(path) {
            continue;
        }
        for f in &unit.program.functions {
            if !seen.insert(f.name.to_string()) {
                continue;
            }
            let Ok(instructions) = bincode::serialize(&f.instructions) else {
                continue;
            };
            functions.push(RkyvFn {
                name: f.name.to_string(),
                param_count: f.param_count,
                local_count: f.local_count,
                instructions,
                has_begin_try: f.has_begin_try,
                can_stack_bind: f.can_stack_bind,
            });
        }
    }
    if functions.is_empty() {
        return None;
    }
    let extension_methods: Vec<(String, String)> = graph
        .stdlib_extension_methods
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
    let mut module_exports = Vec::new();
    for (path, unit) in &graph.modules {
        if !is_stdlib_module_path(path) {
            continue;
        }
        let exports = graph
            .stdlib_exports
            .get(path)
            .cloned()
            .unwrap_or_else(|| export_symbols(&unit.source));
        module_exports.push((
            path.to_string_lossy().into_owned(),
            exports
                .into_iter()
                .map(|(n, ty)| (n.to_string(), encode_type_id(&ty)))
                .collect(),
        ));
    }
    Some(RkyvStdlibImage {
        cache_version_tag: tag,
        functions,
        extension_methods,
        module_exports,
    })
}

pub(crate) fn serialize_rkyv_image(image: &RkyvStdlibImage) -> Result<Vec<u8>, String> {
    rkyv::to_bytes::<rkyv::rancor::Error>(image)
        .map(|b| b.to_vec())
        .map_err(|e| e.to_string())
}

pub(crate) fn write_rkyv_image(path: &Path, image: &RkyvStdlibImage) -> Result<(), String> {
    let bytes = serialize_rkyv_image(image)?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

fn rkyv_fn_to_unit(f: &RkyvFn) -> Option<FunctionUnit> {
    let instructions: Vec<syx_codegen::Instruction> = bincode::deserialize(&f.instructions).ok()?;
    Some(FunctionUnit {
        name: SmolStr::new(&f.name),
        param_count: f.param_count,
        local_count: f.local_count,
        instructions,
        instruction_spans: Vec::new(),
        local_names: Vec::new(),
        local_types: Vec::new(),
        source_name: SmolStr::new(""),
        init_body_pc: 0,
        init_wrap_pc: 0,
        has_begin_try: f.has_begin_try,
        can_stack_bind: f.can_stack_bind,
    })
}

pub(crate) fn apply_rkyv_image(graph: &mut ModuleGraph, image: &RkyvStdlibImage) -> bool {
    if image.cache_version_tag != cache_version_tag() || image.functions.is_empty() {
        return false;
    }
    graph.stdlib_extension_methods = image
        .extension_methods
        .iter()
        .map(|(a, b)| (SmolStr::new(a), SmolStr::new(b)))
        .collect();
    graph.stdlib_exports = image
        .module_exports
        .iter()
        .map(|(path, exports)| {
            (
                PathBuf::from(path),
                exports
                    .iter()
                    .map(|(n, tw)| (SmolStr::new(n), decode_type_id(tw)))
                    .collect::<IndexMap<SmolStr, TypeId>>(),
            )
        })
        .collect();

    let mut program = CompiledProgram::default();
    for f in &image.functions {
        if let Some(unit) = rkyv_fn_to_unit(f) {
            program.functions.push(unit);
        }
    }
    // Export-only module stubs (empty programs) so import resolution finds paths.
    for (path, _) in &graph.stdlib_exports {
        if graph.modules.contains_key(path) {
            continue;
        }
        graph.order.push(path.clone());
        graph.modules.insert(
            path.clone(),
            ModuleUnit {
                path: path.clone(),
                source: SourceFile::new(path.to_string_lossy(), String::new()),
                imported_modules: Vec::new(),
                program: CompiledProgram::default(),
            },
        );
    }
    // Single image module holds all bytecode for `stdlib_fn_index`.
    let image_path = PathBuf::from("<stdlib>/__vm_image__");
    graph.order.push(image_path.clone());
    graph.modules.insert(
        image_path.clone(),
        ModuleUnit {
            path: image_path,
            source: SourceFile::new("<stdlib>/__vm_image__", String::new()),
            imported_modules: Vec::new(),
            program,
        },
    );
    true
}

pub(crate) fn load_rkyv_bytes(bytes: &[u8]) -> Option<RkyvStdlibImage> {
    rkyv::from_bytes::<RkyvStdlibImage, rkyv::rancor::Error>(bytes).ok()
}

/// Try embedded rkyv image, then optional mmap'd disk path.
/// Only loads into an empty graph — never replaces modules already present.
pub(crate) fn try_load_rkyv_into_graph(graph: &mut ModuleGraph, disk: Option<&Path>) -> bool {
    if !graph.modules.is_empty() {
        return false;
    }
    if let Some(existing) = take_hydrated_graph() {
        *graph = existing;
        return true;
    }
    if !EMBEDDED_RKYV_IMAGE.is_empty() {
        if let Some(image) = load_rkyv_bytes(EMBEDDED_RKYV_IMAGE) {
            if apply_rkyv_image(graph, &image) {
                store_hydrated_graph(graph.clone());
                return true;
            }
        }
    }
    if let Some(path) = disk {
        if let Ok(file) = std::fs::File::open(path) {
            if let Ok(map) = unsafe { memmap2::Mmap::map(&file) } {
                if let Some(image) = load_rkyv_bytes(&map) {
                    if apply_rkyv_image(graph, &image) {
                        store_hydrated_graph(graph.clone());
                        return true;
                    }
                }
            }
        }
    }
    false
}
