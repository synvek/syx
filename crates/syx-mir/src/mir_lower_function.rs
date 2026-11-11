//! SYX MIR function lowering with params.
//! Extracted in S2-I210 (zero semantics; crate-private).

use crate::instance_self_mut_ref_method;
use crate::lower_stmt;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_lower_support::{
    flatten_blocks, mir_local_names_for_slots, mir_local_types_for_slots,
};
use crate::mir_module_index::{mir_type_expr_name, ModuleIndex};
use crate::mir_types::{MirFunction, MirInstr, MirOp};
use indexmap::IndexSet;
use smol_str::SmolStr;
use std::cell::Cell;
use std::rc::Rc;
use syx_ast::{Stmt, TypeExpr};
use syx_diagnostics::SyxResult;

/// Phase 126: compile-time stubs so `Call Pingable_ping` lowers; runtime vtable/__type dispatch
/// resolves to the concrete `@Service` implementation.
/// Phase 70/126: inject protocol witness slots for unified MIR (WASM/native + user protocols).
/// Phase 125: vtable slots on class init dict (parity with codegen `compile_class_init`).
/// Align unified MIR class init with codegen `emit_mut_ref_cell_wrap`.
#[allow(dead_code)] // class/struct method self injection helper (parked)
pub(crate) fn lower_function_with_params(
    name: &SmolStr,
    params: &[syx_ast::Param],
    body: &[Stmt],
    module_idx: &ModuleIndex,
    is_async: bool,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    captures: &[SmolStr],
    mut_captures: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<(MirFunction, Vec<MirFunction>)> {
    let mut ctx = LowerCtx::new(
        name.clone(),
        module_idx.clone(),
        imports.clone(),
        globals.to_vec(),
        closure_ids,
    );
    ctx.in_async = is_async;
    ctx.await_segment = 0;
    for param in params {
        ctx.alloc_local(&param.name);
        ctx.param_names.insert(param.name.clone());
        register_param_local_type(&mut ctx, param);
        if matches!(param.ty, Some(TypeExpr::Reference(_, true, _))) {
            ctx.mut_ref_params.insert(param.name.clone());
        }
    }
    if params.first().is_some_and(|p| p.name.as_str() == "self")
        && instance_self_mut_ref_method(name, &ctx.module)
    {
        ctx.mut_ref_params.insert(SmolStr::new("self"));
    }
    for cap in captures {
        ctx.alloc_local(cap);
        // R-35: mutable captures are MutRefCell-shared with the outer frame.
        if mut_captures.iter().any(|m| m == cap) {
            ctx.mut_ref_params.insert(cap.clone());
        }
    }
    let entry = ctx.fresh_block();
    ctx.active_block = entry;
    for stmt in body {
        let block = ctx.active_block;
        lower_stmt(stmt, &mut ctx, block);
    }
    let has_explicit_return = body.iter().any(|s| matches!(s, Stmt::Return { .. }));
    // Phase 101: auto-call deinit for class locals at scope exit (before Return).
    if !has_explicit_return {
        ctx.emit_scope_exit_deinits(ctx.active_block);
    }
    let returns_self = !has_explicit_return
        && params.first().is_some_and(|p| p.name.as_str() == "self")
        && ctx
            .module
            .actor_types
            .iter()
            .any(|t| name.starts_with(&format!("{t}_")));
    if returns_self {
        if let Some(&self_slot) = ctx.locals.get(&SmolStr::new("self")) {
            ctx.emit(
                ctx.active_block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
        }
    } else if name.as_str() == "main" && !has_explicit_return {
        ctx.emit(
            ctx.active_block,
            MirInstr {
                dest: None,
                op: MirOp::ConstNil,
                span: None,
            },
        );
    }
    ctx.emit(
        ctx.active_block,
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    );
    if let Some(d) = ctx.take_error_diagnostic() {
        return Err(d);
    }
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    let closures = std::mem::take(&mut ctx.pending_closures);
    Ok((
        MirFunction {
            name: name.clone(),
            param_count: params.len() as u32,
            local_count,
            entry,
            blocks,
            instructions,
            local_names,
            local_types,
            init_body_pc: 0,
            init_wrap_pc: 0,
        },
        closures,
    ))
}

/// Extension returns are keyed by bare `Set` / `Array`; locals often carry `Set<Int>`.
/// Hosted collection ops (manifest `host_ops`) share MethodId return surfaces.
/// Phase 121/123/126: binding key for service resolve host calls (includes dynamic `Di.resolve`).
pub(crate) fn register_param_local_type(ctx: &mut LowerCtx, param: &syx_ast::Param) {
    if let Some(type_expr) = &param.ty {
        if let Some(type_name) = mir_type_expr_name(type_expr) {
            ctx.local_types.insert(param.name.clone(), type_name);
        }
    }
}
