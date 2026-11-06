//! SYX MIR host / collection / numeric-cast helpers.
//! Extracted in S2-I207 (zero semantics; crate-private).

use crate::lower_expr;
use crate::method_id_mir_op;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_type_infer_helpers::{
    infer_expr_init_type,
};
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::{CallArg, Expr};
use syx_stdlib::{
    from_type_name, host_fn_index, host_fn_index_by_name, is_float,
    is_implicit_convertible, is_integer, is_integer_narrowing, lookup_array_method,
    lookup_dict_method, lookup_float32_math_host, lookup_float64_math_host, lookup_set_method,
    lookup_type_method, manifest_host_name, resolve_alias, widen_for_binary,
    HostFnId, PLATFORM_INT,
};

pub(crate) fn resolve_host_call(
    object: &Expr,
    method: &str,
    ctx: &LowerCtx,
) -> Option<(HostFnId, bool)> {
    // ADR-236 / M2-S4-01: HTTPResponse MethodId accessors must not fall through to ObjectGet
    // (status is also a bag field — ObjectGet "works" by accident; text/ok do not).
    // Parity with codegen `resolve_call`: user enum/FinalClass case ctors (e.g. `WireStatus.ok(...)`)
    // and Result/BigInt statics must not be stolen by HttpResponse method-name shortcuts.
    if let Expr::Ident { name: type_name, .. } = object {
        if matches!(type_name.as_str(), "Result" | "BigInt") {
            return None;
        }
        if ctx.module.enum_classes.contains(type_name)
            || ctx.module.final_classes.contains(type_name)
        {
            return None;
        }
    }
    match method {
        "json" => return Some((HostFnId::HttpResponseJson, true)),
        "status" => return Some((HostFnId::HttpResponseStatus, true)),
        "text" | "body" => return Some((HostFnId::HttpResponseText, true)),
        "ok" => return Some((HostFnId::HttpResponseOk, true)),
        "header" => return Some((HostFnId::HttpResponseHeader, true)),
        "headers" => return Some((HostFnId::HttpResponseHeaders, true)),
        _ => {}
    }
    if let Expr::Ident { name: type_name, .. } = object {
        if let Some(tm) = lookup_type_method(type_name, method) {
            return tm.host.map(|h| (h, false));
        }
        if matches!(type_name.as_str(), "Float64" | "Double") {
            if let Some(h) = lookup_float64_math_host(method) {
                return Some((h, false));
            }
        }
        if matches!(type_name.as_str(), "Float32" | "Float") {
            if let Some(h) = lookup_float32_math_host(method) {
                return Some((h, false));
            }
        }
    }
    None
}

pub(crate) fn extension_call_op(type_name: &str, method: &str, ctx: &LowerCtx) -> MirOp {
    match method {
        // Runtime values (arrays from split, dict literals, etc.) are unwrapped VM
        // values; stdlib `{Type}_description` expects a MutRefCell wrapper.
        "description" => MirOp::CallHost(host_fn_index(HostFnId::ValueDescription)),
        "debugDescription" => MirOp::CallHost(host_fn_index(HostFnId::ValueDebugDescription)),
        _ => {
            let sym = SmolStr::new(format!("{type_name}_{method}"));
            if ctx.module.callable_extensions.contains(&sym) {
                MirOp::Call(sym)
            } else {
                MirOp::Call(sym)
            }
        }
    }
}

pub(crate) fn is_hosted_collection_method(type_base: &str, method: &str) -> bool {
    syx_stdlib::is_manifest_host_op(type_base, method)
        || matches!(method, "description" | "debugDescription")
}

pub(crate) fn collection_method_mir_op(type_base: &str, method: &str, ctx: &LowerCtx) -> MirOp {
    let base = SmolStr::new(type_base);
    let name = SmolStr::new(method);
    if let Some(host) = manifest_host_name(type_base, method) {
        if let Some(idx) = host_fn_index_by_name(&host) {
            return MirOp::CallHost(idx);
        }
    }
    if ctx.module.extension_methods.contains(&(base.clone(), name.clone())) {
        if matches!(method, "description" | "debugDescription") {
            return extension_call_op(type_base, method, ctx);
        }
        return MirOp::Call(SmolStr::new(format!("{type_base}_{method}")));
    }
    if matches!(method, "description" | "debugDescription") {
        return extension_call_op(type_base, method, ctx);
    }
    MirOp::Call(SmolStr::new(format!("{type_base}_{method}")))
}

