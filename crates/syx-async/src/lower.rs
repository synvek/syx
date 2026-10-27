use smol_str::SmolStr;
use syx_ast::{
    AccessLevel, BinOp, CallArg, Decl, Expr, Module, Param, ParamLabel, Stmt, TypeExpr,
};
use syx_diagnostics::{Span, SyxDiagnostic, SyxResult};

const SLEEP_WORKER: &str = "__syx_async_sleep_worker";

/// Async lowering: sleep rewrite, for-await desugar, async-let TaskGroup, CF segment switch machines.
pub fn lower_async_module(module: &mut Module) -> SyxResult<()> {
    let mut needs_worker = false;
    let mut extra_decls = Vec::new();
    for decl in &mut module.decls {
        if let Decl::Export { item, .. } = decl {
            if let Decl::Function {
                is_async: true,
                body,
                ..
            } = item.as_mut()
            {
                if rewrite_async_sleep_awaits(body) {
                    needs_worker = true;
                }
                desugar_for_await_in_stmts(body)?;
                // CF/segments use the outer Decl; Export wraps Function — lower via item.
                transform_cf_await_stmts(body, &mut extra_decls, "cf");
                if let Some(segment_decls) = lower_async_segments(item) {
                    extra_decls.extend(segment_decls);
                }
            }
        } else if let Decl::Function {
            is_async: true,
            body,
            ..
        } = decl
        {
            if rewrite_async_sleep_awaits(body) {
                needs_worker = true;
            }
            desugar_for_await_in_stmts(body)?;
            transform_cf_await_stmts(body, &mut extra_decls, "cf");
            if let Some(segment_decls) = lower_async_segments(decl) {
                extra_decls.extend(segment_decls);
            }
        }
    }
    for decl in &mut module.decls {
        let body_fn = match decl {
            Decl::Export { item, .. } => item.as_mut(),
            other => other,
        };
        if let Decl::Function { name, body, is_async: false, .. } = body_fn {
            if !name.as_str().starts_with("__async_") && !name.as_str().starts_with("__syx_") {
                lower_async_let_in_stmts(body, &mut 0);
            }
        }
    }
    module.decls.extend(extra_decls);
    if needs_worker && !module_has_function(module, SLEEP_WORKER) {
        module.decls.push(async_sleep_worker_decl());
    }
    Ok(())
}

/// Reject await inside loops before state-machine lowering (A003).
pub fn validate_no_unlowered_loop_await(module: &Module) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Function {
            name,
            body,
            is_async: true,
            ..
        } = decl
        {
            if find_await_in_loop(body).is_some() {
                return Err(SyxDiagnostic::error(format!(
                    "Function '{name}' uses await inside a loop without async state machine"
                ))
                .with_code("A003"));
            }
        }
    }
    Ok(())
}

