//! SYX driver MIR merge/unify face.
//! Extracted in S2-I218 (zero semantics).

use crate::cache_fingerprint::cache_version_tag;
use crate::{
    contextual_numeric_kinds_from_semantic, core_prelude_generic_class_methods,
    core_prelude_import_symbols, export_symbols, inject_core_collection_builtins,
    inject_imported_dataclass_decls, inject_imported_public_structs,
    inject_virtual_prelude_imports, resolve_module, DriverOptions, ModuleGraph, ModuleUnit,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use syx_ast::Decl;
use syx_diagnostics::SyxResult;
use syx_frontend::{expand_macros, FrontendOptions, FrontendPipeline, StopAfter};
use syx_mir::{lower_module, optimize, MirFunction, MirOp};
use syx_mono::monomorphize_module;
use syx_parser::Parser;
use syx_pkg::SyxManifest;
use syx_semantic::{SemanticAnalyzer, TypeId};

/// Env `SYX_STDLIB_MIR=0/false` forces off; `=1/true` forces on; else use [`DriverOptions::stdlib_mir`].
pub(crate) fn want_stdlib_mir(options: &DriverOptions) -> bool {
    match std::env::var("SYX_STDLIB_MIR") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        _ => options.stdlib_mir,
    }
}

/// Process-local cache of stdlib MIR fragments.
///
/// Keyed by embed tag + mono set + **stdlib module set** in the graph.
/// Nested `compile_entry_with_unified_mir` runs while imports are still loading;
/// a key that ignored the module set would cache a prelude-only fragment and
/// then miss later imports (Math / Crypto / …) on wasm hard-zero emit.
struct StdlibMirCacheEntry {
    tag: String,
    mono_key: u64,
    modules_key: u64,
    functions: Arc<Vec<MirFunction>>,
}

fn stdlib_mir_cache() -> &'static Mutex<Option<StdlibMirCacheEntry>> {
    static CACHE: OnceLock<Mutex<Option<StdlibMirCacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

pub(crate) static STDLIB_MIR_CACHE_HIT: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

fn instantiations_fingerprint(
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
) -> u64 {
    let mut items: Vec<(String, Vec<String>)> = instantiations
        .iter()
        .map(|(n, args)| {
            (
                n.to_string(),
                args.iter().map(|a| a.to_string()).collect::<Vec<_>>(),
            )
        })
        .collect();
    items.sort_unstable();
    let mut h = DefaultHasher::new();
    items.hash(&mut h);
    h.finish()
}

fn stdlib_modules_fingerprint(graph: &ModuleGraph) -> u64 {
    crate::stdlib_mir_image::stdlib_modules_key(graph)
}

fn finalize_unified_mir(mir: &mut syx_mir::MirModule) {
    syx_mir::optimize(mir);
    mir.functions
        .sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
    renumber_protocol_witnesses(mir);
}

/// After function sort, refresh `__witness_*` fn indices stored in class inits.
fn renumber_protocol_witnesses(mir: &mut syx_mir::MirModule) {
    let order: Vec<smol_str::SmolStr> = mir.functions.iter().map(|f| f.name.clone()).collect();
    for func in &mut mir.functions {
        let Some(type_name) = func.name.as_str().strip_suffix("_init") else {
            continue;
        };
        for block in &mut func.blocks {
            let ins = &mut block.instructions;
            for i in 0..ins.len().saturating_sub(2) {
                let MirOp::ConstStr(key) = ins[i + 2].op.clone() else {
                    continue;
                };
                let Some(rest) = key.strip_prefix("__witness_") else {
                    continue;
                };
                let Some(method) = rest.rsplit('_').next() else {
                    continue;
                };
                if !matches!(ins[i + 1].op, MirOp::ConstI32(_)) {
                    continue;
                }
                let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
                let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
                ins[i + 1].op = MirOp::ConstI32(idx);
            }
            for i in 0..ins.len().saturating_sub(1) {
                let MirOp::ConstStr(key) = ins[i + 1].op.clone() else {
                    continue;
                };
                let Some(rest) = key.strip_prefix("__witness_") else {
                    continue;
                };
                let Some(method) = rest.rsplit('_').next() else {
                    continue;
                };
                if !matches!(ins[i].op, MirOp::ConstI32(_)) {
                    continue;
                }
                let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
                let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
                ins[i].op = MirOp::ConstI32(idx);
            }
        }
        let ins = &mut func.instructions;
        for i in 0..ins.len().saturating_sub(2) {
            let MirOp::ConstStr(key) = ins[i + 2].op.clone() else {
                continue;
            };
            let Some(rest) = key.strip_prefix("__witness_") else {
                continue;
            };
            let Some(method) = rest.rsplit('_').next() else {
                continue;
            };
            if !matches!(ins[i + 1].op, MirOp::ConstI32(_)) {
                continue;
            }
            let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
            let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
            ins[i + 1].op = MirOp::ConstI32(idx);
        }
        for i in 0..ins.len().saturating_sub(1) {
            let MirOp::ConstStr(key) = ins[i + 1].op.clone() else {
                continue;
            };
            let Some(rest) = key.strip_prefix("__witness_") else {
                continue;
            };
            let Some(method) = rest.rsplit('_').next() else {
                continue;
            };
            if !matches!(ins[i].op, MirOp::ConstI32(_)) {
                continue;
            }
            let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
            let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
            ins[i].op = MirOp::ConstI32(idx);
        }
    }
}

