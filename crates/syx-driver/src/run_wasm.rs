//! SYX driver WASM / native-MIR run face.
//! Extracted in S2-I229 (zero semantics).

use crate::cache_fingerprint::{cache_version_tag, driver_options_with_deps, hash_source};
use crate::driver_options::project_cache_dir;
use crate::module_ref::ModuleRef;
use crate::{
    build_unified_mir, compile_module_recursive, compile_project_with_options, driver_error_at,
    ensure_project_lock, DriverOptions, ModuleGraph, ModuleUnit,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use syx_diagnostics::{Span, SyxResult};
use syx_mir::MirModule;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WasmEntryCache {
    cache_version_tag: String,
    source_hash: String,
    wasm_aot: bool,
    mir: MirModule,
    wasm: Vec<u8>,
    program: syx_codegen::CompiledProgram,
}

/// Sorted `import …` lines from entry source (plan: import keys in cache fingerprint).
fn sorted_import_keys(source: &str) -> String {
    let mut keys: Vec<&str> = source
        .lines()
        .filter_map(|line| {
            let t = line.trim();
            t.strip_prefix("import ").map(|rest| rest.trim())
        })
        .collect();
    keys.sort_unstable();
    keys.join("\n")
}

fn wasm_entry_cache_path(root: &Path, path: &Path, source: &str, wasm_aot: bool) -> PathBuf {
    let mut key = String::new();
    key.push_str(&cache_version_tag());
    key.push('\n');
    key.push_str(&hash_source(source));
    key.push('\n');
    key.push_str(&sorted_import_keys(source));
    key.push('\n');
    key.push_str(path.to_string_lossy().as_ref());
    key.push('\n');
    key.push_str(if wasm_aot { "aot1" } else { "aot0" });
    let digest = hash_source(&key);
    project_cache_dir(root).join(format!("wasm_{digest}.bin"))
}

/// Process-local last-entry cache (daemon multi-run without `--with-cache`).
struct WasmProcessEntry {
    path: PathBuf,
    source_hash: String,
    import_keys: String,
    wasm_aot: bool,
    tag: String,
    program: syx_codegen::CompiledProgram,
    mir: MirModule,
    wasm: Vec<u8>,
}

fn wasm_process_cache() -> &'static Mutex<Option<WasmProcessEntry>> {
    static CACHE: OnceLock<Mutex<Option<WasmProcessEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

fn try_load_wasm_process(
    path: &Path,
    source: &str,
    wasm_aot: bool,
) -> Option<(syx_codegen::CompiledProgram, MirModule, Vec<u8>)> {
    let guard = wasm_process_cache().lock().ok()?;
    let e = guard.as_ref()?;
    if e.path == path
        && e.source_hash == hash_source(source)
        && e.import_keys == sorted_import_keys(source)
        && e.wasm_aot == wasm_aot
        && e.tag == cache_version_tag()
        && !e.wasm.is_empty()
    {
        return Some((e.program.clone(), e.mir.clone(), e.wasm.clone()));
    }
    None
}

fn store_wasm_process(
    path: &Path,
    source: &str,
    wasm_aot: bool,
    program: &syx_codegen::CompiledProgram,
    mir: &MirModule,
    wasm: &[u8],
) {
    if let Ok(mut guard) = wasm_process_cache().lock() {
        *guard = Some(WasmProcessEntry {
            path: path.to_path_buf(),
            source_hash: hash_source(source),
            import_keys: sorted_import_keys(source),
            wasm_aot,
            tag: cache_version_tag(),
            program: program.clone(),
            mir: mir.clone(),
            wasm: wasm.to_vec(),
        });
    }
}

fn try_load_wasm_entry_cache(
    root: &Path,
    path: &Path,
    source: &str,
    wasm_aot: bool,
) -> Option<(syx_codegen::CompiledProgram, MirModule, Vec<u8>)> {
    let cache_path = wasm_entry_cache_path(root, path, source, wasm_aot);
    let bytes = std::fs::read(&cache_path).ok()?;
    let cached: WasmEntryCache = bincode::deserialize(&bytes).ok()?;
    if cached.cache_version_tag != cache_version_tag()
        || cached.source_hash != hash_source(source)
        || cached.wasm_aot != wasm_aot
        || cached.wasm.is_empty()
    {
        return None;
    }
    Some((cached.program, cached.mir, cached.wasm))
}

fn store_wasm_entry_cache(
    root: &Path,
    path: &Path,
    source: &str,
    wasm_aot: bool,
    program: &syx_codegen::CompiledProgram,
    mir: &MirModule,
    wasm: &[u8],
) {
    let cache_path = wasm_entry_cache_path(root, path, source, wasm_aot);
    if let Some(parent) = cache_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let cached = WasmEntryCache {
        cache_version_tag: cache_version_tag(),
        source_hash: hash_source(source),
        wasm_aot,
        mir: mir.clone(),
        wasm: wasm.to_vec(),
        program: program.clone(),
    };
    if let Ok(bytes) = bincode::serialize(&cached) {
        let _ = std::fs::write(cache_path, bytes);
    }
}

fn compile_wasm_artifacts(
    path: &Path,
    parent: &Path,
    options: &DriverOptions,
) -> SyxResult<(ModuleUnit, MirModule, Vec<u8>)> {
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let source = std::fs::read_to_string(path).unwrap_or_default();
    // Process → disk (`--with-cache`) → compile+emit.
    if let Some((program, mir, wasm)) = try_load_wasm_process(path, &source, opts.wasm_aot) {
        let unit = ModuleUnit {
            path: path.to_path_buf(),
            source: syx_diagnostics::SourceFile::new(path.display().to_string(), source),
            imported_modules: Default::default(),
            program,
        };
        return Ok((unit, mir, wasm));
    }
    if opts.incremental_cache {
        if let Some((program, mir, wasm)) =
            try_load_wasm_entry_cache(parent, path, &source, opts.wasm_aot)
        {
            store_wasm_process(path, &source, opts.wasm_aot, &program, &mir, &wasm);
            let unit = ModuleUnit {
                path: path.to_path_buf(),
                source: syx_diagnostics::SourceFile::new(path.display().to_string(), source),
                imported_modules: Default::default(),
                program,
            };
            return Ok((unit, mir, wasm));
        }
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
    store_wasm_process(
        path,
        &unit.source.contents,
        opts.wasm_aot,
        &unit.program,
        &mir,
        &wasm,
    );
    if opts.incremental_cache {
        store_wasm_entry_cache(
            parent,
            path,
            &unit.source.contents,
            opts.wasm_aot,
            &unit.program,
            &mir,
            &wasm,
        );
    }
    Ok((unit, mir, wasm))
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
            let parent = syx.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            syx_host_plugin::register_compile_plugin_manifests(&parent);
            let (_unit, mir, _) = compile_wasm_artifacts(&syx, &parent, options)?;
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
    let main_path = project.entry.path.clone();
    let source = std::fs::read_to_string(&main_path).unwrap_or_default();
    if let Some((_program, mir, wasm)) = try_load_wasm_process(&main_path, &source, opts.wasm_aot) {
        return run_wasm_bytes_with_module(&wasm, entry, &mir);
    }
    if opts.incremental_cache {
        if let Some((program, mir, wasm)) =
            try_load_wasm_entry_cache(root, &main_path, &source, opts.wasm_aot)
        {
            store_wasm_process(&main_path, &source, opts.wasm_aot, &program, &mir, &wasm);
            return run_wasm_bytes_with_module(&wasm, entry, &mir);
        }
    }
    let mir = build_unified_mir(
        &project.graph,
        &project.entry,
        root,
        &opts,
        &project.manifest,
    )?;
    let wasm = syx_wasm_emit::emit_wasm_with_options(&mir, opts.wasm_aot)?;
    store_wasm_process(
        &main_path,
        &source,
        opts.wasm_aot,
        &project.entry.program,
        &mir,
        &wasm,
    );
    if opts.incremental_cache {
        store_wasm_entry_cache(
            root,
            &main_path,
            &source,
            opts.wasm_aot,
            &project.entry.program,
            &mir,
            &wasm,
        );
    }
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
    let (_unit, mir, wasm) = compile_wasm_artifacts(path, parent, options)?;
    run_wasm_bytes_with_module(&wasm, entry, &mir)
}

pub fn run_wasm_source_printed(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<(i32, Vec<String>)> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let (unit, mir, wasm) = compile_wasm_artifacts(path, parent, options)?;
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
