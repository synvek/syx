//! SYX MIR `Expr::Ternary` lowering.
//! Extracted in S2-I184 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use syx_ast::Expr;

pub(crate) fn lower_expr_ternary(
    condition: &Box<Expr>,
    then_branch: &Box<Expr>,
    else_branch: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    // S1-I7: `cond ? then : else` → JumpIfFalse + merge.
    lower_expr(condition, dest, ctx, block);
    let then_block = ctx.fresh_block();
    let else_block = ctx.fresh_block();
    let end_block = ctx.fresh_block();
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::JumpIfFalse(else_block),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::Jump(then_block),
            span: None,
        },
    );
    ctx.link(block, then_block);
    ctx.link(block, else_block);
    lower_expr(then_branch, dest, ctx, then_block);
    ctx.emit(
        then_block,
        MirInstr {
            dest: None,
            op: MirOp::Jump(end_block),
            span: None,
        },
    );
    ctx.link(then_block, end_block);
    lower_expr(else_branch, dest, ctx, else_block);
    ctx.emit(
        else_block,
        MirInstr {
            dest: None,
            op: MirOp::Jump(end_block),
            span: None,
        },
    );
    ctx.link(else_block, end_block);
    ctx.active_block = end_block;
}
