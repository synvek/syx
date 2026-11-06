//! SYX MIR type-inference / expr-type helpers.
//! Extracted in S2-I208 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::ModuleIndex;
use smol_str::SmolStr;
use syx_ast::{BinOp, CallArg, ClosureBody, Expr, Literal, Stmt};
use syx_stdlib::{host_fn_id_by_name, host_fn_return_type, lookup_array_method, lookup_dict_method, lookup_set_method, method_return_type};

pub(crate) fn type_name_is_array(name: &str) -> bool {
    name == "Array" || name.starts_with("Array<")
}

pub(crate) fn type_name_is_set(name: &str) -> bool {
    name == "Set" || name.starts_with("Set<")
}

pub(crate) fn extension_or_hosted_return_type(
    recv_ty: &SmolStr,
    method: &SmolStr,
    ctx: &LowerCtx,
) -> Option<SmolStr> {
    let base = type_name_dispatch_base(recv_ty);
    if let Some(ret) = ctx
        .module
        .extension_method_returns
        .get(&(base.clone(), method.clone()))
        .or_else(|| {
            ctx.module
                .extension_method_returns
                .get(&(recv_ty.clone(), method.clone()))
        })
    {
        // Preserve element params when the extension only declares bare `Set` / `Array`.
        if (ret.as_str() == "Set" && type_name_is_set(recv_ty.as_str()))
            || (ret.as_str() == "Array" && type_name_is_array(recv_ty.as_str()))
        {
            return Some(recv_ty.clone());
        }
        return Some(ret.clone());
    }
    let hosted = match base.as_str() {
        "Set" => lookup_set_method(method).map(method_return_type),
        "Array" => lookup_array_method(method).map(method_return_type),
        "Dictionary" => lookup_dict_method(method).map(method_return_type),
        _ => None,
    }?;
    if (hosted == "Set" && type_name_is_set(recv_ty.as_str()))
        || (hosted == "Array" && type_name_is_array(recv_ty.as_str()))
    {
        return Some(recv_ty.clone());
    }
    Some(SmolStr::new(hosted))
}


pub(crate) fn infer_service_resolve_type_name(callee: &Expr, args: &[CallArg]) -> Option<SmolStr> {
    match callee {
        Expr::Ident { name, .. }
            if matches!(name.as_str(), "__syx_service_resolve" | "resolve") => {}
        Expr::Member { object, name, .. }
            if name.as_str() == "resolve"
                && matches!(
                    object.as_ref(),
                    Expr::Ident { name: m, .. } if m.as_str() == "Di"
                ) => {}
        _ => return None,
    }
    let arg = args.first()?;
    match &arg.value {
        Expr::Literal {
            value: Literal::String(type_name),
            ..
        } => Some(SmolStr::new(type_name.as_str())),
        Expr::Ident { name: type_name, .. } => Some(type_name.clone()),
        _ => None,
    }
}


