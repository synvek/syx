//! SYX semantic analyze_decl face (S2-I253).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    type_expr_to_id,
    type_expr_to_id_with_aliases, type_id_to_name, Scope,
    Symbol, TypeId, MAX_COLLECTED_DIAGNOSTICS,
};
use smol_str::SmolStr;
use syx_ast::{
    AccessLevel, Decl, Expr, TypeExpr,
};
use syx_diagnostics::SyxResult;
use syx_stdlib::is_hashable_type;

impl SemanticAnalyzer {
    pub(crate) fn analyze_decls(&mut self, decls: &[Decl]) -> SyxResult<()> {
        for decl in decls {
            if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                break;
            }
            // Collecting mode: do not abort the rest of the module on the first Err.
            if let Err(e) = self.analyze_decl(decl) {
                self.push_diagnostic(e);
            }
        }
        Ok(())
    }

    pub(crate) fn analyze_decl(&mut self, decl: &Decl) -> SyxResult<()> {
        match decl {
            Decl::Import { .. } => Ok(()),
            Decl::Export { item, .. } => self.analyze_decl(item),
            Decl::ExportList { .. } => Ok(()),
            Decl::TopLevelStmt { span, .. } => {
                self.errors.push(self.error_at(
                    "Top-level statements are only allowed in the main module file",
                    *span,
                    "S045",
                ));
                Ok(())
            }
            Decl::Function {
                name,
                span,
                body,
                params,
                type_params,
                where_clause,
                return_type,
                is_async,
                ..
            } => {
                let prev_fn = self.current_function.clone();
                self.current_function = Some(name.clone());
                let prev_async = self.current_function_is_async;
                self.current_function_is_async = *is_async;
                let prev_eq_params = std::mem::take(&mut self.current_equatable_params);
                let prev_type_params = std::mem::take(&mut self.current_type_params);
                for tp in type_params {
                    self.current_type_params.insert(tp.clone());
                }
                for pred in where_clause {
                    if pred.constraint.as_str() == "Equatable" {
                        self.current_equatable_params.insert(pred.type_name.clone());
                    }
                }
                let prev_local_refs = std::mem::take(&mut self.local_ref_bindings);
                let prev_main_actor_mutable = std::mem::take(&mut self.main_actor_mutable_locals);
                let prev_main_actor_frozen = std::mem::take(&mut self.main_actor_frozen_at_await);
                let prev_main_actor_await = self.main_actor_await_crossed;
                self.main_actor_await_crossed = false;
                let prev_return_lt = self.current_return_lifetime.clone();
                self.current_return_lifetime = return_type
                    .as_ref()
                    .and_then(Self::reference_lifetime)
                    .or_else(|| Self::infer_return_lifetime(params, return_type));
                let prev_bindings = std::mem::take(&mut self.binding_lifetimes);
                let elided_lt = SmolStr::new("'a");
                let mut params = params.clone();
                for param in &mut params {
                    if let Some(ty) = param.ty.as_mut() {
                        Self::assign_elided_lifetimes(ty, &elided_lt);
                    }
                }
                let mut return_type = return_type.clone();
                if let Some(rt) = return_type.as_mut() {
                    if self.current_return_lifetime.is_some() {
                        Self::assign_elided_lifetimes(rt, &elided_lt);
                    }
                }
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                for tp in type_params {
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: tp.clone(),
                            ty: TypeId::GenericParam(tp.clone()),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                }
                for pred in where_clause {
                    if !self.is_protocol_constraint(pred.constraint.as_str())
                        && !Self::is_associated_type_constraint(pred.constraint.as_str())
                    {
                        return Err(self.error_at(
                            format!(
                                "Unknown protocol constraint '{}' in where clause",
                                pred.constraint
                            ),
                            *span,
                            "S009",
                        ));
                    }
                }
                for param in &params {
                    if let Some(ty) = &param.ty {
                        self.validate_type_expr(ty, *span);
                    }
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(|t| self.resolve_type_expr(t))
                        .unwrap_or(TypeId::Unknown);
                    let param_mutable = matches!(param_ty, TypeId::MutReference(_));
                    if let Some(lt) = param.ty.as_ref().and_then(Self::infer_elided_lifetime) {
                        self.binding_lifetimes.insert(param.name.clone(), lt);
                    }
                    self.define_in_scope(
                        &mut fn_scope,
                        Symbol {
                            name: param.name.clone(),
                            ty: param_ty.clone(),
                            mutable: param_mutable,
                            access: AccessLevel::Internal,
                        },
                        *span,
                    )?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                self.moved_bindings.clear();
                self.mut_borrows.clear();
                self.analyze_block(body)?;
                self.check_local_mut_returns(body, &params);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                self.current_function = prev_fn;
                self.current_function_is_async = prev_async;
                self.current_equatable_params = prev_eq_params;
                self.current_type_params = prev_type_params;
                self.local_ref_bindings = prev_local_refs;
                self.main_actor_mutable_locals = prev_main_actor_mutable;
                self.main_actor_frozen_at_await = prev_main_actor_frozen;
                self.main_actor_await_crossed = prev_main_actor_await;
                self.current_return_lifetime = prev_return_lt;
                self.binding_lifetimes = prev_bindings;
                Ok(())
            }
            Decl::VarGlobal {
                init,
                ty,
                name,
                span,
                ..
            } => {
                if let Some(expr) = init {
                    let expected = ty.as_ref().map(|t| {
                        self.validate_type_expr(t, *span);
                        self.resolve_type_expr(t)
                    });
                    let analyzed = self.analyze_expr_with_expected(expr, expected.as_ref())?;
                    let expr_ty = if matches!(expr, Expr::Closure { .. }) {
                        TypeId::Named(SmolStr::new("Closure"))
                    } else {
                        analyzed
                    };
                    let final_ty = expected.clone().unwrap_or(expr_ty.clone());
                    if let Some(sym) = self.globals.get_mut(name) {
                        sym.ty = final_ty;
                    }
                    if let Some(expected) = expected {
                        self.check_assignable_with_literal(
                            expected,
                            expr_ty,
                            name,
                            Some(expr),
                            *span,
                        )?;
                    }
                }
                Ok(())
            }
            Decl::Protocol { methods, name, associated_types, span, .. } => {
                if !associated_types.is_empty() {
                    self.protocol_assoc_types.insert(
                        name.clone(),
                        associated_types
                            .iter()
                            .map(|(n, c)| (n.clone(), c.clone()))
                            .collect(),
                    );
                }
                // S1-I8: surface protocol requirements so `Array<Shape>` / `s.area()` type-check.
                let mut proto_methods: Vec<(SmolStr, TypeId)> = Vec::new();
                for m in methods {
                    match m {
                        Decl::Function {
                            name: method,
                            return_type,
                            ..
                        } => {
                            let ret = return_type
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Void);
                            proto_methods.push((method.clone(), ret.clone()));
                            let mangled = SmolStr::new(format!("{name}_{method}"));
                            self.function_return_types
                                .insert(mangled.clone(), ret.clone());
                            self.globals.insert(
                                mangled,
                                Symbol {
                                    name: SmolStr::new(format!("{name}_{method}")),
                                    ty: ret,
                                    mutable: false,
                                    access: AccessLevel::Internal,
                                },
                            );
                        }
                        Decl::VarGlobal {
                            name: prop,
                            ty: Some(ty),
                            ..
                        } => {
                            proto_methods.push((prop.clone(), type_expr_to_id(ty)));
                        }
                        _ => {}
                    }
                }
                if !proto_methods.is_empty() {
                    self.class_methods.insert(name.clone(), proto_methods);
                }
                let has_next = methods.iter().any(|m| {
                    matches!(
                        m,
                        Decl::Function {
                            name: n,
                            ..
                        } if n == "next"
                    )
                });
                // Marker / capability protocols (ADR-230 coding + ADR-234 IO + SA-2-02 Error)
                // may be empty — they must not be forced through the Iterator `next()` gate.
                let is_builtin_value_protocol = matches!(
                    name.as_str(),
                    "Equatable"
                        | "Hashable"
                        | "CustomStringConvertible"
                        | "Codable"
                        | "Encodable"
                        | "Decodable"
                        | "Encoder"
                        | "Decoder"
                        | "Reader"
                        | "Writer"
                        | "Seekable"
                        | "Closeable"
                        | "Error"
                );
                if name.as_str() == "Comparable" {
                    let has_compare = methods.iter().any(|m| {
                        matches!(
                            m,
                            Decl::Function {
                                name: n,
                                ..
                            } if n == "compare"
                        )
                    });
                    if !has_compare {
                        return Err(self.error_at(
                            "Protocol 'Comparable' must declare func compare(other: Self) -> Int",
                            *span,
                            "S006",
                        ));
                    }
                }
                // ADR-322/324: assoc-type-only or method-bearing protocols need not declare Iterator next().
                if !has_next
                    && !is_builtin_value_protocol
                    && name.as_str() != "Comparable"
                    && associated_types.is_empty()
                    && methods.is_empty()
                {
                    return Err(self.error_at(
                        format!("Protocol '{name}' must declare func next()"),
                        *span,
                        "S006",
                    ));
                }
                Ok(())
            }
            Decl::Class { name, members, .. } => {
                let mut field_names: Vec<&str> = Vec::new();
                for member in members {
                    if let Decl::VarGlobal { name: field, .. } = member {
                        field_names.push(field.as_str());
                    }
                    self.analyze_type_member(name, member, false)?;
                }
                syx_meta::register_type(name.as_str(), field_names);
                Ok(())
            }
            Decl::Actor { name, members, .. } => {
                for member in members {
                    self.analyze_type_member(name, member, true)?;
                }
                Ok(())
            }
            Decl::Struct {
                fields,
                name,
                members,
                ..
            } => {
                for (_, ty) in fields {
                    let _ = type_expr_to_id(ty);
                }
                for member in members {
                    self.analyze_type_member(name, member, false)?;
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
            Decl::Init { span, params, body, .. } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
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
                self.analyze_block(body)?;
                self.scope = *self.scope.parent.take().unwrap_or_default();
                Ok(())
            }
            Decl::Deinit { body, .. } => {
                self.analyze_block(body)?;
                Ok(())
            }
            Decl::TypeAlias { .. } | Decl::MacroDecl { .. } | Decl::MacroSiteInvoke { .. } | Decl::Subscript { .. } => Ok(()),
            Decl::Extension {
                type_name,
                where_clause,
                methods,
                span,
                ..
            } => {
                for pred in where_clause {
                    if !self.is_protocol_constraint(pred.constraint.as_str()) {
                        return Err(self.error_at(
                            format!(
                                "unknown protocol constraint '{}' in extension where",
                                pred.constraint
                            ),
                            *span,
                            "S009",
                        ));
                    }
                }
                for method in methods {
                    self.analyze_extension_method(type_name, method)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn resolve_type_expr(&self, ty: &TypeExpr) -> TypeId {
        type_expr_to_id_with_aliases(ty, &self.type_aliases)
    }

    pub(crate) fn has_subscript_get(&self, type_name: &SmolStr) -> bool {
        self.subscript_types.contains(type_name)
    }

    pub(crate) fn has_subscript_set(&self, type_name: &SmolStr) -> bool {
        self.subscript_set_types.contains(type_name)
    }

    pub(crate) fn subscript_return_type(&self, type_name: &SmolStr) -> Option<TypeId> {
        self.subscript_return_types.get(type_name).cloned()
    }

    pub(crate) fn validate_type_expr(&mut self, ty: &TypeExpr, span: syx_diagnostics::Span) {
        match ty {
            TypeExpr::Named { name, generics } => {
                if name.as_str() == "Array" && generics.is_empty() {
                    self.errors.push(self.error_at(
                        "Array requires a type argument; use Array<T> (e.g. Array<Int>)",
                        span,
                        "S040",
                    ));
                }
                if name.as_str() == "Dictionary"
                    && !generics.is_empty()
                    && generics.len() < 2
                {
                    self.errors.push(self.error_at(
                        "Dictionary requires key and value type arguments: Dictionary<K, V> where K: Hashable",
                        span,
                        "S041",
                    ));
                }
                for g in generics {
                    self.validate_type_expr(g, span);
                }
                if name.as_str() == "Dictionary" && generics.len() >= 2 {
                    let key_ty = self.resolve_type_expr(&generics[0]);
                    self.validate_dict_key_hashable(&key_ty, span);
                }
                if name.as_str() == "Set" && generics.len() == 1 {
                    let elem_ty = self.resolve_type_expr(&generics[0]);
                    self.validate_set_elem_hashable(&elem_ty, span);
                }
            }
            TypeExpr::Optional(inner)
            | TypeExpr::Array(inner)
            | TypeExpr::OpaqueSome(inner)
            | TypeExpr::OpaqueAny(inner) => self.validate_type_expr(inner, span),
            TypeExpr::Dictionary { key, value } => {
                self.validate_type_expr(key, span);
                self.validate_type_expr(value, span);
                let key_ty = self.resolve_type_expr(key);
                self.validate_dict_key_hashable(&key_ty, span);
            }
            TypeExpr::Reference(inner, _, _) => self.validate_type_expr(inner, span),
            TypeExpr::Function { params, returns } => {
                for p in params {
                    self.validate_type_expr(p, span);
                }
                if let Some(r) = returns {
                    self.validate_type_expr(r, span);
                }
            }
            TypeExpr::Tuple { elements } => {
                for (_, el) in elements {
                    self.validate_type_expr(el, span);
                }
            }
            TypeExpr::Void => {}
        }
    }

    pub(crate) fn validate_dict_key_hashable(
        &mut self,
        key_ty: &TypeId,
        span: syx_diagnostics::Span,
    ) {
        // S1-I6: open formals (`Element`/`Key`/current type params) — checked at call site.
        if self.is_open_generic_elem(key_ty)
            || matches!(
                type_id_to_name(key_ty).as_str(),
                "Element" | "Key" | "Value"
            )
        {
            return;
        }
        let key_name = type_id_to_name(key_ty);
        if !is_hashable_type(key_name.as_str())
            && !self.type_id_satisfies_protocol(key_ty, "Hashable")
        {
            self.errors.push(self.error_at(
                format!(
                    "Dictionary key type '{}' does not satisfy Hashable",
                    key_name
                ),
                span,
                "S042",
            ));
        }
    }

    pub(crate) fn validate_set_elem_hashable(
        &mut self,
        elem_ty: &TypeId,
        span: syx_diagnostics::Span,
    ) {
        // S1-I6: `Set<Element>` inside `extension Array where Element: Hashable`.
        if self.is_open_generic_elem(elem_ty)
            || matches!(
                type_id_to_name(elem_ty).as_str(),
                "Element" | "Key" | "Value"
            )
        {
            return;
        }
        let elem_name = type_id_to_name(elem_ty);
        if !is_hashable_type(elem_name.as_str())
            && !self.type_id_satisfies_protocol(elem_ty, "Hashable")
        {
            self.errors.push(self.error_at(
                format!(
                    "Set element type '{}' does not satisfy Hashable",
                    elem_name
                ),
                span,
                "S015",
            ));
        }
    }
}
