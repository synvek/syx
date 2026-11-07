//! SYX driver compile_linked / resolve / project compile face.
//! Extracted in S2-I220 (zero semantics).

use crate::cache_fingerprint::dependency_hashes_from_graph;
use crate::farm_tcp::{incremental_cache_path, purge_stale_incremental_caches};
use crate::module_ref::{embedded_module, ModuleRef};
use crate::query;
use crate::{
    bincode_encode, build_syx, cache_entry_fresh, compile_file_with_context, embedded_std,
    ensure_prelude_modules_in_graph, project_root_if_manifest_entry, CachedModuleUnit,
    CompiledProject, DriverOptions, ModuleGraph, ModuleUnit,
};
use std::path::{Path, PathBuf};
use syx_ast::Decl;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_parser::Parser;
use syx_pkg::{
    load_lockfile, load_manifest, load_resolved_packages, InstalledPackage, SyxManifest,
};

pub fn compile_linked(path: &Path) -> SyxResult<ModuleUnit> {
    compile_linked_with_options(path, &DriverOptions::default())
}

pub fn compile_linked_with_options(path: &Path, options: &DriverOptions) -> SyxResult<ModuleUnit> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    syx_macro::register_package_macros(manifest.macros.as_slice());
    let mut opts = options.clone();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    // M3-E02 / ADR-311: merge syx.toml [permissions] into runtime grants (OR).
    {
        let mut cfg = syx_config::runtime_config();
        cfg.permission.merge_flags(
            manifest.permissions.sandbox,
            manifest.permissions.allow_net,
            manifest.permissions.allow_fs,
            manifest.permissions.allow_env,
            manifest.permissions.allow_process,
            manifest.permissions.allow_plugin,
        );
        syx_config::install_runtime_config(cfg);
    }
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let root = parent;
    purge_stale_incremental_caches(root);
    let cache = incremental_cache_path(root, path);
    let package_main = project_root_if_manifest_entry(path).is_some();
    let current_source = if package_main {
        merge_package_unit_source(root, &manifest.main)
            .map(|(_, s)| s)
            .unwrap_or_default()
    } else {
        std::fs::read_to_string(path).unwrap_or_default()
    };
    if cache.exists() {
        if let Ok(bytes) = std::fs::read(&cache) {
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    if let Some(unit) =
                        query::lookup_cached(path, &current_source, &cached.dependency_hashes)
                    {
                        refresh_service_di_from_source(path)?;
                        return Ok(unit);
                    }
                    let deps = cached.dependency_hashes.clone();
                    let unit: ModuleUnit = cached.into();
                    query::store_cached(unit.clone(), &deps);
                    refresh_service_di_from_source(path)?;
                    return Ok(unit);
                }
            }
        }
    }
    syx_meta::service_di_clear();
    let mut graph = ModuleGraph::default();
    // When this file is the package `main`, merge sibling `.sy` sources (no file-as-module).
    let entry_ref = if package_main {
        let (entry_path, merged) = merge_package_unit_source(root, &manifest.main)?;
        ModuleRef::from_inline(entry_path, path.display().to_string(), merged)
    } else {
        ModuleRef::from_file(path.to_path_buf())
    };
    let unit = compile_module_recursive(entry_ref, root, &mut graph, &manifest, &opts)?;
    if let Some(parent_dir) = cache.parent() {
        let _ = std::fs::create_dir_all(parent_dir);
    }
    if let Ok(bytes) = bincode_encode(&unit, &graph) {
        let _ = std::fs::write(&cache, bytes);
    }
    let deps = dependency_hashes_from_graph(&graph, &unit.path);
    query::store_cached(unit.clone(), &deps);
    Ok(unit)
}

pub fn compile_file(path: &Path) -> SyxResult<ModuleUnit> {
    compile_linked(path)
}

pub fn compile_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<ModuleUnit> {
    compile_linked_with_options(path, options)
}

fn refresh_service_di_from_source(path: &Path) -> SyxResult<()> {
    syx_meta::service_di_clear();
    let source = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let file = SourceFile::new(path.display().to_string(), source);
    let mut module = Parser::parse(file)?;
    syx_macro::expand_service_macros(&mut module)?;
    Ok(())
}

fn scan_syx_file_services(path: &Path) -> SyxResult<()> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let stem = path.file_stem().and_then(|s| s.to_str()).map(String::from);
    let file = SourceFile::new(path.display().to_string(), source);
    let module = Parser::parse(file)?;
    syx_meta::service_di_set_scan_stem(stem);
    let result = syx_macro::queue_services_from_module(&module);
    syx_meta::service_di_set_scan_stem(None);
    result
}

fn scan_dir_services(dir: &Path) -> SyxResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", dir.display())))?
    {
        let entry =
            entry.map_err(|e| SyxDiagnostic::error(format!("Failed to read entry: {e}")))?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| {
                n == "tests" || n == "target" || n == ".sy-cache" || n == "node_modules"
            }) {
                continue;
            }
            // Nested packages (own syx.toml) are scanned only via resolved_packages,
            // not by walking sibling example projects under the same parent folder.
            if path.join("syx.toml").is_file() {
                continue;
            }
            scan_dir_services(&path)?;
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("sy") {
            continue;
        }
        scan_syx_file_services(&path)?;
    }
    Ok(())
}

