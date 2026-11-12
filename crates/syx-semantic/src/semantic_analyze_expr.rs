//! SYX semantic analyze_expr face (S2-I251).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    borrow_regions_v6_enabled, borrow_regions_v7_enabled, expr_span, is_compatible,
    reflect_type_lower, retired_ops_hint, type_expr_to_id, type_id_to_diag_name, type_id_to_name,
    Scope, Symbol, TypeId,
};
use smol_str::SmolStr;
use syx_ast::{AccessLevel, BinOp, Expr, Literal, TypeExpr, UnaryOp};
use syx_diagnostics::SyxResult;
use syx_stdlib::{default_float_kind, default_integer_kind, from_suffix, int_range, resolve_alias};

impl SemanticAnalyzer {
    pub(crate) fn analyze_expr(&mut self, expr: &Expr) -> SyxResult<TypeId> {
        let ty = self.analyze_expr_inner(expr)?;
        self.record_expr_type(expr, ty.clone());
        Ok(ty)
    }

    pub(crate) fn record_expr_type(&mut self, expr: &Expr, ty: TypeId) {
        self.expr_types.insert(expr_span(expr).start as u32, ty);
    }

    pub(crate) fn literal_numeric_type(&self, value: &Literal) -> TypeId {
        match value {
            Literal::Integer { suffix, .. } => {
                if suffix.as_deref() == Some("n") {
                    return TypeId::Named(SmolStr::new("BigInt"));
                }
                if let Some(s) = suffix {
                    if let Some(kind) = from_suffix(s) {
                        return TypeId::numeric(kind);
                    }
                }
                // Preserve `Int` alias (do not resolve via `TypeId::numeric`).
                TypeId::Numeric(default_integer_kind())
            }
            Literal::Double { suffix, .. } => {
                if let Some(s) = suffix {
                    if let Some(kind) = from_suffix(s) {
                        return TypeId::numeric(kind);
                    }
                }
                TypeId::numeric(default_float_kind())
            }
            _ => TypeId::Unknown,
        }
    }

    pub(crate) fn reference_lifetime(ty: &TypeExpr) -> Option<SmolStr> {
        if let TypeExpr::Reference(_, _, lt) = ty {
            lt.clone()
        } else {
            None
        }
    }

    /// Phase 22: partial `'a` inference for elided reference lifetimes.
    pub(crate) fn infer_elided_lifetime(ty: &TypeExpr) -> Option<SmolStr> {
        if let TypeExpr::Reference(_, _, lt) = ty {
            Some(lt.clone().unwrap_or_else(|| SmolStr::new("'a")))
        } else {
            None
        }
    }

    pub(crate) fn infer_return_lifetime(
        params: &[syx_ast::Param],
        return_type: &Option<TypeExpr>,
    ) -> Option<SmolStr> {
        let return_ty = return_type.as_ref()?;
        let TypeExpr::Reference(_, _, lt) = return_ty else {
            return None;
        };
        if lt.is_some() {
            return lt.clone();
        }
        let elided_ref_params = params
            .iter()
            .filter(|p| {
                matches!(
                    p.ty.as_ref(),
                    Some(TypeExpr::Reference(_, _, inner_lt)) if inner_lt.is_none()
                )
            })
            .count();
        if elided_ref_params == 1 {
            Some(SmolStr::new("'a"))
        } else {
            None
        }
    }

    pub(crate) fn assign_elided_lifetimes(ty: &mut TypeExpr, lifetime: &SmolStr) {
        if let TypeExpr::Reference(inner, _, lt) = ty {
            if lt.is_none() {
                *lt = Some(lifetime.clone());
            }
            Self::assign_elided_lifetimes(inner, lifetime);
        }
    }

    pub(crate) fn check_return_lifetime(&mut self, expr: &Expr) {
        let Some(ret_lt) = &self.current_return_lifetime else {
            return;
        };
        if let Expr::Ident { name, .. } = expr {
            if let Some(bind_lt) = self.binding_lifetimes.get(name) {
                if bind_lt != ret_lt {
                    self.errors.push(self.error_at(
                        format!("Lifetime '{bind_lt}' does not outlive '{ret_lt}' in return"),
                        expr_span(expr),
                        "S011",
                    ));
                }
            }
        }
    }