pub(crate) fn infer_expr_init_type(expr: &Expr, ctx: &LowerCtx) -> Option<SmolStr> {
    match expr {
        Expr::Array { .. } => Some(SmolStr::new("Array")),
        Expr::Ident { name, .. } => ctx.local_types.get(name).cloned(),
        Expr::Literal {
            value: Literal::String(_) | Literal::Interpolated(_),
            ..
        } => Some(SmolStr::new("String")),
        Expr::Literal {
            value: Literal::BigInt(_),
            ..
        } => Some(SmolStr::new("BigInt")),
        Expr::Literal {
            value: Literal::Integer { suffix, .. },
            ..
        } if suffix.as_deref() == Some("n") => Some(SmolStr::new("BigInt")),
        // S1-I7: `let x = 3; x.doubled()` — bind platform int so extension Call resolves.
        Expr::Literal {
            value: Literal::Integer { suffix, .. },
            ..
        } => Some(SmolStr::new(
            suffix
                .as_deref()
                .map(|s| s.trim_end_matches('i').trim_end_matches('u'))
                .unwrap_or(if cfg!(target_pointer_width = "64") {
                    "Int64"
                } else {
                    "Int32"
                }),
        )),
        Expr::Literal {
            value: Literal::Dict(_),
            ..
        } => Some(SmolStr::new("Dictionary")),
        Expr::Literal {
            value: Literal::Double { suffix, .. },
            ..
        } => {
            let s = suffix.as_deref().unwrap_or("");
            Some(SmolStr::new(
                if s == "f32" || s == "f" {
                    "Float32"
                } else {
                    "Float64"
                },
            ))
        }
        Expr::Call { callee, args, .. } => {
            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                return Some(type_name);
            }
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if let Some(ret) = ctx.module.function_returns.get(name) {
                    return Some(ret.clone());
                }
                if let Some(host_id) = host_fn_id_by_name(name.as_str()) {
                    let ret = host_fn_return_type(host_id);
                    if ret == "Array" {
                        return Some(SmolStr::new("Array"));
                    }
                    if ret == "String" {
                        return Some(SmolStr::new("String"));
                    }
                    if !ret.is_empty() && ret != "Nil" && ret != "Void" {
                        return Some(SmolStr::new(ret));
                    }
                }
            }
            if let Some(mangled) = syx_ast::nested_type_ctor_mangled(callee) {
                return Some(mangled);
            }
            if let Expr::Member { object, .. } = callee.as_ref() {
                if let Expr::Ident { name: mod_name, .. } = object.as_ref() {
                    if matches!(mod_name.as_str(), "BigInt" | "Result") {
                        return Some(mod_name.clone());
                    }
                }
            }
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                if matches!(
                    name.as_str(),
                    "split" | "matches" | "readLines" | "map" | "filter" | "sorted" | "reverse"
                ) {
                    return Some(SmolStr::new("Array"));
                }
                if let Some(recv) = infer_object_type_name(object, ctx) {
                    if let Some(ret) = extension_or_hosted_return_type(&recv, name, ctx) {
                        return Some(ret);
                    }
                }
            }
            None
        }
        _ => None,
    }
}


pub(crate) fn expr_is_float64_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Literal {
        value: Literal::Double { suffix, .. },
        ..
    } = expr
    {
        // bare / f64 → Float64; f32 / f → Float32 (not this helper)
        let s = suffix.as_deref().unwrap_or("");
        return s.is_empty() || s == "f64" || s == "d";
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| {
            matches!(
                type_name_dispatch_base(t).as_str(),
                "Float64" | "Double"
            )
        }) {
            return true;
        }
    }
    infer_object_type_name(expr, ctx).is_some_and(|t| {
        matches!(
            type_name_dispatch_base(&t).as_str(),
            "Float64" | "Double"
        )
    })
}


pub(crate) fn expr_is_float32_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Literal {
        value: Literal::Double { suffix, .. },
        ..
    } = expr
    {
        let s = suffix.as_deref().unwrap_or("");
        return s == "f32" || s == "f";
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| {
            matches!(
                type_name_dispatch_base(t).as_str(),
                "Float32" | "Float"
            )
        }) {
            return true;
        }
    }
    infer_object_type_name(expr, ctx).is_some_and(|t| {
        matches!(
            type_name_dispatch_base(&t).as_str(),
            "Float32" | "Float"
        )
    })
}


pub(crate) fn expr_is_array_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(expr, Expr::Array { .. }) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx
            .local_types
            .get(name)
            .is_some_and(|t| type_name_is_array(t.as_str()))
        {
            return true;
        }
    }
    infer_expr_init_type(expr, ctx).is_some_and(|t| type_name_is_array(t.as_str()))
}


pub(crate) fn expr_is_set_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if let Expr::Ident { name, .. } = expr {
        return ctx
            .local_types
            .get(name)
            .is_some_and(|t| type_name_is_set(t.as_str()));
    }
    false
}


pub(crate) fn expr_is_dict_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(expr, Expr::Literal { value: Literal::Dict(_), .. }) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        return ctx.local_types.get(name).is_some_and(|t| {
            let base = type_name_dispatch_base(t);
            base.as_str() == "Dictionary" || base.as_str() == "Object"
        });
    }
    false
}