fn find_await_in_loop(stmts: &[Stmt]) -> Option<Span> {
    for stmt in stmts {
        match stmt {
            Stmt::While { body, .. } | Stmt::ForIn { body, .. } => {
                if find_await_in_stmts(body).is_some() {
                    return Some(Span::DUMMY);
                }
            }
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                if find_await_in_loop(then_branch).is_some()
                    || else_branch
                        .as_ref()
                        .is_some_and(|b| find_await_in_loop(b).is_some())
                {
                    return Some(Span::DUMMY);
                }
            }
            Stmt::Block { stmts, .. } | Stmt::Seq { stmts, .. } => {
                if find_await_in_loop(stmts).is_some() {
                    return Some(Span::DUMMY);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_await_in_stmts(stmts: &[Stmt]) -> Option<Span> {
    stmts.iter().find_map(find_await_in_stmt)
}

fn find_await_in_stmt(stmt: &Stmt) -> Option<Span> {
    match stmt {
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
            find_await_in_expr(expr)
        }
        Stmt::VarDecl { init: Some(expr), .. } => find_await_in_expr(expr),
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => find_await_in_stmts(then_branch).or_else(|| {
            else_branch
                .as_ref()
                .and_then(|b| find_await_in_stmts(b))
        }),
        Stmt::Block { stmts, .. } | Stmt::Seq { stmts, .. } => find_await_in_stmts(stmts),
        Stmt::While { body, .. } | Stmt::ForIn { body, .. } => find_await_in_stmts(body),
        _ => None,
    }
}

fn find_await_in_expr(expr: &Expr) -> Option<Span> {
    match expr {
        Expr::Await { span, .. } => Some(*span),
        Expr::Binary { left, right, .. } => {
            find_await_in_expr(left).or_else(|| find_await_in_expr(right))
        }
        Expr::Unary { expr, .. } => find_await_in_expr(expr),
        Expr::Call { callee, args, .. } => find_await_in_expr(callee).or_else(|| {
            args.iter().find_map(|a| find_await_in_expr(&a.value))
        }),
        Expr::Member { object, .. } | Expr::Index { object, .. } => find_await_in_expr(object),
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => find_await_in_expr(value),
        Expr::Group { expr, .. } => find_await_in_expr(expr),
        Expr::Tuple { elements, .. } => elements.iter().find_map(|(_, e)| find_await_in_expr(e)),
        Expr::Array { elements, .. } => elements.iter().find_map(find_await_in_expr),
        Expr::TaskBlock { body, .. } => find_await_in_stmts(body),
        Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => find_await_in_expr(e),
            syx_ast::ClosureBody::Block(stmts) => find_await_in_stmts(stmts),
        },
        Expr::Borrow { expr, .. } => find_await_in_expr(expr),
        Expr::OptionalCast { expr, .. } => find_await_in_expr(expr),
        Expr::KeyPath { .. } => None,
        Expr::MacroInvoke { .. } => None,
        _ => None,
    }
}

fn transform_cf_await_stmts(stmts: &mut [Stmt], extra_decls: &mut Vec<Decl>, suffix: &str) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                maybe_lower_cf_body(then_branch, extra_decls, &format!("{suffix}_then"));
                transform_cf_await_stmts(then_branch, extra_decls, &format!("{suffix}_then"));
                if let Some(branch) = else_branch {
                    maybe_lower_cf_body(branch, extra_decls, &format!("{suffix}_else"));
                    transform_cf_await_stmts(branch, extra_decls, &format!("{suffix}_else"));
                }
            }
            Stmt::While { body, .. } => {
                maybe_lower_cf_body(body, extra_decls, &format!("{suffix}_while"));
                transform_cf_await_stmts(body, extra_decls, &format!("{suffix}_while"));
            }
            Stmt::Block { stmts: inner, .. } => {
                transform_cf_await_stmts(inner, extra_decls, suffix);
            }
            _ => {}
        }
    }
}

fn maybe_lower_cf_body(body: &mut Vec<Stmt>, extra_decls: &mut Vec<Decl>, suffix: &str) {
    if count_await_in_stmts(body) < 2 {
        return;
    }
    let segments = split_top_level_segments(body);
    if segments.len() < 2 {
        return;
    }
    let segment_count = segments.len() as u64;
    let (hoisted, segments) = hoist_segment_locals(&segments);
    let base = SmolStr::new(format!("__async_cf_{suffix}"));
    for (idx, segment) in segments.iter().enumerate() {
        extra_decls.push(Decl::Function {
            span: Span::DUMMY,
            access: AccessLevel::Internal,
            attrs: Vec::new(),
            name: SmolStr::new(format!("{base}_seg_{idx}")),
            type_params: Vec::new(),
            params: Vec::new(),
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: Vec::new(),
            body: {
                let mut seg_body = hoisted.clone();
                seg_body.extend(segment.clone());
                seg_body
            },
            is_async: true,
        is_override: false,
        is_final: false,
        });
    }
    *body = build_segment_dispatcher(Span::DUMMY, &hoisted, &segments, segment_count);
}

fn is_async_let_var(stmt: &Stmt) -> bool {
    matches!(
        stmt,
        Stmt::VarDecl { init: Some(init), .. } if is_task_detach_call(init)
    )
}

fn is_task_detach_call(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Call {
            callee,
            ..
        } if matches!(
            callee.as_ref(),
            Expr::Ident { name, .. } if name.as_str() == "__syx_async_let_detach"
        )
    )
}

fn lower_async_let_in_stmts(stmts: &mut Vec<Stmt>, counter: &mut u32) {
    let mut out = Vec::new();
    let mut i = 0;
    while i < stmts.len() {
        if matches!(&stmts[i], Stmt::Block { .. }) {
            if let Stmt::Block { stmts: inner, .. } = &mut stmts[i] {
                lower_async_let_in_stmts(inner, counter);
            }
            out.push(stmts[i].clone());
            i += 1;
            continue;
        }
        if is_async_let_var(&stmts[i]) {
            let start = i;
            while i < stmts.len() && is_async_let_var(&stmts[i]) {
                i += 1;
            }
            let run: Vec<Stmt> = stmts[start..i].to_vec();
            if run.len() >= 2 {
                out.extend(desugar_async_let_run(&run, counter));
            } else {
                out.extend(run);
            }
            continue;
        }
        out.push(stmts[i].clone());
        i += 1;
    }
    *stmts = out;
}

