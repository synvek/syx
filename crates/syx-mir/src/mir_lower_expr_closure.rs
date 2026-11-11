//! SYX MIR `Expr::Closure` lowering.
//! Extracted in S2-I180 (zero semantics; crate-private).

use crate::mir_collect_closure_free_vars::collect_closure_free_vars_detailed;
use crate::lower_function_with_params;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirFunction, MirInstr, MirOp};
use smol_str::SmolStr;
use std::rc::Rc;
use syx_ast::{ClosureBody, Param, Stmt};

pub(crate) fn lower_expr_closure(
    params: &[Param],
    body: &ClosureBody,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let closure_name = SmolStr::new(format!("__mir_closure_{}", ctx.next_closure_id()));
    let closure_stmts = match body {
        ClosureBody::Expr(e) => vec![Stmt::Return {
            span: syx_diagnostics::Span::DUMMY,
            value: Some(e.as_ref().clone()),
        }],
        ClosureBody::Block(stmts) => stmts.clone(),
    };
    let detailed = collect_closure_free_vars_detailed(params, body, &ctx.locals);
    let free_vars = detailed.free;
    let mut_caps: Vec<SmolStr> = free_vars
        .iter()
        .filter(|n| detailed.mutated.contains(*n))
        .cloned()
        .collect();
    for name in &free_vars {
        let is_mut = detailed.mutated.contains(name);
        if let Some(&slot) = ctx.locals.get(name) {
            if is_mut {
                // R-35: wrap outer local in MutRefCell so closure + outer share storage.
                emit_wrap_local_mut_ref_cell(ctx, block, slot);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if let Some(slot) = ctx.global_slot(name) {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
        }
    }
    let (mir_fn, nested) = lower_function_with_params(
        &closure_name,
        params,
        &closure_stmts,
        &ctx.module,
        false,
        &ctx.imports,
        &ctx.globals,
        &free_vars,
        &mut_caps,
        Rc::clone(&ctx.closure_ids),
    )
    .unwrap_or_else(|e| {
        ctx.set_error(e.to_string());
        (
            MirFunction {
                name: closure_name.clone(),
                param_count: params.len() as u32,
                local_count: 0,
                entry: 0,
                blocks: vec![],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
            },
            vec![],
        )
    });
    ctx.pending_closures.push(mir_fn);
    ctx.pending_closures.extend(nested);
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::MakeClosure(closure_name, free_vars.len() as u32),
            span: None,
        },
    );
}

/// Stack: (empty) → wrap locals[slot] as `{__syx_ref: old, __type: MutRefCell}` and store back.
fn emit_wrap_local_mut_ref_cell(ctx: &mut LowerCtx, block: u32, slot: u32) {
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__syx_ref".into()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("MutRefCell".into()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__type".into()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::DictBuild(2),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(slot),
            span: None,
        },
    );
}
