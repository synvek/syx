//! SYX MIR `Expr::Borrow` lowering.
//! Extracted in S2-I198 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use syx_ast::Expr;

pub(crate) fn lower_expr_borrow(
    expr: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(expr, dest, ctx, block);
}
