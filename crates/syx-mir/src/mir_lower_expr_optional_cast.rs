//! SYX MIR `Expr::OptionalCast` lowering.
//! Extracted in S2-I187 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::mir_type_expr_name;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::{Expr, TypeExpr};
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_optional_cast(
    expr: &Box<Expr>,
    target: &TypeExpr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(expr, dest, ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr(
                mir_type_expr_name(target)
                    .unwrap_or_else(|| SmolStr::new("Unknown"))
                    .to_string(),
            ),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::CallHost(host_fn_index(HostFnId::AsOptional)),
            span: None,
        },
    );
}