/// Phase 122: aggregate `@Service` across workspace packages before entry expand.
fn scan_workspace_services(
    root: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<()> {
    scan_dir_services(root)?;
    if let Some(packages) = &options.resolved_packages {
        for pkg in packages.values() {
            scan_dir_services(&pkg.root)?;
        }
    }
    let _ = manifest;
    Ok(())
}

pub fn compile_project(root: &Path) -> SyxResult<CompiledProject> {
    compile_project_with_options(root, &DriverOptions::default())
}

/// Same-package unit = all `.sy` files in the directory that contains `main`
/// (Swift/Go-style: files share one namespace; no file-as-module import).
fn package_unit_sy_files(root: &Path, main_rel: &str) -> SyxResult<Vec<PathBuf>> {
    let main_path = root.join(main_rel);
    let dir = main_path.parent().unwrap_or(root);
    let entries = std::fs::read_dir(dir).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read package dir {}: {e}", dir.display()))
    })?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry
            .map_err(|e| SyxDiagnostic::error(format!("Failed to read dir entry: {e}")))?;
        let p = entry.path();
        if p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("sy") {
            files.push(p);
        }
    }
    files.sort();
    if files.is_empty() {
        return Err(SyxDiagnostic::error(format!(
            "No .sy sources in package directory {}",
            dir.display()
        )));
    }
    // Compile main last so its `main` wins ordering-wise for entry discovery.
    if let Some(pos) = files.iter().position(|p| p == &main_path) {
        let main = files.remove(pos);
        files.push(main);
    }
    Ok(files)
}

fn package_local_stems(files: &[PathBuf]) -> indexmap::IndexSet<String> {
    files
        .iter()
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect()
}

/// Drop `import sibling` lines that target another file in the same package unit.
fn strip_package_local_imports(source: &str, local_stems: &indexmap::IndexSet<String>) -> String {
    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("import ") {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(|c| c == ';' || c == ',');
            let stem = name.split('.').next().unwrap_or(name);
            if local_stems.contains(stem) {
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn merge_package_unit_source(root: &Path, main_rel: &str) -> SyxResult<(PathBuf, String)> {
    let main_path = root.join(main_rel);
    let files = package_unit_sy_files(root, main_rel)?;
    let stems = package_local_stems(&files);
    let mut merged = String::new();
    for file in &files {
        let text = std::fs::read_to_string(file).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read {}: {e}", file.display()))
        })?;
        merged.push_str(&format!("// --- package unit: {} ---\n", file.display()));
        merged.push_str(&strip_package_local_imports(&text, &stems));
        if !merged.ends_with('\n') {
            merged.push('\n');
        }
        merged.push('\n');
    }
    Ok((main_path, merged))
}

pub fn compile_project_with_options(
    root: &Path,
    options: &DriverOptions,
) -> SyxResult<CompiledProject> {
    ensure_project_lock(root, options)?;
    let mut manifest = load_manifest(root)?;
    syx_host_plugin::register_compile_plugin_manifests(root);
    syx_macro::register_package_macros(manifest.macros.as_slice());
    let mut opts = options.clone();
    if !opts.in_build_syx_task {
        if let Some(plan) = build_syx::load_build_plan(root, &manifest)? {
            build_syx::prepare_build_plan(&plan, &mut opts, &mut manifest, root)?;
        }
    }
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        opts.resolved_packages = Some(load_resolved_packages(root)?);
    }
    syx_meta::service_di_clear();
    scan_workspace_services(root, &manifest, &opts)?;
    let (entry_path, merged_source) = merge_package_unit_source(root, &manifest.main)?;
    let display = entry_path.display().to_string();
    let mut graph = ModuleGraph::default();
    let entry = compile_module_recursive(
        ModuleRef::from_inline(entry_path.clone(), display, merged_source),
        root,
        &mut graph,
        &manifest,
        &opts,
    )?;
    Ok(CompiledProject {
        manifest,
        entry,
        graph,
    })
}

