//! SYX driver prelude / inject / export glue face.
//! Extracted in S2-I222 (zero semantics).

use crate::compile_cache::{load_prelude_snapshot, store_prelude_snapshot};
use crate::embedded_std;
use crate::module_ref::ModuleRef;
use crate::{
    collect_stdlib_extension_methods, compile_to_program, cross_module_function_name,
    inject_virtual_prelude_imports, merge_stdlib_mono_instances, mir_merge_stdlib_function,
    resolve_module, DriverOptions, ModuleGraph, ModuleUnit,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::{Path, PathBuf};
use syx_ast::{AccessLevel, Decl};
use syx_codegen::CompiledProgram;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_frontend::{expand_macros, FrontendOptions, FrontendPipeline, StopAfter};
use syx_meta::register_data_classes;
use syx_parser::Parser;
use syx_pkg::SyxManifest;
use syx_semantic::{type_expr_to_id, SemanticAnalyzer, TypeId};

pub(crate) fn core_prelude_import_symbols(
    options: &DriverOptions,
    graph: Option<&ModuleGraph>,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    if !options.enable_std {
        return Vec::new();
    }
    if let Some(graph) = graph {
        if !graph.stdlib_exports.is_empty() {
            let mut out = Vec::new();
            for (name, _) in embedded_std::core_prelude() {
                let path = embedded_std::virtual_path(name);
                if let Some(exports) = graph.stdlib_exports.get(&path) {
                    out.push((SmolStr::new(*name), exports.clone()));
                } else {
                    // Fallback per-module if snapshot missed one.
                    let src = embedded_std::core_prelude()
                        .iter()
                        .find(|(n, _)| *n == *name)
                        .map(|(_, s)| *s)
                        .unwrap_or("");
                    let source = SourceFile::new(
                        path.to_string_lossy().into_owned(),
                        src.to_string(),
                    );
                    out.push((SmolStr::new(*name), export_symbols(&source)));
                }
            }
            return out;
        }
    }
    embedded_std::core_prelude()
        .iter()
        .map(|(name, src)| {
            let source = SourceFile::new(
                embedded_std::virtual_path(name)
                    .to_string_lossy()
                    .into_owned(),
                (*src).to_string(),
            );
            (SmolStr::new(*name), export_symbols(&source))
        })
        .collect()
}

pub(crate) fn ensure_prelude_modules_in_graph(
    graph: &mut ModuleGraph,
    _manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    // S3-I49: dedicated prelude disk snapshot (cache_version_tag keyed).
    // Disk probe/store only when `incremental_cache` (CLI `--with-cache` / `syx build`).
    // Rkyv slim image is interpret-only (empty sources); MIR re-lower needs bincode.
    let allow_rkyv = !crate::mir_unify::want_stdlib_mir(options);
    if load_prelude_snapshot(graph, project_root, options.incremental_cache, allow_rkyv) {
        return Ok(());
    }
    let mut prelude_imports: Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> = Vec::new();
    for (name, src) in embedded_std::core_prelude() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            if let Some(unit) = graph.modules.get(&key) {
                prelude_imports.push((SmolStr::new(*name), export_symbols(&unit.source)));
            }
            continue;
        }
        // SA-4-04: prelude value modules call `SyxHost.*`; prepend host before analyze.
        let hosted = embedded_std::ensure_syx_host(src);
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), hosted);
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, &prelude_imports)?;
        let mut program = CompiledProgram::default();
        let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
            .modules
            .iter()
            .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
            .collect();
        for dep in graph.modules.values() {
            let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            for func in &dep.program.functions {
                let merge =
                    mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
                if merge && !program.functions.iter().any(|f| f.name == func.name) {
                    program.functions.push(func.clone());
                }
            }
        }
        merge_stdlib_mono_instances(graph, &mut program, &semantic.instantiations, options)?;
        let prelude_ext = collect_stdlib_extension_methods(graph);
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &prelude_ext,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit.clone());
        prelude_imports.push((SmolStr::new(*name), export_symbols(&unit.source)));
    }
    ensure_embedded_protocols_in_graph(graph, options, &prelude_imports)?;
    ensure_reference_builtins_in_graph(graph, options, &prelude_imports)?;
    if graph.stdlib_extension_methods.is_empty() {
        graph.stdlib_extension_methods = collect_stdlib_extension_methods(graph);
    }
    if graph.stdlib_exports.is_empty() {
        for (path, unit) in &graph.modules {
            if path.to_string_lossy().starts_with("<stdlib>/") {
                graph
                    .stdlib_exports
                    .insert(path.clone(), export_symbols(&unit.source));
            }
        }
    }
    store_prelude_snapshot(graph, project_root, options.incremental_cache);
    Ok(())
}

