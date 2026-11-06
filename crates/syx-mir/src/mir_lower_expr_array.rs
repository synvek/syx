//! SYX MIR `Expr::Array` lowering.
//! Extracted in S2-I190 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use syx_ast::Expr;

pub(crate) fn lower_expr_array(
    elements: &[Expr],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    for el in elements {
        let r = ctx.fresh_reg();
        lower_expr(el, r, ctx, block);
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: if ctx.prefer_set_build {
                MirOp::SetBuild(elements.len() as u32)
            } else {
                MirOp::ArrayBuild(elements.len() as u32)
            },
            span: None,
        },
    );
}
