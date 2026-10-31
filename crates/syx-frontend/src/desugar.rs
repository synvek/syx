//! Surface desugars + macro expand (M1-A01; moved from syx-driver).

use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

pub fn expand_macros(module: syx_ast::Module) -> SyxResult<syx_ast::Module> {
    let mut module = syx_macro::expand_module(module)?;
    desugar_subscript_decls(&mut module)?;
    desugar_property_decls(&mut module)?;
    desugar_qualified_ctor_calls(&mut module);
    Ok(module)
}
/// SYX 39.0: `Outer.Inner()` / `mod.Outer.Inner()` → mangled constructor name.
pub fn desugar_qualified_ctor_calls(module: &mut syx_ast::Module) {
    use syx_ast::{Decl, Expr, Stmt};

    fn member_chain_ctor_name(expr: &Expr) -> Option<SmolStr> {
        syx_ast::nested_type_ctor_mangled(expr)
    }

    fn walk_expr(expr: &mut Expr) {
        match expr {
            Expr::Call { callee, args, span } => {
                for arg in args {
                    walk_expr(&mut arg.value);
                }
                if let Some(mangled) = member_chain_ctor_name(callee) {
                    *callee = Box::new(Expr::Ident {
                        span: *span,
                        name: mangled,
                    });
                } else {
                    walk_expr(callee);
                }
            }
            Expr::Binary { left, right, .. } => {
                walk_expr(left);
                walk_expr(right);
            }
            Expr::Unary { expr: inner, .. } => walk_expr(inner),
            Expr::Member { object, .. } => walk_expr(object),
            Expr::Index { object, index, .. } => {
                walk_expr(object);
                walk_expr(index);
            }
            Expr::Assign { value, .. } => walk_expr(value),
            Expr::CompoundAssign { value, .. } => walk_expr(value),
            Expr::MemberAssign { object, value, .. } => {
                walk_expr(object);
                walk_expr(value);
            }
            Expr::IndexAssign { object, index, value, .. } => {
                walk_expr(object);
                walk_expr(index);
                walk_expr(value);
            }
            Expr::Array { elements, .. } => {
                for el in elements {
                    walk_expr(el);
                }
            }
            Expr::Closure { body, .. } => match body {
                syx_ast::ClosureBody::Expr(e) => walk_expr(e),
                syx_ast::ClosureBody::Block(stmts) => walk_stmts(stmts),
            },
            Expr::OptionalCast { expr, .. } | Expr::Is { expr, .. } | Expr::Cast { expr, .. } => {
                walk_expr(expr);
            }
            Expr::TypeSelf { .. } => {}
            Expr::Await { expr, .. } | Expr::ForceUnwrap { expr, .. } => walk_expr(expr),
            Expr::MacroInvoke { args, .. } => {
                for arg in args {
                    walk_expr(arg);
                }
            }
            _ => {}
        }
    }

    fn walk_stmts(stmts: &mut [Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::VarDecl { init, .. } => {
                    if let Some(v) = init {
                        walk_expr(v);
                    }
                }
                Stmt::Return { value: Some(v), .. } => walk_expr(v),
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    walk_expr(condition);
                    walk_stmts(then_branch);
                    if let Some(e) = else_branch {
                        walk_stmts(e);
                    }
                }
                Stmt::While { condition, body, .. } => {
                    walk_expr(condition);
                    walk_stmts(body);
                }
                Stmt::ForIn { iterable, body, .. } => {
                    walk_expr(iterable);
                    walk_stmts(body);
                }
                Stmt::Print { expr, .. } | Stmt::Expr { expr, .. } => walk_expr(expr),
                Stmt::Switch { subject, cases, default, .. } => {
                    walk_expr(subject);
                    for case in cases {
                        walk_expr(&mut case.pattern);
                        if let Some(g) = &mut case.guard {
                            walk_expr(g);
                        }
                        walk_stmts(&mut case.body);
                    }
                    if let Some(d) = default {
                        walk_stmts(d);
                    }
                }
                Stmt::Block { stmts, .. } => walk_stmts(stmts),
                Stmt::Try { body, catch_body, .. } => {
                    walk_stmts(body);
                    walk_stmts(catch_body);
                }
                _ => {}
            }
        }
    }

    fn walk_decls(decls: &mut [Decl]) {
        for decl in decls {
            match decl {
                Decl::Function { body, .. } => walk_stmts(body),
                Decl::Class { members, .. } => walk_decls(members),
                Decl::Extension { methods, .. } | Decl::Protocol { methods, .. } => {
                    walk_decls(methods);
                }
                Decl::VarGlobal { init, .. } => {
                    if let Some(v) = init {
                        walk_expr(v);
                    }
                }
                _ => {}
            }
        }
    }

    walk_decls(&mut module.decls);
}
pub fn desugar_subscript_decls(module: &mut syx_ast::Module) -> SyxResult<()> {
    use syx_ast::{AccessLevel, Decl, Param, ParamLabel};

    fn desugar_in_decls(decls: &mut Vec<Decl>, enclosing: Option<smol_str::SmolStr>) -> SyxResult<()> {
        let mut extra = Vec::new();
        decls.retain_mut(|decl| {
            if let Decl::Subscript {
                span,
                params,
                return_type,
                getter,
                setter,
                ..
            } = decl
            {
                // S1-I9: do not inject `self` into AST params — class method analysis already
                // defines `self` in scope (matches property getter desugar).
                let get_name = smol_str::SmolStr::new("__subscript_get");
                let get_params = params.clone();
                let _ = enclosing;
                extra.push(Decl::Function {
                    span: *span,
                    access: AccessLevel::Internal,
                    attrs: vec![],
                    name: get_name,
                    type_params: vec![],
                    params: get_params,
                    return_type: Some(return_type.clone()),
                    throws_type: None,
                    effect: None,
                    where_clause: vec![],
                    body: getter.clone(),
                    is_async: false,
                    is_override: false,
                    is_final: false,
                });
                if let Some(set_body) = setter {
                    let mut set_params = params.clone();
                    set_params.push(Param {
                        name: smol_str::SmolStr::new("newValue"),
                        external_label: ParamLabel::Same,
                        ty: Some(return_type.clone()),
                        default: None,
                        isolated: false,
                        sending: false,
                        attrs: vec![],
                    });
                    let set_stmts = set_body.clone();
                    extra.push(Decl::Function {
                        span: *span,
                        access: AccessLevel::Internal,
                        attrs: vec![],
                        name: smol_str::SmolStr::new("__subscript_set"),
                        type_params: vec![],
                        params: set_params,
                        return_type: None,
                        throws_type: None,
                        effect: None,
                        where_clause: vec![],
                        body: set_stmts,
                        is_async: false,
                        is_override: false,
                        is_final: false,
                    });
                }
                false
            } else {
                if let Decl::Class { name, members, .. } = decl {
                    desugar_in_decls(members, Some(name.clone())).ok();
                }
                if let Decl::Extension { methods, type_name, .. } = decl {
                    desugar_in_decls(methods, Some(type_name.clone())).ok();
                }
                true
            }
        });
        decls.extend(extra);
        Ok(())
    }
    desugar_in_decls(&mut module.decls, None)
}