fn desugar_async_let_run(run: &[Stmt], counter: &mut u32) -> Vec<Stmt> {
    let span = run
        .first()
        .and_then(|s| match s {
            Stmt::VarDecl { span, .. } => Some(*span),
            _ => None,
        })
        .unwrap_or(Span::DUMMY);
    let group_name = SmolStr::new(format!("__syx_async_let_group_{counter}"));
    *counter += 1;
    let mut out = vec![Stmt::VarDecl {
        span,
        mutable: false,
        name: group_name.clone(),
        ty: None,
        init: Some(Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: "__syx_task_group_create".into(),
            }),
            args: Vec::new(),
        }),
    }];
    for stmt in run {
        if let Stmt::VarDecl { name, .. } = stmt {
            out.push(stmt.clone());
            out.push(Stmt::Expr {
                span,
                expr: Expr::Call {
                    span,
                    callee: Box::new(Expr::Ident {
                        span,
                        name: "__syx_task_group_add".into(),
                    }),
                    args: vec![
                        CallArg {
                            label: None,
                            value: Expr::Ident {
                                span,
                                name: group_name.clone(),
                            },
                        },
                        CallArg {
                            label: None,
                            value: Expr::Ident {
                                span,
                                name: name.clone(),
                            },
                        },
                    ],
                },
            });
        }
    }
    out.push(Stmt::Expr {
        span,
        expr: Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: "__syx_task_group_wait".into(),
            }),
            args: vec![CallArg {
                label: None,
                value: Expr::Ident {
                    span,
                    name: group_name,
                },
            }],
        },
    });
    out
}

fn lower_async_segments(decl: &mut Decl) -> Option<Vec<Decl>> {
    let Decl::Function {
        span,
        access: _access,
        attrs,
        name,
        type_params,
        params,
        return_type,
        throws_type,
        effect,
        where_clause,
        body,
        is_async,
        is_override: _,
        is_final: _,
    } = decl
    else {
        return None;
    };
    if count_await_in_stmts(body) < 2 {
        return None;
    }
    let segments = split_top_level_segments(body);
    if segments.len() < 2 {
        return None;
    }
    let segment_count = segments.len() as u64;
    let (hoisted, segments) = hoist_segment_locals(&segments);
    let mut segment_decls = Vec::new();
    for (idx, segment) in segments.iter().enumerate() {
        let seg_name = SmolStr::new(format!("__async_{name}_seg_{idx}"));
        let mut seg_body = hoisted.clone();
        seg_body.extend(segment.clone());
        segment_decls.push(Decl::Function {
            span: *span,
            access: AccessLevel::Internal,
            attrs: attrs.clone(),
            name: seg_name,
            type_params: type_params.clone(),
            params: params.clone(),
            return_type: return_type.clone(),
            throws_type: throws_type.clone(),
            effect: *effect,
            where_clause: where_clause.clone(),
            body: seg_body,
            is_async: *is_async,
        is_override: false,
        is_final: false,
        });
    }
    *body = build_segment_dispatcher(*span, &hoisted, &segments, segment_count);
    Some(segment_decls)
}

fn build_segment_dispatcher(
    span: Span,
    hoisted: &[Stmt],
    segments: &[Vec<Stmt>],
    segment_count: u64,
) -> Vec<Stmt> {
    let mut stmts = vec![Stmt::VarDecl {
        span,
        mutable: true,
        name: "__syx_async_state".into(),
        ty: Some(TypeExpr::Named {
            name: "Int".into(),
            generics: Vec::new(),
        }),
        init: Some(int_literal(span, 0)),
    }];
    stmts.extend(hoisted.iter().cloned());
    for (idx, segment) in segments.iter().enumerate() {
        let mut body = segment.clone();
        if idx + 1 < segments.len() {
            body.push(make_state_assign((idx + 1) as u64));
        } else {
            body.push(make_state_assign(segment_count));
        }
        stmts.push(Stmt::If {
            span,
            condition: Expr::Binary {
                span,
                op: BinOp::Eq,
                left: Box::new(Expr::Ident {
                    span,
                    name: "__syx_async_state".into(),
                }),
                right: Box::new(int_literal(span, idx as i64)),
            },
            then_branch: body,
            else_branch: None,
        });
    }
    stmts
}