pub(crate) fn expr_is_string_like(expr: &Expr, ctx: &LowerCtx) -> bool {
    if matches!(
        expr,
        Expr::Literal {
            value: Literal::String(_) | Literal::Interpolated(_),
            ..
        }
    ) {
        return true;
    }
    if let Expr::Ident { name, .. } = expr {
        if ctx.local_types.get(name).is_some_and(|t| t.as_str() == "String") {
            return true;
        }
    }
    // Chained `a + b + c`: the left of the outer Add is itself a string Concat.
    // Without this, the second `+` lowers to numeric Add → 0 (breaks SynUI handle CSVs).
    if let Expr::Binary { left, op, right, .. } = expr {
        if matches!(op, BinOp::Add)
            && (expr_is_string_like(left, ctx) || expr_is_string_like(right, ctx))
        {
            return true;
        }
    }
    infer_expr_init_type(expr, ctx).is_some_and(|t| t.as_str() == "String")
        || infer_object_type_name(expr, ctx).is_some_and(|t| t.as_str() == "String")
}


fn normalize_type_arg(name: SmolStr) -> SmolStr {
    match name.as_str() {
        "Int" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "Int64"
        } else {
            "Int32"
        }),
        "UInt" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "UInt64"
        } else {
            "UInt32"
        }),
        // Bare float literals lower as "Float"; semantic mono uses Float64 (`type_name_with_alias`).
        "Float" | "Double" => SmolStr::new("Float64"),
        other => SmolStr::new(other),
    }
}


pub(crate) fn infer_static_method_type_args(args: &[CallArg], ctx: &LowerCtx) -> Vec<SmolStr> {
    for arg in args {
        if let Expr::Ident { name, .. } = &arg.value {
            if let Some(ty) = ctx.local_types.get(name.as_str()) {
                if ty.starts_with("Array<") && ty.ends_with('>') {
                    let inner = ty.trim_start_matches("Array<").trim_end_matches('>');
                    return vec![normalize_type_arg(SmolStr::new(inner))];
                }
            }
        }
    }
    vec![SmolStr::new("Unknown")]
}


pub(crate) fn is_platform_value_type(name: &str) -> bool {
    matches!(
        name,
        "Array"
            | "Set"
            | "Dictionary"
            | "String"
            | "Object"
            | "OrderedSet"
            | "Deque"
            | "Result"
            | "BigInt"
            | "Int"
            | "Int8"
            | "Int16"
            | "Int32"
            | "Int64"
            | "UInt"
            | "UInt8"
            | "UInt16"
            | "UInt32"
            | "UInt64"
            | "Float"
            | "Float32"
            | "Float64"
            | "Double"
            | "Bool"
    )
}


pub(crate) fn instance_self_mut_ref_method(name: &SmolStr, module: &ModuleIndex) -> bool {
    let Some((type_name, rest)) = syx_ast::parse_mangled_instance_method(name.as_str()) else {
        return false;
    };
    if rest.as_str() == "init" || rest.as_str() == "deinit" {
        return false;
    }
    // Core value / bag surfaces are not MutRefCell-wrapped class instances.
    // Treating them as such ObjectGet(`__syx_ref`)→Nil and breaks `array.count` / hosts.
    // S1-I7: platform ints in `extension Int` must keep bare numeric `self`.
    if is_platform_value_type(type_name.as_str()) {
        return false;
    }
    module.class_types.contains_key(&type_name) || module.actor_types.contains(&type_name)
}


pub(crate) fn type_name_dispatch_base(name: &SmolStr) -> SmolStr {
    SmolStr::new(name.split('<').next().unwrap_or(name.as_str()))
}


pub(crate) fn is_static_type_member_call(object: &Expr, ctx: &LowerCtx) -> bool {
    let Expr::Ident { name, .. } = object else {
        return false;
    };
    ctx.imports.contains(name)
        || ctx.module.class_types.contains_key(name)
        || ctx.module.enum_classes.contains(name)
        || ctx.module.final_classes.contains(name)
        || ctx.module.dataclass_names.contains(name)
        || syx_stdlib::is_builtin_type(name.as_str())
}