pub fn desugar_property_decls(module: &mut syx_ast::Module) -> SyxResult<()> {
    use syx_ast::{
        lazy_storage_field, static_var_global_name, var_global_getter_method,
        var_global_is_static, var_global_setter_method, AccessLevel, Attribute, Decl, Expr,
        Param, ParamLabel, Stmt, TypeExpr,
    };

    fn member_getter(
        span: syx_diagnostics::Span,
        _type_name: &smol_str::SmolStr,
        method: &smol_str::SmolStr,
        return_type: Option<TypeExpr>,
        body: Vec<Stmt>,
        is_override: bool,
        is_static: bool,
        access: AccessLevel,
    ) -> Decl {
        // Class methods do not list `self` in AST params — lowering injects it
        // for instance accessors; `@static` getters stay nullary.
        let attrs = if is_static {
            vec![Attribute {
                name: smol_str::SmolStr::new("static"),
                args: vec![],
            }]
        } else {
            vec![]
        };
        Decl::Function {
            span,
            access,
            attrs,
            name: method.clone(),
            type_params: vec![],
            params: vec![],
            return_type,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body,
            is_async: false,
            is_override,
            is_final: false,
        }
    }

    fn member_setter(
        span: syx_diagnostics::Span,
        _type_name: &smol_str::SmolStr,
        method: &smol_str::SmolStr,
        value_type: Option<TypeExpr>,
        body: Vec<Stmt>,
        is_override: bool,
        is_static: bool,
        access: AccessLevel,
    ) -> Decl {
        let attrs = if is_static {
            vec![Attribute {
                name: smol_str::SmolStr::new("static"),
                args: vec![],
            }]
        } else {
            vec![]
        };
        Decl::Function {
            span,
            access,
            attrs,
            name: method.clone(),
            type_params: vec![],
            params: vec![Param {
                name: smol_str::SmolStr::new("newValue"),
                external_label: ParamLabel::Same,
                ty: value_type,
                default: None,
                isolated: false,
                sending: false,
                attrs: vec![],
            }],
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body,
            is_async: false,
            is_override,
            is_final: false,
        }
    }

    fn lazy_getter_body(
        field: &smol_str::SmolStr,
        storage: &smol_str::SmolStr,
        inited: &smol_str::SmolStr,
        init: &Expr,
        span: syx_diagnostics::Span,
    ) -> Vec<Stmt> {
        let self_expr = Expr::Ident {
            span,
            name: smol_str::SmolStr::new("self"),
        };
        let inited_get = Expr::Member {
            span,
            object: Box::new(self_expr.clone()),
            name: inited.clone(),
        };
        vec![
            Stmt::If {
                span,
                condition: inited_get,
                then_branch: vec![Stmt::Return {
                    span,
                    value: Some(Expr::Member {
                        span,
                        object: Box::new(self_expr.clone()),
                        name: storage.clone(),
                    }),
                }],
                else_branch: None,
            },
            Stmt::VarDecl {
                span,
                mutable: false,
                name: smol_str::SmolStr::new(format!("__lazy_v_{field}")),
                ty: None,
                init: Some(init.clone()),
            },
            Stmt::Expr {
                span,
                expr: Expr::MemberAssign {
                    span,
                    object: Box::new(self_expr.clone()),
                    name: storage.clone(),
                    value: Box::new(Expr::Ident {
                        span,
                        name: smol_str::SmolStr::new(format!("__lazy_v_{field}")),
                    }),
                },
            },
            Stmt::Expr {
                span,
                expr: Expr::MemberAssign {
                    span,
                    object: Box::new(self_expr.clone()),
                    name: inited.clone(),
                    value: Box::new(Expr::Literal {
                        span,
                        value: syx_ast::Literal::Bool(true),
                    }),
                },
            },
            Stmt::Return {
                span,
                value: Some(Expr::Ident {
                    span,
                    name: smol_str::SmolStr::new(format!("__lazy_v_{field}")),
                }),
            },
        ]
    }

    fn walk(
        decls: &mut Vec<Decl>,
        enclosing: Option<smol_str::SmolStr>,
        hoisted: &mut Vec<Decl>,
    ) -> SyxResult<()> {
        let mut extra = Vec::new();
        decls.retain_mut(|decl| {
            if let Decl::Class { name, members, .. } | Decl::Actor { name, members, .. } = decl {
                walk(members, Some(name.clone()), hoisted).ok();
            }
            // Phase 136: extension computed `{ get }` / `{ get set }` → `__get_` / `__set_` methods.
            if let Decl::Extension {
                type_name, methods, ..
            } = decl
            {
                walk(methods, Some(type_name.clone()), hoisted).ok();
            }
            if let Decl::VarGlobal {
                span,
                access,
                attrs,
                mutable,
                name,
                ty,
                init,
                will_set,
                did_set,
                computed_getter,
                computed_setter,
                is_override,
                is_lazy,
            } = decl
            {
                // Phase 137: `@Published var` on class stored → inject willSet notify print.
                // Prefer willSet over didSet: didSet observer codegen + class writeback can drop stores.
                if enclosing.is_some()
                    && computed_getter.is_none()
                    && *mutable
                    && attrs.iter().any(|a| a.name.as_str() == "Published")
                {
                    let notify = Stmt::Expr {
                        span: *span,
                        expr: Expr::Call {
                            span: *span,
                            callee: Box::new(Expr::Ident {
                                span: *span,
                                name: smol_str::SmolStr::new("print"),
                            }),
                            args: vec![syx_ast::CallArg {
                                label: None,
                                value: Expr::Literal {
                                    span: *span,
                                    value: syx_ast::Literal::String(format!("published:{name}")),
                                },
                            }],
                        },
                    };
                    match will_set {
                        Some(body) => body.push(notify),
                        None => *will_set = Some(vec![notify]),
                    }
                    attrs.retain(|a| a.name.as_str() != "Published");
                }
                let Some(type_name) = enclosing.clone() else {
                    return true;
                };
                let is_static = var_global_is_static(attrs);
                // Static computed `{ get }` / `{ get set }` and `static let` →
                // `@static func __get_` (cross-module mergeable; Member → Call).
                // Stored `static var` keeps the hoisted `syx_static_var` global.
                if is_static {
                    if let Some(getter) = computed_getter.take() {
                        let method = var_global_getter_method(name);
                        extra.push(member_getter(
                            *span,
                            &type_name,
                            &method,
                            ty.clone(),
                            getter,
                            *is_override,
                            true,
                            *access,
                        ));
                        if let Some(setter) = computed_setter.take() {
                            let set_method = var_global_setter_method(name);
                            extra.push(member_setter(
                                *span,
                                &type_name,
                                &set_method,
                                ty.clone(),
                                setter,
                                *is_override,
                                true,
                                *access,
                            ));
                        }
                        return false;
                    }
                    if !*mutable {
                        let Some(init_expr) = init.take() else {
                            return true;
                        };
                        let method = var_global_getter_method(name);
                        extra.push(member_getter(
                            *span,
                            &type_name,
                            &method,
                            ty.clone(),
                            vec![Stmt::Return {
                                span: *span,
                                value: Some(init_expr),
                            }],
                            *is_override,
                            true,
                            *access,
                        ));
                        return false;
                    }
                    let global_name = static_var_global_name(&type_name, name);
                    hoisted.push(Decl::VarGlobal {
                        span: *span,
                        access: *access,
                        attrs: vec![Attribute {
                            name: smol_str::SmolStr::new("syx_static_var"),
                            args: vec![
                                syx_ast::AttributeArg::Ident(type_name.clone()),
                                syx_ast::AttributeArg::Ident(name.clone()),
                            ],
                        }],
                        mutable: *mutable,
                        name: global_name,
                        ty: ty.clone(),
                        init: init.clone(),
                        will_set: will_set.clone(),
                        did_set: did_set.clone(),
                        computed_getter: None,
                        computed_setter: None,
                        is_override: false,
                        is_lazy: false,
                    });
                    return false;
                }
                if let Some(getter) = computed_getter.take() {
                    let method = var_global_getter_method(name);
                    extra.push(member_getter(
                        *span,
                        &type_name,
                        &method,
                        ty.clone(),
                        getter,
                        *is_override,
                        false,
                        *access,
                    ));
                    if let Some(setter) = computed_setter.take() {
                        let set_method = var_global_setter_method(name);
                        extra.push(member_setter(
                            *span,
                            &type_name,
                            &set_method,
                            ty.clone(),
                            setter,
                            *is_override,
                            false,
                            *access,
                        ));
                    }
                    return false;
                }
                if *is_lazy {
                    let Some(init_expr) = init.take() else {
                        return true;
                    };
                    let storage = lazy_storage_field(name);
                    let inited = smol_str::SmolStr::new(format!("__lazy_inited_{name}"));
                    extra.push(Decl::VarGlobal {
                        span: *span,
                        access: AccessLevel::Internal,
                        attrs: vec![],
                        mutable: true,
                        name: storage.clone(),
                        ty: ty.clone(),
                        init: None,
                        will_set: None,
                        did_set: None,
                        computed_getter: None,
                        computed_setter: None,
                        is_override: false,
                        is_lazy: false,
                    });
                    extra.push(Decl::VarGlobal {
                        span: *span,
                        access: AccessLevel::Internal,
                        attrs: vec![],
                        mutable: true,
                        name: inited.clone(),
                        ty: Some(TypeExpr::Named {
                            name: smol_str::SmolStr::new("Bool"),
                            generics: vec![],
                        }),
                        init: Some(Expr::Literal {
                            span: *span,
                            value: syx_ast::Literal::Bool(false),
                        }),
                        will_set: None,
                        did_set: None,
                        computed_getter: None,
                        computed_setter: None,
                        is_override: false,
                        is_lazy: false,
                    });
                    let method = var_global_getter_method(name);
                    extra.push(member_getter(
                        *span,
                        &type_name,
                        &method,
                        ty.clone(),
                        lazy_getter_body(name, &storage, &inited, &init_expr, *span),
                        false,
                        false,
                        *access,
                    ));
                    return false;
                }
            }
            true
        });
        decls.extend(extra);
        Ok(())
    }

    let mut hoisted = Vec::new();
    walk(&mut module.decls, None, &mut hoisted)?;
    module.decls.extend(hoisted);
    Ok(())
}
/// Phase 60: wrap top-level executable statements into `__syx_main_top_level()` (main file only).
/// S1-I9: if the top-level body uses `await`, mark the wrapper (and `main`) `async` and await the call.
pub fn desugar_top_level_stmts(module: &mut syx_ast::Module) -> SyxResult<()> {
    use syx_ast::{AccessLevel, Decl, Expr, Stmt};

    let mut top_stmts = Vec::new();
    module.decls.retain(|decl| {
        if let Decl::TopLevelStmt { stmt, .. } = decl {
            top_stmts.push(stmt.clone());
            false
        } else {
            true
        }
    });
    if top_stmts.is_empty() {
        return Ok(());
    }

    let needs_async = syx_async::count_await_in_stmts(&top_stmts) > 0;
    let wrapper_span = top_stmts
        .first()
        .map(|s| match s {
            Stmt::Expr { span, .. }
            | Stmt::Print { span, .. }
            | Stmt::VarDecl { span, .. }
            | Stmt::Return { span, .. }
            | Stmt::If { span, .. }
            | Stmt::IfLet { span, .. }
            | Stmt::IfCase { span, .. }
            | Stmt::GuardLet { span, .. }
            | Stmt::Switch { span, .. }
            | Stmt::While { span, .. }
            | Stmt::ForIn { span, .. }
            | Stmt::Try { span, .. }
            | Stmt::Throw { span, .. }
            | Stmt::RepeatWhile { span, .. }
            | Stmt::Break { span, .. }
            | Stmt::Continue { span, .. }
            | Stmt::Block { span, .. }
            | Stmt::Seq { span, .. }
            | Stmt::MacroInvoke { span, .. } => *span,
        })
        .unwrap_or(syx_diagnostics::Span::DUMMY);

    let wrapper = Decl::Function {
        span: wrapper_span,
        access: AccessLevel::Internal,
        attrs: vec![],
        name: SmolStr::new("__syx_main_top_level"),
        type_params: vec![],
        params: vec![],
        return_type: None,
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body: top_stmts,
        is_async: needs_async,
        is_override: false,
        is_final: false,
    };

    let call_expr = Expr::Call {
        span: syx_diagnostics::Span::DUMMY,
        callee: Box::new(Expr::Ident {
            span: syx_diagnostics::Span::DUMMY,
            name: SmolStr::new("__syx_main_top_level"),
        }),
        args: vec![],
    };
    let call = Stmt::Expr {
        span: syx_diagnostics::Span::DUMMY,
        expr: if needs_async {
            Expr::Await {
                span: syx_diagnostics::Span::DUMMY,
                expr: Box::new(call_expr),
            }
        } else {
            call_expr
        },
    };

    let mut injected = false;
    for decl in &mut module.decls {
        if let Decl::Function {
            name,
            body,
            is_async,
            ..
        } = decl
        {
            if name == "main" {
                body.insert(0, call.clone());
                if needs_async {
                    *is_async = true;
                }
                injected = true;
                break;
            }
        }
    }
    if !injected {
        module.decls.push(Decl::Function {
            span: syx_diagnostics::Span::DUMMY,
            access: AccessLevel::Internal,
            attrs: vec![],
            name: SmolStr::new("main"),
            type_params: vec![],
            params: vec![],
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body: vec![call],
            is_async: needs_async,
            is_override: false,
            is_final: false,
        });
    }

    let main_idx = module
        .decls
        .iter()
        .position(|d| matches!(d, Decl::Function { name, .. } if name == "main"));
    let insert_at = main_idx.unwrap_or(module.decls.len());
    module.decls.insert(insert_at, wrapper);
    Ok(())
}