/// Hoist `let` bindings to dispatcher scope so locals survive across await segments.
fn hoist_segment_locals(segments: &[Vec<Stmt>]) -> (Vec<Stmt>, Vec<Vec<Stmt>>) {
    let mut hoisted: Vec<Stmt> = Vec::new();
    let mut transformed = Vec::with_capacity(segments.len());
    for segment in segments {
        let mut new_segment = Vec::new();
        for stmt in segment.iter().cloned() {
            match stmt {
                Stmt::VarDecl {
                    span,
                    mutable: _,
                    name,
                    ty,
                    init,
                } => {
                    if !hoisted.iter().any(|h| {
                        matches!(h, Stmt::VarDecl { name: n, .. } if n == &name)
                    }) {
                        hoisted.push(Stmt::VarDecl {
                            span,
                            mutable: true,
                            name: name.clone(),
                            ty: ty.clone(),
                            init: None,
                        });
                    }
                    if let Some(init_expr) = init {
                        new_segment.push(Stmt::Expr {
                            span,
                            expr: Expr::Assign {
                                span,
                                name: name.clone(),
                                value: Box::new(init_expr),
                            },
                        });
                    }
                }
                other => new_segment.push(other),
            }
        }
        transformed.push(new_segment);
    }
    (hoisted, transformed)
}

fn split_top_level_segments(stmts: &[Stmt]) -> Vec<Vec<Stmt>> {
    let mut segments: Vec<Vec<Stmt>> = vec![Vec::new()];
    for stmt in stmts {
        segments.last_mut().unwrap().push(stmt.clone());
        if count_await_in_stmt(stmt) > 0 {
            segments.push(Vec::new());
        }
    }
    if segments.last().is_some_and(|s| s.is_empty()) {
        segments.pop();
    }
    segments
}

fn int_literal(span: Span, value: i64) -> Expr {
    Expr::Literal {
        span,
        value: syx_ast::Literal::Integer {
            value: value as i128,
            suffix: None,
        },
    }
}

fn desugar_for_await_in_stmts(stmts: &mut Vec<Stmt>) -> SyxResult<()> {
    for stmt in stmts.iter_mut() {
        match stmt {
            Stmt::ForIn {
                span,
                name,
                iterable,
                body,
                is_async_iter: true,
            } => {
                let expanded = desugar_for_await_stmt(*span, name, iterable, body);
                *stmt = Stmt::Block {
                    span: *span,
                    stmts: expanded,
                };
            }
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                desugar_for_await_in_stmts(then_branch)?;
                if let Some(branch) = else_branch {
                    desugar_for_await_in_stmts(branch)?;
                }
            }
            Stmt::Block { stmts: inner, .. } => desugar_for_await_in_stmts(inner)?,
            Stmt::While { body, .. } => desugar_for_await_in_stmts(body)?,
            _ => {}
        }
    }
    Ok(())
}

