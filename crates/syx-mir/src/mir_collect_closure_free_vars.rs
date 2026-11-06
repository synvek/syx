//! SYX MIR collect_closure_free_vars.
//! Extracted in S2-I201 (zero semantics; crate-private).

use smol_str::SmolStr;
use syx_ast::{ClosureBody, Expr, Stmt};

pub(crate) fn collect_closure_free_vars(
    params: &[syx_ast::Param],
    body: &ClosureBody,
    outer_locals: &indexmap::IndexMap<SmolStr, u32>,
) -> Vec<SmolStr> {
    use std::collections::HashSet;
    let mut bound: HashSet<SmolStr> = params.iter().map(|p| p.name.clone()).collect();
    let mut seen = HashSet::new();
    let mut free = Vec::new();

    fn visit_expr(
        expr: &Expr,
        bound: &HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
    ) {
        match expr {
            Expr::Ident { name, .. } => {
                if bound.contains(name) {
                    return;
                }
                if outer_locals.contains_key(name) && seen.insert(name.clone()) {
                    free.push(name.clone());
                }
            }
            Expr::Binary { left, right, .. } => {
                visit_expr(left, bound, outer_locals, seen, free);
                visit_expr(right, bound, outer_locals, seen, free);
            }
            Expr::Unary { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Call { callee, args, .. } => {
                visit_expr(callee, bound, outer_locals, seen, free);
                for arg in args {
                    visit_expr(&arg.value, bound, outer_locals, seen, free);
                }
            }
            Expr::Member { object, .. } => visit_expr(object, bound, outer_locals, seen, free),
            Expr::Index { object, index, .. } => {
                visit_expr(object, bound, outer_locals, seen, free);
                visit_expr(index, bound, outer_locals, seen, free);
            }
            Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
                visit_expr(value, bound, outer_locals, seen, free);
            }
            Expr::Group { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
            Expr::Array { elements, .. } => {
                for el in elements {
                    visit_expr(el, bound, outer_locals, seen, free);
                }
            }
            Expr::Closure { params, body, .. } => {
                let mut nested_bound = bound.clone();
                for p in params {
                    nested_bound.insert(p.name.clone());
                }
                match body {
                    ClosureBody::Expr(e) => visit_expr(e, &nested_bound, outer_locals, seen, free),
                    ClosureBody::Block(stmts) => {
                        visit_stmts(stmts, &mut nested_bound, outer_locals, seen, free);
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_stmts(
        stmts: &[Stmt],
        bound: &mut HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
    ) {
        for stmt in stmts {
            match stmt {
                Stmt::VarDecl { name, init, .. } => {
                    if let Some(expr) = init {
                        visit_expr(expr, bound, outer_locals, seen, free);
                    }
                    bound.insert(name.clone());
                }
                Stmt::Return { value: Some(v), .. } => visit_expr(v, bound, outer_locals, seen, free),
                Stmt::Expr { expr, .. } => visit_expr(expr, bound, outer_locals, seen, free),
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    visit_expr(condition, bound, outer_locals, seen, free);
                    visit_stmts(then_branch, bound, outer_locals, seen, free);
                    if let Some(eb) = else_branch {
                        visit_stmts(eb, bound, outer_locals, seen, free);
                    }
                }
                Stmt::Block { stmts, .. } => visit_stmts(stmts, bound, outer_locals, seen, free),
                _ => {}
            }
        }
    }

    match body {
        ClosureBody::Expr(e) => visit_expr(e, &bound, outer_locals, &mut seen, &mut free),
        ClosureBody::Block(stmts) => visit_stmts(stmts, &mut bound, outer_locals, &mut seen, &mut free),
    }
    free
}

