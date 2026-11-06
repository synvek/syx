//! SYX MIR `Expr::Super` lowering.
//! Extracted in S2-I200 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use smol_str::SmolStr;

pub(crate) fn lower_expr_super(
    _dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    ctx.lower_load_binding_raw(&SmolStr::new("self"), block);
}