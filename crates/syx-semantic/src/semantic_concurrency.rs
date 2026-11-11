//! SYX semantic concurrency face (S2-I256).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    attr_has_ident_arg, borrow_regions_v7_sending_enabled, global_actor_v4_cross_module_enabled,
    global_actor_v5_sendable_enabled, global_actor_v6_sending_enabled, has_attr,
    module_first_task_detach_span, TypeId,
};
use smol_str::SmolStr;
use syx_ast::{Decl, Expr, Module, Stmt};

impl SemanticAnalyzer {
    pub fn check_concurrency_rules(&mut self, module: &Module) {
        self.seed_sendable_types(module);
        for decl in &module.decls {
            if let Decl::Function { name, attrs, .. } = decl {
                self.collect_function_isolation(name, attrs);
            }
        }
        self.apply_module_default_isolation(module);
        self.check_concurrency_call_sites(module);
        if !self.strict_concurrency {
            return;
        }
        let has_non_sendable = !self.class_names.is_empty();
        if has_non_sendable {
            if let Some(span) = module_first_task_detach_span(module) {
                self.errors.push(self.error_at(
                    "Task.detached / __syx_task_detach cannot be used with non-Sendable class types in strict concurrency mode",
                    span,
                    "C003",
                ));
            }
        }
    }

    fn check_concurrency_call_sites(&mut self, module: &Module) {
        for decl in &module.decls {
            if let Decl::Function { name, body, .. } = decl {
                let caller_actor = self.caller_global_actor(name);
                let caller_nonisolated = self.nonisolated_functions.contains(name);
                self.check_stmts_concurrency(body, caller_actor.as_ref(), caller_nonisolated, None);
            }
            if let Decl::Actor { name, members, .. } = decl {
                for member in members {
                    if let Decl::Function { body, .. } = member {
                        self.check_stmts_concurrency(body, Some(name), false, Some(name.clone()));
                    }
                }
            }
        }
    }

    pub(crate) fn caller_global_actor(&self, name: &SmolStr) -> Option<SmolStr> {
        if name.as_str() == "main" {
            return Some(SmolStr::new("MainActor"));
        }
        self.function_global_actor.get(name).cloned()
    }

    fn caller_in_actor(&self, actor: Option<&SmolStr>, required: &SmolStr) -> bool {
        match actor {
            Some(current) => current == required,
            None => false,
        }
    }

    fn check_stmts_concurrency(
        &mut self,
        stmts: &[Stmt],
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<SmolStr>,
    ) {
        for stmt in stmts {
            self.check_stmt_concurrency(
                stmt,
                caller_actor,
                caller_nonisolated,
                enclosing_actor.as_ref(),
            );
        }
    }

