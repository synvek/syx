//! SYX semantic analyze_type_member face (S2-I255).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{type_expr_to_id, type_id_to_name, Scope, Symbol, TypeId};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{AccessLevel, BinOp, CallArg, Decl, Expr, Stmt, TypeExpr};
use syx_diagnostics::SyxResult;
use syx_stdlib::{
    from_type_name, is_collection_builtin_type, is_comparable_type, is_equatable_type,
    is_numeric_builtin_type, is_text_builtin_type, operator_overload_forbidden_type, type_conforms,
};

impl SemanticAnalyzer {
    pub(crate) fn analyze_type_member(
        &mut self,
        type_name: &SmolStr,
        member: &Decl,
        in_actor: bool,
    ) -> SyxResult<()> {
        let prev = self.current_actor_context.clone();
        let prev_class = self.current_class_context.clone();
        self.current_class_context = Some(type_name.clone());
        if in_actor {
            self.current_actor_context = Some(type_name.clone());
        }
        let result = match member {
            Decl::Function {
                span, params, body, ..
            }
            | Decl::Init {
                span, params, body, ..
            } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                self.define_in_scope(
                    &mut fn_scope,
                    Symbol {
                        name: SmolStr::new("self"),
                        ty: TypeId::Named(type_name.clone()),
                        mutable: true,
                        access: AccessLevel::Internal,
                    },
                    *span,
                )?;
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: param.name.clone(),
                            ty: param_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            Decl::Deinit { span, body, .. } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                self.define_in_scope(
                    &mut fn_scope,
                    Symbol {
                        name: SmolStr::new("self"),
                        ty: TypeId::Named(type_name.clone()),
                        mutable: true,
                        access: AccessLevel::Internal,
                    },
                    *span,
                )?;
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            Decl::VarGlobal {
                init,
                ty,
                name,
                mutable,
                will_set,
                did_set,
                computed_getter,
                computed_setter,
                is_lazy,
                attrs,
                span,
                ..
            } => {
                let is_static = syx_ast::var_global_is_static(attrs);
                if *is_lazy && !*mutable {
                    self.errors.push(self.error_at(
                        "lazy is only allowed on var properties",
                        *span,
                        "S057",
                    ));
                }
                if *is_lazy && is_static {
                    self.errors.push(self.error_at(
                        "lazy cannot be combined with static",
                        *span,
                        "S057",
                    ));
                }
                if computed_getter.is_some() && computed_setter.is_some() && !*mutable {
                    self.errors.push(self.error_at(
                        "Computed property setter requires 'var'",
                        *span,
                        "S057",
                    ));
                }
                if computed_getter.is_some() {
                    if init.is_some() {
                        self.errors.push(self.error_at(
                            "computed properties cannot have an initializer",
                            *span,
                            "S057",
                        ));
                    }
                    if will_set.is_some() || did_set.is_some() {
                        self.errors.push(self.error_at(
                            "willSet/didSet are not allowed on computed properties",
                            *span,
                            "S056",
                        ));
                    }
                }
                if (will_set.is_some() || did_set.is_some()) && !*mutable {
                    self.errors.push(self.error_at(
                        "willSet/didSet are only allowed on stored mutable (var) properties",
                        *span,
                        "S056",
                    ));
                }
                if *is_lazy && init.is_none() {
                    self.errors.push(self.error_at(
                        "lazy var requires an initializer",
                        *span,
                        "S057",
                    ));
                }
                if in_actor {
                    if let Some(type_expr) = ty {
                        let field_ty = type_expr_to_id(type_expr);
                        if self.strict_concurrency && self.is_non_sendable_type(&field_ty) {
                            self.errors.push(self.error_at(
                                format!(
                                    "Actor '{type_name}' cannot store non-Sendable field '{name}'"
                                ),
                                *span,
                                "C007",
                            ));
                        }
                    }
                }
                if let Some(expr) = init {
                    self.analyze_expr(expr)?;
                }
                let field_ty = ty.as_ref().map(type_expr_to_id).unwrap_or(TypeId::Unknown);
                if let Some(stmts) = will_set {
                    let mut obs_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    self.define_in_scope(
                        &mut obs_scope,
                        Symbol {
                            name: SmolStr::new("newValue"),
                            ty: field_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.define_in_scope(
                        &mut obs_scope,
                        Symbol {
                            name: SmolStr::new("oldValue"),
                            ty: field_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    std::mem::swap(&mut self.scope, &mut obs_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = did_set {
                    let mut obs_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    self.define_in_scope(
                        &mut obs_scope,
                        Symbol {
                            name: SmolStr::new("oldValue"),
                            ty: field_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.define_in_scope(
                        &mut obs_scope,
                        Symbol {
                            name: SmolStr::new("newValue"),
                            ty: field_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    std::mem::swap(&mut self.scope, &mut obs_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = computed_getter {
                    let mut get_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    self.define_in_scope(
                        &mut get_scope,
                        Symbol {
                            name: SmolStr::new("self"),
                            ty: TypeId::Named(type_name.clone()),
                            mutable: true,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    std::mem::swap(&mut self.scope, &mut get_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = computed_setter {
                    let mut set_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    self.define_in_scope(
                        &mut set_scope,
                        Symbol {
                            name: SmolStr::new("self"),
                            ty: TypeId::Named(type_name.clone()),
                            mutable: true,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.define_in_scope(
                        &mut set_scope,
                        Symbol {
                            name: SmolStr::new("newValue"),
                            ty: field_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    std::mem::swap(&mut self.scope, &mut set_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                Ok(())
            }
            Decl::Class {
                name: inner,
                members,
                ..
            } => {
                let qualified = SmolStr::new(format!("{type_name}_{inner}"));
                for nested in members {
                    self.analyze_type_member(&qualified, nested, in_actor)?;
                }
                Ok(())
            }
            Decl::Struct { fields, .. } => {
                for (_, ty) in fields {
                    let _ = type_expr_to_id(ty);
                }
                Ok(())
            }
            Decl::Enum { cases, .. } => {
                for (_, value, _) in cases {
                    if let Some(expr) = value {
                        self.analyze_expr(expr)?;
                    }
                }
                Ok(())
            }
            Decl::FinalClass { .. } => Ok(()),
            Decl::OperatorFunc {
                op,
                span,
                params,
                body,
                return_type,
                ..
            } => {
                if operator_overload_forbidden_type(type_name.as_str()) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot be defined on builtin type '{type_name}'"),
                        *span,
                        "S048",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, .. } = t {
                            is_numeric_builtin_type(name.as_str())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload numeric builtin types"),
                        *span,
                        "S043",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, .. } = t {
                            is_text_builtin_type(name.as_str())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload text builtin types"),
                        *span,
                        "S044",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, generics, .. } = t {
                            is_collection_builtin_type(name.as_str())
                                || (name.as_str() == "Array" && !generics.is_empty())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload collection builtin types"),
                        *span,
                        "S047",
                    ));
                } else if return_type.as_ref().is_some_and(|t| {
                    if let TypeExpr::Named { name, .. } = t {
                        is_numeric_builtin_type(name.as_str())
                    } else {
                        false
                    }
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot return numeric builtin type"),
                        *span,
                        "S043",
                    ));
                }
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                self.define_in_scope(
                    &mut fn_scope,
                    Symbol {
                        name: SmolStr::new("self"),
                        ty: TypeId::Named(type_name.clone()),
                        mutable: true,
                        access: AccessLevel::Internal,
                    },
                    *span,
                )?;
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: param.name.clone(),
                            ty: param_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            _ => self.analyze_decl(member),
        };
        self.current_actor_context = prev;
        self.current_class_context = prev_class;
        result
    }

    pub(crate) fn switch_pattern_case_name(pattern: &Expr) -> Option<SmolStr> {
        match pattern {
            Expr::VariantPattern { variant, .. } => Some(variant.clone()),
            Expr::Ident { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    fn is_self_expr(expr: &Expr) -> bool {
        matches!(expr, Expr::Ident { name, .. } if name == "self")
    }

    /// `.length` / `.len` were retired in favor of Swift-aligned `.count`.
    pub(crate) fn check_retired_size_alias(
        &self,
        obj_ty: &TypeId,
        name: &str,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        if !matches!(name, "length" | "len") {
            return Ok(());
        }
        let surface = match obj_ty {
            TypeId::Array(_) | TypeId::Set(_) | TypeId::Dictionary { .. } | TypeId::String => true,
            TypeId::Named(n) => {
                let s = n.as_str();
                s == "String"
                    || s == "Array"
                    || s.starts_with("Array<")
                    || s == "Set"
                    || s.starts_with("Set<")
                    || s == "Dictionary"
                    || s.starts_with("Dictionary<")
                    || s == "Object"
            }
            _ => false,
        };
        if !surface {
            return Ok(());
        }
        Err(self.error_at(
            format!(
                "'.{name}' was removed on {}; use .count instead",
                type_id_to_name(obj_ty)
            ),
            span,
            "S004",
        ))
    }

    pub(crate) fn check_actor_field_access(
        &mut self,
        object: &Expr,
        field: &str,
        obj_ty: &TypeId,
        span: syx_diagnostics::Span,
    ) {
        let TypeId::Named(type_name) = obj_ty else {
            return;
        };
        if !self.actor_types.contains(type_name) {
            return;
        }
        let is_field = self
            .actor_fields
            .get(type_name)
            .is_some_and(|fields| fields.contains(field));
        if !is_field {
            return;
        }
        let allowed =
            self.current_actor_context.as_ref() == Some(type_name) && Self::is_self_expr(object);
        if !allowed {
            self.errors.push(self.error_at(
                format!(
                    "Cross-actor access to field '{field}' on actor '{type_name}' is not allowed"
                ),
                span,
                "S008",
            ));
        }
    }

    pub(crate) fn expr_type(&self, expr: &Expr) -> Option<TypeId> {
        match expr {
            Expr::Ident { name, .. } => self.symbol_type(name),
            _ => None,
        }
    }

    pub(crate) fn type_satisfies_protocol(&self, type_name: &str, protocol: &str) -> bool {
        // S1-I5: `Any` is the unconstrained top type for associatedtype bounds.
        if protocol == "Any" {
            return true;
        }
        // S1-I8: `where Element: User` — same-type / class bound (not only protocols).
        if type_name == protocol {
            return true;
        }
        if type_conforms(type_name, protocol) {
            return true;
        }
        if protocol == "Comparable" && is_comparable_type(type_name) {
            return true;
        }
        if from_type_name(type_name).is_some() {
            return matches!(protocol, "Equatable" | "Hashable")
                || type_conforms(type_name, protocol);
        }
        let unconditional = match (type_name, protocol) {
            (
                "Bool" | "Character",
                "Equatable"
                | "Hashable"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "String",
                "Equatable"
                | "Hashable"
                | "Comparable"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "Array",
                "Equatable"
                | "Hashable"
                | "Sequence"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "Set",
                "Equatable"
                | "Hashable"
                | "Sequence"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "Dictionary",
                "Equatable"
                | "Hashable"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "Object",
                "Equatable"
                | "Hashable"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            (
                "ObjectIdentifier",
                "Equatable"
                | "Hashable"
                | "CustomStringConvertible"
                | "CustomDebugStringConvertible",
            ) => true,
            _ => {
                self.type_conformances
                    .get(type_name)
                    .is_some_and(|c| c.iter().any(|p| p.as_str() == protocol))
                    || self.dataclass_names.contains(&SmolStr::new(type_name))
                        && matches!(
                            protocol,
                            "Equatable"
                                | "Hashable"
                                | "CustomStringConvertible"
                                | "CustomDebugStringConvertible"
                        )
            }
        };
        unconditional || self.conditional_conformance_holds(type_name, protocol)
    }

    /// ADR-324: evaluate `extension Base: Proto where …` against a display type name (`Array<Int>`).
    fn conditional_conformance_holds(&self, type_name: &str, protocol: &str) -> bool {
        let ty = Self::type_id_from_display_name(type_name);
        self.conditional_conformance_holds_ty(&ty, protocol)
    }

    fn conditional_conformance_holds_ty(&self, ty: &TypeId, protocol: &str) -> bool {
        let Some(base) = self.extension_base_for_ty(ty) else {
            return false;
        };
        for (ext_base, ext_proto, preds) in &self.conditional_conformances {
            if ext_base != &base || ext_proto.as_str() != protocol {
                continue;
            }
            let args = self.extension_type_arg_map(base.as_str(), ty);
            if self.where_preds_satisfied(&args, preds) {
                return true;
            }
        }
        false
    }

    fn where_preds_satisfied(
        &self,
        type_args: &IndexMap<SmolStr, SmolStr>,
        preds: &[syx_ast::WherePred],
    ) -> bool {
        for pred in preds {
            let arg_ty = type_args
                .get(&pred.type_name)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    return false;
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                return false;
            }
        }
        true
    }

    fn type_id_from_display_name(name: &str) -> TypeId {
        if let Some(inner) = name
            .strip_prefix("Array<")
            .and_then(|s| s.strip_suffix('>'))
        {
            return TypeId::Array(Box::new(Self::type_id_from_display_name(inner)));
        }
        if let Some(inner) = name.strip_prefix("Set<").and_then(|s| s.strip_suffix('>')) {
            return TypeId::Set(Box::new(Self::type_id_from_display_name(inner)));
        }
        if let Some(kind) = from_type_name(name) {
            return TypeId::Numeric(kind);
        }
        match name {
            "Bool" => TypeId::Bool,
            "String" => TypeId::String,
            "Void" => TypeId::Void,
            "Unknown" => TypeId::Unknown,
            other => TypeId::Named(SmolStr::new(other)),
        }
    }

    pub(crate) fn type_id_satisfies_protocol(&self, ty: &TypeId, protocol: &str) -> bool {
        self.type_satisfies_protocol(type_id_to_name(ty).as_str(), protocol)
            || self.conditional_conformance_holds_ty(ty, protocol)
    }

    pub(crate) fn check_equatable_binary(
        &mut self,
        op: BinOp,
        lt: &TypeId,
        rt: &TypeId,
        span: syx_diagnostics::Span,
    ) {
        if matches!(op, BinOp::IdenticalEq | BinOp::IdenticalNe) {
            return;
        }
        let involves_main_actor =
            |ty: &TypeId| matches!(ty, TypeId::Named(n) if self.main_actor_types.contains(n));
        if involves_main_actor(lt) || involves_main_actor(rt) {
            if let Some(fn_name) = &self.current_function {
                let on_main = self.caller_global_actor(fn_name).as_deref() == Some("MainActor");
                if !on_main {
                    self.errors.push(self.error_at(
                        "MainActor-isolated Equatable conformance requires MainActor context",
                        span,
                        "C006",
                    ));
                }
            }
        }
        if let (TypeId::Named(ln), TypeId::Named(rn)) = (lt, rt) {
            if ln == rn
                && (self.class_names.contains(ln)
                    || self.dataclass_names.contains(ln)
                    || self.enum_cases.contains_key(ln))
            {
                // Same nominal / enum type: allow when Equatable conformance holds.
                if self.type_satisfies_protocol(ln.as_str(), "Equatable")
                    || self.dataclass_names.contains(ln)
                {
                    return;
                }
            }
        }
        if let (TypeId::GenericParam(a), TypeId::GenericParam(b)) = (lt, rt) {
            if a == b && self.current_equatable_params.contains(a) {
                return;
            }
        }
        // Param types `a: T` resolve as Named("T"), not GenericParam.
        if let (TypeId::Named(a), TypeId::Named(b)) = (lt, rt) {
            if a == b && self.current_equatable_params.contains(a) {
                return;
            }
        }
        if matches!(lt, TypeId::OpaqueSome(_) | TypeId::OpaqueAny(_))
            && matches!(rt, TypeId::OpaqueSome(_) | TypeId::OpaqueAny(_))
        {
            return;
        }
        if lt == rt && matches!(lt, TypeId::Unknown) {
            return;
        }
        let lname = type_id_to_name(lt);
        let rname = type_id_to_name(rt);
        if lname == "Unknown" && is_equatable_type(rname.as_str()) {
            return;
        }
        if rname == "Unknown" && is_equatable_type(lname.as_str()) {
            return;
        }
        let l_ok = is_equatable_type(lname.as_str())
            || self.type_satisfies_protocol(lname.as_str(), "Equatable");
        let r_ok = is_equatable_type(rname.as_str())
            || self.type_satisfies_protocol(rname.as_str(), "Equatable");
        if !l_ok || !r_ok {
            self.errors.push(self.error_at(
                format!("Types '{lname}' and '{rname}' are not both Equatable for '=='"),
                span,
                "S016",
            ));
        }
    }

    pub(crate) fn check_local_mut_returns(&mut self, stmts: &[Stmt], params: &[syx_ast::Param]) {
        let param_names: HashSet<SmolStr> = params.iter().map(|p| p.name.clone()).collect();
        for stmt in stmts {
            match stmt {
                Stmt::Return {
                    value:
                        Some(Expr::Borrow {
                            expr,
                            mutable: true,
                            ..
                        }),
                    span,
                    ..
                } => {
                    if let Expr::Ident { name, .. } = expr.as_ref() {
                        if !param_names.contains(name) && self.all_bindings.contains_key(name) {
                            self.errors.push(self.error_at(
                                format!("Cannot return reference to local binding '{name}'"),
                                *span,
                                "S011",
                            ));
                        }
                    }
                }
                Stmt::If {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    self.check_local_mut_returns(then_branch, params);
                    if let Some(else_b) = else_branch {
                        self.check_local_mut_returns(else_b, params);
                    }
                }
                Stmt::While { body, .. } => {
                    self.check_local_mut_returns(body, params);
                }
                Stmt::Block { stmts, .. } => {
                    self.check_local_mut_returns(stmts, params);
                }
                Stmt::Try {
                    body, catch_body, ..
                } => {
                    self.check_local_mut_returns(body, params);
                    self.check_local_mut_returns(catch_body, params);
                }
                _ => {}
            }
        }
    }

    pub(crate) fn validate_generic_instantiation(
        &mut self,
        fn_name: &SmolStr,
        type_args: &[SmolStr],
        span: syx_diagnostics::Span,
    ) {
        let Some((type_params, where_clause)) = self.generic_functions.get(fn_name).cloned() else {
            return;
        };
        self.validate_generic_instantiation_with(
            fn_name,
            &type_params,
            &where_clause,
            type_args,
            span,
        );
    }

    /// Substitute generic return type (`T` → concrete) for check/inference.
    pub(crate) fn instantiate_generic_return(
        &self,
        fn_name: &SmolStr,
        type_params: &[SmolStr],
        type_args: &[SmolStr],
    ) -> TypeId {
        let Some(rt) = self.function_return_types.get(fn_name) else {
            return type_args
                .first()
                .map(|t| TypeId::Named(t.clone()))
                .unwrap_or(TypeId::Unknown);
        };
        Self::subst_type_id(rt, type_params, type_args)
    }

    fn subst_type_id(ty: &TypeId, type_params: &[SmolStr], type_args: &[SmolStr]) -> TypeId {
        match ty {
            TypeId::Named(n) | TypeId::GenericParam(n) => {
                if let Some(idx) = type_params.iter().position(|p| p == n) {
                    if let Some(arg) = type_args.get(idx) {
                        return TypeId::Named(arg.clone());
                    }
                }
                ty.clone()
            }
            TypeId::Optional(inner) => {
                TypeId::Optional(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Array(inner) => {
                TypeId::Array(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Set(inner) => {
                TypeId::Set(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Dictionary { key, value } => TypeId::Dictionary {
                key: Box::new(Self::subst_type_id(key, type_params, type_args)),
                value: Box::new(Self::subst_type_id(value, type_params, type_args)),
            },
            other => other.clone(),
        }
    }

    pub(crate) fn validate_reflect_specialization_call(
        &mut self,
        args: &[CallArg],
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        if args.is_empty() {
            return Err(self.error_at(
                "__syx_reflect_specialization requires a function name and type arguments",
                span,
                "S002",
            ));
        }
        let Some(fn_name) = Self::string_literal_value(&args[0].value) else {
            return Err(self.error_at(
                "__syx_reflect_specialization function name must be a string literal",
                span,
                "S002",
            ));
        };
        let mut type_args = Vec::new();
        for arg in args.iter().skip(1) {
            let Some(ty) = Self::string_literal_value(&arg.value) else {
                return Err(self.error_at(
                    "__syx_reflect_specialization type arguments must be string literals",
                    span,
                    "S002",
                ));
            };
            type_args.push(ty);
        }
        let arg_refs: Vec<&str> = type_args.iter().map(String::as_str).collect();
        if let Err(err) = syx_meta::reflect_specialization(&fn_name, &arg_refs) {
            let code = err.code.as_deref().unwrap_or("M105");
            self.errors.push(self.error_at(err.message, span, code));
        }
        Ok(())
    }

    fn string_literal_value(expr: &Expr) -> Option<String> {
        match expr {
            Expr::Literal {
                value: syx_ast::Literal::String(s),
                ..
            } => Some(s.clone()),
            _ => None,
        }
    }

    pub(crate) fn validate_generic_instantiation_with(
        &mut self,
        generic_name: &SmolStr,
        type_params: &[SmolStr],
        where_clause: &[syx_ast::WherePred],
        type_args: &[SmolStr],
        span: syx_diagnostics::Span,
    ) {
        for pred in where_clause {
            let Some(idx) = type_params.iter().position(|p| p == &pred.type_name) else {
                continue;
            };
            let arg_ty = type_args.get(idx).map(|s| s.as_str()).unwrap_or("Unknown");
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    self.errors.push(self.error_at(
                        format!(
                            "Associated type '{arg_ty}' does not match required '{}' for generic '{generic_name}'",
                            pred.constraint
                        ),
                        span,
                        "S012",
                    ));
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                self.errors.push(self.error_at(
                    format!(
                        "Type '{arg_ty}' does not satisfy where constraint '{}: {}' for generic '{generic_name}'",
                        pred.type_name, pred.constraint
                    ),
                    span,
                    "S010",
                ));
            }
        }
    }

    fn extension_base_for_ty(&self, ty: &TypeId) -> Option<SmolStr> {
        match ty {
            TypeId::Array(_) => Some(SmolStr::new("Array")),
            TypeId::Set(_) => Some(SmolStr::new("Set")),
            TypeId::Dictionary { .. } => Some(SmolStr::new("Dictionary")),
            TypeId::Named(n) => Some(n.clone()),
            TypeId::String => Some(SmolStr::new("String")),
            _ => None,
        }
    }

    fn extension_receiver_type(type_name: &SmolStr) -> TypeId {
        match type_name.as_str() {
            "String" => TypeId::String,
            "Bool" => TypeId::Bool,
            // S1-I5: extension Array methods see `Element` (matches `Array<Element>` annotations).
            "Array" => TypeId::Array(Box::new(TypeId::Named(SmolStr::new("Element")))),
            "Set" => TypeId::Set(Box::new(TypeId::Named(SmolStr::new("Element")))),
            "Dictionary" => TypeId::Dictionary {
                key: Box::new(TypeId::Named(SmolStr::new("Key"))),
                value: Box::new(TypeId::Named(SmolStr::new("Value"))),
            },
            name => {
                if let Some(kind) = from_type_name(name) {
                    TypeId::Numeric(kind)
                } else {
                    TypeId::Named(type_name.clone())
                }
            }
        }
    }

    pub(crate) fn analyze_extension_method(
        &mut self,
        type_name: &SmolStr,
        member: &Decl,
    ) -> SyxResult<()> {
        match member {
            Decl::Function {
                span,
                params,
                body,
                attrs,
                ..
            } => {
                let is_static = attrs.iter().any(|a| a.name.as_str() == "static");
                if is_static {
                    return self.analyze_decl(member);
                }
                let has_self = params.iter().any(|p| p.name.as_str() == "self");
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                if !has_self {
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: SmolStr::new("self"),
                            ty: Self::extension_receiver_type(type_name),
                            mutable: true,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                }
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: param.name.clone(),
                            ty: param_ty.clone(),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            _ => Ok(()),
        }
    }

    fn extension_type_arg_map(&self, base: &str, ty: &TypeId) -> IndexMap<SmolStr, SmolStr> {
        match (base, ty) {
            ("Array", TypeId::Array(inner)) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Element"), type_id_to_name(inner));
                m
            }
            ("Set", TypeId::Set(inner)) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Element"), type_id_to_name(inner));
                m
            }
            ("Dictionary", TypeId::Dictionary { key, value }) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Key"), type_id_to_name(key));
                m.insert(SmolStr::new("Value"), type_id_to_name(value));
                m
            }
            _ => IndexMap::new(),
        }
    }

    pub(crate) fn check_extension_member_where(
        &mut self,
        obj_ty: &TypeId,
        method: &str,
        span: syx_diagnostics::Span,
    ) {
        let Some(base) = self.extension_base_for_ty(obj_ty) else {
            return;
        };
        let key = (base.clone(), SmolStr::new(method));
        let Some(where_clause) = self.extension_method_where.get(&key).cloned() else {
            return;
        };
        if !self
            .type_extension_methods
            .get(&base)
            .is_some_and(|methods| methods.iter().any(|(n, _)| n.as_str() == method))
        {
            return;
        }
        let type_args = self.extension_type_arg_map(base.as_str(), obj_ty);
        for pred in &where_clause {
            let arg_ty = type_args
                .get(&pred.type_name)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            // S1-I6: inside `extension Array where Element: P`, `self`/`seen` are still
            // `Array<Element>` — defer formal placeholders; call sites substitute Int/….
            if arg_ty == pred.type_name.as_str() || matches!(arg_ty, "Element" | "Key" | "Value") {
                continue;
            }
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    self.errors.push(self.error_at(
                        format!(
                            "Associated type '{arg_ty}' does not match required '{}' for extension on '{base}'",
                            pred.constraint
                        ),
                        span,
                        "S012",
                    ));
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                self.errors.push(self.error_at(
                    format!(
                        "Type '{arg_ty}' does not satisfy where constraint '{}: {}' for extension on '{base}'",
                        pred.type_name, pred.constraint
                    ),
                    span,
                    "S010",
                ));
            }
        }
    }

    pub(crate) fn is_associated_type_constraint(constraint: &str) -> bool {
        if syx_stdlib::protocol_associated_types(constraint).is_some() {
            return true;
        }
        matches!(
            constraint,
            "Int"
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
                | "Double"
                | "Bool"
                | "String"
                | "Object"
        )
    }
}