    pub(crate) fn check_literal_in_range(
        &mut self,
        value: &Literal,
        expected: &TypeId,
        span: syx_diagnostics::Span,
    ) {
        let TypeId::Numeric(kind) = expected else {
            return;
        };
        let kind = resolve_alias(*kind);
        let Some((min, max)) = int_range(kind) else {
            return;
        };
        if let Literal::Integer { value: v, .. } = value {
            let v128 = *v as i128;
            if v128 < min || v128 > max {
                self.errors.push(self.error_at(
                    format!(
                        "Integer literal {v} is out of range for type {}",
                        type_id_to_name(expected)
                    ),
                    span,
                    "S011",
                ));
            }
        }
    }

    pub(crate) fn unify_element_types(
        &mut self,
        types: &[TypeId],
        span: syx_diagnostics::Span,
    ) -> TypeId {
        let mut elem = TypeId::Unknown;
        for t in types {
            if matches!(t, TypeId::Unknown) {
                continue;
            }
            if matches!(elem, TypeId::Unknown) {
                elem = t.clone();
                continue;
            }
            if elem != *t && !self.is_compatible(&elem, t) && !self.is_compatible(t, &elem) {
                self.errors.push(self.error_at(
                    format!(
                        "Collection elements have inconsistent types: expected {}, found {}",
                        type_id_to_diag_name(&elem),
                        type_id_to_diag_name(t)
                    ),
                    span,
                    "S014",
                ));
            }
        }
        elem
    }

