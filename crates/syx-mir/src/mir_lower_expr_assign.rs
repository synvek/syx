//! SYX MIR `Expr::Assign` lowering.
//! Extracted in S2-I192 (zero semantics; crate-private).

use crate::emit_integer_narrow_cast_if_needed;
use crate::infer_expr_init_type;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use smol_str::SmolStr;
use syx_ast::Expr;

pub(crate) fn lower_expr_assign(
    name: &SmolStr,
    value: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(value, dest, ctx, block);
    if let Some(to_name) = ctx.local_types.get(name).cloned() {
        if let Some(from_name) = infer_expr_init_type(value, ctx) {
            emit_integer_narrow_cast_if_needed(
                from_name.as_str(),
                to_name.as_str(),
                dest,
                ctx,
                block,
            );
        }
    }
    ctx.lower_store_binding(name, block);
}
