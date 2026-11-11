//! SYX MIR expression lower dispatch shell.
//! Extracted in S2-I213 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use crate::mir_lower_expr_array::lower_expr_array;
use crate::mir_lower_expr_assign::lower_expr_assign;
use crate::mir_lower_expr_await::lower_expr_await;
use crate::mir_lower_expr_binary::lower_expr_binary;
use crate::mir_lower_expr_borrow::lower_expr_borrow;
use crate::mir_lower_expr_call::lower_expr_call;
use crate::mir_lower_expr_cast::lower_expr_cast;
use crate::mir_lower_expr_closure::lower_expr_closure;
use crate::mir_lower_expr_compound_assign::lower_expr_compound_assign;
use crate::mir_lower_expr_group::lower_expr_group;
use crate::mir_lower_expr_ident::lower_expr_ident;
use crate::mir_lower_expr_index::lower_expr_index;
use crate::mir_lower_expr_index_assign::lower_expr_index_assign;
use crate::mir_lower_expr_is::lower_expr_is;
use crate::mir_lower_expr_keypath::lower_expr_keypath;
use crate::mir_lower_expr_literal::lower_expr_literal;
use crate::mir_lower_expr_macro_invoke::lower_expr_macro_invoke;
use crate::mir_lower_expr_member::lower_expr_member;
use crate::mir_lower_expr_member_assign::lower_expr_member_assign;
use crate::mir_lower_expr_optional_cast::lower_expr_optional_cast;
use crate::mir_lower_expr_range::{
    lower_expr_closed_range, lower_expr_half_open_range, lower_expr_range_dyn,
};
use crate::mir_lower_expr_super::lower_expr_super;
use crate::mir_lower_expr_task_block::lower_expr_task_block;
use crate::mir_lower_expr_ternary::lower_expr_ternary;
use crate::mir_lower_expr_tuple::lower_expr_tuple;
use crate::mir_lower_expr_type_self::lower_expr_type_self;
use crate::mir_lower_expr_unary::lower_expr_unary;
use syx_ast::Expr;

pub(crate) fn lower_expr(expr: &Expr, dest: u32, ctx: &mut LowerCtx, block: u32) {
    match expr {
        Expr::Literal { .. } => lower_expr_literal(expr, dest, ctx, block),
        Expr::Ident { name, .. } => {
            lower_expr_ident(name, dest, ctx, block);
        }
        Expr::Ternary {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            lower_expr_ternary(condition, then_branch, else_branch, dest, ctx, block);
        }
        Expr::Binary {
            left, op, right, ..
        } => {
            lower_expr_binary(left, op, right, dest, ctx, block);
        }
        Expr::Unary { op, expr, .. } => {
            lower_expr_unary(op, expr, dest, ctx, block);
        }
        Expr::Call { callee, args, .. } => {
            lower_expr_call(callee, args, dest, ctx, block);
        }
        Expr::Array { elements, .. } => {
            lower_expr_array(elements, dest, ctx, block);
        }
        Expr::Tuple { elements, .. } => {
            lower_expr_tuple(elements, dest, ctx, block);
        }
        Expr::Index { object, index, .. } => {
            lower_expr_index(object, index, dest, ctx, block);
        }
        Expr::Member { object, name, .. } => {
            lower_expr_member(object, name, dest, ctx, block);
        }
        Expr::Await { expr, .. } => {
            lower_expr_await(expr, dest, ctx, block);
        }
        Expr::Closure { params, body, .. } => {
            lower_expr_closure(params, body, dest, ctx, block);
        }
        Expr::TaskBlock { body, detached, .. } => {
            lower_expr_task_block(body, *detached, dest, ctx, block);
        }
        Expr::Assign { name, value, .. } => {
            lower_expr_assign(name, value, dest, ctx, block);
        }
        Expr::CompoundAssign {
            name,
            op,
            value,
            span,
        } => {
            lower_expr_compound_assign(name, op, value, span, dest, ctx, block);
        }
        Expr::Group { expr, .. } => lower_expr_group(expr, dest, ctx, block),
        Expr::Borrow { expr, .. } => lower_expr_borrow(expr, dest, ctx, block),
        Expr::KeyPath { .. } => lower_expr_keypath(dest, ctx, block),
        Expr::MacroInvoke { args, .. } => {
            lower_expr_macro_invoke(args, dest, ctx, block);
        }
        Expr::OptionalCast { expr, target, .. } => {
            lower_expr_optional_cast(expr, target, dest, ctx, block);
        }
        Expr::Is { expr, target, .. } => {
            lower_expr_is(expr, target, dest, ctx, block);
        }
        Expr::Cast { expr, target, .. } => {
            lower_expr_cast(expr, target, dest, ctx, block);
        }
        Expr::TypeSelf { type_name, .. } => {
            lower_expr_type_self(type_name, dest, ctx, block);
        }
        Expr::MemberAssign {
            object,
            name,
            value,
            ..
        } => {
            lower_expr_member_assign(object, name, value, dest, ctx, block);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            lower_expr_index_assign(object, index, value, dest, ctx, block);
        }
        Expr::HalfOpenRange { start, end, .. } => {
            lower_expr_half_open_range(*start, *end, dest, ctx, block);
        }
        Expr::ClosedRange { start, end, .. } => {
            lower_expr_closed_range(*start, *end, dest, ctx, block);
        }
        Expr::RangeDyn {
            start,
            end,
            inclusive,
            ..
        } => {
            lower_expr_range_dyn(start, end, *inclusive, dest, ctx, block);
        }
        Expr::Super { .. } => {
            lower_expr_super(dest, ctx, block);
        }
        _ => {}
    }
}
