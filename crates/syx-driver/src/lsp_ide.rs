//! SYX driver LSP / IDE navigation surface.
//! Extracted in S2-I217 (zero semantics).

use crate::embedded_std;
use crate::query;
use crate::GLOBALS_PACKAGES_REL;
use crate::{
    apply_meta_passes, cache_entry_fresh, compile_project_with_options,
    core_prelude_generic_class_methods, core_prelude_import_symbols, ensure_project_lock,
    expand_core_value_builtin_modules, expand_macros, export_symbols,
    inject_core_collection_builtins, inject_imported_dataclass_decls, prepare_async_module,
    project_cache_dir, register_imported_macros_from_parsed, resolve_module, DriverOptions,
    ModuleGraph,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::Path;
use syx_ast::Decl;
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};
use syx_parser::Parser;
use syx_pkg::{load_manifest, load_resolved_packages};
use syx_semantic::{SemanticAnalyzer, TypeId};

/// Parse and semantically analyze source; returns diagnostics (including semantic warnings).
pub fn diagnostics_for_source(
    name: &str,
    contents: &str,
    options: &DriverOptions,
) -> Vec<SyxDiagnostic> {
    let source = SourceFile::new(name, contents.to_string());
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: false,
        collecting_semantic: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(source.clone());
    match FrontendPipeline::run(source, &opts) {
        Ok(art) => {
            let mut diags = art.diagnostics;
            if let Some(sem) = art.semantic {
                diags.extend(sem.errors);
            }
            diags
        }
        Err(e) => vec![e],
    }
}


pub(crate) fn merge_lsp_import(
    out: &mut Vec<(SmolStr, IndexMap<SmolStr, TypeId>)>,
    module_name: SmolStr,
    symbols: IndexMap<SmolStr, TypeId>,
) {
    if let Some((_, existing)) = out.iter_mut().find(|(n, _)| *n == module_name) {
        for (k, v) in symbols {
            existing.entry(k).or_insert(v);
        }
    } else {
        out.push((module_name, symbols));
    }
}


pub(crate) fn collect_import_symbols_for_lsp(
    module: &syx_ast::Module,
    options: &DriverOptions,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    collect_import_symbols_for_lsp_from(module, options, None)
}


pub(crate) fn collect_import_symbols_for_lsp_from(
    module: &syx_ast::Module,
    options: &DriverOptions,
    importer: Option<&Path>,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    let mut import_symbols = core_prelude_import_symbols(options);
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            if let Some(path_str) = decl.import_path_str() {
                if !options.enable_std {
                    continue;
                }
                let first = path_str.split('.').next().unwrap_or(path_str.as_str());
                if let Some(importer) = importer {
                    if let Some(disk) = embedded_std::find_workspace_stdlib_file(importer, first) {
                        if let Ok(text) = std::fs::read_to_string(&disk) {
                            let source = SourceFile::new(disk.display().to_string(), text);
                            merge_lsp_import(
                                &mut import_symbols,
                                SmolStr::new(first),
                                export_symbols(&source),
                            );
                            continue;
                        }
                    }
                }
                if let Some((name, src)) = embedded_std::lookup(first) {
                    let source = SourceFile::new(
                        embedded_std::virtual_path(name)
                            .to_string_lossy()
                            .into_owned(),
                        src,
                    );
                    merge_lsp_import(
                        &mut import_symbols,
                        SmolStr::new(name),
                        export_symbols(&source),
                    );
                }
            }
        }
    }
    import_symbols
}


pub(crate) fn collect_import_symbols_for_lsp_with_path(
    module: &syx_ast::Module,
    importer: &Path,
    options: &DriverOptions,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    let mut import_symbols = collect_import_symbols_for_lsp_from(module, options, Some(importer));
    let parent = importer.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        if ensure_project_lock(parent, &opts).is_err() {
            return import_symbols;
        }
        opts.resolved_packages = load_resolved_packages(parent).ok();
    }
    let importer_dir = importer.parent().unwrap_or(importer);
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Some(symbol) = decl.import_symbol() else {
                continue;
            };
            let Ok(import_ref) =
                resolve_module(&path_str, importer, importer_dir, &manifest, &opts, parent)
            else {
                continue;
            };
            let Ok((_, dep_source)) = import_ref.load() else {
                continue;
            };
            merge_lsp_import(
                &mut import_symbols,
                symbol.clone(),
                export_symbols(&dep_source),
            );
        }
    }
    import_symbols
}