pub(crate) fn emit_hosted_collection_call(
    type_base: &str,
    method: &SmolStr,
    object: &Expr,
    args: &[CallArg],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_expr(object, ctx.fresh_reg(), ctx, block);
    for arg in args {
        lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
    }
    let op = if is_hosted_collection_method(type_base, method.as_str()) {
        collection_method_mir_op(type_base, method.as_str(), ctx)
    } else if type_base == "Array" {
        if let Some(m) = lookup_array_method(method) {
            method_id_mir_op(m, "Array", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else if type_base == "Set" {
        if let Some(m) = lookup_set_method(method) {
            method_id_mir_op(m, "Set", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else if type_base == "Dictionary" {
        if let Some(m) = lookup_dict_method(method) {
            method_id_mir_op(m, "Dictionary", method.as_str(), ctx)
        } else {
            collection_method_mir_op(type_base, method.as_str(), ctx)
        }
    } else {
        collection_method_mir_op(type_base, method.as_str(), ctx)
    };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op,
            span: None,
        },
    );
}

pub(crate) fn emit_integer_narrow_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let Some(from_k) = from_type_name(from_ty) else {
        return;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return;
    };
    if !is_integer_narrowing(from_k, to_k) {
        return;
    }
    let from = resolve_alias(from_k).to_byte();
    let to = resolve_alias(to_k).to_byte();
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast { from, to },
            span: None,
        },
    );
}

pub(crate) fn emit_integer_numeric_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) -> bool {
    let Some(from_k) = from_type_name(from_ty) else {
        return false;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return false;
    };
    if !is_integer(from_k) || !is_integer(to_k) {
        return false;
    }
    let from_r = resolve_alias(from_k);
    let to_r = resolve_alias(to_k);
    if from_r == to_r {
        return false;
    }
    if !is_integer_narrowing(from_k, to_k) && !is_implicit_convertible(from_k, to_k) {
        return false;
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast {
                from: from_r.to_byte(),
                to: to_r.to_byte(),
            },
            span: None,
        },
    );
    true
}

pub(crate) fn binary_numeric_kind(left: &Expr, right: &Expr, ctx: &LowerCtx) -> u8 {
    let lk = infer_expr_init_type(left, ctx).and_then(|n| from_type_name(n.as_str()));
    let rk = infer_expr_init_type(right, ctx).and_then(|n| from_type_name(n.as_str()));
    match (lk, rk) {
        (Some(a), Some(b)) => widen_for_binary(a, b)
            .unwrap_or(PLATFORM_INT)
            .to_byte(),
        (Some(a), None) => resolve_alias(a).to_byte(),
        (None, Some(b)) => resolve_alias(b).to_byte(),
        (None, None) => PLATFORM_INT.to_byte(),
    }
}

pub(crate) fn unary_numeric_kind(expr: &Expr, ctx: &LowerCtx) -> u8 {
    infer_expr_init_type(expr, ctx)
        .and_then(|n| from_type_name(n.as_str()))
        .map(|k| resolve_alias(k).to_byte())
        .unwrap_or(PLATFORM_INT.to_byte())
}

pub(crate) fn emit_float_to_int_numeric_cast_if_needed(
    from_ty: &str,
    to_ty: &str,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) -> bool {
    let Some(from_k) = from_type_name(from_ty) else {
        return false;
    };
    let Some(to_k) = from_type_name(to_ty) else {
        return false;
    };
    if !is_float(from_k) || !is_integer(to_k) {
        return false;
    }
    let from_r = resolve_alias(from_k);
    let to_r = resolve_alias(to_k);
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::NumericCast {
                from: from_r.to_byte(),
                to: to_r.to_byte(),
            },
            span: None,
        },
    );
    true
}

