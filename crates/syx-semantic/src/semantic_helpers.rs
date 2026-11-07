//! SYX semantic free helpers (attrs, spans, surfaces).
//! Extracted in S2-I249 (zero semantics).

use crate::TypeId;
use smol_str::SmolStr;
use syx_ast::{Decl, Expr, Module, Stmt};
use syx_stdlib::{
    is_manifest_host_op, lookup_array_method, lookup_dict_method, lookup_set_method,
};

// Phase 87/88: optional downcast (`as?`) semantic surface (AsOptional).
#[allow(dead_code)]
pub(crate) fn as_optional_surface() {}

// Phase 86/87/88: `select { case ... }` lowers via __syx_channel_select / __syx_channel_select_default.
#[allow(dead_code)]
pub(crate) const CHANNEL_SELECT_SURFACE: &str = "__syx_channel_select";

pub(crate) fn is_forbidden_internal_import(name: &str) -> bool {
    matches!(name, "Internal" | "_Builtin" | "_Internal" | "internal")
}

pub(crate) fn type_collection_surface(ty: &TypeId) -> Option<&'static str> {
    match ty {
        TypeId::Array(_) => Some("Array"),
        TypeId::Set(_) => Some("Set"),
        TypeId::Dictionary { .. } => Some("Dictionary"),
        TypeId::Named(n) if n.as_str() == "Array" => Some("Array"),
        TypeId::Named(n) if n.as_str() == "Set" => Some("Set"),
        TypeId::Named(n) if n.as_str() == "Dictionary" => Some("Dictionary"),
        _ => None,
    }
}

/// Phase 50 (S025): collection methods that still resolve only via legacy `MethodCall`.
pub(crate) fn is_retired_collection_methodcall(surface: &str, method: &str) -> bool {
    // sorted → MirOp::ArrSorted; flatMap → CallHost `__syx_array_flat_map` (MethodId).
    if surface == "Array" && matches!(method, "sorted" | "flatMap") {
        return false;
    }
    let in_table = match surface {
        "Array" => lookup_array_method(method).is_some(),
        "Set" => lookup_set_method(method).is_some(),
        "Dictionary" => lookup_dict_method(method).is_some(),
        _ => false,
    };
    in_table && !is_manifest_host_op(surface, method)
}

pub(crate) fn default_assoc_type_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    match (protocol, assoc) {
        ("Sequence", "Element") | ("Collection", "Element") | ("IteratorProtocol", "Element") => {
            Some("Int")
        }
        ("Hashable", "HashValue") => Some("Int"),
        _ => None,
    }
}

pub(crate) fn has_attr(attrs: &[syx_ast::Attribute], target: &str) -> bool {
    attrs.iter().any(|a| a.name == target)
}

pub(crate) fn attached_conformance_protocols(attrs: &[syx_ast::Attribute]) -> Vec<SmolStr> {
    attrs
        .iter()
        .filter(|a| a.name.as_str() == "attached")
        .filter_map(|a| {
            let mut args = a.args.iter();
            match args.next() {
                Some(syx_ast::AttributeArg::Ident(role)) if role.as_str() == "conformance" => {}
                _ => return None,
            }
            match args.next() {
                Some(syx_ast::AttributeArg::Ident(proto)) => Some(proto.clone()),
                _ => None,
            }
        })
        .collect()
}

pub(crate) fn type_expr_uses_param(ty: &syx_ast::TypeExpr, params: &[SmolStr]) -> Option<SmolStr> {
    match ty {
        syx_ast::TypeExpr::Named { name, generics } => {
            if params.contains(name) {
                return Some(name.clone());
            }
            for g in generics {
                if let Some(p) = type_expr_uses_param(g, params) {
                    return Some(p);
                }
            }
            None
        }
        syx_ast::TypeExpr::Optional(inner)
        | syx_ast::TypeExpr::Array(inner)
        | syx_ast::TypeExpr::OpaqueSome(inner)
        | syx_ast::TypeExpr::OpaqueAny(inner) => type_expr_uses_param(inner, params),
        syx_ast::TypeExpr::Dictionary { key, value } => type_expr_uses_param(key, params)
            .or_else(|| type_expr_uses_param(value, params)),
        _ => None,
    }
}

