//! Phase 119 Macro M9: `@resultBuilder` lives in the fixed-point pipeline (6th pass).
//! Phase 122: literal+for-in mix uses buildBlock flatten (no E009 reject).
//! Phase 123: HostCall+Closure bodies stay inside flat_map closure (no hoist).
//! Fingerprint: `macro-pipeline-v6:rb-hostcall-closure-fixed`

use smol_str::SmolStr;
use syx_ast::Module;
use syx_diagnostics::SyxResult;

/// Desugar `@resultBuilder` function bodies into `__syx_result_builder_*` calls.
pub fn desugar_result_builders(module: &mut Module) -> SyxResult<()> {
    use syx_ast::{CallArg, ClosureBody, Decl, Expr, Param, ParamLabel, Stmt};

    fn rb_ident(span: syx_diagnostics::Span, name: &str) -> Expr {
        Expr::Ident {
            span,
            name: SmolStr::new(name),
        }
    }

    fn rb_call(span: syx_diagnostics::Span, name: &str, args: Vec<Expr>) -> Expr {
        Expr::Call {
            span,
            callee: Box::new(rb_ident(span, name)),
            args: args
                .into_iter()
                .map(|value| CallArg { label: None, value })
                .collect(),
        }
    }

    fn rb_array(span: syx_diagnostics::Span, elements: Vec<Expr>) -> Expr {
        Expr::Array { span, elements }
    }

    fn rb_wrap_chunk(span: syx_diagnostics::Span, expr: Expr) -> Expr {
        match expr {
            Expr::Array { .. } | Expr::Call { .. } => expr,
            other => rb_array(span, vec![other]),
        }
    }

    fn for_in_builder_tail(body: &[Stmt]) -> (Vec<Stmt>, Option<Expr>) {
        let mut exprs: Vec<(Expr, syx_diagnostics::Span)> = Vec::new();
        for st in body {
            match st {
                Stmt::Expr { expr, span } => exprs.push((expr.clone(), *span)),
                _ => return (body.to_vec(), None),
            }
        }
        if exprs.is_empty() {
            return (Vec::new(), None);
        }
        let (tail_expr, _) = exprs.pop().unwrap();
        let side = exprs
            .into_iter()
            .map(|(expr, span)| Stmt::Expr { expr, span })
            .collect();
        (side, Some(tail_expr))
    }

    fn rb_branch_part(body: &[Stmt]) -> (Vec<Stmt>, Option<Expr>) {
        let mut side = Vec::new();
        let mut tail = None;
        for st in body {
            match st {
                Stmt::Expr { expr, .. } => tail = Some(expr.clone()),
                Stmt::Return { value: Some(expr), .. } => tail = Some(expr.clone()),
                Stmt::Block { stmts, span } if !stmts.is_empty() => {
                    let (inner_side, inner_tail) = rb_branch_part(stmts);
                    side.extend(inner_side);
                    if let Some(e) = inner_tail {
                        tail = Some(e);
                    }
                }
                other => {
                    if tail.take().is_some() {
                        // Trailing builder expr followed by more stmts — keep whole branch.
                        return (body.to_vec(), None);
                    }
                    side.push(other.clone());
                }
            }
        }
        (side, tail)
    }

    fn expr_uses_host_call(expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, .. } => {
                if matches!(callee.as_ref(), Expr::Ident { name, .. } if name.as_str().starts_with("__syx_")) {
                    return true;
                }
                expr_uses_host_call(callee)
            }
            Expr::Array { elements, .. } => elements.iter().any(expr_uses_host_call),
            Expr::Closure { body, .. } => match body {
                ClosureBody::Block(stmts) => stmts.iter().any(|s| match s {
                    Stmt::Expr { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
                        expr_uses_host_call(expr)
                    }
                    _ => false,
                }),
                ClosureBody::Expr(e) => expr_uses_host_call(e),
            },
            _ => false,
        }
    }

    fn expr_uses_flat_map(expr: &Expr) -> bool {
        match expr {
            Expr::Call { callee, args, .. } => {
                if matches!(
                    callee.as_ref(),
                    Expr::Ident { name, .. } if name.as_str() == "__syx_array_flat_map"
                ) {
                    return true;
                }
                expr_uses_flat_map(callee) || args.iter().any(|a| expr_uses_flat_map(&a.value))
            }
            Expr::Array { elements, .. } => elements.iter().any(expr_uses_flat_map),
            _ => false,
        }
    }

    fn ident_expr(span: syx_diagnostics::Span, name: &str) -> Expr {
        Expr::Ident {
            span,
            name: SmolStr::new(name),
        }
    }

    fn rb_combine_parts(
        parts: Vec<Expr>,
        span: syx_diagnostics::Span,
        kept: &mut Vec<Stmt>,
    ) -> Option<Expr> {
        if parts.is_empty() {
            return None;
        }
        if parts.len() == 1 {
            return Some(parts.into_iter().next().unwrap());
        }
        let needs_temps = parts
            .iter()
            .any(|p| expr_uses_flat_map(p) || expr_uses_host_call(p));
        if !needs_temps {
            let mut iter = parts.into_iter();
            let mut acc = iter.next().unwrap();
            for next in iter {
                acc = rb_call(
                    span,
                    "__syx_result_builder_buildBlock",
                    vec![acc, next],
                );
            }
            return Some(acc);
        }
        let mut bound = Vec::new();
        for (idx, part) in parts.into_iter().enumerate() {
            let name = SmolStr::new(format!("__rb_part_{idx}"));
            kept.push(Stmt::VarDecl {
                span,
                mutable: false,
                name: name.clone(),
                ty: None,
                init: Some(part),
            });
            bound.push(ident_expr(span, name.as_str()));
        }
        let mut iter = bound.into_iter();
        let mut acc = iter.next().unwrap();
        for next in iter {
            acc = rb_call(
                span,
                "__syx_result_builder_buildBlock",
                vec![acc, next],
            );
        }
        Some(acc)
    }

    fn rb_desugar_stmts(stmts: Vec<Stmt>) -> SyxResult<(Vec<Stmt>, Option<Expr>)> {
        let mut kept = Vec::new();
        let mut parts = Vec::new();
        let mut span = syx_diagnostics::Span::DUMMY;

        for stmt in stmts {
            match stmt {
                Stmt::Expr { expr, span: s } => {
                    span = s;
                    parts.push(rb_wrap_chunk(s, expr));
                }
                Stmt::Return { value: Some(expr), span: s } => {
                    span = s;
                    parts.push(rb_wrap_chunk(s, expr));
                }
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    span: s,
                } => {
                    // Recurse so nested if/for inside branches become buildEither/buildArray.
                    let (kept_then, part_then) = rb_desugar_stmts(then_branch)?;
                    let (kept_else, part_else) = match else_branch {
                        Some(b) => rb_desugar_stmts(b)?,
                        None => (Vec::new(), Some(rb_array(s, vec![]))),
                    };
                    kept.extend(kept_then);
                    kept.extend(kept_else);
                    match (part_then, part_else) {
                        (Some(t), Some(e)) => {
                            span = s;
                            parts.push(rb_call(
                                s,
                                "__syx_result_builder_buildEither",
                                vec![condition, rb_wrap_chunk(s, t), rb_wrap_chunk(s, e)],
                            ));
                        }
                        (Some(t), None) | (None, Some(t)) => {
                            span = s;
                            parts.push(rb_wrap_chunk(s, t));
                        }
                        (None, None) => {}
                    }
                }
                Stmt::ForIn {
                    name,
                    iterable,
                    body,
                    span: s,
                    is_async_iter: false,
                } => {
                    let (side, tail) = for_in_builder_tail(body.as_slice());
                    if let Some(elem) = tail {
                        span = s;
                        let mut closure_body = side;
                        closure_body.push(Stmt::Return {
                            span: s,
                            value: Some(rb_wrap_chunk(s, elem)),
                        });
                        let closure = Expr::Closure {
                            span: s,
                            params: vec![Param {
                                name: name.clone(),
                                external_label: ParamLabel::Same,
                                ty: None,
                                default: None,
                                isolated: false,
                                sending: false,
                                attrs: vec![],
                            }],
                            return_type: None,
                            body: ClosureBody::Block(closure_body),
                        };
                        let mapped = rb_call(s, "__syx_array_flat_map", vec![iterable, closure]);
                        parts.push(rb_call(
                            s,
                            "__syx_result_builder_buildArray",
                            vec![mapped],
                        ));
                    } else {
                        kept.push(Stmt::ForIn {
                            name,
                            iterable,
                            body: body.clone(),
                            span: s,
                            is_async_iter: false,
                        });
                    }
                }
                Stmt::Block { stmts, span: s } | Stmt::Seq { stmts, span: s } => {
                    let (inner_kept, inner_part) = rb_desugar_stmts(stmts)?;
                    kept.extend(inner_kept);
                    if let Some(expr) = inner_part {
                        span = s;
                        parts.push(expr);
                    }
                }
                Stmt::IfLet {
                    name,
                    pattern,
                    body,
                    else_branch,
                    span: s,
                } => {
                    let mut bound_body = vec![Stmt::VarDecl {
                        span: s,
                        mutable: false,
                        name: name.clone(),
                        ty: None,
                        init: Some(pattern.clone()),
                    }];
                    bound_body.extend(body.clone());
                    let (kept_then, part_then) = rb_desugar_stmts(bound_body)?;
                    let (kept_else, part_else) = match else_branch {
                        Some(b) => rb_desugar_stmts(b)?,
                        None => (Vec::new(), Some(rb_array(s, vec![]))),
                    };
                    kept.extend(kept_then);
                    kept.extend(kept_else);
                    match (part_then, part_else) {
                        (Some(t), Some(e)) => {
                            span = s;
                            parts.push(rb_call(
                                s,
                                "__syx_result_builder_buildOptional",
                                vec![pattern, rb_wrap_chunk(s, t), rb_wrap_chunk(s, e)],
                            ));
                        }
                        (Some(t), None) | (None, Some(t)) => {
                            span = s;
                            parts.push(rb_wrap_chunk(s, t));
                        }
                        (None, None) => {}
                    }
                }
                Stmt::VarDecl { .. }
                | Stmt::Print { .. }
                | Stmt::While { .. }
                | Stmt::Try { .. }
                | Stmt::Switch { .. }
                | Stmt::IfCase { .. }
                | Stmt::GuardLet { .. }
                | Stmt::RepeatWhile { .. }
                | Stmt::Break { .. }
                | Stmt::Continue { .. }
                | Stmt::Throw { .. }
                | Stmt::MacroInvoke { .. } => kept.push(stmt),
                Stmt::Return { value: None, .. } => kept.push(stmt),
                Stmt::ForIn {
                    is_async_iter: true,
                    ..
                } => kept.push(stmt),
            }
        }

        // Fold with binary buildBlock (runtime flatten handles literal + for-in mix).
        let combined = rb_combine_parts(parts, span, &mut kept);
        Ok((kept, combined))
    }

    fn desugar_body(body: &mut Vec<Stmt>) -> SyxResult<()> {
        let stmts = body.drain(..).collect();
        let (mut kept, combined) = rb_desugar_stmts(stmts)?;
        if let Some(result) = combined {
            let span = match &result {
                Expr::Array { span, .. } | Expr::Call { span, .. } => *span,
                _ => syx_diagnostics::Span::DUMMY,
            };
            kept.push(Stmt::Return { span, value: Some(result) });
        }
        *body = kept;
        Ok(())
    }

    fn walk(decls: &mut [Decl]) -> SyxResult<()> {
        for decl in decls {
            match decl {
                Decl::Function { attrs, body, .. } => {
                    if attrs.iter().any(|a| {
                        matches!(
                            a.name.as_str(),
                            "resultBuilder" | "ResultBuilder" | "resultbuilder"
                        )
                    }) {
                        desugar_body(body)?;
                    }
                }
                Decl::Class { members, .. } => walk(members)?,
                Decl::Extension { methods, .. } => walk(methods)?,
                Decl::Protocol { methods, .. } => walk(methods)?,
                _ => {}
            }
        }
        Ok(())
    }

    walk(&mut module.decls)?;
    Ok(())
}