fn desugar_for_await_stmt(
    span: Span,
    name: &SmolStr,
    iterable: &Expr,
    body: &[Stmt],
) -> Vec<Stmt> {
    let iter_name = SmolStr::new(format!("__syx_async_iter_{name}"));
    let item_name = SmolStr::new(format!("__syx_async_item_{name}"));
    let done_flag = SmolStr::new(format!("__syx_async_done_{name}"));
    let mut body_stmts = vec![Stmt::VarDecl {
        span,
        mutable: false,
        name: name.clone(),
        ty: None,
        init: Some(Expr::Member {
            span,
            object: Box::new(Expr::Ident {
                span,
                name: item_name.clone(),
            }),
            name: "__async_iter_value".into(),
        }),
    }];
    body_stmts.extend(body.iter().cloned());
    let loop_body = vec![
        Stmt::VarDecl {
            span,
            mutable: false,
            name: item_name.clone(),
            ty: None,
            init: Some(Expr::Await {
                span,
                expr: Box::new(Expr::Call {
                    span,
                    callee: Box::new(Expr::Ident {
                        span,
                        name: "__syx_async_iter_next".into(),
                    }),
                    args: vec![CallArg {
                        label: None,
                        value: Expr::Ident {
                            span,
                            name: iter_name.clone(),
                        },
                    }],
                }),
            }),
        },
        Stmt::If {
            span,
            condition: Expr::Member {
                span,
                object: Box::new(Expr::Ident {
                    span,
                    name: item_name.clone(),
                }),
                name: "__async_iter_done".into(),
            },
            then_branch: vec![Stmt::Expr {
                span,
                expr: Expr::Assign {
                    span,
                    name: done_flag.clone(),
                    value: Box::new(Expr::Literal {
                        span,
                        value: syx_ast::Literal::Bool(true),
                    }),
                },
            }],
            else_branch: Some(body_stmts),
        },
    ];
    vec![
        Stmt::VarDecl {
            span,
            mutable: false,
            name: iter_name.clone(),
            ty: None,
            init: Some(Expr::Call {
                span,
                callee: Box::new(Expr::Ident {
                    span,
                    name: "__syx_async_iter_from".into(),
                }),
                args: vec![CallArg {
                    label: None,
                    value: iterable.clone(),
                }],
            }),
        },
        Stmt::VarDecl {
            span,
            mutable: true,
            name: done_flag.clone(),
            ty: None,
            init: Some(Expr::Literal {
                span,
                value: syx_ast::Literal::Bool(false),
            }),
        },
        Stmt::While {
            span,
            condition: Expr::Unary {
                span,
                op: syx_ast::UnaryOp::Not,
                expr: Box::new(Expr::Ident {
                    span,
                    name: done_flag,
                }),
            },
            body: loop_body,
        },
    ]
}

fn module_has_function(module: &Module, name: &str) -> bool {
    module.decls.iter().any(|decl| {
        matches!(
            decl,
            Decl::Function { name: fn_name, .. } if fn_name.as_str() == name
        )
    })
}

fn async_sleep_worker_decl() -> Decl {
    Decl::Function {
        span: Span::DUMMY,
        access: AccessLevel::Internal,
        attrs: Vec::new(),
        name: SLEEP_WORKER.into(),
        type_params: Vec::new(),
        params: vec![Param {
            name: "ms".into(),
            external_label: ParamLabel::Same,
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: Vec::new(),
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }],
        return_type: None,
        throws_type: None,
        effect: None,
        where_clause: Vec::new(),
        body: vec![Stmt::Expr {
            span: Span::DUMMY,
            expr: Expr::Call {
                span: Span::DUMMY,
                callee: Box::new(Expr::Ident {
                    span: Span::DUMMY,
                    name: "__syx_task_sleep".into(),
                }),
                args: vec![CallArg {
                    label: None,
                    value: Expr::Ident {
                        span: Span::DUMMY,
                        name: "ms".into(),
                    },
                }],
            },
        }],
        is_async: false,
    is_override: false,
        is_final: false,
    }
}

fn make_state_assign(value: u64) -> Stmt {
    Stmt::Expr {
        span: Span::DUMMY,
        expr: Expr::Assign {
            span: Span::DUMMY,
            name: "__syx_async_state".into(),
            value: Box::new(Expr::Literal {
                span: Span::DUMMY,
                value: syx_ast::Literal::Integer {
                    value: value as i128,
                    suffix: None,
                },
            }),
        },
    }
}

fn count_await_in_stmts(stmts: &[Stmt]) -> usize {
    stmts.iter().map(count_await_in_stmt).sum()
}

fn count_await_in_stmt(stmt: &Stmt) -> usize {
    match stmt {
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
            count_await_in_expr(expr)
        }
        Stmt::VarDecl { init: Some(expr), .. } => count_await_in_expr(expr),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let mut n = count_await_in_expr(condition);
            n += count_await_in_stmts(then_branch);
            if let Some(branch) = else_branch {
                n += count_await_in_stmts(branch);
            }
            n
        }
        Stmt::Block { stmts, .. } | Stmt::Seq { stmts, .. } => count_await_in_stmts(stmts),
        Stmt::While { condition, body, .. } => {
            count_await_in_expr(condition) + count_await_in_stmts(body)
        }
        Stmt::ForIn { iterable, body, .. } => {
            count_await_in_expr(iterable) + count_await_in_stmts(body)
        }
        _ => 0,
    }
}

