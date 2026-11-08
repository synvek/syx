//! SYX driver WASM / native-MIR run face.
//! Extracted in S2-I229 (zero semantics).

use crate::cache_fingerprint::driver_options_with_deps;
use crate::module_ref::ModuleRef;
use crate::{
    build_unified_mir, compile_module_recursive, compile_project_with_options, driver_error_at,
    ensure_project_lock, DriverOptions, ModuleGraph,
};
use std::path::Path;
use syx_diagnostics::{Span, SyxResult};
use syx_pkg::{load_manifest, load_resolved_packages};

pub fn run_wasm_bytes(wasm: &[u8], entry: &str) -> SyxResult<i32> {
    syx_wasm_run::run_wasm(wasm, entry)
}

pub fn run_wasm_bytes_with_module(
    wasm: &[u8],
    entry: &str,
    module: &syx_mir::MirModule,
) -> SyxResult<i32> {
    syx_wasm_run::run_wasm_with_module(wasm, entry, Some(module))
}

pub fn run_wasm_file(path: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_file_with_options(path, entry, &DriverOptions::default())
}

pub fn run_wasm_file_with_options(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let wasm = std::fs::read(path).map_err(|e| {
        driver_error_at(
            format!("Failed to read {}: {e}", path.display()),
            None,
            Span::DUMMY,
            "D001",
        )
    })?;
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        let syx = path.with_file_name(stem).with_extension("sy");
        if syx.exists() {
            let syx_path = syx.clone();
            let parent = syx_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            syx_host_plugin::register_compile_plugin_manifests(&parent);
            let manifest = load_manifest(&parent).unwrap_or_default();
            let mut opts = options.clone();
            if !manifest.dependencies.is_empty() {
                ensure_project_lock(&parent, options)?;
                opts.resolved_packages = Some(load_resolved_packages(&parent)?);
            }
            let mut graph = ModuleGraph::default();
            let unit = compile_module_recursive(
                ModuleRef::from_file(syx_path),
                &parent,
                &mut graph,
                &manifest,
                &opts,
            )?;
            let mir = build_unified_mir(&graph, &unit, &parent, &opts, &manifest)?;
            return run_wasm_bytes_with_module(&wasm, entry, &mir);
        }
    }
    run_wasm_bytes(&wasm, entry)
}

pub fn run_wasm_project(root: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_project_with_options(root, entry, &DriverOptions::default())
}

pub fn run_wasm_project_with_options(
    root: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let project = compile_project_with_options(root, options)?;
    let opts = driver_options_with_deps(root, &project.manifest, options)?;
    let mir = build_unified_mir(
        &project.graph,
        &project.entry,
        root,
        &opts,
        &project.manifest,
    )?;
    let wasm = syx_wasm_emit::emit_wasm(&mir)?;
    run_wasm_bytes_with_module(&wasm, entry, &mir)
}

pub fn run_wasm_source(path: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_source_with_options(path, entry, &DriverOptions::default())
}

pub fn run_wasm_source_with_options(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
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
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let wasm = syx_wasm_emit::emit_wasm_with_options(&mir, opts.wasm_aot)?;
    run_wasm_bytes_with_module(&wasm, entry, &mir)
}

pub fn run_wasm_source_printed(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<(i32, Vec<String>)> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
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
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let wasm = syx_wasm_emit::emit_wasm_with_options(&mir, opts.wasm_aot)?;
    syx_vm::set_program_ctx(syx_vm::ProgramCtx {
        program: std::sync::Arc::new(unit.program.clone()),
        runtime: None,
    });
    let result = syx_wasm_run::run_wasm_capture(&wasm, entry, Some(&mir));
    syx_vm::clear_program_ctx();
    let result = result?;
    Ok((result.exit_code, result.printed))
}

/// Run a `.sy` file via the native MIR interpreter (actor/async parity until Cranelift lowers actors).
pub fn run_native_mir_file_printed(
    path: &Path,
    options: &DriverOptions,
) -> SyxResult<(i32, Vec<String>)> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
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
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let ctx = syx_rt::install_native_module(&mir);
    let mut rt = syx_mir_run::MirRuntime::new();
    rt.install_functions(mir.functions.clone());
    rt.set_runtime(syx_rt::native_vm_runtime_for(ctx));
    syx_mir_run::install_module_string_pool(&mut rt, &mir);
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .ok_or_else(|| {
            driver_error_at("No main function in MIR module", None, Span::DUMMY, "R001")
        })?;
    syx_vm::set_program_ctx(syx_vm::ProgramCtx {
        program: std::sync::Arc::new(unit.program.clone()),
        runtime: None,
    });
    syx_vm::set_mir_closure_executor(syx_mir_run::execute_mir_closure);
    syx_vm::set_mir_closure_preferred(true);
    syx_mir_run::pin_active_mir_runtime(&mut rt);
    let code = syx_mir_run::run_mir_function(main, &mut rt, &mut [])?;
    syx_mir_run::unpin_active_mir_runtime();
    syx_vm::clear_mir_closure_executor();
    syx_vm::clear_program_ctx();
    Ok((code, rt.printed))
}