pub(crate) fn class_name_from_mangled(func_name: &str) -> Option<SmolStr> {
    if let Some(base) = func_name.strip_suffix("_init") {
        return Some(SmolStr::new(base));
    }
    syx_ast::parse_mangled_instance_method(func_name).map(|(class, _)| class)
}


fn infer_member_field_type(object: &Expr, field: &SmolStr, ctx: &LowerCtx) -> Option<SmolStr> {
    let class_name = match object {
        Expr::Ident { name, .. } if name == "self" => {
            class_name_from_mangled(ctx.func_name.as_str())?
        }
        Expr::Ident { name, .. } => ctx.local_types.get(name).cloned()?,
        _ => infer_object_type_name(object, ctx)?,
    };
    ctx.module
        .class_field_types
        .get(&class_name)?
        .get(field)
        .cloned()
}


pub(crate) fn infer_object_type_name(object: &Expr, ctx: &LowerCtx) -> Option<SmolStr> {
    match object {
        Expr::Member { object, name, .. } => infer_member_field_type(object, name, ctx),
        Expr::Ident { name, .. } => ctx
            .local_types
            .get(name)
            .cloned()
            .or_else(|| {
                if ctx.module.dataclass_names.contains(name)
                    || ctx.module.class_types.contains_key(name)
                    || ctx.module.actor_types.contains(name)
                    || ctx.module.enum_classes.contains(name)
                {
                    Some(name.clone())
                } else {
                    None
                }
            }),
        Expr::Literal { value: _, .. } => Some(infer_expr_type_name(object)),
        Expr::Call { callee, args, .. } => {
            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                return Some(type_name);
            }
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if let Some(host_id) = host_fn_id_by_name(name.as_str()) {
                    let ret = host_fn_return_type(host_id);
                    if ret == "String" {
                        return Some(SmolStr::new("String"));
                    }
                    if ret == "Array" {
                        return Some(SmolStr::new("Array"));
                    }
                    if !ret.is_empty() && ret != "Nil" && ret != "Void" {
                        return Some(SmolStr::new(ret));
                    }
                }
                if ctx.module.dataclass_names.contains(name)
                    || ctx.module.class_types.contains_key(name)
                    || ctx.module.actor_types.contains(name)
                {
                    return Some(name.clone());
                }
            }
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                    if let Some(ret) =
                        extension_or_hosted_return_type(type_name, name, ctx)
                    {
                        return Some(ret);
                    }
                }
                if let Some(recv) = infer_object_type_name(object, ctx) {
                    if let Some(ret) = extension_or_hosted_return_type(&recv, name, ctx) {
                        return Some(ret);
                    }
                }
            }
            if let Some(mangled) = syx_ast::nested_type_ctor_mangled(callee) {
                if ctx.module.class_types.contains_key(&mangled) {
                    return Some(mangled);
                }
            }
            None
        }
        _ => None,
    }
}


fn array_method_index(method: &str) -> Option<u32> {
    syx_stdlib::lookup_array_method(method).map(|m| m.index())
}


pub(crate) fn binop_operator_symbol(op: BinOp) -> Option<&'static str> {
    match op {
        BinOp::Add => Some("+"),
        BinOp::Sub => Some("-"),
        BinOp::Mul => Some("*"),
        BinOp::Div => Some("/"),
        BinOp::Eq => Some("=="),
        BinOp::NotEq => Some("!="),
        BinOp::Lt => Some("<"),
        BinOp::LtEq => Some("<="),
        BinOp::Gt => Some(">"),
        BinOp::GtEq => Some(">="),
        BinOp::BitAnd => Some("&"),
        BinOp::BitOr => Some("|"),
        BinOp::BitXor => Some("^"),
        BinOp::Shl => Some("<<"),
        BinOp::Shr => Some(">>"),
        _ => None,
    }
}