/// Parsed module + semantic analysis for LSP requests.
#[derive(Debug)]
pub struct LspDocument {
    pub source: SourceFile,
    pub module: syx_ast::Module,
    pub semantic: SemanticAnalyzer,
    /// Phase 59 LSP v51: true when macro expansion changed the module AST.
    pub macro_expanded: bool,
}


pub fn analyze_document_for_lsp(
    path: Option<&Path>,
    contents: &str,
    options: &DriverOptions,
) -> LspDocument {
    let source = SourceFile::new(
        path.map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "buffer.sy".into()),
        contents.to_string(),
    );
    let outcome = Parser::parse_collecting(source.clone());
    let parse_errors = outcome.errors;
    let mut module = if parse_errors.is_empty() {
        if let Some(p) = path {
            query::store_parse(p, contents, &outcome.module);
        }
        outcome.module
    } else {
        if let Some(p) = path {
            query::invalidate_path(p);
        }
        // Keep partial decls so recovery can surface multiple P001s; semantic is skipped below.
        outcome.module
    };
    let decl_count_before = module.decls.len();
    let mut macro_expanded = false;
    let mut pass_errors: Vec<SyxDiagnostic> = Vec::new();
    let importer = path.unwrap_or_else(|| Path::new("buffer.sy"));
    let importer_dir = importer.parent().unwrap_or_else(|| Path::new("."));
    let project_root = importer_dir;
    let manifest = load_manifest(importer_dir).unwrap_or_default();
    let empty_graph = ModuleGraph::default();
    if parse_errors.is_empty() {
        // Resolve `import macros` (etc.) before expansion so LSP sees `#greet`.
        syx_macro::clear_imported_macros();
        syx_macro::register_package_macros(manifest.macros.as_slice());
        let _ = register_imported_macros_from_parsed(
            &module,
            importer,
            importer_dir,
            &manifest,
            options,
            project_root,
            &empty_graph,
        );
        // M1-A03: fail-closed frontend passes via FrontendPipeline (Prepared).
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        match FrontendPipeline::run_parsed(module.clone(), &prep_opts) {
            Ok(art) => {
                macro_expanded = art.module.decls.len() != decl_count_before
                    || format!("{:?}", art.module) != format!("{module:?}");
                module = art.module;
                if let Some(p) = path {
                    query::store_parse(p, contents, &module);
                }
            }
            Err(e) => pass_errors.push(e),
        }
    }
    // Core collection surfaces for LSP (without Class injection — S023 / Array typing).
    // Numeric static members (`Int.min`) are merged as signatures only after collect_decls
    // (see `value_builtin_modules` below) so foreign method bodies cannot pollute
    // `binding_sites` / `expr_types` with Character.sy / Int128.sy spans.
    if parse_errors.is_empty() && pass_errors.is_empty() {
        inject_core_collection_builtins(&mut module, options);
    }
    let value_builtin_modules = if parse_errors.is_empty() && pass_errors.is_empty() {
        expand_core_value_builtin_modules(options)
    } else {
        Vec::new()
    };
    if parse_errors.is_empty() && pass_errors.is_empty() {
        let _ = inject_imported_dataclass_decls(
            &mut module,
            importer,
            importer_dir,
            &manifest,
            options,
            project_root,
            &empty_graph,
        );
    }
    let imports = if parse_errors.is_empty() && pass_errors.is_empty() {
        if let Some(p) = path {
            collect_import_symbols_for_lsp_with_path(&module, p, options)
        } else {
            collect_import_symbols_for_lsp(&module, options)
        }
    } else {
        Vec::new()
    };
    let strict = path
        .and_then(|p| p.parent())
        .map(|parent| load_manifest(parent).unwrap_or_default())
        .map(|m| {
            (
                m.concurrency.strict,
                m.concurrency
                    .default_isolation
                    .as_ref()
                    .map(|s| SmolStr::new(s.as_str())),
            )
        })
        .unwrap_or((false, None));
    let empty_generics = IndexMap::new();
    let prelude_generics = core_prelude_generic_class_methods(options);
    let run_semantic = parse_errors.is_empty() && pass_errors.is_empty();
    let mut semantic = SemanticAnalyzer::analyze_collecting_with_value_builtins(
        &module,
        &imports,
        if run_semantic { strict.0 } else { false },
        if run_semantic { strict.1 } else { None },
        &[],
        Some(&source),
        if run_semantic {
            &prelude_generics
        } else {
            &empty_generics
        },
        false,
        &value_builtin_modules,
    );
    if !parse_errors.is_empty() {
        semantic.errors = parse_errors;
        enrich_semantic_spans(&source, &mut semantic.errors);
    } else if !pass_errors.is_empty() {
        semantic.errors = pass_errors;
        enrich_semantic_spans(&source, &mut semantic.errors);
    } else {
        enrich_semantic_spans(&source, &mut semantic.errors);
    }
    if let Some(p) = path {
        for decl in &module.decls {
            if let Some(name) = query::decl_name(decl) {
                let body_hash = query::decl_body_hash(decl);
                let decl_errors = semantic
                    .errors
                    .iter()
                    .filter(|e| e.message.contains(name))
                    .count();
                query::record_semantic_decl(p, name, &body_hash, decl_errors);
            }
        }
    }
    LspDocument {
        source,
        module,
        semantic,
        macro_expanded,
    }
}


