//! SYX MIR `Expr::CompoundAssign` lowering.
//! Extracted in S2-I183 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::{BinOp, CompoundAssignOp, Expr};
use syx_diagnostics::Span;
use syx_stdlib::host_fn_index_by_name;

pub(crate) fn lower_expr_compound_assign(
    name: &SmolStr,
    op: &CompoundAssignOp,
    value: &Box<Expr>,
    span: &Span,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
        let bin_op = match op {
            CompoundAssignOp::Add => BinOp::Add,
            CompoundAssignOp::Sub => BinOp::Sub,
            CompoundAssignOp::Mul => BinOp::Mul,
            CompoundAssignOp::Div => BinOp::Div,
            CompoundAssignOp::Mod => BinOp::Mod,
            CompoundAssignOp::BitAnd => BinOp::BitAnd,
            CompoundAssignOp::BitOr => BinOp::BitOr,
            CompoundAssignOp::BitXor => BinOp::BitXor,
            CompoundAssignOp::Shl => BinOp::Shl,
            CompoundAssignOp::Shr => BinOp::Shr,
        };
        // R-35: only Int in-place host when the binding is known Int —
        // do not default missing local_types to Int (field/other types
        // fall through to Assign desugar + enclosing-field load/store).
        if matches!(op, CompoundAssignOp::Add | CompoundAssignOp::Sub) {
            let left_ty = ctx.local_types.get(name).cloned();
            if left_ty.as_deref() == Some("Int") {
                let host_name = match op {
                    CompoundAssignOp::Add => "__syx_int_add_assign",
                    CompoundAssignOp::Sub => "__syx_int_sub_assign",
                    _ => "",
                };
                if let Some(idx) = host_fn_index_by_name(host_name) {
                    let lhs = ctx.fresh_reg();
                    let rhs = ctx.fresh_reg();
                    ctx.lower_load_binding(lhs, name, block);
                    lower_expr(value, rhs, ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::CallHost(idx),
                            span: None,
                        },
                    );
                    ctx.lower_store_binding(name, block);
                    return;
                }
            }
        }
        let desugared = Expr::Assign {
            span: *span,
            name: name.clone(),
            value: Box::new(Expr::Binary {
                span: *span,
                left: Box::new(Expr::Ident {
                    span: *span,
                    name: name.clone(),
                }),
                op: bin_op,
                right: value.clone(),
            }),
        };
        lower_expr(&desugared, dest, ctx, block);
}