    fn check_stmt_concurrency(
        &mut self,
        stmt: &Stmt,
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<&SmolStr>,
    ) {
        match stmt {
            Stmt::Expr { expr, .. }
            | Stmt::Print { expr, .. }
            | Stmt::Return {
                value: Some(expr), ..
            } => {
                self.check_expr_concurrency(
                    expr,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Stmt::VarDecl {
                init: Some(expr), ..
            } => {
                self.check_expr_concurrency(
                    expr,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.check_expr_concurrency(
                    condition,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                self.check_stmts_concurrency(
                    then_branch,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
                if let Some(branch) = else_branch {
                    self.check_stmts_concurrency(
                        branch,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor.cloned(),
                    );
                }
            }
            Stmt::Block { stmts, .. } => self.check_stmts_concurrency(
                stmts,
                caller_actor,
                caller_nonisolated,
                enclosing_actor.cloned(),
            ),
            Stmt::While {
                condition, body, ..
            } => {
                self.check_expr_concurrency(
                    condition,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                self.check_stmts_concurrency(
                    body,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
            }
            Stmt::ForIn { iterable, body, .. } => {
                self.check_expr_concurrency(
                    iterable,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                self.check_stmts_concurrency(
                    body,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
            }
            _ => {}
        }
    }

    fn check_expr_concurrency(
        &mut self,
        expr: &Expr,
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<&SmolStr>,
    ) {
        match expr {
            Expr::Call {
                callee, args, span, ..
            } => {
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if let Some(required) = self.function_global_actor.get(name) {
                        if !caller_nonisolated
                            && !self.caller_in_actor(caller_actor, required)
                            && self.strict_concurrency
                        {
                            self.errors.push(self.error_at(
                                format!(
                                    "Call to @{required} function '{name}' from non-{required} context"
                                ),
                                *span,
                                "C001",
                            ));
                        }
                    }
                }
                if global_actor_v4_cross_module_enabled() {
                    if let Expr::Member {
                        object,
                        name: member,
                        ..
                    } = callee.as_ref()
                    {
                        if let Expr::Ident { name: module, .. } = object.as_ref() {
                            if let Some(required) = self
                                .import_member_actors
                                .get(&(module.clone(), member.clone()))
                            {
                                if !self.caller_in_actor(caller_actor, required)
                                    && self.strict_concurrency
                                {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "Call to @{required} function '{module}.{member}' from non-{required} context"
                                        ),
                                        *span,
                                        "C001",
                                    ));
                                }
                            }
                        }
                    }
                }
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if name.as_str() == "__syx_task_detach" {
                        self.check_detach_captures(args);
                    }
                    if global_actor_v6_sending_enabled() && borrow_regions_v7_sending_enabled() {
                        if let Some(sending_flags) = self.function_sending_params.get(name).cloned()
                        {
                            for (i, arg) in args.iter().enumerate() {
                                if sending_flags.get(i).copied().unwrap_or(false) {
                                    self.check_sending_arg(&arg.value, name.as_str());
                                }
                            }
                        }
                    }
                    if self.nonsending_functions.contains(name)
                        && caller_actor.is_some()
                        && !caller_nonisolated
                        && self.strict_concurrency
                    {
                        self.errors.push(self.error_at(
                            format!(
                                "Call to @nonisolated(nonsending) '{name}' crosses actor isolation"
                            ),
                            *span,
                            "C005",
                        ));
                    }
                }
                if let Expr::Member {
                    object,
                    name: method,
                    ..
                } = callee.as_ref()
                {
                    if let Some(obj_ty) = self.expr_type(object) {
                        if let TypeId::Named(actor_name) = obj_ty {
                            if self.actor_types.contains(&actor_name)
                                && enclosing_actor != Some(&actor_name)
                            {
                                if self.strict_concurrency {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "Cross-actor call to method '{method}' on actor '{actor_name}'"
                                        ),
                                        *span,
                                        "C002",
                                    ));
                                }
                                if let Some(sending_flags) = self
                                    .actor_method_sending
                                    .get(&(actor_name.clone(), method.clone()))
                                    .cloned()
                                {
                                    for (i, arg) in args.iter().enumerate() {
                                        if sending_flags.get(i).copied().unwrap_or(false) {
                                            self.check_sending_arg(&arg.value, actor_name.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.check_expr_concurrency(
                    callee,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                for arg in args {
                    self.check_expr_concurrency(
                        &arg.value,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor,
                    );
                }
            }
            Expr::TaskBlock { body, detached, .. } => {
                if *detached && self.strict_concurrency {
                    self.check_task_body_captures(body);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr_concurrency(
                    left,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                self.check_expr_concurrency(
                    right,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Expr::Unary { expr, .. } => {
                self.check_expr_concurrency(
                    expr,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Expr::Await { expr, .. } => {
                self.check_expr_concurrency(
                    expr,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Expr::Member { object, .. } => {
                self.check_expr_concurrency(
                    object,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
            }
            Expr::Closure { body, .. } => match body {
                syx_ast::ClosureBody::Expr(e) => {
                    self.check_expr_concurrency(
                        e,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor,
                    );
                }
                syx_ast::ClosureBody::Block(stmts) => {
                    self.check_stmts_concurrency(
                        stmts,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor.cloned(),
                    );
                }
            },
            _ => {}
        }
    }

    fn check_detach_captures(&mut self, args: &[syx_ast::CallArg]) {
        for arg in args {
            self.check_capture_expr(&arg.value);
        }
    }

    fn check_task_body_captures(&mut self, body: &[Stmt]) {
        for stmt in body {
            match stmt {
                Stmt::Expr { expr, .. }
                | Stmt::Print { expr, .. }
                | Stmt::Return {
                    value: Some(expr), ..
                } => self.check_capture_expr(expr),
                Stmt::VarDecl {
                    init: Some(expr), ..
                } => self.check_capture_expr(expr),
                Stmt::Block { stmts, .. } => self.check_task_body_captures(stmts),
                _ => {}
            }
        }
    }

    fn check_sending_arg(&mut self, expr: &Expr, context: &str) {
        match expr {
            Expr::Ident { name, span, .. } => {
                if self.is_non_sendable_binding(name) {
                    self.errors.push(self.error_at(
                        format!(
                            "Non-Sendable value '{name}' cannot cross isolation boundary via `sending` parameter in {context}"
                        ),
                        *span,
                        "C004",
                    ));
                }
            }
            _ => {}
        }
    }

    fn check_capture_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident { name, span, .. } => {
                if self.is_non_sendable_binding(name) && self.strict_concurrency {
                    self.errors.push(self.error_at(
                        format!("Non-Sendable value '{name}' captured in Task.detached context"),
                        *span,
                        "C003",
                    ));
                }
            }
            Expr::Member {
                object, name, span, ..
            } => {
                if let Some(TypeId::Named(type_name)) = self.expr_type(object) {
                    if self.class_names.contains(&type_name)
                        && !self.sendable_types.contains(&type_name)
                        && self.strict_concurrency
                    {
                        self.errors.push(self.error_at(
                            format!(
                                "Non-Sendable field access '{name}' on '{type_name}' in Task.detached context"
                            ),
                            *span,
                            "C003",
                        ));
                    }
                }
                self.check_capture_expr(object);
            }
            Expr::Binary { left, right, .. } => {
                self.check_capture_expr(left);
                self.check_capture_expr(right);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.check_capture_expr(&arg.value);
                }
            }
            _ => {}
        }
    }

    pub(crate) fn is_non_sendable_binding(&self, name: &SmolStr) -> bool {
        let ty = self
            .scope
            .lookup(name)
            .or_else(|| self.globals.get(name))
            .map(|s| s.ty.clone())
            .unwrap_or(TypeId::Unknown);
        self.is_non_sendable_type(&ty)
    }

    #[allow(dead_code)] // retained for Sendable / IDE type queries
    fn binding_type(&self, name: &SmolStr) -> TypeId {
        self.scope
            .lookup(name)
            .map(|s| s.ty.clone())
            .unwrap_or(TypeId::Unknown)
    }

    pub(crate) fn is_non_sendable_type(&self, ty: &TypeId) -> bool {
        match ty {
            TypeId::Named(n) => self.class_names.contains(n) && !self.sendable_types.contains(n),
            TypeId::Optional(inner) => self.is_non_sendable_type(inner),
            _ => false,
        }
    }

    fn seed_sendable_types(&mut self, module: &Module) {
        self.sendable_types.clear();
        self.unchecked_sendable_types.clear();
        for decl in &module.decls {
            match decl {
                Decl::Actor { name, .. } => {
                    self.sendable_types.insert(name.clone());
                }
                Decl::DataClass { name, attrs, .. } => {
                    self.sendable_types.insert(name.clone());
                    if global_actor_v6_sending_enabled() && has_attr(attrs, "unchecked") {
                        self.unchecked_sendable_types.insert(name.clone());
                    } else if global_actor_v5_sendable_enabled() && has_attr(attrs, "unchecked") {
                        self.unchecked_sendable_types.insert(name.clone());
                    }
                }
                Decl::Function { name, attrs, .. } if has_attr(attrs, "Sendable") => {
                    self.sendable_types.insert(name.clone());
                }
                Decl::Protocol { name, .. } if name.as_str() == "Sendable" => {}
                _ => {}
            }
        }
        for name in ["Int", "Bool", "String", "Double", "Float"] {
            self.sendable_types.insert(SmolStr::new(name));
        }
        for (type_name, conformances) in &self.type_conformances {
            if conformances.iter().any(|p| p.as_str() == "Sendable") {
                self.sendable_types.insert(type_name.clone());
            }
        }
    }

    pub(crate) fn collect_function_isolation(
        &mut self,
        name: &SmolStr,
        attrs: &[syx_ast::Attribute],
    ) {
        if has_attr(attrs, "MainActor") {
            self.main_actor_functions.insert(name.clone());
            self.function_global_actor
                .insert(name.clone(), SmolStr::new("MainActor"));
        }
        for attr in attrs {
            if self.global_actor_names.contains(&attr.name) {
                self.function_global_actor
                    .insert(name.clone(), attr.name.clone());
            }
        }
        if has_attr(attrs, "nonisolated") {
            self.nonisolated_functions.insert(name.clone());
            if attr_has_ident_arg(attrs, "nonisolated", "nonsending") {
                self.nonsending_functions.insert(name.clone());
            }
        }
        if has_attr(attrs, "concurrent") {
            self.concurrent_functions.insert(name.clone());
        }
    }
}