/// Parse `contents` for member-completion AST only (no semantic analysis).
/// Soft-fail on pass errors is intentional for completion UX (not EC-M1-1 hard path).
pub fn parse_for_lsp_completion(
    path: Option<&Path>,
    contents: &str,
) -> (SourceFile, syx_ast::Module) {
    let source = SourceFile::new(
        path.map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "buffer.sy".into()),
        contents.to_string(),
    );
    let mut module = Parser::parse(source.clone()).unwrap_or(syx_ast::Module {
        name: "error".into(),
        decls: vec![],
    });
    // Soft path: best-effort expand; do not pretend success when expand fails — keep parse AST.
    if let Ok(expanded) = expand_macros(module.clone()) {
        module = expanded;
        let _ = apply_meta_passes(&module);
        let _ = prepare_async_module(&mut module);
    }
    (source, module)
}


/// Phase 52 LSP: preview macro expansion for `syx.macro.expand` command.
pub fn macro_expand_preview(source: &str) -> SyxResult<String> {
    let file = SourceFile::new("macro_preview.sy", source.to_string());
    let expanded = expand_macros(Parser::parse(file)?)?;
    Ok(format!("{expanded:#?}"))
}


/// Attach source spans to semantic diagnostics when possible (LSP publishDiagnostics).
pub fn enrich_semantic_spans(source: &SourceFile, errors: &mut [SyxDiagnostic]) {
    for err in errors.iter_mut() {
        if err.primary_span.is_some() {
            continue;
        }
        let name = err
            .message
            .strip_prefix("Undefined identifier '")
            .and_then(|s| s.strip_suffix('\''))
            .or_else(|| {
                err.message
                    .strip_prefix("Undefined function '")
                    .and_then(|s| s.strip_suffix('\''))
            })
            .or_else(|| {
                err.message
                    .strip_prefix("Use of moved value '")
                    .and_then(|s| s.strip_suffix('\''))
            })
            .or_else(|| {
                err.message
                    .strip_prefix("Unknown macro '#")
                    .and_then(|s| s.strip_suffix('\''))
            });
        if let Some(name) = name {
            // Prefer `#name` for macro diagnostics so the squiggle lands on the invoke.
            let needle = if err.message.starts_with("Unknown macro '#") {
                format!("#{name}")
            } else {
                name.to_string()
            };
            if let Some(off) = source.contents.find(&needle) {
                *err = SyxDiagnostic {
                    message: err.message.clone(),
                    severity: err.severity,
                    code: err.code.clone(),
                    source: None,
                    primary_span: None,
                }
                .with_span(source, Span::new(off, needle.len()), name);
            }
        }
    }
}


