//! SYX MIR `Expr::IndexAssign` lowering.
//! Extracted in S2-I177 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use crate::type_name_dispatch_base;
use smol_str::SmolStr;
use syx_ast::Expr;

pub(crate) fn lower_expr_index_assign(
    object: &Box<Expr>,
    index: &Box<Expr>,
    value: &Box<Expr>,
    _dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
        if let Expr::Member { object: base, name: field, .. } = object.as_ref() {
            if let Expr::Ident { name: recv, .. } = base.as_ref() {
                if ctx.locals.contains_key(recv) {
                    lower_expr(base, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(field.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ObjectGet,
                            span: None,
                        },
                    );
                    lower_expr(index, ctx.fresh_reg(), ctx, block);
                    lower_expr(value, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ArraySet,
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(field.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ObjectSet,
                            span: None,
                        },
                    );
                    if let Some(&slot) = ctx.locals.get(recv) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(slot),
                                span: None,
                            },
                        );
                    }
                    return;
                }
            }
        }
        if let Expr::Ident { name: recv, .. } = object.as_ref() {
            if let Some(type_name) = ctx.local_types.get(recv) {
                let base = type_name_dispatch_base(type_name);
                if ctx.module.extension_methods.contains(&(
                    base.clone(),
                    SmolStr::new("__subscript_set"),
                )) {
                    let ext_name = SmolStr::new(format!("{base}___subscript_set"));
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                    if let Expr::Tuple { elements, .. } = index.as_ref() {
                        for (_, el) in elements {
                            lower_expr(el, ctx.fresh_reg(), ctx, block);
                        }
                    } else {
                        lower_expr(index, ctx.fresh_reg(), ctx, block);
                    }
                    lower_expr(value, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::Call(ext_name),
                            span: None,
                        },
                    );
                    if let Some(&slot) = ctx.locals.get(recv) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(slot),
                                span: None,
                            },
                        );
                    }
                    return;
                }
            }
        }
        let obj_ty = match object.as_ref() {
            Expr::Ident { name, .. } => ctx.local_types.get(name).cloned(),
            _ => None,
        };
        let use_object_set = obj_ty.as_ref().is_some_and(|t| {
            let s = t.as_str();
            s == "Object" || s == "Dictionary" || s.starts_with("Dictionary<")
        });
        if use_object_set {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            lower_expr(value, ctx.fresh_reg(), ctx, block);
            lower_expr(index, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectSet,
                    span: None,
                },
            );
        } else {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            lower_expr(index, ctx.fresh_reg(), ctx, block);
            lower_expr(value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ArraySet,
                    span: None,
                },
            );
        }
        if let Expr::Ident { name: recv, .. } = object.as_ref() {
            if let Some(&slot) = ctx.locals.get(recv) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
            }
        }
}