fn ensure_embedded_protocols_in_graph(
    graph: &mut ModuleGraph,
    options: &DriverOptions,
    prelude_imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    for (name, src) in embedded_std::embedded_protocols() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            continue;
        }
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), (*src).to_string());
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, prelude_imports)?;
        let mut program = CompiledProgram::default();
        let empty_prelude = indexmap::IndexSet::new();
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &empty_prelude,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit);
    }
    Ok(())
}

/// Load `Object` / `ObjectIdentifier` Core reference builtins into the graph so
/// their extension witnesses (`Object_valueForKey`, …) merge like Dictionary.
fn ensure_reference_builtins_in_graph(
    graph: &mut ModuleGraph,
    options: &DriverOptions,
    prelude_imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    for (name, src) in embedded_std::embedded_reference_builtins() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            continue;
        }
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), (*src).to_string());
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, prelude_imports)?;
        let mut program = CompiledProgram::default();
        let empty_prelude = indexmap::IndexSet::new();
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &empty_prelude,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit);
    }
    Ok(())
}

fn expanded_dep_module(dep_source: &SourceFile) -> Option<syx_ast::Module> {
    let parsed = Parser::parse(dep_source.clone()).ok()?;
    expand_macros(parsed).ok()
}

fn dataclass_decls_from_module(module: &syx_ast::Module) -> Vec<Decl> {
    module
        .decls
        .iter()
        .filter(|d| matches!(d, Decl::DataClass { .. }))
        .cloned()
        .collect()
}