pub(crate) fn infer_expr_type_name(expr: &Expr) -> SmolStr {
    match expr {
        Expr::Literal { value, .. } => match value {
            Literal::Integer { suffix, .. } if suffix.as_deref() == Some("n") => {
                SmolStr::new("BigInt")
            }
            Literal::Integer { suffix, .. } => SmolStr::new(
                suffix
                    .as_deref()
                    .map(|s| s.trim_end_matches('i').trim_end_matches('u'))
                    .unwrap_or(if cfg!(target_pointer_width = "64") {
                        "Int64"
                    } else {
                        "Int32"
                    }),
            ),
            Literal::Double { suffix, .. } => SmolStr::new(
                suffix
                    .as_deref()
                    .map(|s| s.trim_end_matches('f'))
                    .unwrap_or("Float"),
            ),
            Literal::Bool(_) => SmolStr::new("Bool"),
            Literal::String(_) | Literal::Interpolated(_) => SmolStr::new("String"),
            Literal::BigInt(_) => SmolStr::new("BigInt"),
            _ => SmolStr::new("Unknown"),
        },
        // Uncontexted Ident fallback (binop / host paths): keep historical name-as-type.
        Expr::Ident { name, .. } => name.clone(),
        Expr::Array { elements, .. } => {
            let elem = elements
                .first()
                .map(infer_expr_type_name)
                .unwrap_or_else(|| SmolStr::new("Unknown"));
            SmolStr::new(format!("Array<{elem}>"))
        }
        _ => SmolStr::new("Unknown"),
    }
}


fn unwrap_collection_elem_type_arg(ty: &SmolStr) -> Option<SmolStr> {
    ty.as_str()
        .strip_prefix("Array<")
        .and_then(|s| s.strip_suffix('>'))
        .or_else(|| {
            ty.as_str()
                .strip_prefix("Set<")
                .and_then(|s| s.strip_suffix('>'))
        })
        .map(|inner| normalize_type_arg(SmolStr::new(inner)))
}


fn infer_closure_result_type_name(body: &ClosureBody, ctx: &LowerCtx) -> SmolStr {
    match body {
        ClosureBody::Expr(e) => infer_generic_arg_type_name(e, ctx),
        ClosureBody::Block(stmts) => stmts
            .iter()
            .rev()
            .find_map(|s| match s {
                Stmt::Return { value: Some(v), .. } | Stmt::Expr { expr: v, .. } => {
                    Some(infer_generic_arg_type_name(v, ctx))
                }
                _ => None,
            })
            .unwrap_or_else(|| SmolStr::new("Unknown")),
    }
}


pub(crate) fn infer_generic_arg_type_name(expr: &Expr, ctx: &LowerCtx) -> SmolStr {
    match expr {
        // S1-I8: zipMap/map U/C from closure body result (not always platform Int).
        Expr::Closure { body, .. } => {
            let inferred = infer_closure_result_type_name(body, ctx);
            if inferred.as_str() != "Unknown" {
                return normalize_type_arg(inferred);
            }
            SmolStr::new(if cfg!(target_pointer_width = "64") {
                "Int64"
            } else {
                "Int32"
            })
        }
        Expr::Ident { name, .. } => {
            let ty = ctx
                .local_types
                .get(name)
                .or_else(|| ctx.module.global_types.get(name));
            if let Some(ty) = ty {
                // `first(arr)` with `arr: Array<Int>` → type arg Int, not Array<…>.
                if let Some(elem) = unwrap_collection_elem_type_arg(ty) {
                    return elem;
                }
                return normalize_nested_type_arg(ty.clone());
            }
            SmolStr::new("Unknown")
        }
        Expr::Array { elements, .. } => {
            // `func f<T>(items: Array<T>)` — type arg is the element, not `Array<…>`.
            let elem = elements
                .first()
                .map(|e| infer_generic_arg_type_name(e, ctx))
                .unwrap_or_else(|| SmolStr::new("Unknown"));
            normalize_type_arg(elem)
        }
        other => normalize_nested_type_arg(infer_expr_type_name(other)),
    }
}


fn normalize_nested_type_arg(name: SmolStr) -> SmolStr {
    if let Some(inner) = name
        .as_str()
        .strip_prefix("Array<")
        .and_then(|s| s.strip_suffix('>'))
    {
        let inner_n = normalize_nested_type_arg(SmolStr::new(inner));
        return SmolStr::new(format!("Array<{inner_n}>"));
    }
    normalize_type_arg(name)
}

