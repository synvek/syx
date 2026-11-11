//! SYX driver MIR merge/unify face.
//! Extracted in S2-I218 (zero semantics).

use crate::{
    contextual_numeric_kinds_from_semantic, core_prelude_generic_class_methods,
    core_prelude_import_symbols, export_symbols, inject_core_collection_builtins,
    inject_imported_dataclass_decls, inject_virtual_prelude_imports, resolve_module, DriverOptions,
    ModuleGraph, ModuleUnit,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::{Path, PathBuf};
use syx_ast::Decl;
use syx_diagnostics::SyxResult;
use syx_frontend::{expand_macros, FrontendOptions, FrontendPipeline, StopAfter};
use syx_mir::{lower_module, optimize, MirOp};
use syx_mono::monomorphize_module;
use syx_parser::Parser;
use syx_pkg::SyxManifest;
use syx_semantic::{SemanticAnalyzer, TypeId};

fn stdlib_mir_enabled() -> bool {
    std::env::var("SYX_STDLIB_MIR")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|disabled| !disabled)
        .unwrap_or(true)
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

fn merge_stdlib_modules_into_mir(
    graph: &ModuleGraph,
    mir: &mut syx_mir::MirModule,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
) -> SyxResult<()> {
    for (path, dep) in &graph.modules {
        if !path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        if let Ok(dep_module) = Parser::parse(dep.source.clone()) {
            let dep_module = expand_macros(dep_module)?;
            let dep_exports = export_symbols(&dep.source);
            let mut dep_mono = monomorphize_module(&dep_module, instantiations);
            inject_virtual_prelude_imports(&mut dep_mono, options);
            syx_mir::merge_mir_shell_functions(mir, syx_mir::lower_dataclass_shells(&dep_mono));
            syx_mir::merge_mir_shell_functions(mir, syx_mir::lower_enum_shells(&dep_mono));
            let dep_mir = lower_module(&dep_mono)?;
            let module_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            for mut func in dep_mir.functions {
                if mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports) {
                    func.name = cross_module_function_name(path, module_stem, func.name);
                    if !mir.functions.iter().any(|f| f.name == func.name) {
                        mir.functions.push(func);
                    }
                }
            }
        }
    }
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
    let (mut mir, instantiations) =
        build_mir_from_graph(graph, entry, project_root, options, manifest)?;
    if stdlib_mir_enabled() {
        // M1-A02: reuse instantiations from the single frontend pass (no second parse/expand).
        merge_stdlib_modules_into_mir(graph, &mut mir, &instantiations, options)?;
    }
    finalize_unified_mir(&mut mir);
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
    let mut import_symbols = core_prelude_import_symbols(options);
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
                import_symbols.push((symbol.clone(), export_symbols(&dep.source)));
            }
        }
    }

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
    let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
        .modules
        .iter()
        .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
        .collect();

    // Merge dependency public symbols referenced by the project graph (stdlib + user modules).
    for dep in graph.modules.values() {
        let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
        if dep_exports.is_empty() && !dep.path.to_string_lossy().contains("<stdlib>/") {
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
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if is_stdlib_module_path(&dep.path) {
                for mut func in dep_mir.functions {
                    let merge =
                        mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
                    if merge {
                        func.name = cross_module_function_name(&dep.path, module_stem, func.name);
                        if !mir.functions.iter().any(|f| f.name == func.name) {
                            mir.functions.push(func);
                        }
                    }
                }
            } else {
                let mut batch: Vec<_> = dep_mir
                    .functions
                    .into_iter()
                    .filter(|f| f.name.as_str() != "main")
                    .collect();
                if !batch.is_empty() {
                    syx_mir::prefix_user_dep_mir_batch(&mut batch, module_stem);
                    for func in batch {
                        if !mir.functions.iter().any(|f| f.name == func.name) {
                            mir.functions.push(func);
                        }
                    }
                }
            }
        }
    }

    let mut mono_module = monomorphize_module(&module, &semantic.instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    syx_mir::merge_mir_shell_functions(&mut mir, syx_mir::lower_dataclass_shells(&mono_module));
    syx_mir::merge_mir_shell_functions(&mut mir, syx_mir::lower_enum_shells(&mono_module));
    let callable_extensions: indexmap::IndexSet<SmolStr> =
        mir.functions.iter().map(|f| f.name.clone()).collect();
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
    for func in entry_mir.functions {
        if let Some(pos) = mir.functions.iter().position(|f| f.name == func.name) {
            // S1-I7: extension convenience `init() { self.init(x:0,y:0) }` embeds
            // ConstNumeric field literals — replace the memberwise shell so `Point()`
            // works. Do NOT replace with Struct's empty 0-arg synthetic init.
            if func.name.ends_with("_init") && func.param_count == 0 {
                let has_lit_fields = func.blocks.iter().any(|b| {
                    b.instructions.iter().any(|i| {
                        matches!(
                            i.op,
                            syx_mir::MirOp::ConstNumeric { .. } | syx_mir::MirOp::ConstI32(_)
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