/// Convert a source span to 0-based LSP line/character ranges.
pub fn span_to_lsp_range(source: &SourceFile, span: Span) -> ((usize, usize), (usize, usize)) {
    let (sl, sc) = source.line_col(span.start as usize);
    let (el, ec) = source.line_col(span.end());
    (
        (sl.saturating_sub(1), sc.saturating_sub(1)),
        (el.saturating_sub(1), ec.saturating_sub(1)),
    )
}


pub fn name_span_in_container(source: &SourceFile, container: Span, name: &str) -> Span {
    let start = container.start as usize;
    let end = container.end().min(source.contents.len());
    if start >= end {
        return container;
    }
    let slice = &source.contents[start..end];
    if let Some(rel) = slice.find(name) {
        return Span::new(start + rel, name.len());
    }
    container
}


pub fn find_definition_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Option<Span> {
    find_definition_in_decls(source, &module.decls, name)
        .or_else(|| find_definition_in_function_bodies(source, &module.decls, name))
}


pub(crate) fn find_definition_in_decls(source: &SourceFile, decls: &[Decl], name: &str) -> Option<Span> {
    for decl in decls {
        match decl {
            Decl::Function { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::VarGlobal { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::Class {
                name: n,
                span,
                members,
                ..
            }
            | Decl::Actor {
                name: n,
                span,
                members,
                ..
            } if n.as_str() == name => {
                let _ = members;
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::DataClass { name: n, span, .. }
            | Decl::Struct { name: n, span, .. }
            | Decl::Enum { name: n, span, .. }
            | Decl::FinalClass { name: n, span, .. }
                if n.as_str() == name =>
            {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::Class {
                name: n,
                span,
                members,
                ..
            }
            | Decl::Actor {
                name: n,
                span,
                members,
                ..
            } if n.as_str() != name => {
                let _ = (n, span);
                if let Some(span) = find_definition_in_decls(source, members, name) {
                    return Some(span);
                }
            }
            Decl::Extension { methods, .. } => {
                if let Some(span) = find_definition_in_decls(source, methods, name) {
                    return Some(span);
                }
            }
            Decl::Function { body, .. } | Decl::Init { body, .. } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
            }
            _ => {}
        }
    }
    None
}


pub(crate) fn find_definition_in_function_bodies(
    source: &SourceFile,
    decls: &[Decl],
    name: &str,
) -> Option<Span> {
    for decl in decls {
        if let Decl::Function { body, .. } = decl {
            if let Some(span) = find_definition_in_stmts(source, body, name) {
                return Some(span);
            }
        }
    }
    None
}


pub(crate) fn find_definition_in_stmts(
    source: &SourceFile,
    stmts: &[syx_ast::Stmt],
    name: &str,
) -> Option<Span> {
    for stmt in stmts {
        match stmt {
            syx_ast::Stmt::VarDecl { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            syx_ast::Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                if let Some(span) = find_definition_in_stmts(source, then_branch, name) {
                    return Some(span);
                }
                if let Some(branch) = else_branch {
                    if let Some(span) = find_definition_in_stmts(source, branch, name) {
                        return Some(span);
                    }
                }
            }
            syx_ast::Stmt::While { body, .. }
            | syx_ast::Stmt::ForIn { body, .. }
            | syx_ast::Stmt::Block { stmts: body, .. }
            | syx_ast::Stmt::RepeatWhile { body, .. } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
            }
            syx_ast::Stmt::Try {
                body, catch_body, ..
            } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
                if let Some(span) = find_definition_in_stmts(source, catch_body, name) {
                    return Some(span);
                }
            }
            syx_ast::Stmt::Switch { cases, default, .. } => {
                for case in cases {
                    if let Some(span) = find_definition_in_stmts(source, &case.body, name) {
                        return Some(span);
                    }
                }
                if let Some(body) = default {
                    if let Some(span) = find_definition_in_stmts(source, body, name) {
                        return Some(span);
                    }
                }
            }
            _ => {}
        }
    }
    None
}


pub fn find_references_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Vec<Span> {
    let mut spans = Vec::new();
    collect_references_in_decls(source, &module.decls, name, &mut spans);
    spans
}


/// Find references to a symbol across all modules in a project (when `syx.toml` exists).
pub fn find_references_in_project(
    path: &Path,
    symbol: &str,
    options: &DriverOptions,
) -> Option<Vec<(String, Span)>> {
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    let mut out = Vec::new();
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        let uri = format!("file://{}", mod_path.display());
        for span in find_references_in_module(&unit.source, &parsed, symbol) {
            out.push((uri.clone(), span));
        }
    }
    Some(out)
}


