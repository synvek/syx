//! SYX MIR `Expr::MacroInvoke` lowering.
//! Extracted in S2-I194 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use syx_ast::Expr;

pub(crate) fn lower_expr_macro_invoke(
    args: &[Expr],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    for a in args {
        lower_expr(a, ctx.fresh_reg(), ctx, block);
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ConstNil,
            span: None,
        },
    );
}