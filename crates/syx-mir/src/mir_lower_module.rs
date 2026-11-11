//! SYX MIR module lowering orchestrator.
//! Extracted in S2-I214 (zero semantics).

use crate::lower_function_with_params;
use crate::lower_type_members;
use crate::mir_module_index::ModuleIndex;
use crate::mir_types::MirModule;
use indexmap::{IndexMap, IndexSet};
use smol_str::SmolStr;
use std::cell::Cell;
use std::rc::Rc;
use syx_ast::{AccessLevel, Decl, Module};
use syx_diagnostics::{SourceFile, SyxResult};
use syx_stdlib::NumericKind;

pub fn extension_methods_for_module(module: &Module) -> IndexSet<(SmolStr, SmolStr)> {
    ModuleIndex::from_module(module).extension_methods
}

pub fn lower_module(module: &Module) -> SyxResult<MirModule> {
    lower_module_with_extra_extensions(module, &IndexSet::new(), &IndexSet::new(), &IndexSet::new())
}

pub fn lower_module_with_extra_extensions(
    module: &Module,
    extra_extensions: &IndexSet<(SmolStr, SmolStr)>,
    callable_extensions: &IndexSet<SmolStr>,
    imported_class_types: &IndexSet<SmolStr>,
) -> SyxResult<MirModule> {
    lower_module_with_extra_extensions_and_kinds(
        module,
        extra_extensions,
        callable_extensions,
        imported_class_types,
        &IndexMap::new(),
    )
}

/// Like [`lower_module_with_extra_extensions`], but applies semantic contextual
/// numeric-literal kinds (span start → `NumericKind`) for unsuffixed consts.
pub fn lower_module_with_extra_extensions_and_kinds(
    module: &Module,
    extra_extensions: &IndexSet<(SmolStr, SmolStr)>,
    callable_extensions: &IndexSet<SmolStr>,
    imported_class_types: &IndexSet<SmolStr>,
    contextual_numeric_kinds: &IndexMap<u32, NumericKind>,
) -> SyxResult<MirModule> {
    lower_module_with_extra_extensions_kinds_and_source(
        module,
        extra_extensions,
        callable_extensions,
        imported_class_types,
        contextual_numeric_kinds,
        None,
    )
}

/// Like [`lower_module_with_extra_extensions_and_kinds`], with optional source for diag spans.
pub fn lower_module_with_extra_extensions_kinds_and_source(
    module: &Module,
    extra_extensions: &IndexSet<(SmolStr, SmolStr)>,
    callable_extensions: &IndexSet<SmolStr>,
    imported_class_types: &IndexSet<SmolStr>,
    contextual_numeric_kinds: &IndexMap<u32, NumericKind>,
    source: Option<&SourceFile>,
) -> SyxResult<MirModule> {
    let mut module_idx = ModuleIndex::from_module(module);
    module_idx.contextual_numeric_kinds = contextual_numeric_kinds.clone();
    module_idx.source = source.cloned();
    for ty in imported_class_types {
        module_idx
            .class_types
            .entry(ty.clone())
            .or_insert_with(Vec::new);
    }
    module_idx
        .extension_methods
        .extend(extra_extensions.iter().cloned());
    module_idx
        .callable_extensions
        .extend(callable_extensions.iter().cloned());
    let mut imports = IndexSet::new();
    let mut globals = Vec::new();
    for decl in &module.decls {
        if let Some(sym) = decl.import_symbol() {
            imports.insert(sym.clone());
        }
        if let Decl::VarGlobal { name, .. } = decl {
            globals.push(name.clone());
        }
    }
    let mut mir = MirModule::default();
    let closure_ids = Rc::new(Cell::new(0u32));
    for decl in &module.decls {
        // W483 / ADR-827: `export { Name, … }` → aot_export_roots (≠ auto-discovery).
        if let Decl::ExportList { names, .. } = decl {
            for name in names {
                if !mir.aot_export_roots.iter().any(|r| r == name) {
                    mir.aot_export_roots.push(name.clone());
                }
            }
        }
        let (is_export, decl) = match decl {
            Decl::Export { item, .. } => (true, item.as_ref()),
            other => (false, other),
        };
        if let Decl::Function {
            name,
            body,
            params,
            type_params,
            is_async,
            access,
            ..
        } = decl
        {
            if !type_params.is_empty() {
                continue;
            }
            // W481 / ADR-825: `export func` → `aot_export_roots`.
            // W923 / ADR-1267: scan-all auto — top-level `public func` also fills roots
            // (≠ exports GA; historical inventories keep auto_discovery=false).
            let auto_public = *access == AccessLevel::Public;
            if (is_export || auto_public) && !mir.aot_export_roots.iter().any(|r| r == name) {
                mir.aot_export_roots.push(name.clone());
            }
            let (func, closures) = lower_function_with_params(
                name,
                params,
                body,
                &module_idx,
                *is_async,
                &imports,
                &globals,
                &[],
                &[],
                Rc::clone(&closure_ids),
            )?;
            mir.functions.push(func);
            mir.functions.extend(closures);
        }
    }
    for decl in &module.decls {
        match decl {
            Decl::Actor {
                name,
                members,
                conformances,
                ..
            } => {
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    true,
                    conformances,
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            Decl::Class {
                name,
                members,
                conformances,
                superclass,
                ..
            } => {
                let mut merged = conformances.to_vec();
                if let Some(base) = superclass {
                    if module_idx.protocol_names.contains(base) && !merged.iter().any(|c| c == base)
                    {
                        merged.push(base.clone());
                    }
                }
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    false,
                    merged.as_slice(),
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            Decl::Extension {
                type_name, methods, ..
            } => {
                lower_type_members(
                    &mut mir,
                    type_name,
                    methods,
                    false,
                    &[],
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            // S1-I2: Struct methods (e.g. Counter.increment) via same member lower as Class.
            Decl::Struct { name, members, .. } => {
                lower_type_members(
                    &mut mir,
                    name,
                    members,
                    false,
                    &[],
                    &module_idx,
                    &imports,
                    &globals,
                    Rc::clone(&closure_ids),
                )?;
            }
            _ => {}
        }
    }
    Ok(mir)
}