/// Workspace symbol search across a compiled project graph.
pub fn workspace_symbols_for_project(
    root: &Path,
    query: &str,
    options: &DriverOptions,
) -> SyxResult<Vec<(String, String, Span)>> {
    let project = compile_project_with_options(root, options)?;
    let q = query.to_ascii_lowercase();
    let mut out = Vec::new();
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone())?;
        let uri = format!("file://{}", mod_path.display());
        for decl in &parsed.decls {
            match decl {
                Decl::Function { name, span, .. }
                | Decl::VarGlobal { name, span, .. }
                | Decl::Class { name, span, .. }
                | Decl::Enum { name, span, .. } => {
                    if query.is_empty() || name.to_ascii_lowercase().contains(&q) {
                        out.push((uri.clone(), name.to_string(), *span));
                    }
                }
                _ => {}
            }
        }
    }
    Ok(out)
}


/// Suggest import module paths from stdlib + lightweight project scan.
///
/// Avoids full `compile_project` (too slow for completion keystrokes).
/// Stdlib suggestions are **importable** modules only (`IO`, `Net`, `Crypto`, …),
/// not Core prelude names like `Math` / `Array`.
pub fn import_module_suggestions(root: Option<&Path>, options: &DriverOptions) -> Vec<String> {
    let mut modules = std::collections::BTreeSet::new();
    if options.enable_std {
        for name in embedded_std::importable_module_names() {
            modules.insert(name.to_string());
        }
    }
    if let Some(root) = root {
        let manifest_root = if root.is_file() {
            root.parent().unwrap_or(root)
        } else {
            root
        };
        if let Ok(manifest) = load_manifest(manifest_root) {
            for dep in manifest.dependencies.keys() {
                modules.insert(dep.clone());
            }
        }
        // Sibling / directory `.sy` stems (local modules), no compile.
        let scan_dirs = [manifest_root, root];
        for dir in scan_dirs {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("sy") {
                    continue;
                }
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "main" && !stem.starts_with('.') {
                        modules.insert(stem.to_string());
                    }
                }
            }
        }
    }
    modules.into_iter().collect()
}


pub(crate) fn collect_references_in_decls(
    source: &SourceFile,
    decls: &[Decl],
    name: &str,
    out: &mut Vec<Span>,
) {
    for decl in decls {
        match decl {
            Decl::Function {
                name: n,
                span,
                body,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                collect_references_in_stmts(source, body, name, out);
            }
            Decl::VarGlobal {
                name: n,
                span,
                init,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                if let Some(expr) = init {
                    collect_references_in_expr(source, expr, name, out);
                }
            }
            Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                collect_references_in_decls(source, members, name, out);
            }
            Decl::Extension { methods, .. } => {
                collect_references_in_decls(source, methods, name, out);
            }
            Decl::Init { body, .. } | Decl::Deinit { body, .. } => {
                collect_references_in_stmts(source, body, name, out);
            }
            _ => {}
        }
    }
}


