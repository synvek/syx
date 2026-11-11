//! SYX MIR `Expr::Binary` lowering.
//! Extracted in S2-I178 (zero semantics; crate-private).

use crate::binary_numeric_kind;
use crate::binop_operator_symbol;
use crate::expr_is_opaque_operand;
use crate::expr_is_string_like;
use crate::infer_expr_simd_type;
use crate::infer_expr_type_name;
use crate::infer_object_type_name;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::{BinOp, Expr};
use syx_stdlib::PLATFORM_INT;

pub(crate) fn lower_expr_binary(
    left: &Box<Expr>,
    op: &BinOp,
    right: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    if *op == BinOp::NilCoalesce {
        // JumpIfNil pops on nil and preserves the value when non-nil —
        // no Dup/Pop pair (that left a stale stack slot on the Some path).
        lower_expr(left, dest, ctx, block);
        let right_block = ctx.fresh_block();
        let end_block = ctx.fresh_block();
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::JumpIfNil(right_block),
                span: None,
            },
        );
        ctx.link(block, end_block);
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::Jump(end_block),
                span: None,
            },
        );
        lower_expr(right, dest, ctx, right_block);
        ctx.emit(
            right_block,
            MirInstr {
                dest: None,
                op: MirOp::Jump(end_block),
                span: None,
            },
        );
        ctx.link(right_block, end_block);
        ctx.active_block = end_block;
        return;
    }
    lower_expr(left, dest, ctx, block);
    lower_expr(right, dest + 1, ctx, block);
    let opaque_eq = expr_is_opaque_operand(left, ctx) || expr_is_opaque_operand(right, ctx);
    let left_ty = infer_object_type_name(left, ctx).unwrap_or_else(|| infer_expr_type_name(left));
    if let Some(op_sym) = binop_operator_symbol(*op) {
        let ty_smol = left_ty.clone();
        if (ctx.module.class_types.contains_key(&ty_smol)
            || ctx.module.dataclass_names.contains(&ty_smol))
            && !syx_stdlib::is_builtin_type(ty_smol.as_str())
            && ty_smol.as_str() != "Unknown"
        {
            let fn_name = SmolStr::new(format!("{left_ty}_{op_sym}"));
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::Call(fn_name),
                    span: None,
                },
            );
            return;
        }
    }
    if let (Some((la, ea)), Some((lb, eb))) = (
        infer_expr_simd_type(left, ctx),
        infer_expr_simd_type(right, ctx),
    ) {
        if la == lb && ea == eb {
            let mir_op = match op {
                BinOp::Add => MirOp::SimdAdd,
                BinOp::Sub => MirOp::SimdSub,
                BinOp::Mul => MirOp::SimdMul,
                BinOp::Div => MirOp::SimdDiv,
                BinOp::Eq => MirOp::SimdEq,
                BinOp::NotEq => MirOp::SimdNe,
                BinOp::Lt => MirOp::SimdLt,
                BinOp::LtEq => MirOp::SimdLe,
                BinOp::Gt => MirOp::SimdGt,
                BinOp::GtEq => MirOp::SimdGe,
                BinOp::And => MirOp::SimdAnd,
                BinOp::Or => MirOp::SimdOr,
                // No BinOp::Xor — SIMD xor is HostFn→SimdXor; never fall through to Add.
                _ => MirOp::Add {
                    kind: PLATFORM_INT.to_byte(),
                },
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: mir_op,
                    span: None,
                },
            );
            return;
        }
    }
    let arith_kind = binary_numeric_kind(left, right, ctx);
    let mir_op = match op {
        BinOp::Add if expr_is_string_like(left, ctx) || expr_is_string_like(right, ctx) => {
            MirOp::ConcatStr
        }
        BinOp::Add => MirOp::Add { kind: arith_kind },
        BinOp::Sub => MirOp::Sub { kind: arith_kind },
        BinOp::Mul => MirOp::Mul { kind: arith_kind },
        BinOp::Div => MirOp::Div { kind: arith_kind },
        BinOp::Eq if opaque_eq => MirOp::OpaqueValueEq,
        BinOp::Eq => MirOp::ValueEq,
        BinOp::NotEq if opaque_eq => MirOp::ValueNe,
        BinOp::NotEq => MirOp::ValueNe,
        BinOp::IdenticalEq => MirOp::RefEq,
        BinOp::IdenticalNe => MirOp::RefNe,
        BinOp::Lt => MirOp::Lt { kind: arith_kind },
        BinOp::LtEq => MirOp::Le { kind: arith_kind },
        BinOp::Gt => MirOp::Gt { kind: arith_kind },
        BinOp::GtEq => MirOp::Ge { kind: arith_kind },
        BinOp::And => MirOp::And,
        BinOp::Or => MirOp::Or,
        BinOp::Mod => MirOp::Mod { kind: arith_kind },
        BinOp::BitAnd => MirOp::BitAnd { kind: arith_kind },
        BinOp::BitOr => MirOp::BitOr { kind: arith_kind },
        BinOp::BitXor => MirOp::BitXor { kind: arith_kind },
        BinOp::Shl => MirOp::Shl { kind: arith_kind },
        BinOp::Shr => MirOp::Shr { kind: arith_kind },
        BinOp::NilCoalesce => unreachable!(),
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
