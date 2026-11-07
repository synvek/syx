//! SYX semantic analyze_stmt face (S2-I252).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    borrow_regions_v7_enabled, Scope,
    Symbol, TypeId, MAX_COLLECTED_DIAGNOSTICS,
};
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{
    AccessLevel, Expr, Literal, Stmt,
};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

impl SemanticAnalyzer {
    pub(crate) fn analyze_block(&mut self, stmts: &[Stmt]) -> SyxResult<()> {
        let mut child = Scope {
            parent: Some(Box::new(std::mem::take(&mut self.scope))),
            ..Default::default()
        };
        std::mem::swap(&mut self.scope, &mut child);
        for stmt in stmts {
            if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                break;
            }
            // Keep analyzing later statements so LSP can show multiple errors at once.
            if let Err(e) = self.analyze_stmt(stmt) {
                self.push_diagnostic(e);
            }
        }
        self.scope = *self.scope.parent.take().unwrap_or_default();
        Ok(())
    }


    fn analyze_stmt(&mut self, stmt: &Stmt) -> SyxResult<()> {
        match stmt {
            Stmt::VarDecl {
                name,
                mutable,
                ty,
                init,
                span,
                ..
            } => {
                let annotated = ty.as_ref().map(|t| {
                    self.validate_type_expr(t);
                    self.resolve_type_expr(t)
                });
                let (inferred, expr_ty) = if let Some(expr) = init {
                    let ty = self.analyze_expr_with_expected(expr, annotated.as_ref())?;
                    // Closure expr analyze returns body result (for Array.map U);
                    // stored bindings must remain callable (`inc(1)`).
                    if matches!(expr, Expr::Closure { .. }) {
                        let clos = TypeId::Named(SmolStr::new("Closure"));
                        (clos.clone(), clos)
                    } else {
                        (ty.clone(), ty)
                    }
                } else {
                    (TypeId::Unknown, TypeId::Unknown)
                };
                let final_ty = annotated.unwrap_or(inferred);
                if matches!(final_ty, TypeId::Void) {
                    self.errors.push(self.error_at(
                        "Type 'Void' cannot be used for variables; it is only valid as a return type",
                        *span,
                        "S024",
                    ));
                }
                if init.is_some() {
                    self.check_assignable_with_literal(
                        final_ty.clone(),
                        expr_ty,
                        name,
                        init.as_ref(),
                        *span,
                    )?;
                }
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: final_ty.clone(),
                    mutable: *mutable,
                    access: AccessLevel::Internal,
                })?;
                self.track_binding(name.clone(), final_ty.clone(), span.start);
                if self.current_function_is_async
                    && matches!(final_ty, TypeId::Reference(_) | TypeId::MutReference(_))
                {
                    self.local_ref_bindings.insert(name.clone());
                }
                if let Some(init_expr) = init {
                    if let Expr::Borrow { expr, mutable, .. } = init_expr {
                        if *mutable {
                            if let Expr::Ident { name: src, .. } = expr.as_ref() {
                                if self.borrow_region_depth > 0
                                    && self.immutable_ref_bindings.contains(src)
                                {
                                    self.errors.push(
                                        SyxDiagnostic::error(format!(
                                            "Cannot borrow '{src}' as mutable while an immutable borrow is active in this region"
                                        ))
                                        .with_code("S011"),
                                    );
                                }
                            }
                        } else if let Expr::Ident { name: src, .. } = expr.as_ref() {
                            if borrow_regions_v7_enabled()
                                && self.current_function_is_async
                                && self.borrow_region_depth >= 2
                            {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Nested async borrow region cannot reborrow '{src}' immutably"
                                    ))
                                    .with_code("S011"),
                                );
                            }
                            if self.borrow_region_depth > 1 {
                                if let Some(lt) = ty
                                    .as_ref()
                                    .and_then(|t| Self::infer_elided_lifetime(t))
                                {
                                    if !self.active_lifetime_labels.is_empty()
                                        && !self.active_lifetime_labels.contains(&lt)
                                    {
                                        self.errors.push(
                                            SyxDiagnostic::error(format!(
                                                "Conflicting borrow regions '{lt}' vs active {:?}",
                                                self.active_lifetime_labels
                                            ))
                                            .with_code("S011"),
                                        );
                                    }
                                    self.active_lifetime_labels.insert(lt);
                                }
                            }
                            self.immutable_ref_bindings.insert(src.clone());
                        }
                    }
                }
                if self.current_function_is_async
                    && *mutable
                    && !name.starts_with("__")
                    && !self.skip_main_actor_await_check
                    && self.caller_global_actor(self.current_function.as_ref().unwrap())
                        .as_deref()
                        == Some("MainActor")
                {
                    self.main_actor_mutable_locals.insert(name.clone());
                }
                Ok(())
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.analyze_expr(v)?;
                    self.check_return_lifetime(v);
                    if let Expr::Borrow { expr, .. } = v {
                        if let Expr::Ident { name, .. } = expr.as_ref() {
                            if self.scope.lookup(name).is_some() {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Cannot return reference to local binding '{name}'"
                                    ))
                                    .with_code("S011"),
                                );
                            }
                        }
                    }
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.analyze_expr(condition)?;
                let saved_borrows = self.mut_borrows.clone();
                self.analyze_block(then_branch)?;
                self.mut_borrows = saved_borrows.clone();
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                self.mut_borrows = saved_borrows;
                Ok(())
            }
            Stmt::While { condition, body, .. } => {
                let saved_borrows = self.mut_borrows.clone();
                self.analyze_expr(condition)?;
                self.analyze_block(body)?;
                self.mut_borrows = saved_borrows;
                Ok(())
            }
            Stmt::ForIn { name, iterable, body, span, .. } => {
                let iter_ty = self.analyze_expr(iterable)?;
                let elem_ty = match &iter_ty {
                    TypeId::Array(inner) => inner.as_ref().clone(),
                    TypeId::Set(inner) => inner.as_ref().clone(),
                    TypeId::String => TypeId::String,
                    TypeId::Named(n) if n.as_str() == "Range" => TypeId::platform_int(),
                    TypeId::Named(n) if n.as_str() == "String" => TypeId::String,
                    TypeId::Named(n) => {
                        if !self.type_satisfies_protocol(n.as_str(), "Iterator") {
                            self.errors.push(self.error_at(
                                format!(
                                    "Type '{n}' is not iterable; conform to Iterator or use Array/Set/Range"
                                ),
                                *span,
                                "S012",
                            ));
                            TypeId::Unknown
                        } else {
                            TypeId::platform_int()
                        }
                    }
                    _ => {
                        self.errors.push(self.error_at(
                            "for-in requires an iterable type (Iterator, Array, Set, or Range)",
                            *span,
                            "S012",
                        ));
                        TypeId::Unknown
                    }
                };
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: elem_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(body)?;
                Ok(())
            }
            Stmt::Block { stmts, .. } => {
                let saved_borrows = self.mut_borrows.clone();
                let saved_immutable = self.immutable_ref_bindings.clone();
                let saved_lifetime_labels = self.active_lifetime_labels.clone();
                self.borrow_region_depth = self.borrow_region_depth.saturating_add(1);
                let mut child = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                std::mem::swap(&mut self.scope, &mut child);
                for stmt in stmts {
                    if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                        break;
                    }
                    if let Err(e) = self.analyze_stmt(stmt) {
                        self.push_diagnostic(e);
                    }
                }
                self.scope = *self.scope.parent.take().unwrap_or_default();
                self.mut_borrows = saved_borrows;
                self.immutable_ref_bindings = saved_immutable;
                self.active_lifetime_labels = saved_lifetime_labels;
                self.borrow_region_depth = self.borrow_region_depth.saturating_sub(1);
                Ok(())
            }
            Stmt::Seq { stmts, .. } => {
                for stmt in stmts {
                    if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                        break;
                    }
                    if let Err(e) = self.analyze_stmt(stmt) {
                        self.push_diagnostic(e);
                    }
                }
                Ok(())
            }
            Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => Ok(()),
            Stmt::Try {
                span,
                body,
                catch_name,
                catch_type,
                catch_body,
                ..
            } => {
                self.try_depth += 1;
                self.analyze_block(body)?;
                self.try_depth = self.try_depth.saturating_sub(1);
                let catch_ty = catch_type
                    .as_ref()
                    .map(|t| self.resolve_type_expr(t))
                    .unwrap_or(TypeId::String);
                // Catch binding is scoped to the catch block only, so multiple
                // `catch (e)` in the same function do not collide (S001).
                let mut catch_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                catch_scope.define(Symbol {
                    name: catch_name.clone(),
                    ty: catch_ty.clone(),
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                // Persist for IDE hover after the catch scope is popped (locals
                // must beat host globals such as Math constant host `e`).
                self.track_binding(catch_name.clone(), catch_ty, span.start);
                std::mem::swap(&mut self.scope, &mut catch_scope);
                self.analyze_block(catch_body)?;
                self.scope = *self.scope.parent.take().unwrap_or_default();
                Ok(())
            }
            Stmt::Switch {
                subject,
                cases,
                default,
                ..
            } => {
                let subject_ty = self.analyze_expr(subject)?;
                if let TypeId::Named(ref enum_name) = subject_ty {
                    if default.is_none() {
                        if let Some(all) = self.enum_cases.get(enum_name) {
                            let covered: HashSet<SmolStr> = cases
                                .iter()
                                .filter_map(|c| Self::switch_pattern_case_name(&c.pattern))
                                .collect();
                            let missing: Vec<_> = all
                                .iter()
                                .filter(|c| !covered.contains(*c))
                                .cloned()
                                .collect();
                            if !missing.is_empty() {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Switch on enum '{enum_name}' is not exhaustive; missing case(s): {}",
                                        missing
                                            .iter()
                                            .map(|c| c.as_str())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ))
                                    .with_code("S007"),
                                );
                            }
                        }
                    }
                }
                let has_tuple_pattern = cases
                    .iter()
                    .any(|c| matches!(c.pattern, Expr::TuplePattern { .. }));
                if has_tuple_pattern
                    && !matches!(
                        subject_ty,
                        TypeId::Array(_) | TypeId::Tuple { .. } | TypeId::Unknown
                    )
                {
                    self.errors.push(
                        SyxDiagnostic::error("Tuple pattern requires an array/tuple subject")
                            .with_code("S015"),
                    );
                }
                for case in cases {
                    let pattern = &case.pattern;
                    let body = &case.body;
                    let mut case_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    if let Expr::VariantPattern { bindings, .. } = pattern {
                        for (_label, bind) in bindings {
                            case_scope.define(Symbol {
                                name: bind.clone(),
                                ty: TypeId::Unknown,
                                mutable: false,
                                access: AccessLevel::Internal,
                            })?;
                        }
                    }
                    if let Expr::TuplePattern { bindings, .. } = pattern {
                        for bind in bindings {
                            case_scope.define(Symbol {
                                name: bind.clone(),
                                ty: TypeId::Unknown,
                                mutable: false,
                                access: AccessLevel::Internal,
                            })?;
                        }
                    }
                    std::mem::swap(&mut self.scope, &mut case_scope);
                    if let Some(guard) = &case.guard {
                        let guard_ty = self.analyze_expr(guard)?;
                        if guard_ty != TypeId::Bool {
                            self.errors.push(
                                SyxDiagnostic::error(
                                    "Switch case 'where' guard must be a Bool expression",
                                )
                                .with_code("S013"),
                            );
                        }
                    }
                    self.analyze_block(body)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if default.is_none() {
                    let all_have_where = !cases.is_empty()
                        && cases.iter().all(|c| c.guard.is_some());
                    let has_plain_literal = cases.iter().any(|c| {
                        c.guard.is_none()
                            && matches!(
                                c.pattern,
                                Expr::Literal {
                                    value: Literal::Integer { .. },
                                    ..
                                }
                            )
                    });
                    if all_have_where && !has_plain_literal {
                        self.errors.push(
                            SyxDiagnostic::error(
                                "Switch with only 'where' guards requires a default case for exhaustiveness",
                            )
                            .with_code("S013"),
                        );
                    }
                }
                if let Some(body) = default {
                    self.analyze_block(body)?;
                }
                Ok(())
            }
            Stmt::IfCase {
                pattern,
                subject,
                body,
                else_branch,
                ..
            } => {
                let subject_ty = self.analyze_expr(subject)?;
                if matches!(pattern, Expr::TuplePattern { .. })
                    && !matches!(
                        subject_ty,
                        TypeId::Array(_) | TypeId::Tuple { .. } | TypeId::Unknown
                    )
                {
                    self.errors.push(
                        SyxDiagnostic::error("Tuple pattern requires an array/tuple subject")
                            .with_code("S015"),
                    );
                }
                if let Expr::VariantPattern { bindings, .. } = pattern {
                    for (_label, bind) in bindings {
                        self.scope.define(Symbol {
                            name: bind.clone(),
                            ty: TypeId::Unknown,
                            mutable: false,
                            access: AccessLevel::Internal,
                        })?;
                    }
                }
                if let Expr::TuplePattern { bindings, .. } = pattern {
                    for bind in bindings {
                        self.scope.define(Symbol {
                            name: bind.clone(),
                            ty: TypeId::Unknown,
                            mutable: false,
                            access: AccessLevel::Internal,
                        })?;
                    }
                }
                self.analyze_block(body)?;
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                Ok(())
            }
            Stmt::IfLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                let bind_ty = self.infer_pattern_type(pattern)?;
                self.analyze_expr(pattern)?;
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: bind_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(body)?;
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                Ok(())
            }
            Stmt::GuardLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                let bind_ty = self.infer_pattern_type(pattern)?;
                self.analyze_expr(pattern)?;
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: bind_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(else_branch)?;
                self.analyze_block(body)?;
                Ok(())
            }
            Stmt::Throw { expr, .. } => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::RepeatWhile { body, condition, .. } => {
                self.analyze_block(body)?;
                self.analyze_expr(condition)?;
                Ok(())
            }
            Stmt::MacroInvoke { args, .. } => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
        }
    }

}