/// True when `func_name` is a public export from a stdlib/core module (supports `Class_method` static lowering).
pub(crate) fn mir_merge_exported_function(
    module_stem: &str,
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    let name = SmolStr::new(func_name);
    if dep_exports.contains_key(&name) {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name.starts_with(&format!("{export}_")))
    {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name == format!("{export}_init"))
    {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name == format!("{module_stem}_{export}"))
    {
        return true;
    }
    if let Some(rest) = func_name.strip_prefix(&format!("{module_stem}_")) {
        return dep_exports.keys().any(|export| {
            rest == export.as_str() || rest.starts_with(&format!("{}_", export.as_str()))
        });
    }
    if let Some((_, method)) = func_name.split_once('_') {
        return dep_exports.contains_key(&SmolStr::new(method));
    }
    false
}

pub(crate) fn is_stdlib_module_path(path: &Path) -> bool {
    path.to_string_lossy().contains("<stdlib>/")
}

pub(crate) fn should_prefix_cross_module_export(
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    match dep_exports.get(&SmolStr::new(func_name)) {
        Some(TypeId::Named(name)) if name.as_str() == func_name => false,
        Some(_) => true,
        None => false,
    }
}

pub(crate) fn cross_module_function_name(path: &Path, module_stem: &str, name: SmolStr) -> SmolStr {
    if is_stdlib_module_path(path) || name.as_str() == "main" {
        return name;
    }
    let prefix = format!("{module_stem}_");
    if name.starts_with(prefix.as_str()) {
        return name;
    }
    SmolStr::new(format!("{prefix}{name}"))
}

/// Merge user-dependency bytecode with prefix + relink internal `Call` indices (SYX 40.0).
pub(crate) fn merge_user_dep_program_batch(
    program: &mut syx_codegen::CompiledProgram,
    dep: &ModuleUnit,
    module_stem: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) {
    use syx_codegen::Instruction;
    if is_stdlib_module_path(&dep.path) {
        return;
    }
    let dep_names: Vec<SmolStr> = dep
        .program
        .functions
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let mut batch: Vec<syx_codegen::FunctionUnit> = dep
        .program
        .functions
        .iter()
        .filter(|f| {
            f.name.as_str() != "main"
                && (mir_merge_stdlib_function(module_stem, f.name.as_str(), dep_exports)
                    || dep_exports.contains_key(&f.name))
        })
        .cloned()
        .collect();
    if batch.is_empty() {
        return;
    }
    let mut rename = IndexMap::new();
    for f in &mut batch {
        let prefixed = cross_module_function_name(&dep.path, module_stem, f.name.clone());
        if prefixed != f.name {
            rename.insert(f.name.clone(), prefixed.clone());
            f.name = prefixed;
        }
    }
    let start = program.functions.len();
    for f in batch {
        if !program
            .functions
            .iter()
            .any(|existing| existing.name == f.name)
        {
            program.functions.push(f);
        }
    }
    let name_to_idx: std::collections::HashMap<SmolStr, u32> = program
        .functions
        .iter()
        .enumerate()
        .map(|(i, f)| (f.name.clone(), i as u32))
        .collect();
    fn resolve_dep_call(
        old_idx: u32,
        dep_names: &[SmolStr],
        rename: &IndexMap<SmolStr, SmolStr>,
        name_to_idx: &std::collections::HashMap<SmolStr, u32>,
    ) -> u32 {
        let Some(old_name) = dep_names.get(old_idx as usize) else {
            return old_idx;
        };
        let target = rename.get(old_name).unwrap_or(old_name);
        name_to_idx.get(target).copied().unwrap_or(old_idx)
    }
    for func in &mut program.functions[start..] {
        for ins in &mut func.instructions {
            match ins {
                Instruction::Call(idx)
                | Instruction::ActorCall(idx)
                | Instruction::TaskSpawn(idx) => {
                    *idx = resolve_dep_call(*idx, &dep_names, &rename, &name_to_idx);
                }
                Instruction::MakeClosure(fn_index, _) => {
                    *fn_index = resolve_dep_call(*fn_index, &dep_names, &rename, &name_to_idx);
                }
                _ => {}
            }
        }
    }
}

