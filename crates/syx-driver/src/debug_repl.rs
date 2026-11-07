//! SYX driver debug / REPL face.
//! Extracted in S2-I232 (zero semantics).

use crate::{
    compile_file_with_options, compile_project_with_options, compile_to_program, DriverOptions,
};
use std::path::Path;
use syx_codegen::CompiledProgram;
use syx_debug::DebugSession;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};
use syx_vm::WasmVM;

pub fn debug_project(root: &Path) -> SyxResult<DebugSession> {
    debug_project_with_options(root, &DriverOptions::default())
}

pub fn debug_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    let project = compile_project_with_options(root, options)?;
    let text = std::fs::read_to_string(&project.entry.path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read entry: {e}")))?;
    let source = SourceFile::new(project.entry.path.display().to_string(), text);
    Ok(DebugSession::new(source, project.entry.program))
}

pub fn debug_file(path: &Path) -> SyxResult<DebugSession> {
    debug_file_with_options(path, &DriverOptions::default())
}

pub fn debug_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let text = std::fs::read_to_string(&path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let source = SourceFile::new(path.display().to_string(), text);
    let unit = compile_file_with_options(&path, options)?;
    Ok(DebugSession::new(source, unit.program))
}

pub fn debug_path(path: &Path) -> SyxResult<DebugSession> {
    debug_path_with_options(path, &DriverOptions::default())
}

pub fn debug_path_with_options(path: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    if path.is_dir() {
        debug_project_with_options(path, options)
    } else {
        debug_file_with_options(path, options)
    }
}

pub fn debug_interactive(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    let mut session = debug_path_with_options(path, options)?;
    syx_debug::run_interactive_session(&mut session)
}

pub fn debug_dap(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    match debug_path_with_options(path, options) {
        Ok(mut session) => syx_debug::run_dap_server(&mut session),
        Err(err) => {
            // Compile/parse failure before DAP handshake used to exit silently in VS Code.
            // Stay on the DAP wire long enough to report the error in the Debug Console.
            syx_debug::run_dap_compile_failed(&err)
        }
    }
}

pub fn repl_eval(source: &str) -> SyxResult<()> {
    let file = SourceFile::new("repl.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Mir,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts)?;
    let semantic = art
        .semantic
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no semantic artifact"))?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("repl.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        &DriverOptions::default(),
        "repl.sy",
        &empty_prelude,
    )?;
    let mut vm = WasmVM::default();
    vm.run_program(&program)?;
    Ok(())
}
