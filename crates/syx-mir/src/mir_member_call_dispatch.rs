//! SYX MIR member-call classification / MethodId dispatch.
//! Extracted in S2-I211 (zero semantics; crate-private).

use crate::collection_method_mir_op;
use crate::expr_is_array_like;
use crate::expr_is_float32_like;
use crate::expr_is_float64_like;
use crate::expr_is_string_like;
use crate::is_static_type_member_call;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::MirOp;
use syx_ast::{Expr, Literal};
use syx_stdlib::{
    lookup_array_method, lookup_float32_math_host, lookup_float64_math_host, lookup_string_method,
    method_id_call_host_index, MethodId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MemberCallKind {
    /// Free function / static type / host manifest — lower call args before callee dispatch.
    ArgsEarly,
    /// Instance method — receiver and args lowered in member branch.
    InstanceMethod,
    /// String/array hosted methods — receiver first, args in branch.
    ReceiverFirstHosted,
}

pub(crate) fn expr_as_i32_literal(expr: &Expr) -> Option<i32> {
    match expr {
        Expr::Literal {
            value: Literal::Integer { value, .. },
            ..
        } => Some(*value as i32),
        _ => None,
    }
}

pub(crate) fn classify_member_call(callee: &Expr, ctx: &LowerCtx) -> MemberCallKind {
    let Expr::Member { object, name, .. } = callee else {
        return MemberCallKind::ArgsEarly;
    };
    if is_static_type_member_call(object, ctx) {
        return MemberCallKind::ArgsEarly;
    }
    if matches!(
        object.as_ref(),
        Expr::Ident { name: mod_name, .. } if ctx.imports.contains(mod_name)
    ) {
        return MemberCallKind::ArgsEarly;
    }
    if expr_is_string_like(object, ctx) && lookup_string_method(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_array_like(object, ctx) && lookup_array_method(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_float64_like(object, ctx) && lookup_float64_math_host(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    if expr_is_float32_like(object, ctx) && lookup_float32_math_host(name).is_some() {
        return MemberCallKind::ReceiverFirstHosted;
    }
    MemberCallKind::InstanceMethod
}

pub(crate) fn method_id_mir_op(
    method: MethodId,
    surface: &str,
    method_name: &str,
    ctx: &LowerCtx,
) -> MirOp {
    if method == MethodId::ArrSorted {
        return MirOp::ArrSorted;
    }
    if let Some(idx) = method_id_call_host_index(method) {
        return MirOp::CallHost(idx);
    }
    collection_method_mir_op(surface, method_name, ctx)
}
