//! SYX MIR `Expr::Cast` lowering.
//! Extracted in S2-I186 (zero semantics; crate-private).

use crate::emit_float_to_int_numeric_cast_if_needed;
use crate::emit_integer_numeric_cast_if_needed;
use crate::infer_expr_init_type;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::mir_type_expr_name;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::{Expr, TypeExpr};
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_cast(
    expr: &Box<Expr>,
    target: &TypeExpr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(expr, dest, ctx, block);
    let to_name = mir_type_expr_name(target).unwrap_or_else(|| SmolStr::new("Unknown"));
    if let Some(from_name) = infer_expr_init_type(expr, ctx) {
        if emit_integer_numeric_cast_if_needed(
            from_name.as_str(),
            to_name.as_str(),
            dest,
            ctx,
            block,
        ) || emit_float_to_int_numeric_cast_if_needed(
            from_name.as_str(),
            to_name.as_str(),
            dest,
            ctx,
            block,
        ) {
            return;
        }
    }
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr(to_name.to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::CallHost(host_fn_index(HostFnId::TypeCast)),
            span: None,
        },
    );
}