pub(crate) fn compile_module_recursive(
    module_ref: ModuleRef,
    root: &Path,
    graph: &mut ModuleGraph,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<ModuleUnit> {
    let key = module_ref.key();
    if let Some(existing) = graph.modules.get(&key) {
        return Ok(existing.clone());
    }

    ensure_prelude_modules_in_graph(graph, manifest, options, root)?;

    let (key, source) = module_ref.load()?;
    let parsed = Parser::parse(source.clone())?;
    let importer = module_ref.importer_path(root);
    let importer_dir = importer.parent().unwrap_or(root);
    for decl in &parsed.decls {
        if let Decl::Import { .. } = decl {
            if let Some(path_str) = decl.import_path_str() {
                let import_ref =
                    resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
                compile_module_recursive(import_ref, root, graph, manifest, options)?;
            }
        }
    }

    let unit = compile_file_with_context(&key, &source, importer, graph, manifest, options, root)?;
    graph.order.push(key.clone());
    graph.modules.insert(key.clone(), unit.clone());
    Ok(unit)
}

fn module_dir_candidates(dir: &Path, path_str: &str) -> Vec<PathBuf> {
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    let dotted_file = path_str.replace('.', "/");
    vec![
        dir.join(format!("{path_str}.sy")),
        dir.join(format!("{dotted_file}.sy")),
        dir.join(first_segment).join("mod.sy"),
        dir.join(first_segment).join(format!("{first_segment}.sy")),
    ]
}

pub(crate) fn ensure_project_lock(root: &Path, options: &DriverOptions) -> SyxResult<()> {
    if options.skip_install_check {
        return Ok(());
    }
    if syx_pkg::requires_lockfile(root)? {
        return Err(syx_pkg::lockfile_error(root));
    }
    if !load_manifest(root)?.dependencies.is_empty() {
        load_lockfile(root)?;
    }
    Ok(())
}

fn dep_root_from_lock(pkg: &InstalledPackage, path_str: &str) -> PathBuf {
    let dep_manifest = load_manifest(&pkg.root).unwrap_or_default();
    let root = pkg.root.clone();
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    let dotted_file = path_str.replace('.', "/");
    let candidates = [
        root.join(format!("{path_str}.sy")),
        root.join(format!("{dotted_file}.sy")),
        root.join(format!("{first_segment}.sy")),
        root.join("src").join(format!("{first_segment}.sy")),
        root.join(&dep_manifest.main),
    ];
    for candidate in candidates {
        if candidate.exists() {
            return candidate.parent().unwrap_or(&root).to_path_buf();
        }
    }
    root
}

pub(crate) fn resolve_module(
    path_str: &str,
    _current: &Path,
    importer_dir: &Path,
    _manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<ModuleRef> {
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    if embedded_std::is_forbidden_import(first_segment) {
        return Err(SyxDiagnostic::error(format!(
            "Module '{path_str}' is internal to the compiler and cannot be imported"
        ))
        .with_code("M003"));
    }
    if options.enable_std {
        // Prefer on-disk monorepo `packages/` for single-file importables (e.g. Os)
        // so LSP does not serve a stale `include_str!` snapshot after stdlib edits.
        if let Some(disk) = embedded_std::find_workspace_stdlib_file(importer_dir, first_segment)
            .or_else(|| embedded_std::find_workspace_stdlib_file(project_root, first_segment))
            .or_else(|| embedded_std::find_workspace_stdlib_file(_current, first_segment))
        {
            return Ok(ModuleRef::from_file(disk));
        }
        if let Some((name, source)) = embedded_std::lookup_core(path_str) {
            return Ok(ModuleRef::Embedded {
                key: embedded_std::virtual_path(name),
                display_name: format!("<stdlib>/{name}.sy"),
                source,
            });
        }
        if let Some(embedded) = embedded_module(path_str) {
            return Ok(embedded);
        }
    }
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    if let Some(packages) = &options.resolved_packages {
        if let Some(pkg) = packages.get(first_segment) {
            let dep_root = dep_root_from_lock(pkg, path_str);
            let dep_manifest = load_manifest(&pkg.root).unwrap_or_default();
            let first_segment = path_str.split('.').next().unwrap_or(path_str);
            let dotted_file = path_str.replace('.', "/");
            let candidates = [
                dep_root.join(format!("{path_str}.sy")),
                dep_root.join(format!("{dotted_file}.sy")),
                dep_root.join(format!("{first_segment}.sy")),
                dep_root.join("src").join(format!("{first_segment}.sy")),
                dep_root.join(&dep_manifest.main),
            ];
            for candidate in candidates {
                if candidate.exists() {
                    return Ok(ModuleRef::from_file(candidate));
                }
            }
        }
    }
    // File-as-module is not supported (Swift-aligned): a bare name must resolve
    // to stdlib / lock dependencies / `--module-dir`, not a sibling `.sy` file.
    // Same-package multi-file projects are merged by `compile_project` instead.
    let same_dir = importer_dir.join(format!("{}.sy", path_str.replace('.', "/")));
    if same_dir.is_file() {
        return Err(SyxDiagnostic::error(format!(
            "Cannot import '{path_str}' as a file module; same-package `.sy` files share one namespace without import (use `syx.toml` project, or a path dependency)"
        ))
        .with_code("M010"));
    }
    let mut candidates = Vec::new();
    for dir in &options.module_dirs {
        candidates.extend(module_dir_candidates(dir, path_str));
    }
    for candidate in candidates {
        if candidate.exists() {
            return Ok(ModuleRef::from_file(candidate));
        }
    }
    Err(SyxDiagnostic::error(format!("Module '{path_str}' not found")).with_code("M001"))
}