pub(crate) fn collect_references_in_stmts(
    source: &SourceFile,
    stmts: &[syx_ast::Stmt],
    name: &str,
    out: &mut Vec<Span>,
) {
    for stmt in stmts {
        match stmt {
            syx_ast::Stmt::VarDecl {
                name: n,
                span,
                init,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                if let Some(expr) = init {
                    collect_references_in_expr(source, expr, name, out);
                }
            }
            syx_ast::Stmt::Expr { expr, .. }
            | syx_ast::Stmt::Print { expr, .. }
            | syx_ast::Stmt::Return {
                value: Some(expr), ..
            } => {
                collect_references_in_expr(source, expr, name, out);
            }
            syx_ast::Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                collect_references_in_expr(source, condition, name, out);
                collect_references_in_stmts(source, then_branch, name, out);
                if let Some(branch) = else_branch {
                    collect_references_in_stmts(source, branch, name, out);
                }
            }
            syx_ast::Stmt::While {
                condition, body, ..
            } => {
                collect_references_in_expr(source, condition, name, out);
                collect_references_in_stmts(source, body, name, out);
            }
            syx_ast::Stmt::ForIn {
                name: n,
                span,
                iterable,
                body,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                collect_references_in_expr(source, iterable, name, out);
                collect_references_in_stmts(source, body, name, out);
            }
            syx_ast::Stmt::Block { stmts, .. } => {
                collect_references_in_stmts(source, stmts, name, out);
            }
            syx_ast::Stmt::Try {
                body, catch_body, ..
            } => {
                collect_references_in_stmts(source, body, name, out);
                collect_references_in_stmts(source, catch_body, name, out);
            }
            _ => {}
        }
    }
}


pub(crate) fn collect_references_in_expr(
    source: &SourceFile,
    expr: &syx_ast::Expr,
    name: &str,
    out: &mut Vec<Span>,
) {
    match expr {
        syx_ast::Expr::Ident { name: n, span, .. } if n.as_str() == name => {
            out.push(*span);
        }
        syx_ast::Expr::Binary { left, right, .. } => {
            collect_references_in_expr(source, left, name, out);
            collect_references_in_expr(source, right, name, out);
        }
        syx_ast::Expr::Unary { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::Call { callee, args, .. } => {
            collect_references_in_expr(source, callee, name, out);
            for arg in args {
                collect_references_in_expr(source, &arg.value, name, out);
            }
        }
        syx_ast::Expr::Member { object, .. } => {
            collect_references_in_expr(source, object, name, out);
        }
        syx_ast::Expr::Index { object, index, .. } => {
            collect_references_in_expr(source, object, name, out);
            collect_references_in_expr(source, index, name, out);
        }
        syx_ast::Expr::Assign { value, .. }
        | syx_ast::Expr::CompoundAssign { value, .. }
        | syx_ast::Expr::MemberAssign { value, .. }
        | syx_ast::Expr::IndexAssign { value, .. } => {
            collect_references_in_expr(source, value, name, out);
        }
        syx_ast::Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => collect_references_in_expr(source, e, name, out),
            syx_ast::ClosureBody::Block(stmts) => {
                collect_references_in_stmts(source, stmts, name, out);
            }
        },
        syx_ast::Expr::TaskBlock { body, .. } => {
            collect_references_in_stmts(source, body, name, out);
        }
        syx_ast::Expr::Group { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::Array { elements, .. } => {
            for el in elements {
                collect_references_in_expr(source, el, name, out);
            }
        }
        syx_ast::Expr::Await { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::OptionalChain { object, .. } => {
            collect_references_in_expr(source, object, name, out);
        }
        syx_ast::Expr::ForceUnwrap { expr, .. } => {
            collect_references_in_expr(source, expr, name, out);
        }
        _ => {}
    }
}


pub fn definition_for_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    symbol: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if let Some(span) = find_definition_in_module(&doc.source, &doc.module, symbol) {
        return Some((uri.to_string(), span));
    }
    if let Some(hit) = definition_for_global_builtin(symbol, path) {
        return Some(hit);
    }
    let path = path?;
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        if let Some(span) = find_definition_in_module(&unit.source, &parsed, symbol) {
            let mod_uri = format!("file://{}", mod_path.display());
            return Some((mod_uri, span));
        }
    }
    None
}


/// Resolve `print` / `readLine` / `reflect` to the Core globals authority file.
pub fn definition_for_global_builtin(
    symbol: &str,
    from: Option<&Path>,
) -> Option<(String, Span)> {
    let embedded = embedded_std::lookup_global_builtin(symbol)?;
    let disk = from.and_then(|p| {
        embedded_std::find_workspace_packages_file(p, GLOBALS_PACKAGES_REL)
    });
    let (uri, text) = if let Some(disk) = disk {
        (
            format!("file://{}", disk.display()),
            std::fs::read_to_string(&disk).unwrap_or_else(|_| embedded.to_string()),
        )
    } else {
        (
            format!("file://{}", embedded_std::virtual_global_builtin_path().display()),
            embedded.to_string(),
        )
    };
    let path_for_source = uri.strip_prefix("file://").unwrap_or(uri.as_str());
    let source = SourceFile::new(path_for_source.to_string(), text);
    let parsed = Parser::parse(source.clone()).ok()?;
    let span = find_definition_in_module(&source, &parsed, symbol)?;
    Some((uri, span))
}


