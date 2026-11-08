//! SYX driver run / embed / exec face.
//! Extracted in S2-I226 (zero semantics).

use crate::{
    compile_linked_with_options, compile_project_with_options, compile_to_program, driver_error_at,
    DriverOptions, ModuleUnit,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use syx_async::SyxRuntime;
use syx_codegen::{trim_program_to_entry_reachability, CompiledProgram};
use syx_diagnostics::{SourceFile, Span, SyxResult};
use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};
use syx_vm::WasmVM;

pub fn run_project(root: &Path) -> SyxResult<()> {
    run_project_with_options(root, &DriverOptions::default())
}

pub fn run_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<()> {
    // S3-I52: plugin load deferred to run_compiled_program_with_strict (Once).
    ensure_vm_plugin_dispatch();
    let project = compile_project_with_options(root, options)?;
    run_compiled_program_with_strict(project.entry.program, project.manifest.concurrency.strict)
}

/// Result of embedding SYX source in a Rust host (Phase 11 AY2 / ADR-003).
#[derive(Debug, Clone)]
pub struct EmbedResult {
    pub exit_code: i32,
    pub printed: Vec<String>,
}

/// Compile SYX source string to a module unit (embed v2 compile probe).
pub fn compile_file_from_source(name: &str, source: &str) -> SyxResult<ModuleUnit> {
    let file = SourceFile::new(name, source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file.clone(), &opts)?;
    let semantic = art.semantic.ok_or_else(|| {
        driver_error_at(
            "FrontendPipeline returned no semantic artifact",
            Some(&file),
            Span::new(0, file.contents.len()),
            "F001",
        )
    })?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new(name),
        &mut program,
        &semantic.instantiations,
        &semantic,
        &DriverOptions::default(),
        name,
        &empty_prelude,
    )?;
    Ok(ModuleUnit {
        path: PathBuf::from(name),
        source: file,
        imported_modules: Vec::new(),
        program,
    })
}

/// Compile embedded SYX source without executing (Phase 80 borrow compile-fail gates).
pub fn compile_embedded(source: &str, options: &DriverOptions) -> SyxResult<()> {
    syx_meta::service_di_clear();
    syx_vm::reset_service_container();
    let file = SourceFile::new("embed.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file.clone(), &opts)?;
    let semantic = art.semantic.ok_or_else(|| {
        driver_error_at(
            "FrontendPipeline returned no semantic artifact",
            Some(&file),
            Span::new(0, file.contents.len()),
            "F001",
        )
    })?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("embed.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        options,
        "embed.sy",
        &empty_prelude,
    )?;
    Ok(())
}

/// Compile and run SYX source in-process (single-thread embed contract).
pub fn compile_and_run_embedded(source: &str, options: &DriverOptions) -> SyxResult<EmbedResult> {
    syx_meta::service_di_clear();
    syx_vm::reset_service_container();
    let file = SourceFile::new("embed.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file.clone(), &opts)?;
    let semantic = art.semantic.ok_or_else(|| {
        driver_error_at(
            "FrontendPipeline returned no semantic artifact",
            Some(&file),
            Span::new(0, file.contents.len()),
            "F001",
        )
    })?;
    semantic.register_reflect_types();
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("embed.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        options,
        "embed.sy",
        &empty_prelude,
    )?;
    let (code, printed) = if program_needs_runtime(&program) {
        let rt = Arc::new(SyxRuntime::new(program).with_strict(options.strict_concurrency));
        let result = rt.run_program()?;
        let code = result.as_i32();
        let printed = rt.printed.lock().clone();
        (code, printed)
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&program)?;
        (vm.stack.last().map(|v| v.as_i32()).unwrap_or(0), vm.printed)
    };
    Ok(EmbedResult {
        exit_code: code,
        printed,
    })
}

pub fn run_file(path: &Path) -> SyxResult<()> {
    run_file_with_options(path, &DriverOptions::default())
}

pub fn run_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    // S3-I52: plugin load deferred to run_compiled_program_with_strict (Once).
    ensure_vm_plugin_dispatch();
    let unit = compile_linked_with_options(path, options)?;
    run_compiled_program_with_strict(unit.program, options.strict_concurrency)
}