fn imported_class_type_names(
    import_symbols: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> indexmap::IndexSet<SmolStr> {
    let mut out = indexmap::IndexSet::new();
    for (mod_sym, exports) in import_symbols {
        for (name, ty) in exports {
            if matches!(ty, TypeId::Named(_)) {
                out.insert(name.clone());
                out.insert(SmolStr::new(format!("{mod_sym}_{name}")));
            }
        }
    }
    out
}

/// Phase 45/46: merge Core extension witnesses (`Int8_hash`, `Array_map`, …) from prelude modules.
pub(crate) fn mir_merge_stdlib_function(
    module_stem: &str,
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    if mir_merge_exported_function(module_stem, func_name, dep_exports) {
        return true;
    }
    if func_name.starts_with(&format!("{module_stem}_")) {
        return true;
    }
    // SA-4-04: always merge internal host surface methods.
    if func_name.starts_with("SyxHost_") {
        return true;
    }
    if let Some((ty, _method)) = func_name.split_once('_') {
        if dep_exports.contains_key(&SmolStr::new(ty)) {
            return true;
        }
    }
    false
}

pub(crate) fn collect_stdlib_extension_methods(
    graph: &ModuleGraph,
) -> indexmap::IndexSet<(SmolStr, SmolStr)> {
    if !graph.stdlib_extension_methods.is_empty() {
        return graph.stdlib_extension_methods.clone();
    }
    let mut set = indexmap::IndexSet::new();
    for (path, dep) in &graph.modules {
        if !path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        if let Ok(m) = Parser::parse(dep.source.clone()) {
            if let Ok(expanded) = expand_macros(m) {
                set.extend(syx_mir::extension_methods_for_module(&expanded));
            }
        }
    }
    set
}

fn lower_stdlib_modules_to_functions(
    graph: &ModuleGraph,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
    only_paths: Option<&std::collections::HashSet<PathBuf>>,
) -> SyxResult<Vec<MirFunction>> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::<SmolStr>::new();
    // C-3a: pre-advertise arity-qualified overload MIR names so cross-module Calls
    // (e.g. Concurrency → SyxHost.task_detach) resolve before bodies are merged.
    let mut arity_callables: indexmap::IndexSet<SmolStr> = indexmap::IndexSet::new();
    let mut prepared: Vec<(PathBuf, syx_ast::Module, IndexMap<SmolStr, TypeId>)> = Vec::new();
    for (path, dep) in &graph.modules {
        if !is_stdlib_module_path(path) {
            continue;
        }
        if let Some(filter) = only_paths {
            if !filter.contains(path) {
                continue;
            }
        }
        let Ok(dep_module) = Parser::parse(dep.source.clone()) else {
            continue;
        };
        let dep_module = expand_macros(dep_module)?;
        let dep_exports = crate::stdlib_fastpath::cached_exports_for(graph, path)
            .unwrap_or_else(|| export_symbols(&dep.source));
        let mut dep_mono = monomorphize_module(&dep_module, instantiations);
        inject_virtual_prelude_imports(&mut dep_mono, options);
        arity_callables.extend(syx_mir::collect_arity_overload_mir_names(&dep_mono));
        prepared.push((path.clone(), dep_mono, dep_exports));
    }
    for (path, dep_mono, dep_exports) in prepared {
        // Shells are merged unfiltered (same as the historical unify path).
        let mut shell = syx_mir::MirModule::default();
        syx_mir::merge_mir_shell_functions(&mut shell, syx_mir::lower_dataclass_shells(&dep_mono));
        syx_mir::merge_mir_shell_functions(&mut shell, syx_mir::lower_enum_shells(&dep_mono));
        for func in shell.functions {
            if seen.insert(func.name.clone()) {
                out.push(func);
            }
        }
        let dep_mir = syx_mir::lower_module_with_extra_extensions(
            &dep_mono,
            &indexmap::IndexSet::new(),
            &arity_callables,
            &indexmap::IndexSet::new(),
        )?;
        let module_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        for mut func in dep_mir.functions {
            if !mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports) {
                continue;
            }
            func.name = cross_module_function_name(&path, module_stem, func.name);
            if seen.insert(func.name.clone()) {
                out.push(func);
            }
        }
    }
    Ok(out)
}

