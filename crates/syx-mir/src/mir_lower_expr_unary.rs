//! SYX MIR `Expr::Unary` lowering.
//! Extracted in S2-I179 (zero semantics; crate-private).

use crate::infer_expr_simd_type;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use crate::unary_numeric_kind;
use syx_ast::{Expr, Literal, UnaryOp};
use syx_stdlib::{float_literal_bits, from_suffix, is_float, PLATFORM_INT};

pub(crate) fn lower_expr_unary(
    op: &UnaryOp,
    expr: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    if *op == UnaryOp::Try {
        let err_block = ctx.fresh_block();
        let after_block = ctx.fresh_block();
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::BeginTry(err_block),
                span: None,
            },
        );
        lower_expr(expr, dest, ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::EndTry,
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::BindResult(err_block),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::Jump(after_block),
                span: None,
            },
        );
        // R-30: exception edge must be a CFG successor or remove_unreachable_blocks
        // drops the ReturnErr pad (BeginTry target stays as a stale block id).
        ctx.link(block, err_block);
        ctx.link(block, after_block);
        ctx.emit(
            err_block,
            MirInstr {
                dest: None,
                op: MirOp::ReturnErr,
                span: None,
            },
        );
        ctx.link(err_block, after_block);
        ctx.active_block = after_block;
        return;
    }
    // Phase 127: fold `-literal` so ArrayBuild lane counts stay exact.
    if *op == UnaryOp::Neg {
        if let Expr::Literal {
            value: Literal::Integer { value, suffix },
            span: lit_span,
        } = expr.as_ref()
        {
            if suffix.as_deref() != Some("n") {
                let kind = suffix
                    .as_ref()
                    .and_then(|s| from_suffix(s))
                    .or_else(|| {
                        ctx.module
                            .contextual_numeric_kinds
                            .get(&(lit_span.start as u32))
                            .copied()
                    })
                    .unwrap_or(PLATFORM_INT);
                let resolved = syx_stdlib::resolve_alias(kind);
                let bits = if is_float(resolved) {
                    float_literal_bits(resolved, -(*value as f64))
                } else {
                    (*value as i128).wrapping_neg() as u64
                };
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ConstNumeric {
                            kind: resolved.to_byte(),
                            bits,
                        },
                        span: None,
                    },
                );
                return;
            }
        }
    }
    lower_expr(expr, dest, ctx, block);
    let unary_kind = unary_numeric_kind(expr, ctx);
    let mir_op = if matches!(op, UnaryOp::Not) && infer_expr_simd_type(expr, ctx).is_some() {
        MirOp::SimdNot
    } else {
        match op {
            UnaryOp::Neg => MirOp::Neg { kind: unary_kind },
            UnaryOp::Not => MirOp::Not,
            UnaryOp::BitNot => MirOp::BitNot { kind: unary_kind },
            UnaryOp::Try => MirOp::Neg {
                kind: PLATFORM_INT.to_byte(),
            },
        }
    };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: mir_op,
            span: None,
        },
    );
}