pub(crate) fn retired_ops_hint(name: &str) -> String {
    match name {
        "Text" => "String".to_string(),
        "Arrays" => "Array".to_string(),
        "BoolOps" => "Bool".to_string(),
        "CharacterOps" => "Character".to_string(),
        n if n.ends_with("Ops") => n.strip_suffix("Ops").unwrap_or(n).to_string(),
        _ => name.to_string(),
    }
}

pub(crate) fn attr_has_ident_arg(attrs: &[syx_ast::Attribute], name: &str, arg: &str) -> bool {
    attrs.iter().any(|a| {
        a.name == name
            && a
                .args
                .iter()
                .any(|a| matches!(a, syx_ast::AttributeArg::Ident(i) if i.as_str() == arg))
    })
}

pub(crate) fn module_uses_task_detach(module: &Module) -> bool {
    module.decls.iter().any(|decl| {
        if let Decl::Function { body, .. } = decl {
            stmts_use_task_detach(body)
        } else {
            false
        }
    })
}

pub(crate) fn stmts_use_task_detach(stmts: &[Stmt]) -> bool {
    stmts.iter().any(|s| match s {
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
            expr_uses_task_detach(expr)
        }
        Stmt::VarDecl { init: Some(expr), .. } => expr_uses_task_detach(expr),
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            stmts_use_task_detach(then_branch)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| stmts_use_task_detach(b))
        }
        Stmt::Block { stmts, .. } => stmts_use_task_detach(stmts),
        _ => false,
    })
}

pub(crate) fn expr_uses_task_detach(expr: &Expr) -> bool {
    match expr {
        Expr::Call { callee, .. } => {
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if name.as_str() == "__syx_task_detach" {
                    return true;
                }
            }
            false
        }
        Expr::TaskBlock { detached, .. } => *detached,
        Expr::Binary { left, right, .. } => {
            expr_uses_task_detach(left) || expr_uses_task_detach(right)
        }
        Expr::Unary { expr, .. } => expr_uses_task_detach(expr),
        Expr::Group { expr, .. } => expr_uses_task_detach(expr),
        Expr::Await { expr, .. } => expr_uses_task_detach(expr),
        _ => false,
    }
}

pub fn expr_span_start(expr: &Expr) -> u32 {
    expr_span(expr).start as u32
}

pub(crate) fn expr_span(expr: &Expr) -> syx_diagnostics::Span {
    match expr {
        Expr::Literal { span, .. }
        | Expr::Ident { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Call { span, .. }
        | Expr::Member { span, .. }
        | Expr::Index { span, .. }
        | Expr::Assign { span, .. }
        | Expr::CompoundAssign { span, .. }
        | Expr::MemberAssign { span, .. }
        | Expr::IndexAssign { span, .. }
        | Expr::Closure { span, .. }
        | Expr::Group { span, .. }
        | Expr::Array { span, .. }
        | Expr::Await { span, .. }
        | Expr::OptionalChain { span, .. }
        | Expr::ForceUnwrap { span, .. }
        | Expr::HalfOpenRange { span, .. }
        | Expr::ClosedRange { span, .. }
        | Expr::RangeDyn { span, .. }
        | Expr::VariantPattern { span, .. }
        | Expr::TuplePattern { span, .. }
        | Expr::TaskBlock { span, .. }
        | Expr::Borrow { span, .. }
        | Expr::KeyPath { span, .. }
        | Expr::MacroInvoke { span, .. }
        | Expr::OptionalCast { span, .. }
        | Expr::Is { span, .. }
        | Expr::Cast { span, .. }
        | Expr::TypeSelf { span, .. }
        | Expr::Super { span, .. }
        | Expr::Tuple { span, .. } => *span,
    }
}