fn append_mir_functions(mir: &mut syx_mir::MirModule, functions: &[MirFunction]) {
    for func in functions {
        if !mir.functions.iter().any(|f| f.name == func.name) {
            mir.functions.push(func.clone());
        }
    }
    // C-3l: function-only fragments (stdlib cache) may omit MirModule.global_count —
    // raise bound from max Load/StoreGlobal slot so legitimate stdlib does not MIR012.
    syx_mir::absorb_mir_global_count(mir, syx_mir::mir_global_slot_bound(functions.iter()));
}

fn absorb_fragment_global_count(mir: &mut syx_mir::MirModule, fragment: &syx_mir::MirModule) {
    syx_mir::absorb_mir_global_count(mir, fragment.global_count);
    syx_mir::absorb_mir_global_count(
        mir,
        syx_mir::mir_global_slot_bound(fragment.functions.iter()),
    );
}

/// C-3j: resolve path stem → ModuleId index into `mir.module_names` (None if missing).
fn owner_module_for_path(mir: &syx_mir::MirModule, path: &Path) -> Option<u32> {
    let stem = path.file_stem().and_then(|s| s.to_str())?;
    mir.module_names
        .iter()
        .position(|n| n.as_str() == stem)
        .map(|i| i as u32)
}

fn set_owner_module(func: &mut MirFunction, owner: Option<u32>) {
    func.owner_module = owner;
}

fn store_stdlib_mir_process(
    tag: String,
    mono_key: u64,
    modules_key: u64,
    functions: Arc<Vec<MirFunction>>,
) {
    if let Ok(mut guard) = stdlib_mir_cache().lock() {
        *guard = Some(StdlibMirCacheEntry {
            tag,
            mono_key,
            modules_key,
            functions,
        });
    }
}

/// Warm process cache from embed (daemon). Returns whether embed loaded.
pub(crate) fn warm_stdlib_mir_from_embed() -> bool {
    let Some(image) = crate::stdlib_mir_image::try_load_embed() else {
        return false;
    };
    store_stdlib_mir_process(
        image.cache_version_tag.clone(),
        image.mono_key,
        image.modules_key,
        Arc::new(image.functions),
    );
    true
}

