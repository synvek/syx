//! SYX MIR switch / pattern bind lowering.
//! Extracted in S2-I202 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::mir_type_expr_name;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::Expr;
use syx_stdlib::{host_fn_index, HostFnId, PLATFORM_INT};

pub(crate) fn lower_switch_pattern_match(
    pattern: &Expr,
    case_slot: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    match pattern {
        Expr::TuplePattern { bindings, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
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
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(bindings.len() as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Eq,
                    span: None,
                },
            );
        }
        Expr::VariantPattern { variant, .. } if variant.as_str() == "__let" => {
            // `case let x` always matches; binding copies the subject.
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstBool(true),
                    span: None,
                },
            );
        }
        Expr::VariantPattern { variant, .. } => {
            let tag = ctx
                .module
                .enum_case_tags
                .iter()
                .find_map(|((_, case), tag)| if case == variant { Some(*tag) } else { None })
                .unwrap_or(0);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr("tag".into()),
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
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(tag as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Eq,
                    span: None,
                },
            );
        }
        Expr::Is { target, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
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
                    dest: None,
                    op: MirOp::CallHost(host_fn_index(HostFnId::TypeIs)),
                    span: None,
                },
            );
        }
        Expr::HalfOpenRange { start, end, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*start as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Ge {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*end as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Lt {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::And,
                    span: None,
                },
            );
        }
        Expr::ClosedRange { start, end, .. } => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*start as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Ge {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(*end as i32),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Le {
                        kind: PLATFORM_INT.to_byte(),
                    },
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::And,
                    span: None,
                },
            );
        }
        other => {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            let rhs = ctx.fresh_reg();
            lower_expr(other, rhs, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ValueEq,
                    span: None,
                },
            );
        }
    }
}

pub(crate) fn bind_tuple_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    if let Expr::TuplePattern { bindings, .. } = pattern {
        for (i, bind) in bindings.iter().enumerate() {
            let bind_slot = ctx.alloc_local(bind);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(i.to_string()),
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
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(bind_slot),
                    span: None,
                },
            );
        }
    }
}

pub(crate) fn bind_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    bind_variant_pattern(pattern, case_slot, ctx, block);
    bind_tuple_pattern(pattern, case_slot, ctx, block);
}

pub(crate) fn bind_variant_pattern(pattern: &Expr, case_slot: u32, ctx: &mut LowerCtx, block: u32) {
    if let Expr::VariantPattern {
        variant,
        bindings,
        ..
    } = pattern
    {
        if bindings.is_empty() {
            return;
        }
        // S1-I5: `case let x` — bind the whole subject.
        if variant.as_str() == "__let" {
            for (_label, bind) in bindings {
                let bind_slot = ctx.alloc_local(bind);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(case_slot),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(bind_slot),
                        span: None,
                    },
                );
            }
            return;
        }
        let fields: Vec<SmolStr> = ctx
            .module
            .enum_associated_fields
            .iter()
            .find_map(|((_, case), fields)| {
                if case == variant {
                    Some(fields.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        for (i, (label, bind)) in bindings.iter().enumerate() {
            // Unlabeled assoc fields are stored as `_0`, `_1`, … (S1-I2 enum parse).
            let field = label
                .clone()
                .or_else(|| fields.get(i).cloned())
                .unwrap_or_else(|| SmolStr::new(format!("_{i}")));
            let bind_slot = ctx.alloc_local(bind);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(case_slot),
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
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(bind_slot),
                    span: None,
                },
            );
        }
    }
}

