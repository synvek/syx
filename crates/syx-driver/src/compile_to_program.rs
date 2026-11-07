//! SYX driver compile_to_program / mono merge face.
//! Extracted in S2-I230 (zero semantics).

use crate::{backfill_mono_witness_cache, DriverOptions, ModuleGraph};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::Path;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_frontend::expand_macros;
use syx_mir::{lower_module, optimize};
use syx_mono::monomorphize_module;
use syx_parser::Parser;
use syx_pkg::SyxManifest;
use syx_semantic::{SemanticAnalyzer, TypeId};

fn use_mir_pipeline() -> bool {
    true
}

fn use_mir_for_module(_module_key: &Path, _options: &DriverOptions) -> bool {
    use_mir_pipeline()
}

/// Span-start → `NumericKind` for unsuffixed literals contextual-typed by semantic.
pub(crate) fn contextual_numeric_kinds_from_semantic(
    semantic: &SemanticAnalyzer,
) -> indexmap::IndexMap<u32, syx_stdlib::NumericKind> {
    use syx_stdlib::{is_float, is_integer};
    semantic
        .expr_types
        .iter()
        .filter_map(|(span, ty)| match ty {
            TypeId::Numeric(kind) if is_integer(*kind) || is_float(*kind) => Some((*span, *kind)),
            _ => None,
        })
        .collect()
}

pub(crate) fn compile_to_program(
    module: &syx_ast::Module,
    module_key: &Path,
    program: &mut CompiledProgram,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    semantic: &SemanticAnalyzer,
    options: &DriverOptions,
    source_name: &str,
    prelude_extensions: &indexmap::IndexSet<(SmolStr, SmolStr)>,
) -> SyxResult<()> {
    let mut mono_module = monomorphize_module(module, instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    // W477 / ADR-821: MIR-only production path (`use_mir_pipeline()` is always true).
    // AST `compile_into_with_semantic` else-arm cleared; API retained in syx-codegen.
    debug_assert!(
        use_mir_for_module(module_key, options),
        "W477 expects MIR pipeline for compile_to_program"
    );
    let int_kinds = contextual_numeric_kinds_from_semantic(semantic);
    let mut mir = syx_mir::lower_module_with_extra_extensions_and_kinds(
        &mono_module,
        prelude_extensions,
        &program.functions.iter().map(|f| f.name.clone()).collect(),
        &indexmap::IndexSet::new(),
        &int_kinds,
    )?;
    optimize(&mut mir);
    backfill_mono_witness_cache(instantiations, &mir);
    CodeGenerator::compile_with_mir(
        &mono_module,
        &mir,
        program,
        Some(semantic),
        Some(source_name),
        Some(prelude_extensions),
    )?;
    Ok(())
}

pub(crate) fn inject_virtual_prelude_imports(module: &mut syx_ast::Module, options: &DriverOptions) {
    if !options.enable_std {
        return;
    }
    use std::collections::HashSet;
    let existing: HashSet<SmolStr> = module
        .decls
        .iter()
        .filter_map(|d| d.import_symbol())
        .cloned()
        .collect();
    for &name in syx_config::CORE_PRELUDE_MODULES {
        let sym = SmolStr::new(name);
        if existing.contains(&sym) {
            continue;
        }
        module.decls.insert(
            0,
            syx_ast::Decl::Import {
                span: syx_diagnostics::Span::DUMMY,
                path: vec![sym],
                alias: None,
            },
        );
    }
}

/// Merge monomorphized stdlib instances (e.g. `Array_len_Int`) referenced by `instantiations`.
pub(crate) fn merge_stdlib_mono_instances(
    graph: &ModuleGraph,
    program: &mut CompiledProgram,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
) -> SyxResult<()> {
    if instantiations.is_empty() {
        return Ok(());
    }
    for dep in graph.modules.values() {
        if !dep.path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let relevant = instantiations.iter().any(|(name, _)| {
            let n = name.as_str();
            n == module_stem || n.starts_with(&format!("{module_stem}_"))
        });
        if !relevant {
            continue;
        }
        let dep_module = expand_macros(Parser::parse(dep.source.clone())?)?;
        let mut dep_mono = monomorphize_module(&dep_module, instantiations);
        inject_virtual_prelude_imports(&mut dep_mono, options);
        let known: std::collections::HashSet<_> = dep
            .program
            .functions
            .iter()
            .map(|f| f.name.clone())
            .collect();
        // W477 / ADR-821: stdlib mono merge via MIR + compile_with_mir (≠ delete compile_into API).
        let mut mono_program = CompiledProgram::default();
        let mut dep_mir = lower_module(&dep_mono)?;
        optimize(&mut dep_mir);
        CodeGenerator::compile_with_mir(&dep_mono, &dep_mir, &mut mono_program, None, None, None)?;
        for func in mono_program.functions {
            if !known.contains(&func.name) && !program.functions.iter().any(|f| f.name == func.name)
            {
                program.functions.push(func);
            }
        }
    }
    Ok(())
}

pub(crate) fn collect_main_actor_await_errors(
    module: &syx_ast::Module,
    imports: &[(SmolStr, IndexMap<SmolStr, syx_semantic::TypeId>)],
    manifest: &SyxManifest,
) -> Vec<SyxDiagnostic> {
    let analyzer = SemanticAnalyzer::analyze_collecting_with_options(
        module,
        imports,
        manifest.concurrency.strict,
        manifest
            .concurrency
            .default_isolation
            .as_ref()
            .map(|s| SmolStr::new(s.as_str())),
        &[],
        None,
        &IndexMap::new(),
        false,
    );
    analyzer
        .errors
        .into_iter()
        .filter(|e| e.code.as_deref() == Some("S020"))
        .collect()
}
