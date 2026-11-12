//! Phase 63 unified compile pipeline — single `build_unified_mir` for VM bytecode and emit paths.

use std::path::Path;

use indexmap::IndexSet;
use smol_str::SmolStr;
use syx_ast::Module;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_diagnostics::SyxResult;
use syx_mir::MirModule;
use syx_mono::monomorphize_module;
use syx_semantic::SemanticAnalyzer;

use crate::{
    build_unified_mir, inject_virtual_prelude_imports, ModuleGraph, ModuleUnit, SyxManifest,
};

/// Compiled artifact shared by VM, WASM, and native backends.
pub struct UnifiedCompileArtifact {
    pub program: CompiledProgram,
    pub mir: MirModule,
}

/// Lower the module graph once, then emit bytecode via MIR hybrid codegen.
pub fn compile_entry_with_unified_mir(
    graph: &ModuleGraph,
    module_key: &Path,
    source_name: &str,
    compile_module: &Module,
    program: &mut CompiledProgram,
    semantic: &SemanticAnalyzer,
    options: &crate::DriverOptions,
    prelude_extensions: &IndexSet<(SmolStr, SmolStr)>,
    manifest: &SyxManifest,
    project_root: &Path,
    entry_source: &syx_diagnostics::SourceFile,
) -> SyxResult<MirModule> {
    let entry_unit = ModuleUnit {
        path: module_key.to_path_buf(),
        source: entry_source.clone(),
        imported_modules: semantic.imported_modules.clone(),
        program: program.clone(),
    };
    let mir = build_unified_mir(graph, &entry_unit, project_root, options, manifest)?;
    // Interpret fastpath: pull only MIR-referenced stdlib fns (with Call remap).
    if !crate::mir_unify::want_stdlib_mir(options) {
        let seeds = crate::stdlib_fastpath::mir_call_names(&mir);
        crate::stdlib_fastpath::link_stdlib_callees(program, graph, &seeds);
    }
    let mut mono_module = monomorphize_module(compile_module, &semantic.instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    // R-36: interpreter R003 / C020 Display need SourceFile for file:line:col.
    program.debug_source = Some(entry_source.clone());
    CodeGenerator::compile_with_mir(
        &mono_module,
        &mir,
        program,
        Some(semantic),
        Some(source_name),
        Some(prelude_extensions),
    )?;
    // Second link pass: user bytecode may resolve new Call names after emit stubs.
    if !crate::mir_unify::want_stdlib_mir(options) {
        let mut seeds = crate::stdlib_fastpath::mir_call_names(&mir);
        for f in &program.functions {
            seeds.insert(f.name.clone());
            for ins in &f.instructions {
                if let syx_codegen::Instruction::ConstStr(s) = ins {
                    seeds.insert(SmolStr::new(s.as_str()));
                }
            }
        }
        crate::stdlib_fastpath::link_stdlib_callees(program, graph, &seeds);
    }
    Ok(mir)
}
