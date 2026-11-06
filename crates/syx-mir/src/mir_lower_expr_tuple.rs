//! SYX MIR `Expr::Tuple` lowering.
//! Extracted in S2-I185 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::Expr;

pub(crate) fn lower_expr_tuple(
    elements: &[(Option<SmolStr>, Expr)],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let arity = elements.len();
    for (i, (label, el)) in elements.iter().enumerate() {
        lower_expr(el, ctx.fresh_reg(), ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(i.to_string()),
                span: None,
            },
        );
        if let Some(name) = label {
            lower_expr(el, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(name.to_string()),
                    span: None,
                },
            );
        }
    }
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstI32(arity as i32),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__tuple_len".into()),
            span: None,
        },
    );
    let pair_count = arity + elements.iter().filter(|(l, _)| l.is_some()).count() + 1;
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::DictBuild(pair_count as u32),
            span: None,
        },
    );
}
