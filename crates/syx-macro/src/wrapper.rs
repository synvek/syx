//! Phase 58: `@attached(wrapper)` GA — property wrapper desugar in macro engine.

use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::{Attribute, AttributeArg, Decl, Expr, Stmt, TypeExpr};
use syx_diagnostics::SyxResult;

use crate::warnings;

fn is_wrapper_class(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        matches!(
            a.name.as_str(),
            "propertyWrapper" | "Wrapper" | "propertywrapper"
        ) || (a.name.as_str() == "attached"
            && a.args.iter().any(|arg| matches!(arg, AttributeArg::Ident(id) if id.as_str() == "wrapper")))
    })
}

/// Expand `@propertyWrapper` / `@attached(wrapper)` fields to backing storage + `wrappedValue` access.
pub(crate) fn expand_property_wrappers(module: &mut syx_ast::Module) -> SyxResult<()> {
    let mut wrapper_types = indexmap::IndexSet::new();
    for decl in &module.decls {
        if let Decl::Class { name, attrs, .. } = decl {
            if is_wrapper_class(attrs) {
                if attrs.iter().any(|a| {
                    matches!(
                        a.name.as_str(),
                        "propertyWrapper" | "Wrapper" | "propertywrapper"
                    )
                }) {
                    warnings::push_macro_warning(
                        syx_diagnostics::SyxDiagnostic::warning(
                            "@propertyWrapper is deprecated; use @attached(wrapper)",
                        )
                        .with_code("W001"),
                    );
                }
                wrapper_types.insert(name.clone());
            }
        }
    }

    type FieldRewrite = (SmolStr, SmolStr);

    fn rewrite_member_expr(expr: &mut Expr, rewrites: &IndexMap<SmolStr, FieldRewrite>) {
        match expr {
            Expr::Member { span, object, name } => {
                rewrite_member_expr(object, rewrites);
                if let Some((storage, _)) = rewrites.get(name) {
                    if let Expr::Ident { .. } | Expr::Member { .. } = object.as_ref() {
                        *expr = Expr::Member {
                            span: *span,
                            object: Box::new(Expr::Member {
                                span: *span,
                                object: object.clone(),
                                name: storage.clone(),
                            }),
                            name: SmolStr::new("wrappedValue"),
                        };
                    }
                }
            }
            Expr::MemberAssign {
                span,
                object,
                name,
                value,
            } => {
                rewrite_member_expr(object, rewrites);
                rewrite_member_expr(value, rewrites);
                if let Some((storage, wrapper)) = rewrites.get(name) {
                    let span = *span;
                    let wrapped = Expr::Call {
                        span,
                        callee: Box::new(Expr::Ident {
                            span,
                            name: wrapper.clone(),
                        }),
                        args: vec![syx_ast::CallArg {
                            label: Some(SmolStr::new("wrappedValue")),
                            value: *value.clone(),
                        }],
                    };
                    *expr = Expr::MemberAssign {
                        span,
                        object: object.clone(),
                        name: storage.clone(),
                        value: Box::new(wrapped),
                    };
                }
            }
            Expr::Binary { left, right, .. } => {
                rewrite_member_expr(left, rewrites);
                rewrite_member_expr(right, rewrites);
            }
            Expr::Call { callee, args, .. } => {
                rewrite_member_expr(callee, rewrites);
                for a in args {
                    rewrite_member_expr(&mut a.value, rewrites);
                }
            }
            Expr::Group { expr, .. } => rewrite_member_expr(expr, rewrites),
            Expr::Index { object, index, .. } => {
                rewrite_member_expr(object, rewrites);
                rewrite_member_expr(index, rewrites);
            }
            _ => {}
        }
    }

    fn rewrite_stmts(stmts: &mut [Stmt], rewrites: &IndexMap<SmolStr, FieldRewrite>) {
        for stmt in stmts {
            match stmt {
                Stmt::Print { expr, .. } | Stmt::Expr { expr, .. } => {
                    rewrite_member_expr(expr, rewrites);
                }
                Stmt::Return { value: Some(expr), .. } => rewrite_member_expr(expr, rewrites),
                Stmt::VarDecl { init: Some(expr), .. } => rewrite_member_expr(expr, rewrites),
                Stmt::Block { stmts, .. } => rewrite_stmts(stmts, rewrites),
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    rewrite_member_expr(condition, rewrites);
                    rewrite_stmts(then_branch, rewrites);
                    if let Some(eb) = else_branch {
                        rewrite_stmts(eb, rewrites);
                    }
                }
                Stmt::While { condition, body, .. } => {
                    rewrite_member_expr(condition, rewrites);
                    rewrite_stmts(body, rewrites);
                }
                _ => {}
            }
        }
    }

    let mut field_rewrites: IndexMap<SmolStr, FieldRewrite> = IndexMap::new();
    for decl in &mut module.decls {
        let members = match decl {
            Decl::Class { members, .. } | Decl::Actor { members, .. } => Some(members),
            _ => None,
        };
        if let Some(members) = members {
            for member in members.iter_mut() {
                if let Decl::VarGlobal {
                    attrs,
                    name: field,
                    access,
                    mutable,
                    span,
                    ..
                } = member
                {
                    if let Some(wrapper_name) = attrs.iter().find_map(|a| {
                        if wrapper_types.contains(&a.name) {
                            Some(a.name.clone())
                        } else {
                            None
                        }
                    }) {
                        let storage = SmolStr::new(format!("_{field}"));
                        field_rewrites.insert(field.clone(), (storage.clone(), wrapper_name.clone()));
                        *member = Decl::VarGlobal {
                            span: *span,
                            access: *access,
                            attrs: vec![],
                            mutable: *mutable,
                            name: storage,
                            ty: Some(TypeExpr::Named {
                                name: wrapper_name,
                                generics: vec![],
                            }),
                            init: None,
                            will_set: None,
                            did_set: None,
                            computed_getter: None,
                            computed_setter: None,
                            is_override: false,
                            is_lazy: false,
                        };
                    }
                }
            }
        }
    }
    if !field_rewrites.is_empty() {
        fn rewrite_module_decls(
            decls: &mut [Decl],
            rewrites: &IndexMap<SmolStr, FieldRewrite>,
        ) {
            for decl in decls {
                match decl {
                    Decl::Function { body, .. }
                    | Decl::Init { body, .. }
                    | Decl::Deinit { body, .. } => rewrite_stmts(body, rewrites),
                    Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                        rewrite_module_decls(members, rewrites);
                    }
                    Decl::Extension { methods, .. } => rewrite_module_decls(methods, rewrites),
                    _ => {}
                }
            }
        }
        rewrite_module_decls(&mut module.decls, &field_rewrites);
    }
    Ok(())
}
