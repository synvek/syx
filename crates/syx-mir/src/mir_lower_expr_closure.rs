//! SYX MIR `Expr::Closure` lowering.
//! Extracted in S2-I180 (zero semantics; crate-private).

use crate::mir_collect_closure_free_vars::collect_closure_free_vars;
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
    let free_vars = collect_closure_free_vars(params, body, &ctx.locals);
    for name in &free_vars {
        if let Some(&slot) = ctx.locals.get(name) {
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