fn count_await_in_expr(expr: &Expr) -> usize {
    match expr {
        Expr::Await { .. } => 1,
        Expr::Binary { left, right, .. } => {
            count_await_in_expr(left) + count_await_in_expr(right)
        }
        Expr::Unary { expr, .. } => count_await_in_expr(expr),
        Expr::Call { callee, args, .. } => {
            let mut c = count_await_in_expr(callee);
            for arg in args {
                c += count_await_in_expr(&arg.value);
            }
            c
        }
        Expr::Member { object, .. } | Expr::Index { object, .. } => count_await_in_expr(object),
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => count_await_in_expr(value),
        Expr::MemberAssign { object, value, .. } | Expr::IndexAssign { object, value, .. } => {
            count_await_in_expr(object) + count_await_in_expr(value)
        }
        Expr::Group { expr, .. } => count_await_in_expr(expr),
        Expr::Tuple { elements, .. } => elements.iter().map(|(_, e)| count_await_in_expr(e)).sum(),
        Expr::Array { elements, .. } => elements.iter().map(count_await_in_expr).sum(),
        Expr::TaskBlock { body, .. } => count_await_in_stmts(body),
        Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => count_await_in_expr(e),
            syx_ast::ClosureBody::Block(stmts) => count_await_in_stmts(stmts),
        },
        Expr::Borrow { expr, .. } => count_await_in_expr(expr),
        Expr::OptionalCast { expr, .. } => count_await_in_expr(expr),
        Expr::KeyPath { .. } => 0,
        Expr::MacroInvoke { .. } => 0,
        _ => 0,
    }
}

fn rewrite_async_sleep_awaits(stmts: &mut [Stmt]) -> bool {
    let mut changed = false;
    for stmt in stmts {
        if rewrite_async_sleep_in_stmt(stmt) {
            changed = true;
        }
    }
    changed
}

fn rewrite_async_sleep_in_stmt(stmt: &mut Stmt) -> bool {
    match stmt {
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
            rewrite_async_sleep_in_expr(expr)
        }
        Stmt::VarDecl { init: Some(expr), .. } => rewrite_async_sleep_in_expr(expr),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let mut changed = rewrite_async_sleep_in_expr(condition);
            changed |= rewrite_async_sleep_awaits(then_branch);
            if let Some(branch) = else_branch {
                changed |= rewrite_async_sleep_awaits(branch);
            }
            changed
        }
        Stmt::Block { stmts, .. } | Stmt::Seq { stmts, .. } => rewrite_async_sleep_awaits(stmts),
        Stmt::While { condition, body, .. } => {
            let mut changed = rewrite_async_sleep_in_expr(condition);
            changed |= rewrite_async_sleep_awaits(body);
            changed
        }
        Stmt::ForIn { iterable, body, .. } => {
            let mut changed = rewrite_async_sleep_in_expr(iterable);
            changed |= rewrite_async_sleep_awaits(body);
            changed
        }
        _ => false,
    }
}

fn rewrite_async_sleep_in_expr(expr: &mut Expr) -> bool {
    let mut changed = false;
    match expr {
        Expr::Await { expr: inner, .. } => {
            if is_task_sleep_call(inner) {
                let span = inner.span();
                let ms = sleep_call_arg(inner).unwrap_or(Expr::Literal {
                    span,
                    value: syx_ast::Literal::Integer {
                        value: 0,
                        suffix: None,
                    },
                });
                *expr = make_cooperative_sleep_await(ms, span);
                return true;
            }
            changed |= rewrite_async_sleep_in_expr(inner);
        }
        Expr::Binary { left, right, .. } => {
            changed |= rewrite_async_sleep_in_expr(left);
            changed |= rewrite_async_sleep_in_expr(right);
        }
        Expr::Unary { expr, .. } => changed |= rewrite_async_sleep_in_expr(expr),
        Expr::Call { callee, args, .. } => {
            changed |= rewrite_async_sleep_in_expr(callee);
            for arg in args {
                changed |= rewrite_async_sleep_in_expr(&mut arg.value);
            }
        }
        Expr::Member { object, .. } => changed |= rewrite_async_sleep_in_expr(object),
        Expr::Index { object, index, .. } => {
            changed |= rewrite_async_sleep_in_expr(object);
            changed |= rewrite_async_sleep_in_expr(index);
        }
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
            changed |= rewrite_async_sleep_in_expr(value);
        }
        Expr::MemberAssign { object, value, .. } | Expr::IndexAssign { object, value, .. } => {
            changed |= rewrite_async_sleep_in_expr(object);
            changed |= rewrite_async_sleep_in_expr(value);
        }
        Expr::Group { expr, .. } => changed |= rewrite_async_sleep_in_expr(expr),
        Expr::Tuple { elements, .. } => {
            for (_, el) in elements {
                changed |= rewrite_async_sleep_in_expr(el);
            }
        }
        Expr::Array { elements, .. } => {
            for el in elements {
                changed |= rewrite_async_sleep_in_expr(el);
            }
        }
        Expr::TaskBlock { body, .. } => changed |= rewrite_async_sleep_awaits(body),
        Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => changed |= rewrite_async_sleep_in_expr(e),
            syx_ast::ClosureBody::Block(stmts) => changed |= rewrite_async_sleep_awaits(stmts),
        },
        _ => {}
    }
    changed
}

