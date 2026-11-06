//! SYX MIR `Expr::Ident` lowering.
//! Extracted in S2-I196 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use smol_str::SmolStr;

pub(crate) fn lower_expr_ident(
    name: &SmolStr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    ctx.lower_load_binding(dest, name, block);
}
