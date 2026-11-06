//! SYX MIR `Expr::Index` lowering.
//! Extracted in S2-I181 (zero semantics; crate-private).

use crate::expr_is_dict_like;
use crate::expr_is_string_like;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use crate::type_name_dispatch_base;
use smol_str::SmolStr;
use syx_ast::Expr;
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_index(
    object: &Box<Expr>,
    index: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
        if expr_is_string_like(object, ctx) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            lower_expr(index, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::StringCharAt)),
                    span: None,
                },
            );
            return;
        }
        if expr_is_dict_like(object, ctx) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            lower_expr(index, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            return;
        }
        if let Expr::Ident { name: recv, .. } = object.as_ref() {
            if let Some(type_name) = ctx.local_types.get(recv) {
                let base = type_name_dispatch_base(type_name);
                if ctx.module.extension_methods.contains(&(
                    base.clone(),
                    SmolStr::new("__subscript_get"),
                )) {
                    let ext_name = SmolStr::new(format!("{base}___subscript_get"));
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    // S1-I9: `m[a, b]` → Tuple indices lowered as separate args.
                    if let Expr::Tuple { elements, .. } = index.as_ref() {
                        for (_, el) in elements {
                            lower_expr(el, ctx.fresh_reg(), ctx, block);
                        }
                    } else {
                        lower_expr(index, ctx.fresh_reg(), ctx, block);
                    }
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::Call(ext_name),
                            span: None,
                        },
                    );
                    return;
                }
            }
        }
        lower_expr(object, dest, ctx, block);
        let idx = ctx.fresh_reg();
        lower_expr(index, idx, ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ArrayGet,
                span: None,
            },
        );
}
