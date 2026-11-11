//! SYX driver compile_file_with_context face.
//! Extracted in S2-I221 (zero semantics).

use crate::{
    collect_main_actor_await_errors, collect_stdlib_extension_methods,
    core_prelude_generic_class_methods, core_prelude_import_symbols, cross_module_function_name,
    export_symbols, inject_core_collection_builtins, inject_imported_dataclass_decls,
    is_stdlib_module_path, merge_imported_dataclass_functions, merge_stdlib_mono_instances,
    merge_user_dep_program_batch, mir_merge_stdlib_function,
    register_imported_data_classes_from_parsed, register_imported_macros_from_parsed,
    resolve_module, should_desugar_top_level_stmts, DriverOptions, ModuleGraph, ModuleUnit,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use syx_ast::{AccessLevel, Decl};
use syx_codegen::CompiledProgram;
use syx_diagnostics::{SourceFile, SyxResult};
use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};
use syx_parser::Parser;
use syx_pkg::{load_manifest, SyxManifest};
use syx_semantic::{SemanticAnalyzer, TypeId};
use crate::unified_compile::compile_entry_with_unified_mir;

pub fn compile_file_with_context(
    module_key: &Path,
    source: &SourceFile,
    importer: &Path,
    graph: &ModuleGraph,
    manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<ModuleUnit> {
    let importer_dir = importer.parent().unwrap_or(importer);
    let parsed = Parser::parse(source.clone())?;
    register_imported_macros_from_parsed(
        &parsed,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    register_imported_data_classes_from_parsed(
        &parsed,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    let stem = module_key
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from);
    syx_meta::service_di_set_scan_stem(stem);
    let prep_opts = FrontendOptions {
        stop_after: StopAfter::Prepared,
        desugar_top_level: should_desugar_top_level_stmts(module_key, manifest),
        source: Some(source.clone()),
        ..FrontendOptions::default()
    };
    let art = FrontendPipeline::run_parsed(parsed, &prep_opts)?;
    syx_meta::service_di_set_scan_stem(None);
    let mut module = art.module;
    // Object extensions (`valueForKey`/`put`) need to be in the AST so
    // `type_extension_methods` types their returns (≠ Unknown → S004).
    inject_core_collection_builtins(&mut module, options);
    let main_actor_await_errors = collect_main_actor_await_errors(&module, &[], manifest);

    let mut import_symbols = core_prelude_import_symbols(options);
    let mut import_isolation = Vec::new();
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
                importer,
                importer_dir,
                manifest,
                options,
                project_root,
            )?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            import_symbols.push((symbol.clone(), export_symbols(&dep_source)));
            let dep_parent = import_key.parent().unwrap_or(project_root);
            let dep_manifest = load_manifest(dep_parent).unwrap_or_default();
            let fn_actors = collect_dep_function_actors(&dep_source, &dep_manifest);
            if !fn_actors.is_empty() {
                import_isolation.push((symbol.clone(), fn_actors));
            }
        }
    }

    // SA-4-04: inject `SyxHost` before semantic (import façades call it).
    crate::compile_to_program::inject_syx_host_enum(&mut module);
    let semantic = SemanticAnalyzer::analyze_with_imports_strict_skip_main_actor(
        &module,
        &import_symbols,
        manifest.concurrency.strict,
        manifest
            .concurrency
            .default_isolation
            .as_ref()
            .map(|s| SmolStr::new(s.as_str())),
        &import_isolation,
        Some(source),
        &core_prelude_generic_class_methods(options),
    )?;
    if let Some(err) = main_actor_await_errors.first() {
        return Err(err.clone());
    }
    let mut program = CompiledProgram::default();
    let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
        .modules
        .iter()
        .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
        .collect();
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Ok(import_ref) = resolve_module(
                &path_str,
                importer,
                importer_dir,
                manifest,
                options,
                project_root,
            ) else {
                continue;
            };
            let import_key = import_ref.key();
            let Some(dep) = graph.modules.get(&import_key) else {
                continue;
            };
            let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if is_stdlib_module_path(&dep.path) {
                for func in &dep.program.functions {
                    let merge =
                        mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports)
                            || dep_exports.contains_key(&func.name);
                    if merge {
                        let mut merged = func.clone();
                        merged.name =
                            cross_module_function_name(&dep.path, module_stem, merged.name);
                        if !program.functions.iter().any(|f| f.name == merged.name) {
                            program.functions.push(merged);
                        }
                    }
                }
            } else {
                merge_user_dep_program_batch(&mut program, dep, module_stem, &dep_exports);
            }
            for (export, ty) in &dep_exports {
                merge_imported_dataclass_functions(&mut program, export, dep);
                let merge_ctor = matches!(ty, TypeId::Named(n) if n.as_str() == export.as_str());
                if !merge_ctor && syx_meta::reflect_type(export.as_str()).is_none() {
                    continue;
                }
                for suffix in ["_init", "_copy", "_describe", "_hash_into", "_==", "_!="] {
                    let fname = SmolStr::new(format!("{export}{suffix}"));
                    if let Some(func) = dep.program.functions.iter().find(|f| f.name == fname) {
                        let mut merged = func.clone();
                        merged.name =
                            cross_module_function_name(&dep.path, module_stem, fname.clone());
                        if !program.functions.iter().any(|f| f.name == merged.name) {
                            program.functions.push(merged);
                        }
                    }
                }
            }
        }
    }
    for dep in graph.modules.values() {
        let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
        let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        for func in &dep.program.functions {
            let merge = mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
            if merge {
                let mut merged = func.clone();
                merged.name = cross_module_function_name(&dep.path, module_stem, merged.name);
                if !program.functions.iter().any(|f| f.name == merged.name) {
                    program.functions.push(merged);
                }
            }
        }
    }
    merge_stdlib_mono_instances(graph, &mut program, &semantic.instantiations, options)?;
    let prelude_extensions = collect_stdlib_extension_methods(graph);
    let mut compile_module = module.clone();
    inject_imported_dataclass_decls(
        &mut compile_module,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    compile_entry_with_unified_mir(
        graph,
        module_key,
        source.name.as_str(),
        &compile_module,
        &mut program,
        &semantic,
        options,
        &prelude_extensions,
        manifest,
        project_root,
        source,
    )?;

    Ok(ModuleUnit {
        path: module_key.to_path_buf(),
        source: source.clone(),
        imported_modules: semantic.imported_modules.clone(),
        program,
    })
}

