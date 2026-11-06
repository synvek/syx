//! SYX MIR `Expr::Await` lowering.
//! Extracted in S2-I191 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use syx_ast::Expr;

pub(crate) fn lower_expr_await(
    expr: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    if ctx.in_async {
        lower_expr(expr, dest, ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::AwaitSuspend {
                    resume_fn: 0,
                    state: ctx.await_segment,
                },
                span: None,
            },
        );
        ctx.await_segment += 1;
    }
}
