//! `syx emit` — bytecode / IR text and binary export.

use std::path::{Path, PathBuf};

use syx_codegen::format_compiled_program;
use syx_diagnostics::SyxResult;
use syx_mir::format_mir_module;

use crate::{
    build_unified_mir, compile_file_with_options, compile_module_recursive,
    compile_project_with_options, ensure_project_lock, load_manifest, load_resolved_packages,
    DriverOptions, ModuleGraph, ModuleRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitFormat {
    Syxbc,
    Mir,
    Wasm,
    Wat,
    Llvm,
    Clif,
    All,
}

impl EmitFormat {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "syxbc" => Some(Self::Syxbc),
            "mir" => Some(Self::Mir),
            "wasm" => Some(Self::Wasm),
            "wat" => Some(Self::Wat),
            "llvm" => Some(Self::Llvm),
            "clif" => Some(Self::Clif),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    pub fn default_extension(self) -> &'static str {
        match self {
            Self::Syxbc => "syxbc",
            Self::Mir => "mir",
            Self::Wasm => "wasm",
            Self::Wat => "wat",
            Self::Llvm => "ll",
            Self::Clif => "clif",
            Self::All => "",
        }
    }
}

#[derive(Debug, Default)]
pub struct EmitArtifacts {
    pub syxbc: Option<String>,
    pub mir: Option<String>,
    pub wasm: Option<Vec<u8>>,
    pub wat: Option<String>,
    pub llvm: Option<String>,
    pub clif: Option<String>,
}

fn build_mir_for_path(path: &Path, options: &DriverOptions) -> SyxResult<syx_mir::MirModule> {
    if path.is_dir() {
        let project = compile_project_with_options(path, options)?;
        return build_unified_mir(
            &project.graph,
            &project.entry,
            path,
            options,
            &project.manifest,
        );
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = if path.extension().and_then(|e| e.to_str()) == Some("sy") {
        compile_module_recursive(
            ModuleRef::from_file(path.to_path_buf()),
            parent,
            &mut graph,
            &manifest,
            &opts,
        )?
    } else {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "emit requires a project directory or .sy file",
        ));
    };
    build_unified_mir(&graph, &unit, parent, &opts, &manifest)
}

pub fn emit_syxbc_text(path: &Path, options: &DriverOptions) -> SyxResult<String> {
    let unit = if path.is_dir() {
        compile_project_with_options(path, options)?.entry
    } else {
        compile_file_with_options(path, options)?
    };
    Ok(format_compiled_program(&unit.program))
}

pub fn emit_mir_text(path: &Path, options: &DriverOptions) -> SyxResult<String> {
    let mir = build_mir_for_path(path, options)?;
    Ok(format_mir_module(&mir))
}

/// Lower a source file to MIR (for wasm-run tests that must pass `Some(&mir)`).
pub fn emit_mir_for_path(path: &Path, options: &DriverOptions) -> SyxResult<syx_mir::MirModule> {
    build_mir_for_path(path, options)
}

pub fn emit_wasm_bytes(path: &Path, options: &DriverOptions) -> SyxResult<Vec<u8>> {
    let mir = build_mir_for_path(path, options)?;
    syx_wasm_emit::emit_wasm_with_options(&mir, options.wasm_aot)
}

pub fn emit_wat_text(path: &Path, options: &DriverOptions) -> SyxResult<String> {
    let mir = build_mir_for_path(path, options)?;
    syx_wasm_emit::emit_wat(&mir)
}

pub fn emit_llvm_text(path: &Path, options: &DriverOptions) -> SyxResult<String> {
    let mir = build_mir_for_path(path, options)?;
    syx_llvm::emit_llvm_ir_with_gate(&mir)
}

pub fn emit_clif_text(path: &Path, options: &DriverOptions) -> SyxResult<String> {
    let mir = build_mir_for_path(path, options)?;
    Ok(crate::emit_clif_module_text(&mir))
}

pub fn emit_with_options(
    path: &Path,
    format: EmitFormat,
    options: &DriverOptions,
) -> SyxResult<EmitArtifacts> {
    let mut out = EmitArtifacts::default();
    match format {
        EmitFormat::Syxbc => out.syxbc = Some(emit_syxbc_text(path, options)?),
        EmitFormat::Mir => out.mir = Some(emit_mir_text(path, options)?),
        EmitFormat::Wasm => out.wasm = Some(emit_wasm_bytes(path, options)?),
        EmitFormat::Wat => out.wat = Some(emit_wat_text(path, options)?),
        EmitFormat::Llvm => out.llvm = Some(emit_llvm_text(path, options)?),
        EmitFormat::Clif => out.clif = Some(emit_clif_text(path, options)?),
        EmitFormat::All => {
            out.syxbc = Some(emit_syxbc_text(path, options)?);
            out.mir = Some(emit_mir_text(path, options)?);
            out.wasm = Some(emit_wasm_bytes(path, options)?);
            out.wat = Some(emit_wat_text(path, options)?);
            out.llvm = Some(emit_llvm_text(path, options)?);
            out.clif = Some(emit_clif_text(path, options)?);
        }
    }
    Ok(out)
}

/// Write emit artifacts to `dir` using `stem` as the file basename (e.g. `main`).
pub fn write_emit_all(dir: &Path, stem: &str, artifacts: &EmitArtifacts) -> SyxResult<()> {
    std::fs::create_dir_all(dir).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("Failed to create {}: {e}", dir.display()))
    })?;
    if let Some(s) = &artifacts.syxbc {
        write_text_file(&dir.join(format!("{stem}.syxbc")), s)?;
    }
    if let Some(s) = &artifacts.mir {
        write_text_file(&dir.join(format!("{stem}.mir")), s)?;
    }
    if let Some(b) = &artifacts.wasm {
        std::fs::write(dir.join(format!("{stem}.wasm")), b).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("Failed to write wasm: {e}"))
        })?;
    }
    if let Some(s) = &artifacts.wat {
        write_text_file(&dir.join(format!("{stem}.wat")), s)?;
    }
    if let Some(s) = &artifacts.llvm {
        write_text_file(&dir.join(format!("{stem}.ll")), s)?;
    }
    if let Some(s) = &artifacts.clif {
        write_text_file(&dir.join(format!("{stem}.clif")), s)?;
    }
    Ok(())
}

fn write_text_file(path: &Path, contents: &str) -> SyxResult<()> {
    std::fs::write(path, contents).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("Failed to write {}: {e}", path.display()))
    })
}

pub fn default_emit_stem(path: &Path) -> String {
    if path.is_dir() {
        "main".to_string()
    } else {
        path.file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("main"))
            .to_string_lossy()
            .into_owned()
    }
}

pub fn default_emit_output(path: &Path, format: EmitFormat) -> PathBuf {
    let stem = default_emit_stem(path);
    if path.is_dir() {
        path.join(format!("{stem}.{}", format.default_extension()))
    } else {
        path.with_file_name(format!("{stem}.{}", format.default_extension()))
    }
}