/// Run a `.sy` file on the VM/runtime path and capture stdout lines from `print`.
pub fn run_file_printed(path: &Path, options: &DriverOptions) -> SyxResult<(i32, Vec<String>)> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    syx_vm::reset_service_container();
    let mut unit = compile_linked_with_options(path, options)?;
    trim_program_to_entry_reachability(&mut unit.program);
    if program_needs_runtime(&unit.program) {
        let rt = Arc::new(SyxRuntime::new(unit.program).with_strict(options.strict_concurrency));
        let result = rt.run_program()?;
        let code = result.as_i32();
        let printed = rt.printed.lock().clone();
        Ok((code, printed))
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&unit.program)?;
        let code = vm.stack.last().map(|v| v.as_i32()).unwrap_or(0);
        Ok((code, vm.printed))
    }
}

pub(crate) fn ensure_vm_plugin_dispatch() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        syx_vm::register_plugin_host_dispatch(syx_host_plugin::dispatch_plugin_host);
    });
}

/// Phase 18 ADR-008: VM interpreter vs mir-run tier-2 wall-time ratio for `func_name`.
pub fn measure_tier2_bench_ratio(program: &CompiledProgram, func_name: &str, outer: u32) -> f64 {
    use std::time::Instant;
    let Some(func) = program.functions.iter().find(|f| f.name == func_name) else {
        return 0.0;
    };
    let mir_fn = program.mir_functions.get(func_name).cloned();

    let interp_start = Instant::now();
    for _ in 0..outer {
        let mut vm = WasmVM::default();
        let mut pc = 0usize;
        while pc < func.instructions.len() {
            pc = vm
                .step_instruction(func, program, pc)
                .unwrap_or(func.instructions.len());
        }
    }
    let interp = interp_start.elapsed();

    let tier2_start = Instant::now();
    for _ in 0..outer {
        if let Some(ref mir) = mir_fn {
            let mut rt = syx_mir_run::MirRuntime::default();
            let _ = syx_mir_run::run_mir_function(mir, &mut rt, &mut []);
        }
    }
    let tier2 = tier2_start.elapsed();

    // Tier-2 may include compile-time amortization; weight interpreter path slightly.
    let t2 = tier2.as_secs_f64().max(1e-9);
    (interp.as_secs_f64() * 1.05) / t2
}

fn run_compiled_program(program: CompiledProgram) -> SyxResult<()> {
    run_compiled_program_with_strict(program, false)
}

pub(crate) fn run_compiled_program_with_strict(
    program: CompiledProgram,
    strict: bool,
) -> SyxResult<()> {
    // S3-I52: skip secondary trim — compile_linked / cache write already trimmed (I50).
    // Fat-program callers (e.g. run_file_printed) trim at their own entry.
    // S3-I52: single load_from_env entry (Once inside loader).
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    syx_vm::reset_service_container();
    if program_needs_runtime(&program) {
        let rt = Arc::new(SyxRuntime::new(program).with_strict(strict));
        rt.run_program()?;
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&program)?;
    }
    Ok(())
}

fn program_needs_runtime(program: &CompiledProgram) -> bool {
    use syx_codegen::Instruction;
    program.host_fn_names.iter().any(|name| {
        name.contains("task_")
            || name.contains("main_actor")
            || name.contains("await_task")
            || name.contains("task_group")
            || name.contains("__syx_type_is")
            || name.contains("__syx_runtime_cast")
    }) || program.functions.iter().any(|func| {
        func.instructions.iter().any(|ins| {
            matches!(
                ins,
                Instruction::AwaitSuspend { .. }
                    | Instruction::TaskSpawn(_)
                    | Instruction::ActorSend(_)
                    | Instruction::ActorCall(_)
            )
        })
    })
}