fn is_task_sleep_call(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Call {
            callee,
            ..
        } if matches!(
            callee.as_ref(),
            Expr::Ident { name, .. } if name.as_str() == "__syx_task_sleep"
        )
    )
}

fn sleep_call_arg(expr: &Expr) -> Option<Expr> {
    if let Expr::Call { args, .. } = expr {
        return args.first().map(|a| a.value.clone());
    }
    None
}

fn make_cooperative_sleep_await(ms: Expr, span: Span) -> Expr {
    let detach = Expr::Call {
        span,
        callee: Box::new(Expr::Ident {
            span,
            name: "__syx_task_detach".into(),
        }),
        args: vec![
            CallArg {
                label: None,
                value: Expr::Literal {
                    span,
                    value: syx_ast::Literal::String(SLEEP_WORKER.into()),
                },
            },
            CallArg {
                label: None,
                value: Expr::Array {
                    span,
                    elements: vec![ms],
                },
            },
        ],
    };
    let await_task = Expr::Call {
        span,
        callee: Box::new(Expr::Ident {
            span,
            name: "__syx_await_task".into(),
        }),
        args: vec![CallArg {
            label: None,
            value: detach,
        }],
    };
    Expr::Await {
        span,
        expr: Box::new(await_task),
    }
}

trait ExprSpan {
    fn span(&self) -> Span;
}

impl ExprSpan for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Literal { span, .. }
            | Expr::Ident { span, .. }
            | Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Call { span, .. }
            | Expr::Member { span, .. }
            | Expr::Index { span, .. }
            | Expr::Assign { span, .. }
            | Expr::CompoundAssign { span, .. }
            | Expr::MemberAssign { span, .. }
            | Expr::IndexAssign { span, .. }
            | Expr::Group { span, .. }
            | Expr::Array { span, .. }
            | Expr::Await { span, .. }
            | Expr::OptionalChain { span, .. }
            | Expr::ForceUnwrap { span, .. }
            | Expr::HalfOpenRange { span, .. }
            | Expr::ClosedRange { span, .. }
            | Expr::VariantPattern { span, .. }
            | Expr::TuplePattern { span, .. }
            | Expr::Closure { span, .. }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;

    #[test]
    fn lowers_multi_await_into_switch_segments() {
        let source = SourceFile::new(
            "t.sy",
            r#"async func f() {
    print("a")
    await g()
    print("b")
    await h()
    print("c")
}"#,
        );
        let mut module = Parser::parse(source).unwrap();
        lower_async_module(&mut module).unwrap();
        let f = module
            .decls
            .iter()
            .find_map(|d| {
                if let Decl::Function { name, body, .. } = d {
                    if name.as_str() == "f" {
                        return Some(body);
                    }
                }
                None
            })
            .unwrap();
        assert!(matches!(f.first(), Some(Stmt::VarDecl { name, .. }) if name.as_str() == "__syx_async_state"));
        assert!(f.iter().any(|s| matches!(s, Stmt::If { .. })));
        assert!(module.decls.iter().any(|d| {
            matches!(d, Decl::Function { name, .. } if name.as_str() == "__async_f_seg_0")
        }));
    }

    #[test]
    fn single_await_skips_segment_lowering() {
        let source = SourceFile::new(
            "t.sy",
            "async func f() { await g() }",
        );
        let mut module = Parser::parse(source).unwrap();
        lower_async_module(&mut module).unwrap();
        assert!(!module.decls.iter().any(|d| {
            matches!(d, Decl::Function { name, .. } if name.as_str().contains("_seg_"))
        }));
    }
}