fn dep_decl_has_attr(attrs: &[syx_ast::Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.name.as_str() == name)
}

/// Phase 22 CY2: collect public function isolation from a dependency module (attrs + default isolation).
fn collect_dep_function_actors(
    dep_source: &SourceFile,
    dep_manifest: &SyxManifest,
) -> IndexMap<SmolStr, SmolStr> {
    let mut actors = IndexMap::new();
    let Ok(parsed) = Parser::parse(dep_source.clone()) else {
        return actors;
    };
    let mut global_actor_names = HashSet::new();
    for decl in &parsed.decls {
        if let Decl::GlobalActor { name, .. } = decl {
            global_actor_names.insert(name.clone());
        }
    }
    let default_iso = dep_manifest.concurrency.default_isolation.as_deref();
    for decl in &parsed.decls {
        let Decl::Function {
            access: AccessLevel::Public,
            name,
            attrs,
            ..
        } = decl
        else {
            continue;
        };
        if dep_decl_has_attr(attrs, "nonisolated") {
            continue;
        }
        if dep_decl_has_attr(attrs, "MainActor") {
            actors.insert(name.clone(), SmolStr::new("MainActor"));
            continue;
        }
        for attr in attrs {
            if global_actor_names.contains(&attr.name) {
                actors.insert(name.clone(), attr.name.clone());
            }
        }
        if actors.contains_key(name) {
            continue;
        }
        if let Some(default) = default_iso {
            actors.insert(name.clone(), SmolStr::new(default));
        }
    }
    actors
}