    pub(crate) fn analyze_expr_inner(&mut self, expr: &Expr) -> SyxResult<TypeId> {
        match expr {
            Expr::Literal { value, .. } => Ok(match value {
                Literal::BigInt(_) => TypeId::Named(SmolStr::new("BigInt")),
                Literal::Integer { .. } | Literal::Double { .. } => {
                    self.literal_numeric_type(value)
                }
                Literal::Bool(_) => TypeId::Bool,
                Literal::String(_) | Literal::Interpolated(_) | Literal::Regex(_) => TypeId::String,
                Literal::Nil => TypeId::Optional(Box::new(TypeId::Unknown)),
                // Default: dict literal → homogeneous `Dictionary`. Heterogeneous
                // bags require an explicit `: Object` annotation (contextual typing
                // via `analyze_expr_with_expected`).
                Literal::Dict(entries) => {
                    let mut key_types = Vec::new();
                    let mut value_types = Vec::new();
                    for (k, v) in entries {
                        key_types.push(self.analyze_expr(k)?);
                        value_types.push(self.analyze_expr(v)?);
                    }
                    let key_ty = self.unify_element_types(&key_types, expr_span(expr));
                    // Empty `{}` / `[:]` unify to Unknown; Hashable is checked against the
                    // annotation when present (`analyze_expr_with_expected`), not here.
                    if !matches!(key_ty, TypeId::Unknown) {
                        self.validate_dict_key_hashable(&key_ty, expr_span(expr));
                    }
                    let val_ty = self.unify_element_types(&value_types, expr_span(expr));
                    TypeId::Dictionary {
                        key: Box::new(key_ty),
                        value: Box::new(val_ty),
                    }
                }
            }),
            Expr::Ident { name, span, .. } => {
                if matches!(
                    name.as_str(),
                    "Result" | "BigInt" | "SIMD4" | "SIMD8" | "SIMD16"
                ) {
                    return Ok(TypeId::Named(name.clone()));
                }
                // Phase 134: bare class/type name for `Type.staticVar` member access.
                if self.class_names.contains(name) || self.dataclass_names.contains(name) {
                    return Ok(TypeId::Named(name.clone()));
                }
                if syx_stdlib::retired_ops_module_names().contains(&name.as_str()) {
                    return Err(self.error_at(
                        format!(
                            "Module '{name}' was removed in SYX 22.0; use Core builtin '{}' instead",
                            retired_ops_hint(name.as_str())
                        ),
                        *span,
                        "S025",
                    ));
                }
                if self.moved_bindings.contains(name) {
                    return Err(self.error_at(
                        format!("Use of moved value '{name}'"),
                        *span,
                        "S011",
                    ));
                }
                self.check_main_actor_after_await(name, *span);
                // Locals, then enclosing field (before globals — Core `count` etc.).
                if let Some(ty) = self.scope.lookup(name).map(|s| s.ty.clone()) {
                    return Ok(self.deref_type(&ty));
                }
                // S1-I2: bare stored property in struct/class method (`value = value + 1`).
                if let Some(ty) = self.lookup_enclosing_type_field(name) {
                    return Ok(ty);
                }
                if let Some(ty) = self.globals.get(name).map(|s| s.ty.clone()) {
                    return Ok(self.deref_type(&ty));
                }
                Err(self.error_at(format!("Undefined identifier '{name}'"), *span, "S002"))
            }
            Expr::Borrow {
                expr: borrowed,
                mutable,
                ..
            } => {
                if *mutable {
                    if let Expr::Ident { name, .. } = borrowed.as_ref() {
                        if self.mut_borrows.contains(name) {
                            return Err(self.error_at(
                                format!("Cannot borrow '{name}' as mutable more than once"),
                                expr_span(expr),
                                "S011",
                            ));
                        }
                        self.mut_borrows.insert(name.clone());
                    }
                    let inner = self.analyze_expr(borrowed)?;
                    Ok(TypeId::MutReference(Box::new(inner)))
                } else {
                    let inner = self.analyze_expr(borrowed)?;
                    Ok(TypeId::Reference(Box::new(inner)))
                }
            }
            Expr::KeyPath { .. } => Ok(TypeId::Named(SmolStr::new("KeyPath"))),
            Expr::MacroInvoke { args, .. } => {
                for a in args {
                    self.analyze_expr(a)?;
                }
                Ok(TypeId::Unknown)
            }
            Expr::OptionalCast { expr, target, .. } => {
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                Ok(TypeId::Optional(Box::new(to)))
            }
            Expr::Is { expr, target, span } => {
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                if to.is_numeric() {
                    self.errors.push(self.error_at(
                        "'is' cannot be used with numeric types",
                        *span,
                        "S046",
                    ));
                }
                Ok(TypeId::Bool)
            }
            Expr::Cast { expr, target, span } => {
                if let Expr::Call {
                    callee,
                    args: resolve_args,
                    ..
                } = expr.as_ref()
                {
                    let is_typed_resolve = match callee.as_ref() {
                        Expr::Ident { name, .. } => name.as_str() == "__syx_service_resolve",
                        Expr::Member { object, name, .. }
                            if name.as_str() == "resolve"
                                && matches!(
                                    object.as_ref(),
                                    Expr::Ident { name: m, .. } if m.as_str() == "Di"
                                ) =>
                        {
                            true
                        }
                        _ => false,
                    };
                    if is_typed_resolve {
                        if let Some(arg) = resolve_args.first() {
                            if let Some(type_name) =
                                self.service_resolve_type_name_from_arg(&arg.value)
                            {
                                let to = self.resolve_type_expr(target);
                                let resolved = TypeId::Named(type_name.clone());
                                if to != resolved {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "service resolve({type_name}) cast to incompatible type"
                                        ),
                                        *span,
                                        "E010",
                                    ));
                                } else if !self.service_class_names.contains(&type_name) {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "service resolve: '{type_name}' is not a @Service/@Component type"
                                        ),
                                        *span,
                                        "E010",
                                    ));
                                }
                            }
                        }
                    }
                }
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                Ok(to)
            }
            Expr::TypeSelf { type_name, .. } => {
                let _ = reflect_type_lower(type_name.as_str());
                Ok(TypeId::Named(SmolStr::new("Metatype")))
            }
            Expr::Super { span, .. } => {
                if self.current_class_context.is_none() {
                    self.errors.push(self.error_at(
                        "'super' is only valid inside class instance methods",
                        *span,
                        "S055",
                    ));
                }
                Ok(TypeId::Unknown)
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
                ..
            } => {
                let lt = self.analyze_expr(left)?;
                let rt = self.analyze_expr(right)?;
                if matches!(
                    *op,
                    BinOp::Eq | BinOp::NotEq | BinOp::IdenticalEq | BinOp::IdenticalNe
                ) {
                    self.check_equatable_binary(*op, &lt, &rt, *span);
                }
                match op {
                    BinOp::Add => {
                        // String concatenation: `"a" + "b"` → String
                        let l_str = matches!(lt, TypeId::String)
                            || matches!(&lt, TypeId::Named(n) if n.as_str() == "String");
                        let r_str = matches!(rt, TypeId::String)
                            || matches!(&rt, TypeId::Named(n) if n.as_str() == "String");
                        if l_str && r_str {
                            return Ok(TypeId::String);
                        }
                        if let (Some((la, ea)), Some((lb, eb))) = (lt.as_simd(), rt.as_simd()) {
                            if la == lb && ea == eb {
                                if !syx_config::simd_enabled() {
                                    self.errors.push(self.error_at(
                                        "SIMD operations require SIMD to be enabled",
                                        *span,
                                        "S022",
                                    ));
                                }
                                return Ok(TypeId::Simd {
                                    lanes: la,
                                    element: ea,
                                });
                            }
                            self.errors.push(self.error_at(
                                "SIMD operands must have matching lane width and element type",
                                *span,
                                "S014",
                            ));
                            return Ok(TypeId::Unknown);
                        }
                        Ok(self.promote_numeric_binop(lt, rt, *span))
                    }
                    BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Mod
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor => {
                        if let (Some((la, ea)), Some((lb, eb))) = (lt.as_simd(), rt.as_simd()) {
                            if la == lb && ea == eb {
                                if !syx_config::simd_enabled() {
                                    self.errors.push(self.error_at(
                                        "SIMD operations require SIMD to be enabled",
                                        *span,
                                        "S022",
                                    ));
                                }
                                return Ok(TypeId::Simd {
                                    lanes: la,
                                    element: ea,
                                });
                            }
                            self.errors.push(self.error_at(
                                "SIMD operands must have matching lane width and element type",
                                *span,
                                "S014",
                            ));
                            return Ok(TypeId::Unknown);
                        }
                        Ok(self.promote_numeric_binop(lt, rt, *span))
                    }
                    // R-44: shift result type follows the left operand (count may be Int).
                    BinOp::Shl | BinOp::Shr => {
                        if let (Some((la, ea)), Some((lb, eb))) = (lt.as_simd(), rt.as_simd()) {
                            if la == lb && ea == eb {
                                if !syx_config::simd_enabled() {
                                    self.errors.push(self.error_at(
                                        "SIMD operations require SIMD to be enabled",
                                        *span,
                                        "S022",
                                    ));
                                }
                                return Ok(TypeId::Simd {
                                    lanes: la,
                                    element: ea,
                                });
                            }
                            self.errors.push(self.error_at(
                                "SIMD operands must have matching lane width and element type",
                                *span,
                                "S014",
                            ));
                            return Ok(TypeId::Unknown);
                        }
                        if lt.as_numeric_kind().is_some() {
                            Ok(lt)
                        } else {
                            Ok(self.promote_numeric_binop(lt, rt, *span))
                        }
                    }
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::IdenticalEq
                    | BinOp::IdenticalNe
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq => Ok(TypeId::Bool),
                    BinOp::And | BinOp::Or => Ok(TypeId::Bool),
                    BinOp::NilCoalesce => Ok(rt),
                }
            }
            Expr::Ternary {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.analyze_expr(condition)?;
                let then_ty = self.analyze_expr(then_branch)?;
                let else_ty = self.analyze_expr(else_branch)?;
                if is_compatible(&then_ty, &else_ty) || is_compatible(&else_ty, &then_ty) {
                    Ok(then_ty)
                } else if matches!(then_ty, TypeId::Unknown) {
                    Ok(else_ty)
                } else if matches!(else_ty, TypeId::Unknown) {
                    Ok(then_ty)
                } else {
                    Ok(then_ty)
                }
            }
            Expr::Unary { op, expr, .. } => {
                if *op == UnaryOp::Try {
                    self.try_depth += 1;
                    let ty = self.analyze_expr(expr)?;
                    self.try_depth = self.try_depth.saturating_sub(1);
                    return Ok(ty);
                }
                let ty = self.analyze_expr(expr)?;
                match op {
                    UnaryOp::Neg | UnaryOp::BitNot => Ok(self.promote_numeric(ty.clone(), ty)),
                    UnaryOp::Not => Ok(TypeId::Bool),
                    UnaryOp::Try => Ok(ty),
                }
            }
            Expr::Call { callee, args, .. } => {
                let ret = self.analyze_call(callee, args)?;
                for arg in args {
                    if let Expr::Borrow {
                        expr,
                        mutable: true,
                        ..
                    } = &arg.value
                    {
                        if let Expr::Ident { name, .. } = expr.as_ref() {
                            self.mut_borrows.remove(name);
                        }
                    }
                }
                Ok(ret)
            }
            Expr::Member {
                object, name, span, ..
            } => {
                let obj_ty = self.analyze_expr(object)?;
                if let Expr::Ident { name: module, .. } = object.as_ref() {
                    self.check_import_member_access(module.as_str(), name.as_str(), *span)?;
                }
                if let TypeId::Named(enum_name) = &obj_ty {
                    if self
                        .enum_cases
                        .get(enum_name)
                        .is_some_and(|cases| cases.iter().any(|c| c == name))
                    {
                        return Ok(TypeId::Named(enum_name.clone()));
                    }
                }
                self.check_retired_size_alias(&obj_ty, name.as_str(), *span)?;
                self.check_actor_field_access(object, name, &obj_ty, *span);
                self.check_extension_member_where(&obj_ty, name, *span);
                self.check_known_collection_member(&obj_ty, name.as_str(), *span);
                Ok(self.infer_member_type(&obj_ty, name))
            }
            Expr::Index {
                object,
                index,
                span,
                ..
            } => {
                self.analyze_expr(object)?;
                let obj_ty = self.type_of_expr(object);
                match obj_ty {
                    TypeId::Array(inner) => {
                        let idx_ty = TypeId::platform_int();
                        let found_idx = self.analyze_expr_with_expected(index, Some(&idx_ty))?;
                        self.check_assignable_with_literal(
                            idx_ty,
                            found_idx,
                            "index",
                            Some(index),
                            *span,
                        )?;
                        Ok(*inner)
                    }
                    TypeId::String => {
                        self.analyze_expr(index)?;
                        Ok(TypeId::Named(SmolStr::new("Character")))
                    }
                    TypeId::Dictionary { key, value } => {
                        if !matches!(key.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) {
                            let found =
                                self.analyze_expr_with_expected(index, Some(key.as_ref()))?;
                            self.check_assignable_with_literal(
                                *key,
                                found,
                                "key",
                                Some(index),
                                *span,
                            )?;
                        } else {
                            self.analyze_expr(index)?;
                        }
                        Ok(*value)
                    }
                    TypeId::Named(type_name) if self.has_subscript_get(&type_name) => {
                        self.analyze_expr(index)?;
                        Ok(self
                            .subscript_return_type(&type_name)
                            .unwrap_or(TypeId::Unknown))
                    }
                    _ => {
                        self.analyze_expr(index)?;
                        Ok(TypeId::Unknown)
                    }
                }
            }
            Expr::Assign {
                name, value, span, ..
            } => {
                self.check_main_actor_after_await(name, *span);
                let (_mutable, sym_ty, is_enclosing_field) = match self.resolve_assign_target(name)
                {
                    Ok(v) => v,
                    Err((msg, code)) => {
                        return Err(self.error_at(msg, *span, code));
                    }
                };
                let expected = self.assign_expected_type(&sym_ty);
                let ty = if matches!(expected, TypeId::Unknown) {
                    self.analyze_expr(value)?
                } else {
                    self.analyze_expr_with_expected(value, Some(&expected))?
                };
                // Reassignment rebinds the name; prior move of the old value no longer applies.
                self.moved_bindings.remove(name);
                self.mut_borrows.remove(name);
                if is_enclosing_field {
                    self.check_assignable_with_literal(expected, ty, name, Some(value), *span)?;
                } else if matches!(expected, TypeId::Unknown) {
                    if let Some(binding) = self.scope.lookup_mut(name) {
                        binding.ty = ty.clone();
                    }
                    self.track_binding(name.clone(), ty.clone(), span.start);
                } else {
                    self.check_assignable_with_literal(expected, ty, name, Some(value), *span)?;
                }
                Ok(TypeId::Void)
            }
            Expr::CompoundAssign {
                name, value, span, ..
            } => {
                // R-35: bare `x += 1` in methods — same implicit-field path as Assign.
                self.check_main_actor_after_await(name, *span);
                let ty = self.analyze_expr(value)?;
                let (_mutable, sym_ty, _is_enclosing_field) = match self.resolve_assign_target(name)
                {
                    Ok(v) => v,
                    Err((msg, code)) => {
                        return Err(self.error_at(msg, *span, code));
                    }
                };
                let expected = self.assign_expected_type(&sym_ty);
                if matches!(expected, TypeId::Numeric(_)) {
                    let _ = self.promote_numeric(expected.clone(), ty);
                } else {
                    self.check_assignable(expected, ty, name, expr_span(value))?;
                }
                Ok(TypeId::Void)
            }
            Expr::MemberAssign {
                object,
                name,
                value,
                span,
                ..
            } => {
                let obj_ty = self.analyze_expr(object)?;
                if let Some(obj_ty) = self.expr_type(object) {
                    self.check_actor_field_access(object, name, &obj_ty, *span);
                    if let TypeId::Named(type_name) = &obj_ty {
                        if self
                            .getter_only_properties
                            .contains(&(type_name.clone(), name.clone()))
                        {
                            return Err(self.error_at(
                                format!("Cannot assign to read-only property '{type_name}.{name}'"),
                                *span,
                                "S057",
                            ));
                        }
                    }
                }
                self.analyze_expr(value)?;
                let _ = obj_ty;
                Ok(TypeId::Void)
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                span,
                ..
            } => {
                self.analyze_expr(object)?;
                let obj_ty = self.type_of_expr(object);
                if let TypeId::Named(ref type_name) = obj_ty {
                    if self.has_subscript_set(type_name) {
                        self.analyze_expr(index)?;
                        self.analyze_expr(value)?;
                        return Ok(TypeId::Void);
                    }
                }
                if let TypeId::Dictionary { key, value: val_ty } = obj_ty {
                    if !matches!(key.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) {
                        let found_key =
                            self.analyze_expr_with_expected(index, Some(key.as_ref()))?;
                        self.check_assignable_with_literal(
                            *key,
                            found_key,
                            "key",
                            Some(index),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(index)?;
                    }
                    if !matches!(val_ty.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) {
                        let found_val =
                            self.analyze_expr_with_expected(value, Some(val_ty.as_ref()))?;
                        self.check_assignable_with_literal(
                            *val_ty,
                            found_val,
                            "value",
                            Some(value),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(value)?;
                    }
                    return Ok(TypeId::Void);
                }
                if let TypeId::Array(elem) = obj_ty {
                    let idx_ty = TypeId::platform_int();
                    let found_idx = self.analyze_expr_with_expected(index, Some(&idx_ty))?;
                    self.check_assignable_with_literal(
                        idx_ty,
                        found_idx,
                        "index",
                        Some(index),
                        *span,
                    )?;
                    if !matches!(elem.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) {
                        let found_val =
                            self.analyze_expr_with_expected(value, Some(elem.as_ref()))?;
                        self.check_assignable_with_literal(
                            *elem,
                            found_val,
                            "value",
                            Some(value),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(value)?;
                    }
                    return Ok(TypeId::Void);
                }
                self.analyze_expr(index)?;
                self.analyze_expr(value)?;
                Ok(TypeId::Void)
            }
            Expr::Closure {
                params,
                body,
                return_type,
                span,
                ..
            } => {
                if !self.mut_borrows.is_empty() {
                    self.errors.push(self.error_at(
                        "Closure cannot capture an active `&mut` borrow from an outer region",
                        expr_span(expr),
                        "S011",
                    ));
                }
                if borrow_regions_v7_enabled()
                    && self.current_function_is_async
                    && !self.immutable_ref_bindings.is_empty()
                {
                    for name in self.immutable_ref_bindings.clone() {
                        self.errors.push(self.error_at(
                            format!(
                                "Immutable borrow '{name}' cannot be captured in nested async closure region"
                            ),
                            expr_span(expr),
                            "S011",
                        ));
                    }
                }
                let annotated = return_type.as_ref().map(type_expr_to_id);
                let parent = Some(Box::new(std::mem::take(&mut self.scope)));
                let mut closure_scope = Scope {
                    parent,
                    ..Default::default()
                };
                for param in params {
                    self.define_in_scope(
                        &mut closure_scope,
                        Symbol {
                            name: param.name.clone(),
                            ty: param
                                .ty
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Unknown),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                }
                std::mem::swap(&mut self.scope, &mut closure_scope);
                // Expression type of a closure is its result type (for HOF inference).
                let result = if let Some(ann) = annotated {
                    match body {
                        syx_ast::ClosureBody::Expr(e) => {
                            let _ = self.analyze_expr(e)?;
                        }
                        syx_ast::ClosureBody::Block(stmts) => {
                            self.analyze_block(stmts)?;
                        }
                    }
                    Ok(ann)
                } else {
                    match body {
                        syx_ast::ClosureBody::Expr(e) => self.analyze_expr(e),
                        syx_ast::ClosureBody::Block(stmts) => {
                            self.analyze_block(stmts)?;
                            // Unannotated block: no stored result type (use `-> T` or expr body).
                            Ok(TypeId::Unknown)
                        }
                    }
                };
                self.scope = *self.scope.parent.take().unwrap_or_default();
                result
            }
            Expr::TaskBlock { body, .. } => {
                self.analyze_block(body)?;
                Ok(TypeId::Unknown)
            }
            Expr::Group { expr, .. } => self.analyze_expr(expr),
            Expr::Tuple { elements, .. } => {
                let mut elem_types = Vec::new();
                for (_, el) in elements {
                    elem_types.push(self.analyze_expr(el)?);
                }
                let typed: Vec<(Option<SmolStr>, TypeId)> = elements
                    .iter()
                    .zip(elem_types.iter())
                    .map(|((label, _), ty)| (label.clone(), ty.clone()))
                    .collect();
                Ok(TypeId::Tuple { elements: typed })
            }
            Expr::Array { elements, .. } => {
                let mut elem_types = Vec::new();
                for el in elements {
                    elem_types.push(self.analyze_expr(el)?);
                }
                let elem_ty = self.unify_element_types(&elem_types, expr_span(expr));
                Ok(TypeId::Array(Box::new(elem_ty)))
            }
            Expr::Await {
                expr: awaited,
                span,
            } => {
                if !self.mut_borrows.is_empty() {
                    self.errors.push(self.error_at(
                        "Cannot await while an active `&mut` borrow is in scope",
                        *span,
                        "S011",
                    ));
                }
                if self.current_function_is_async && !self.local_ref_bindings.is_empty() {
                    for name in self.local_ref_bindings.clone() {
                        self.errors.push(self.error_at(
                            format!("Reference '{name}' cannot cross await boundary"),
                            *span,
                            "S017",
                        ));
                    }
                }
                if borrow_regions_v6_enabled()
                    && self.current_function_is_async
                    && !self.immutable_ref_bindings.is_empty()
                {
                    for name in self.immutable_ref_bindings.clone() {
                        self.errors.push(self.error_at(
                            format!(
                                "Immutable borrow '{name}' cannot be reborrowed across await boundary"
                            ),
                            *span,
                            "S011",
                        ));
                    }
                }
                // R-35: type the await operand first (exempt from S020), then freeze
                // MainActor mutable locals for subsequent uses.
                self.await_operand_depth = self.await_operand_depth.saturating_add(1);
                let awaited_ty = self.analyze_expr(awaited);
                self.await_operand_depth = self.await_operand_depth.saturating_sub(1);
                if !self.skip_main_actor_await_check
                    && self.current_function_is_async
                    && self
                        .caller_global_actor(self.current_function.as_ref().unwrap())
                        .as_deref()
                        == Some("MainActor")
                {
                    self.main_actor_frozen_at_await = self.main_actor_mutable_locals.clone();
                    self.main_actor_await_crossed = true;
                }
                awaited_ty
            }
            Expr::OptionalChain { object, .. } => {
                self.analyze_expr(object)?;
                Ok(TypeId::Optional(Box::new(TypeId::Unknown)))
            }
            // S1-I12: `present!.count` — force-unwrap peels one Optional layer.
            Expr::ForceUnwrap { expr, .. } => {
                let ty = self.analyze_expr(expr)?;
                match ty {
                    TypeId::Optional(inner) => Ok(*inner),
                    other => Ok(other),
                }
            }
            Expr::HalfOpenRange { .. } | Expr::ClosedRange { .. } => {
                Ok(TypeId::Named(SmolStr::new("Range")))
            }
            Expr::RangeDyn { start, end, .. } => {
                self.analyze_expr(start)?;
                self.analyze_expr(end)?;
                Ok(TypeId::Named(SmolStr::new("Range")))
            }
            Expr::VariantPattern { .. } | Expr::TuplePattern { .. } => Ok(TypeId::Unknown),
        }
    }
}