fn extension_decls_for_type(module: &syx_ast::Module, type_name: &SmolStr) -> Vec<Decl> {
    module
        .decls
        .iter()
        .filter_map(|d| {
            if let Decl::Extension {
                type_name: tn,
                methods,
                span,
                conformances,
                where_clause,
            } = d
            {
                if tn == type_name {
                    Some(Decl::Extension {
                        span: *span,
                        type_name: tn.clone(),
                        conformances: conformances.clone(),
                        where_clause: where_clause.clone(),
                        methods: methods.clone(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// Merge Core Array/Set/Dictionary/Object extensions into a buffer module for LSP.
/// OrderedSet / Math / … stay available via `core_prelude_import_symbols` —
/// do not inject their classes (S023 prelude-name shadow).
pub(crate) fn inject_core_collection_builtins(
    module: &mut syx_ast::Module,
    options: &DriverOptions,
) {
    if !options.enable_std {
        return;
    }
    for surface in ["Array", "Set", "Dictionary", "Object"] {
        let Some((_, src)) = embedded_std::lookup_core(surface) else {
            continue;
        };
        let source = SourceFile::new(
            embedded_std::virtual_path(surface)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        let Some(dep_module) = expanded_dep_module(&source) else {
            continue;
        };
        for decl in &dep_module.decls {
            if let Decl::Extension { type_name, .. } = decl {
                if type_name.as_str() == surface {
                    merge_imported_extension(module, decl.clone());
                }
            }
        }
    }
}

/// Expanded Core value-builtin modules for LSP signature merge (no body analysis).
pub(crate) fn expand_core_value_builtin_modules(options: &DriverOptions) -> Vec<syx_ast::Module> {
    if !options.enable_std {
        return Vec::new();
    }
    const SURFACES: &[&str] = &[
        "Int8",
        "Int16",
        "Int32",
        "Int64",
        "Int128",
        "Int",
        "UInt8",
        "UInt16",
        "UInt32",
        "UInt64",
        "UInt128",
        "UInt",
        "Float32",
        "Float64",
        "Float",
        "Double",
        "Bool",
        "Character",
        "BigInt",
    ];
    let mut out = Vec::new();
    for surface in SURFACES {
        let Some((_, src)) = embedded_std::lookup_core(surface) else {
            continue;
        };
        let source = SourceFile::new(
            embedded_std::virtual_path(surface)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        if let Some(dep_module) = expanded_dep_module(&source) {
            out.push(dep_module);
        }
    }
    out
}

/// Phase 59: inject imported dependency data classes, public classes, and type
/// extensions into the compiling module (so Core prelude Array helpers and
/// OrderedSet, or `import Networking`, land in semantic / LSP scope).
pub(crate) fn inject_imported_dataclass_decls(
    module: &mut syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    let imports: Vec<(String, SmolStr)> = module
        .decls
        .iter()
        .filter_map(|d| {
            let Decl::Import { .. } = d else {
                return None;
            };
            Some((d.import_path_str()?, d.import_symbol()?.clone()))
        })
        .collect();
    for (path_str, symbol) in imports {
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
        let Some(dep_module) = expanded_dep_module(&dep_source) else {
            continue;
        };
        for dc in dataclass_decls_from_module(&dep_module) {
            let Decl::DataClass { name, .. } = &dc else {
                continue;
            };
            if module
                .decls
                .iter()
                .any(|d| matches!(d, Decl::DataClass { name: n, .. } if n == name))
            {
                continue;
            }
            module.decls.push(dc.clone());
            for ext in extension_decls_for_type(&dep_module, name) {
                merge_imported_extension(module, ext);
            }
        }
        for decl in &dep_module.decls {
            if let Decl::Class {
                access: AccessLevel::Public,
                name,
                ..
            } = decl
            {
                // Do not inject Core TypeId surfaces (Array/String/…) as named
                // classes — that collapses `TypeId::Array` into `Named("Array")`
                // and drops builtin method tables. Still merge their extensions.
                if is_core_typeid_surface(name.as_str()) {
                    continue;
                }
                // Host-only builtins (e.g. HTTPResponse) skip inject. SA-3-01:
                // former module façades (`Env`/`Path`/`Time`/`Os`) are real
                // `class` decls under `import System` and must inject even when
                // also listed in the host builtin registry.
                if syx_stdlib::is_builtin_type(name.as_str())
                    && !dep_module.decls.iter().any(|d| {
                        matches!(d, Decl::Class { name: n, access: AccessLevel::Public, .. } if n == name)
                    })
                {
                    continue;
                }
                // Module-homonym class (Crypto in `import Crypto`) stays available
                // via import_exports; injecting it forces Crypto_sha256 label checks.
                if name == &symbol {
                    continue;
                }
                if !module.decls.iter().any(|d| {
                    matches!(d, Decl::Class { name: n, .. } if n == name)
                        || matches!(d, Decl::DataClass { name: n, .. } if n == name)
                        || matches!(d, Decl::Enum { name: n, .. } if n == name)
                        || matches!(d, Decl::Struct { name: n, .. } if n == name)
                }) {
                    module.decls.push(decl.clone());
                }
            }
            // SA-3-01: public enums (`ProcessInfo`, `SystemError`, `Env`, …) must
            // inject like classes — caseless enum namespaces + Error pilots.
            // SA-4-04a: also inject internal `SyxHost` so injected public façades that
            // call `SyxHost.*` still resolve (bodies are cloned into the importer).
            if let Decl::Enum { access, name, .. } = decl {
                let inject_enum = matches!(access, AccessLevel::Public)
                    || (matches!(access, AccessLevel::Internal) && name.as_str() == "SyxHost");
                if !inject_enum {
                    continue;
                }
                if name == &symbol {
                    continue;
                }
                if !module.decls.iter().any(|d| {
                    matches!(d, Decl::Enum { name: n, .. } if n == name)
                        || matches!(d, Decl::Class { name: n, .. } if n == name)
                        || matches!(d, Decl::DataClass { name: n, .. } if n == name)
                        || matches!(d, Decl::Struct { name: n, .. } if n == name)
                }) {
                    module.decls.push(decl.clone());
                    for ext in extension_decls_for_type(&dep_module, name) {
                        merge_imported_extension(module, ext);
                    }
                }
            }
            // R-47: public structs (`FilePath` value type) inject like classes/enums.
            if let Decl::Struct {
                access: AccessLevel::Public,
                name,
                ..
            } = decl
            {
                if name == &symbol {
                    continue;
                }
                if !module.decls.iter().any(|d| {
                    matches!(d, Decl::Struct { name: n, .. } if n == name)
                        || matches!(d, Decl::Class { name: n, .. } if n == name)
                        || matches!(d, Decl::DataClass { name: n, .. } if n == name)
                        || matches!(d, Decl::Enum { name: n, .. } if n == name)
                }) {
                    module.decls.push(decl.clone());
                    for ext in extension_decls_for_type(&dep_module, name) {
                        merge_imported_extension(module, ext);
                    }
                }
            }
            if let Decl::Extension { type_name, .. } = decl {
                // Skip extensions already handled via dataclass / enum / struct injection above.
                let is_dc = dep_module
                    .decls
                    .iter()
                    .any(|d| matches!(d, Decl::DataClass { name: n, .. } if n == type_name));
                let is_enum = dep_module
                    .decls
                    .iter()
                    .any(|d| matches!(d, Decl::Enum { name: n, .. } if n == type_name));
                let is_struct = dep_module
                    .decls
                    .iter()
                    .any(|d| matches!(d, Decl::Struct { name: n, .. } if n == type_name));
                if !is_dc && !is_enum && !is_struct {
                    merge_imported_extension(module, decl.clone());
                }
            }
        }
    }
    Ok(())
}

/// Builtin type names that use dedicated `TypeId` variants (not `Named`).
fn is_core_typeid_surface(name: &str) -> bool {
    matches!(
        name,
        "Array"
            | "Set"
            | "Dictionary"
            | "String"
            | "Bool"
            | "Character"
            | "Void"
            | "Int"
            | "Int8"
            | "Int16"
            | "Int32"
            | "Int64"
            | "Int128"
            | "UInt"
            | "UInt8"
            | "UInt16"
            | "UInt32"
            | "UInt64"
            | "UInt128"
            | "Float"
            | "Float32"
            | "Float64"
            | "Double"
            | "BigInt"
            | "Object"
            | "ObjectIdentifier"
            | "Result"
            | "SIMD4"
    )
}

fn merge_imported_extension(module: &mut syx_ast::Module, ext: Decl) {
    let Decl::Extension {
        type_name,
        methods,
        span,
        conformances,
        where_clause,
    } = ext
    else {
        return;
    };
    if let Some(existing) = module.decls.iter_mut().find_map(|d| {
        if let Decl::Extension {
            type_name: tn,
            methods: existing_methods,
            ..
        } = d
        {
            if tn == &type_name {
                Some(existing_methods)
            } else {
                None
            }
        } else {
            None
        }
    }) {
        for method in methods {
            let method_name = match &method {
                Decl::Function { name, .. } => Some(name.clone()),
                _ => None,
            };
            let Some(method_name) = method_name else {
                existing.push(method);
                continue;
            };
            let already = existing
                .iter()
                .any(|m| matches!(m, Decl::Function { name, .. } if name == &method_name));
            if !already {
                existing.push(method);
            }
        }
    } else {
        module.decls.push(Decl::Extension {
            span,
            type_name,
            conformances,
            where_clause,
            methods,
        });
    }
}

pub(crate) fn merge_imported_dataclass_functions(
    program: &mut CompiledProgram,
    export: &SmolStr,
    dep: &ModuleUnit,
) {
    let prefix = format!("{export}_");
    for func in &dep.program.functions {
        if func.name.as_str() == export.as_str() || func.name.starts_with(prefix.as_str()) {
            if !program.functions.iter().any(|f| f.name == func.name) {
                program.functions.push(func.clone());
            }
        }
    }
}

pub(crate) fn register_imported_data_classes_from_parsed(
    module: &syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let import_ref =
                resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            if let Ok(dep_ast) = Parser::parse(dep_source) {
                if let Ok(expanded) = expand_macros(dep_ast) {
                    register_data_classes(&expanded);
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn register_imported_macros_from_parsed(
    module: &syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let import_ref =
                resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            if let Ok(dep_ast) = Parser::parse(dep_source) {
                syx_macro::register_imported_macros(&dep_ast.decls);
            }
        }
    }
    Ok(())
}

pub(crate) fn export_symbols(source: &SourceFile) -> IndexMap<SmolStr, TypeId> {
    export_function_details(source)
        .into_iter()
        .map(|(name, (ty, _))| (name, ty))
        .collect()
}

/// Public function signatures from a module source (`name(args) -> Ret`).
pub fn export_function_signatures(source: &SourceFile) -> IndexMap<SmolStr, String> {
    export_function_details(source)
        .into_iter()
        .map(|(name, (_, sig))| (name, sig))
        .collect()
}

/// Signature for a stdlib module export (e.g. Math.add).
pub fn stdlib_export_signature(module: &str, func: &str) -> Option<String> {
    let (_, src) = embedded_std::lookup_core(module).or_else(|| embedded_std::lookup(module))?;
    let source = SourceFile::new(
        embedded_std::virtual_path(module)
            .to_string_lossy()
            .into_owned(),
        src.to_string(),
    );
    export_function_signatures(&source).get(func).cloned()
}

fn type_expr_display(ty: &syx_ast::TypeExpr) -> String {
    use syx_ast::TypeExpr;
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                name.to_string()
            } else {
                let args = generics
                    .iter()
                    .map(type_expr_display)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}<{args}>")
            }
        }
        TypeExpr::Optional(inner) => format!("{}?", type_expr_display(inner)),
        TypeExpr::Array(inner) => format!("Array<{}>", type_expr_display(inner)),
        TypeExpr::Dictionary { key, value } => {
            format!(
                "Dictionary<{}, {}>",
                type_expr_display(key),
                type_expr_display(value)
            )
        }
        TypeExpr::Function { .. } => "Function".into(),
        TypeExpr::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_expr_display(ty))
                    } else {
                        type_expr_display(ty)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("({inner})")
        }
        TypeExpr::Reference(inner, mutable, _) => {
            if *mutable {
                format!("&mut {}", type_expr_display(inner))
            } else {
                format!("&{}", type_expr_display(inner))
            }
        }
        TypeExpr::Void => "Void".into(),
        TypeExpr::OpaqueSome(inner) => format!("some {}", type_expr_display(inner)),
        TypeExpr::OpaqueAny(inner) => format!("any {}", type_expr_display(inner)),
    }
}

fn extract_generic_class_methods(
    source: &SourceFile,
) -> IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)> {
    let mut methods = IndexMap::new();
    if let Ok(parsed) = Parser::parse(source.clone()) {
        for decl in &parsed.decls {
            let Decl::Class {
                name: class_name,
                members,
                ..
            } = decl
            else {
                continue;
            };
            for member in members {
                let Decl::Function {
                    name: method,
                    type_params,
                    where_clause,
                    attrs,
                    ..
                } = member
                else {
                    continue;
                };
                if type_params.is_empty() || !attrs.iter().any(|a| a.name.as_str() == "static") {
                    continue;
                }
                let qname = SmolStr::new(format!("{class_name}_{method}"));
                methods.insert(qname, (type_params.clone(), where_clause.clone()));
            }
        }
    }
    methods
}

pub(crate) fn core_prelude_generic_class_methods(
    options: &DriverOptions,
) -> IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)> {
    if !options.enable_std {
        return IndexMap::new();
    }
    let mut out = IndexMap::new();
    for (name, src) in embedded_std::core_prelude() {
        let source = SourceFile::new(
            embedded_std::virtual_path(name)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        out.extend(extract_generic_class_methods(&source));
    }
    out
}

fn export_function_details(source: &SourceFile) -> IndexMap<SmolStr, (TypeId, String)> {
    let mut exports = IndexMap::new();

    fn export_nested_classes(
        exports: &mut IndexMap<SmolStr, (TypeId, String)>,
        outer: &SmolStr,
        members: &[Decl],
    ) {
        for member in members {
            let Decl::Class {
                access,
                name,
                members: nested,
                ..
            } = member
            else {
                continue;
            };
            if !matches!(access, AccessLevel::Public) {
                continue;
            }
            let qualified = SmolStr::new(format!("{outer}_{name}"));
            exports.insert(
                qualified.clone(),
                (
                    TypeId::Named(qualified.clone()),
                    format!("class {qualified}"),
                ),
            );
            export_nested_classes(exports, &qualified, nested);
        }
    }

    if let Ok(parsed) = Parser::parse(source.clone()) {
        for decl in &parsed.decls {
            match decl {
                Decl::Function {
                    access: AccessLevel::Public,
                    name,
                    params,
                    return_type,
                    ..
                } => {
                    insert_export_fn(&mut exports, name, params, return_type);
                }
                Decl::Class {
                    access,
                    name: class_name,
                    members,
                    ..
                } => {
                    if matches!(access, AccessLevel::Public) {
                        exports.insert(
                            class_name.clone(),
                            (
                                TypeId::Named(class_name.clone()),
                                format!("class {class_name}"),
                            ),
                        );
                        export_nested_classes(&mut exports, class_name, members);
                    }
                    for member in members {
                        if let Decl::Function {
                            access: AccessLevel::Public,
                            name,
                            params,
                            return_type,
                            attrs,
                            ..
                        } = member
                        {
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                insert_export_fn(&mut exports, name, params, return_type);
                            }
                        }
                    }
                }
                Decl::DataClass { name, .. } => {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("data class {name}")),
                    );
                }
                // SA-3-01: enums (`ProcessInfo`, `SystemError`, …) must appear in import_exports.
                // SA-4-04: also export internal `SyxHost` so `SyxHost_*` MIR merge
                // (methods live in a pending Extension; merge uses the SyxHost_ prefix).
                Decl::Enum { access, name, .. }
                    if matches!(access, AccessLevel::Public)
                        || (matches!(access, AccessLevel::Internal)
                            && name.as_str() == "SyxHost") =>
                {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("enum {name}")),
                    );
                }
                // R-47: public structs (`FilePath`) must appear in import_exports like enums.
                Decl::Struct {
                    access: AccessLevel::Public,
                    name,
                    members,
                    ..
                } => {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("struct {name}")),
                    );
                    for member in members {
                        if let Decl::Function {
                            access: AccessLevel::Public,
                            name: fn_name,
                            params,
                            return_type,
                            attrs,
                            ..
                        } = member
                        {
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                insert_export_fn(&mut exports, fn_name, params, return_type);
                            }
                        }
                    }
                }
                Decl::Actor { access, name, .. } if matches!(access, AccessLevel::Public) => {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("actor {name}")),
                    );
                }
                _ => {}
            }
        }
    }
    exports
}

fn insert_export_fn(
    exports: &mut IndexMap<SmolStr, (TypeId, String)>,
    name: &SmolStr,
    params: &[syx_ast::Param],
    return_type: &Option<syx_ast::TypeExpr>,
) {
    let ty = return_type
        .as_ref()
        .map(type_expr_to_id)
        .unwrap_or(TypeId::Void);
    let param_str = params
        .iter()
        .map(|p| {
            let pt =
                p.ty.as_ref()
                    .map(type_expr_display)
                    .unwrap_or_else(|| "?".into());
            format!("{}: {pt}", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let ret = return_type
        .as_ref()
        .map(type_expr_display)
        .unwrap_or_else(|| "Void".into());
    let sig = format!("{name}({param_str}) -> {ret}");
    exports.insert(name.clone(), (ty, sig));
}