/// SYX 38.0: resolve `importedModule.symbol` or nested `Outer.Inner` definitions.
pub fn definition_for_qualified_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    qualifier: &str,
    member: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if doc
        .semantic
        .imported_modules
        .iter()
        .any(|m| m.as_str() == qualifier)
    {
        let path = path?;
        let parent = path.parent()?;
        if !parent.join("syx.toml").exists() {
            return None;
        }
        let project = compile_project_with_options(parent, options).ok()?;
        for (mod_path, unit) in &project.graph.modules {
            let stem = mod_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem != qualifier && !mod_path.to_string_lossy().contains(qualifier) {
                continue;
            }
            let parsed = Parser::parse(unit.source.clone()).ok()?;
            if let Some(span) = find_definition_in_module(&unit.source, &parsed, member) {
                let mod_uri = format!("file://{}", mod_path.display());
                return Some((mod_uri, span));
            }
        }
    }
    if qualifier.contains('.') {
        let mangled = qualifier.replace('.', "_");
        if let Some(span) = find_definition_in_module(&doc.source, &doc.module, &mangled) {
            return Some((uri.to_string(), span));
        }
        if let Some(span) = find_definition_in_module(&doc.source, &doc.module, member) {
            return Some((uri.to_string(), span));
        }
    }
    definition_for_symbol(uri, path, doc, member, options)
}


/// Phase 18 LSP v10: resolve the type definition for a symbol (not its value binding).
pub fn go_to_type_for_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    symbol: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if let Some(span) = find_type_decl_in_module(&doc.source, &doc.module, symbol) {
        return Some((uri.to_string(), span));
    }
    let ty = doc.semantic.symbol_type(symbol)?;
    let type_name = syx_semantic::type_id_to_name(&ty);
    let base = type_name
        .split('<')
        .next()
        .unwrap_or(type_name.as_str())
        .trim_end_matches('?');
    if let Some(core_src) = embedded_std::lookup_core_builtin(base) {
        let source = SourceFile::new(
            embedded_std::virtual_core_builtin_path(base)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("<stdlib>/{base}.sy")),
            core_src.to_string(),
        );
        if let Some(span) =
            find_type_decl_in_module(&source, &Parser::parse(source.clone()).ok()?, base)
        {
            let mod_uri = format!(
                "file://{}",
                embedded_std::virtual_core_builtin_path(base)
                    .unwrap_or_else(|| embedded_std::virtual_path(base))
                    .display()
            );
            return Some((mod_uri, span));
        }
    }
    if matches!(base, "Unknown" | "Nil") {
        return None;
    }
    if let Some(span) = find_type_decl_in_module(&doc.source, &doc.module, base) {
        return Some((uri.to_string(), span));
    }
    let path = path?;
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        if let Some(span) = find_type_decl_in_module(&unit.source, &parsed, base) {
            let mod_uri = format!("file://{}", mod_path.display());
            return Some((mod_uri, span));
        }
    }
    None
}


pub(crate) fn find_type_decl_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Option<Span> {
    for decl in &module.decls {
        match decl {
            syx_ast::Decl::Struct { name: n, span, .. }
            | syx_ast::Decl::Enum { name: n, span, .. }
            | syx_ast::Decl::DataClass { name: n, span, .. }
            | syx_ast::Decl::FinalClass { name: n, span, .. }
            | syx_ast::Decl::Class { name: n, span, .. }
            | syx_ast::Decl::Actor { name: n, span, .. }
            | syx_ast::Decl::Protocol { name: n, span, .. }
                if n.as_str() == name =>
            {
                return Some(name_span_in_container(source, *span, name));
            }
            _ => {}
        }
    }
    None
}

