//! SYX MIR `Expr::Literal` lowering (first `lower_expr` sub-face).
//! Extracted in S2-I172 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use syx_ast::{Expr, Literal};
use syx_stdlib::{
    default_float_kind, float_literal_bits, from_suffix, host_fn_index_by_name, is_float,
    NumericKind, PLATFORM_INT,
};

fn contextual_const_bits(kind: NumericKind, value: i128) -> u64 {
    if is_float(kind) {
        float_literal_bits(kind, value as f64)
    } else {
        value as u64
    }
}

pub(crate) fn lower_expr_literal(expr: &Expr, dest: u32, ctx: &mut LowerCtx, block: u32) {
    let Expr::Literal { value, .. } = expr else {
        ctx.set_error("lower_expr_literal: expected Expr::Literal");
        return;
    };
    match value {
            Literal::BigInt(text) => {
                let tmp = ctx.fresh_reg();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(tmp),
                        op: MirOp::ConstStr(text.clone()),
                        span: None,
                    },
                );
                if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_string") {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::CallHost(idx),
                            span: None,
                        },
                    );
                } else {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNil,
                            span: None,
                        },
                    );
                }
            }
            Literal::Integer { value, suffix } => {
                if suffix.as_deref() == Some("n") {
                    let bits = *value as u128;
                    let tmp = ctx.fresh_reg();
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(tmp),
                            op: MirOp::ConstNumeric128 {
                                kind: NumericKind::I128.to_byte(),
                                lo: bits as u64,
                                hi: (bits >> 64) as u64,
                            },
                            span: None,
                        },
                    );
                    if let Some(idx) = host_fn_index_by_name("__syx_bigint_from_i128") {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::CallHost(idx),
                                span: None,
                            },
                        );
                    } else {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::ConstNil,
                                span: None,
                            },
                        );
                    }
                    return;
                }
                let kind = suffix
                    .as_ref()
                    .and_then(|s| from_suffix(s))
                    .or_else(|| {
                        let start = match expr {
                            Expr::Literal { span, .. } => span.start as u32,
                            _ => return None,
                        };
                        ctx.module.contextual_numeric_kinds.get(&start).copied()
                    })
                    .unwrap_or(PLATFORM_INT);
                let resolved = syx_stdlib::resolve_alias(kind);
                if syx_stdlib::bit_width(resolved) > 64 {
                    let bits = *value as u128;
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNumeric128 {
                                kind: resolved.to_byte(),
                                lo: bits as u64,
                                hi: (bits >> 64) as u64,
                            },
                            span: None,
                        },
                    );
                } else {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::ConstNumeric {
                                kind: resolved.to_byte(),
                                bits: contextual_const_bits(resolved, *value),
                            },
                            span: None,
                        },
                    );
                }
            }
            Literal::Double { value, suffix } => {
                let kind = suffix
                    .as_ref()
                    .and_then(|s| from_suffix(s))
                    .unwrap_or(default_float_kind());
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ConstNumeric {
                            kind: kind.to_byte(),
                            bits: float_literal_bits(kind, *value),
                        },
                        span: None,
                    },
                );
            }
            Literal::String(s) => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstStr(s.clone()),
                    span: None,
                },
            ),
            Literal::Bool(v) => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstBool(*v),
                    span: None,
                },
            ),
            Literal::Nil => ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstNil,
                    span: None,
                },
            ),
            Literal::Interpolated(parts) => {
                let mut first = true;
                for part in parts {
                    match part {
                        syx_ast::StringPart::Text(t) => {
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ConstStr(t.clone()),
                                    span: None,
                                },
                            );
                        }
                        syx_ast::StringPart::Expr(e) => {
                            lower_expr(e, dest, ctx, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: Some(dest),
                                    op: MirOp::ToStr,
                                    span: None,
                                },
                            );
                        }
                    }
                    if first {
                        first = false;
                    } else {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: Some(dest),
                                op: MirOp::ConcatStr,
                                span: None,
                            },
                        );
                    }
                }
            }
            Literal::Dict(entries) => {
                for (key, val) in entries {
                    lower_expr(val, ctx.fresh_reg(), ctx, block);
                    lower_expr(key, ctx.fresh_reg(), ctx, block);
                }
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::DictBuild(entries.len() as u32),
                        span: None,
                    },
                );
            }
            _ => {
                ctx.set_error(format!("Unsupported literal in MIR lowering"));
            }
    }
}