/// Append stdlib MIR into `mir`: process → disk → embed(+extras) → cold lower.
fn merge_stdlib_modules_into_mir(
    graph: &ModuleGraph,
    mir: &mut syx_mir::MirModule,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<()> {
    let tag = cache_version_tag();
    let mono_key = instantiations_fingerprint(instantiations);
    let modules_key = stdlib_modules_fingerprint(graph);
    let trace = std::env::var_os("SYX_STDLIB_MIR_TRACE").is_some();
    if let Ok(guard) = stdlib_mir_cache().lock() {
        if let Some(entry) = guard.as_ref() {
            if entry.tag == tag && entry.mono_key == mono_key && entry.modules_key == modules_key {
                STDLIB_MIR_CACHE_HIT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if trace {
                    eprintln!("stdlib_mir_trace=process fns={}", entry.functions.len());
                }
                append_mir_functions(mir, &entry.functions);
                return Ok(());
            }
        }
    }

    // Exact disk hit (same tag digest path; modules_key + mono_key must match).
    if let Some(image) = crate::stdlib_mir_image::try_load_disk(project_root, modules_key, mono_key)
    {
        if trace {
            eprintln!(
                "stdlib_mir_trace=disk fns={} modules_key={modules_key} mono={mono_key}",
                image.functions.len()
            );
        }
        append_mir_functions(mir, &image.functions);
        store_stdlib_mir_process(tag, mono_key, modules_key, Arc::new(image.functions));
        return Ok(());
    }

    // Embed as full hit (modules+mono) or as prelude base when graph has companion imports.
    let mut functions = Vec::new();
    let mut used_embed = false;
    if let Some(image) = crate::stdlib_mir_image::try_load_embed() {
        if image.modules_key == modules_key && image.mono_key == mono_key {
            functions = image.functions;
            crate::stdlib_mir_image::note_embed_hit();
            used_embed = true;
            if trace {
                eprintln!(
                    "stdlib_mir_trace=embed_exact fns={} modules_key={modules_key} mono={mono_key}",
                    functions.len()
                );
            }
        } else if crate::stdlib_mir_image::image_paths_subset_of_graph(&image, graph)
            && (image.mono_key == mono_key || image.mono_key == 0 || mono_key == 0)
        {
            let covered: std::collections::HashSet<PathBuf> =
                image.module_paths.iter().map(PathBuf::from).collect();
            let extras: std::collections::HashSet<PathBuf> = graph
                .modules
                .keys()
                .filter(|p| is_stdlib_module_path(p) && !covered.contains(*p))
                .cloned()
                .collect();
            functions = image.functions;
            crate::stdlib_mir_image::note_embed_hit();
            if !extras.is_empty() {
                let more = lower_stdlib_modules_to_functions(
                    graph,
                    instantiations,
                    options,
                    Some(&extras),
                )?;
                let mut seen: std::collections::HashSet<SmolStr> =
                    functions.iter().map(|f| f.name.clone()).collect();
                for f in more {
                    if seen.insert(f.name.clone()) {
                        functions.push(f);
                    }
                }
            }
            used_embed = true;
            if trace {
                eprintln!(
                    "stdlib_mir_trace=embed_subset fns={} extras={} image_key={} graph_key={modules_key}",
                    functions.len(),
                    extras.len(),
                    image.modules_key
                );
            }
        } else if trace {
            eprintln!(
                "stdlib_mir_trace=embed_mismatch image_key={} graph_key={modules_key} image_mono={} mono={mono_key}",
                image.modules_key, image.mono_key
            );
        }
    } else if trace {
        eprintln!("stdlib_mir_trace=embed_unloadable mono={mono_key}");
    }

    if !used_embed {
        if trace {
            eprintln!("stdlib_mir_trace=cold_lower graph_key={modules_key} mono={mono_key}");
        }
        functions = lower_stdlib_modules_to_functions(graph, instantiations, options, None)?;
    }

    append_mir_functions(mir, &functions);
    if let Some(image) = crate::stdlib_mir_image::build_image(graph, functions.clone(), mono_key) {
        crate::stdlib_mir_image::store_disk(project_root, &image);
    }
    store_stdlib_mir_process(tag, mono_key, modules_key, Arc::new(functions));
    Ok(())
}

/// Build a unified MIR module for native/WASM backends from a compiled project graph.
pub fn build_project_mir(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<syx_mir::MirModule> {
    Ok(build_mir_from_graph(graph, entry, project_root, options, manifest)?.0)
}

/// Unified MIR builder for VM codegen, Cranelift, and WASM emit (single canonical path).
pub fn build_unified_mir(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<syx_mir::MirModule> {
    // Stdlib MIR merge (when `stdlib_mir`) happens once inside `build_mir_from_graph`
    // via the process-cached `merge_stdlib_modules_into_mir` — do not re-parse here.
    let (mut mir, _instantiations) =
        build_mir_from_graph(graph, entry, project_root, options, manifest)?;
    finalize_unified_mir(&mut mir);
    // C-3b: thin CFG / terminator floor before any backend consumes MIR.
    syx_mir::verify_mir_cfg_floor(&mir)?;
    // C-3c: name → FuncId table after finalize + CFG floor (order == functions).
    // Table is reused by C-3p named-callee floor; build here fail-closes duplicate names.
    let fn_index = syx_mir::MirFnIndex::build(&mir)?;
    // C-3d: pilot value-stack height floor (MIR004) after CFG + FnIndex.
    syx_mir::verify_mir_stack_floor(&mir)?;
    // C-3e: pilot coarse type-tag floor (MIR005) after stack floor.
    syx_mir::verify_mir_type_floor(&mir)?;
    // C-3f: JumpIfFalse condition tag floor (MIR006) after type floor.
    syx_mir::verify_mir_branch_floor(&mir)?;
    // C-3g: JumpIfNil condition tag floor (MIR007) after branch floor.
    syx_mir::verify_mir_nil_branch_floor(&mir)?;
    // C-3h: module name → ModuleId table after nil-branch (empty names OK).
    let _mod_index = syx_mir::MirModIndex::build(&mir)?;
    // C-3i: try/catch pairing floor (MIR009) after ModuleId.
    syx_mir::verify_mir_try_floor(&mir)?;
    // C-3j: optional owner_module Def floor (MIR010) after try.
    syx_mir::verify_mir_def_floor(&mir)?;
    // C-3k: LoadLocal/StoreLocal slot OOB floor (MIR011) after Def.
    syx_mir::verify_mir_local_floor(&mir)?;
    // C-3l: LoadGlobal/StoreGlobal slot OOB floor (MIR012) after local.
    syx_mir::verify_mir_global_floor(&mir)?;
    // C-3m: absolute CallHost index floor (MIR013) after global.
    syx_mir::verify_mir_call_host_floor(&mir)?;
    // C-3n: param_count ≤ local_count metadata floor (MIR014) after CallHost.
    syx_mir::verify_mir_param_local_floor(&mir)?;
    // C-3o: CallWitness / AwaitSuspend.resume_fn fn-lit OOB floor (MIR015) after param-local.
    syx_mir::verify_mir_fn_lit_floor(&mir)?;
    // C-3p: named callee resolve floor (MIR016) after fn-lit; ConstFnIndex not scanned.
    syx_mir::verify_mir_named_callee_floor_functions(&fn_index, mir.functions.iter())?;
    // C-3q: Array/Dict/SetBuild stack-height floor (MIR017) after named-callee.
    syx_mir::verify_mir_build_stack_floor(&mir)?;
    // C-3r: non-empty local_names/local_types length vs local_count (MIR018) after build-stack.
    syx_mir::verify_mir_local_debug_floor(&mir)?;
    // C-3s: aot_export_roots names must resolve to functions[].name (MIR019) after local-debug.
    syx_mir::verify_mir_aot_export_floor(&mir)?;
    // C-3t: init_body_pc/init_wrap_pc bounds (MIR020) after aot-export.
    syx_mir::verify_mir_init_pc_floor(&mir)?;
    // C-3u: MirBlock.id uniqueness (MIR021) after init-pc.
    syx_mir::verify_mir_block_id_floor(&mir)?;
    // C-3v: flat↔blocks instruction-count (MIR022) after block-id.
    syx_mir::verify_mir_flat_blocks_floor(&mir)?;
    // C-3w: terminator Jump* target ∈ successors (MIR023) after flat-blocks.
    syx_mir::verify_mir_succ_term_floor(&mir)?;
    // C-3x: Pop stack-height floor (MIR024) after succ-term.
    syx_mir::verify_mir_pop_stack_floor(&mir)?;
    // C-3y: non-empty MirFunction.name (MIR025) after pop-stack.
    syx_mir::verify_mir_fn_name_floor(&mir)?;
    // C-3z: non-empty block instructions (MIR026) after fn-name.
    syx_mir::verify_mir_block_instr_floor(&mir)?;
    // C-3aa: non-empty module_names entries (MIR027) after block-instr.
    syx_mir::verify_mir_module_names_floor(&mir)?;
    // C-3ab: unique MirBlock.successors (MIR028) after module-names.
    syx_mir::verify_mir_succ_unique_floor(&mir)?;
    // C-3ac: unique aot_export_roots (MIR029) after succ-unique.
    syx_mir::verify_mir_aot_export_unique_floor(&mir)?;
    // C-3ad: non-empty local_types ban empty slots (MIR030) after aot-export-unique.
    syx_mir::verify_mir_local_types_empty_floor(&mir)?;
    // C-3ae: non-empty local_names ban empty slots (MIR031) after local-types-empty.
    syx_mir::verify_mir_local_names_empty_floor(&mir)?;
    // C-3af: Dup stack-height floor (MIR032) after local-names-empty.
    syx_mir::verify_mir_dup_stack_floor(&mir)?;
    // C-3ag: ToStr stack-height floor (MIR033) after dup-stack.
    syx_mir::verify_mir_tostr_stack_floor(&mir)?;
    // C-3ah: Not stack-height floor (MIR034) after tostr-stack.
    syx_mir::verify_mir_not_stack_floor(&mir)?;
    // C-3ai: Neg stack-height floor (MIR035) after not-stack.
    syx_mir::verify_mir_neg_stack_floor(&mir)?;
    // C-3aj: BitNot stack-height floor (MIR036) after neg-stack.
    syx_mir::verify_mir_bitnot_stack_floor(&mir)?;
    // C-3ak: NumericCast stack-height floor (MIR037) after bitnot-stack.
    syx_mir::verify_mir_numeric_cast_stack_floor(&mir)?;
    // C-3al: MakeClosure stack-height floor (MIR038) after numeric-cast-stack.
    syx_mir::verify_mir_make_closure_stack_floor(&mir)?;
    // C-3am: BitAnd stack-height floor (MIR039) after make-closure-stack.
    syx_mir::verify_mir_bitand_stack_floor(&mir)?;
    // C-3an: BitOr stack-height floor (MIR040) after bitand-stack.
    syx_mir::verify_mir_bitor_stack_floor(&mir)?;
    // C-3ao: BitXor stack-height floor (MIR041) after bitor-stack.
    syx_mir::verify_mir_bitxor_stack_floor(&mir)?;
    // C-3ap: Shl stack-height floor (MIR042) after bitxor-stack.
    syx_mir::verify_mir_shl_stack_floor(&mir)?;
    // C-3aq: Shr stack-height floor (MIR043) after shl-stack.
    syx_mir::verify_mir_shr_stack_floor(&mir)?;
    syx_mir::verify_mir_mod_stack_floor(&mir)?;
    syx_mir::verify_mir_eq_stack_floor(&mir)?;
    syx_mir::verify_mir_ne_stack_floor(&mir)?;
    syx_mir::verify_mir_lt_stack_floor(&mir)?;
    Ok(mir)
}

pub(crate) fn build_mir_from_graph(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<(
    syx_mir::MirModule,
    std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
)> {
    let prep_opts = FrontendOptions {
        stop_after: StopAfter::Prepared,
        desugar_top_level: true,
        source: Some(entry.source.clone()),
        ..FrontendOptions::default()
    };
    let art = FrontendPipeline::run(entry.source.clone(), &prep_opts)?;
    let mut module = art.module;
    let importer = entry.path.parent().unwrap_or(project_root);
    let importer_dir = importer;
    // Object/Array/… extensions must be visible to semantic (e.g. `Object.put` → Object)
    // before analyze; MIR still merges witnesses via the stdlib graph.
    inject_core_collection_builtins(&mut module, options);
    // SA-4-04: `SyxHost` must exist before semantic/MIR — injected façades call it.
    crate::compile_to_program::inject_syx_host_enum(&mut module);
    let mut import_symbols = core_prelude_import_symbols(options, Some(graph));
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Some(symbol) = decl.import_symbol() else {
                continue;
            };
            let import_ref = resolve_module(
                &path_str,
                &entry.path,
                importer_dir,
                manifest,
                options,
                project_root,
            )?;
            let import_key = import_ref.key();
            if let Some(dep) = graph.modules.get(&import_key) {
                let exports = crate::stdlib_fastpath::cached_exports_for(graph, &import_key)
                    .unwrap_or_else(|| export_symbols(&dep.source));
                import_symbols.push((symbol.clone(), exports));
            }
        }
    }

    // U-0: public Structs only before semantic (FilePath.joining); full inject
    // after analyze for MIR (Class/DataClass must not tighten SIMD labels).
    inject_imported_public_structs(
        &mut module,
        &entry.path,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    let semantic = SemanticAnalyzer::analyze_with_imports_and_generics(
        &module,
        &import_symbols,
        &core_prelude_generic_class_methods(options),
    )?;
    inject_imported_dataclass_decls(
        &mut module,
        &entry.path,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    let prelude_extensions = collect_stdlib_extension_methods(graph);
    let mut mir = syx_mir::MirModule::default();
    // C-3h: populate ordered module identity stems (stem collision → MIR008).
    mir.module_names = syx_mir::collect_module_names_from_paths(graph.order.iter().map(|p| p.as_path()))?;
    let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
        .modules
        .iter()
        .map(|(p, u)| {
            let exports = crate::stdlib_fastpath::cached_exports_for(graph, p)
                .unwrap_or_else(|| export_symbols(&u.source));
            (p.clone(), exports)
        })
        .collect();

    // Merge dependency MIR (stdlib + user). Interpret fastpath skips stdlib re-lower —
    // bytecode comes from the embed via on-demand link.
    // Skip the fat prelude merge while compiling a stdlib module itself — nested
    // `compile_entry_with_unified_mir` during import loading would otherwise
    // re-lower the whole prelude for Crypto/Math/… before the user entry runs.
    let stdlib_mir = want_stdlib_mir(options);
    if stdlib_mir && !is_stdlib_module_path(&entry.path) {
        merge_stdlib_modules_into_mir(
            graph,
            &mut mir,
            &semantic.instantiations,
            options,
            project_root,
        )?;
    }
    // User (non-stdlib) deps always need a lower when present in the graph.
    for dep in graph.modules.values() {
        if is_stdlib_module_path(&dep.path) {
            continue;
        }
        let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
        if dep_exports.is_empty() {
            continue;
        }
        if let Ok(dep_module) = Parser::parse(dep.source.clone()) {
            let dep_module = expand_macros(dep_module)?;
            let mut dep_mono = monomorphize_module(&dep_module, &semantic.instantiations);
            inject_virtual_prelude_imports(&mut dep_mono, options);
            syx_mir::merge_mir_shell_functions(
                &mut mir,
                syx_mir::lower_dataclass_shells(&dep_mono),
            );
            syx_mir::merge_mir_shell_functions(&mut mir, syx_mir::lower_enum_shells(&dep_mono));
            let dep_mir = lower_module(&dep_mono)?;
            absorb_fragment_global_count(&mut mir, &dep_mir);
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let owner = owner_module_for_path(&mir, &dep.path);
            let mut batch: Vec<_> = dep_mir
                .functions
                .into_iter()
                .filter(|f| f.name.as_str() != "main")
                .collect();
            if !batch.is_empty() {
                syx_mir::prefix_user_dep_mir_batch(&mut batch, module_stem);
                for mut func in batch {
                    set_owner_module(&mut func, owner);
                    if !mir.functions.iter().any(|f| f.name == func.name) {
                        mir.functions.push(func);
                    }
                }
            }
        }
    }

    let mut mono_module = monomorphize_module(&module, &semantic.instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    syx_mir::merge_mir_shell_functions(&mut mir, syx_mir::lower_dataclass_shells(&mono_module));
    syx_mir::merge_mir_shell_functions(&mut mir, syx_mir::lower_enum_shells(&mono_module));
    // Interpret fastpath: advertise embed function names so `Double.pi` etc. lower to Call
    // without re-parsing stdlib sources into MIR.
    let callable_extensions: indexmap::IndexSet<SmolStr> = if stdlib_mir {
        mir.functions.iter().map(|f| f.name.clone()).collect()
    } else {
        crate::stdlib_fastpath::stdlib_fn_index(graph)
            .keys()
            .cloned()
            .collect()
    };
    let imported_classes = imported_class_type_names(&import_symbols);
    let int_kinds = contextual_numeric_kinds_from_semantic(&semantic);
    let entry_mir = syx_mir::lower_module_with_extra_extensions_kinds_and_source(
        &mono_module,
        &prelude_extensions,
        &callable_extensions,
        &imported_classes,
        &int_kinds,
        Some(&entry.source),
    )?;
    absorb_fragment_global_count(&mut mir, &entry_mir);
    let entry_owner = owner_module_for_path(&mir, &entry.path);
    for mut func in entry_mir.functions {
        set_owner_module(&mut func, entry_owner);
        if let Some(pos) = mir.functions.iter().position(|f| f.name == func.name) {
            // S1-I7: extension convenience `init() { self.init(x:0,y:0) }` embeds
            // ConstNumeric field literals — replace the memberwise shell so `Point()`
            // works. Do NOT replace with Struct's empty 0-arg synthetic init.
            if func.name.ends_with("_init") && func.param_count == 0 {
                // S1-I7 / U-40: Numeric + Bool/Str convenience shells replace memberwise.
                let has_lit_fields = func.blocks.iter().any(|b| {
                    b.instructions.iter().any(|i| {
                        matches!(
                            i.op,
                            syx_mir::MirOp::ConstNumeric { .. }
                                | syx_mir::MirOp::ConstI32(_)
                                | syx_mir::MirOp::ConstBool(_)
                                | syx_mir::MirOp::ConstStr(_)
                        )
                    })
                });
                if has_lit_fields {
                    mir.functions[pos] = func;
                }
            }
        } else {
            mir.functions.push(func);
        }
    }
    optimize(&mut mir);
    Ok((mir, semantic.instantiations.clone()))
}
