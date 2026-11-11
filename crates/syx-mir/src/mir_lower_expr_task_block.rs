//! SYX MIR `Expr::TaskBlock` lowering.
//! Extracted in S2-I182 (zero semantics; crate-private).

use crate::lower_function_with_params;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirFunction, MirInstr, MirOp};
use smol_str::SmolStr;
use std::rc::Rc;
use syx_ast::Stmt;
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_task_block(
    body: &[Stmt],
    detached: bool,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
        let task_name = SmolStr::new(format!("__task_body_{}", ctx.next_closure_id()));
        let (mir_fn, nested) = lower_function_with_params(
            &task_name,
            &[],
            body,
            &ctx.module,
            false,
            &ctx.imports,
            &ctx.globals,
            &[],
            &[],
            Rc::clone(&ctx.closure_ids),
        )
        .unwrap_or_else(|e| {
            ctx.set_error(e.to_string());
            (
                MirFunction {
                    name: task_name.clone(),
                    param_count: 0,
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
        if detached {
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstStr(task_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::TaskDetach)),
                    span: None,
                },
            );
        } else {
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::TaskSpawn(task_name),
                    span: None,
                },
            );
        }
}
