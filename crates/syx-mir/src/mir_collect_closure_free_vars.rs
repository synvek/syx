//! SYX MIR collect_closure_free_vars.
//! Extracted in S2-I201 (zero semantics; crate-private).

use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{ClosureBody, Expr, Stmt};

/// Free names from the outer frame, plus those mutated via Assign/CompoundAssign (R-35).
pub(crate) struct ClosureFreeVars {
    pub free: Vec<SmolStr>,
    pub mutated: HashSet<SmolStr>,
}

pub(crate) fn collect_closure_free_vars(
    params: &[syx_ast::Param],
    body: &ClosureBody,
    outer_locals: &indexmap::IndexMap<SmolStr, u32>,
) -> Vec<SmolStr> {
    collect_closure_free_vars_detailed(params, body, outer_locals).free
}

pub(crate) fn collect_closure_free_vars_detailed(
    params: &[syx_ast::Param],
    body: &ClosureBody,
    outer_locals: &indexmap::IndexMap<SmolStr, u32>,
) -> ClosureFreeVars {
    let mut bound: HashSet<SmolStr> = params.iter().map(|p| p.name.clone()).collect();
    let mut seen = HashSet::new();
    let mut free = Vec::new();
    let mut mutated = HashSet::new();

    fn note_free(
        name: &SmolStr,
        bound: &HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
    ) {
        if bound.contains(name) {
            return;
        }
        if outer_locals.contains_key(name) && seen.insert(name.clone()) {
            free.push(name.clone());
        }
    }

    fn visit_expr(
        expr: &Expr,
        bound: &HashSet<SmolStr>,
        outer_locals: &indexmap::IndexMap<SmolStr, u32>,
        seen: &mut HashSet<SmolStr>,
        free: &mut Vec<SmolStr>,
        mutated: &mut HashSet<SmolStr>,
    ) {
        match expr {
            Expr::Ident { name, .. } => {
                note_free(name, bound, outer_locals, seen, free);
            }
            Expr::Binary { left, right, .. } => {
                visit_expr(left, bound, outer_locals, seen, free, mutated);
                visit_expr(right, bound, outer_locals, seen, free, mutated);
            }
            Expr::Unary { expr, .. } => {
                visit_expr(expr, bound, outer_locals, seen, free, mutated)
            }
            Expr::Call { callee, args, .. } => {
                visit_expr(callee, bound, outer_locals, seen, free, mutated);
                for arg in args {
                    visit_expr(&arg.value, bound, outer_locals, seen, free, mutated);
                }
            }
            Expr::Member { object, .. } => {
                visit_expr(object, bound, outer_locals, seen, free, mutated)
            }
            Expr::Index { object, index, .. } => {
                visit_expr(object, bound, outer_locals, seen, free, mutated);
                visit_expr(index, bound, outer_locals, seen, free, mutated);
            }
            // R-35: LHS is a free var (and mutated) when it names an outer local.
            Expr::Assign { name, value, .. } | Expr::CompoundAssign { name, value, .. } => {
                note_free(name, bound, outer_locals, seen, free);
                if !bound.contains(name) && outer_locals.contains_key(name) {
                    mutated.insert(name.clone());
                }
                visit_expr(value, bound, outer_locals, seen, free, mutated);
            }
            Expr::MemberAssign { object, value, .. }
            | Expr::IndexAssign { object, value, .. } => {
                visit_expr(object, bound, outer_locals, seen, free, mutated);
                visit_expr(value, bound, outer_locals, seen, free, mutated);
            }
            Expr::Group { expr, .. } => {
                visit_expr(expr, bound, outer_locals, seen, free, mutated)
            }
            Expr::Array { elements, .. } => {
                for el in elements {
                    visit_expr(el, bound, outer_locals, seen, free, mutated);
                }
            }
            Expr::Closure {
                params, body, ..
            } => {
                let mut nested_bound = bound.clone();
                for p in params {
                    nested_bound.insert(p.name.clone());
                }
                match body {
                    ClosureBody::Expr(e) => {
                        visit_expr(e, &nested_bound, outer_locals, seen, free, mutated)
                    }
                    ClosureBody::Block(stmts) => {
                        visit_stmts(stmts, &mut nested_bound, outer_locals, seen, free, mutated);
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
        mutated: &mut HashSet<SmolStr>,
    ) {
        for stmt in stmts {
            match stmt {
                Stmt::VarDecl { name, init, .. } => {
                    if let Some(expr) = init {
                        visit_expr(expr, bound, outer_locals, seen, free, mutated);
                    }
                    bound.insert(name.clone());
                }
                Stmt::Return { value: Some(v), .. } => {
                    visit_expr(v, bound, outer_locals, seen, free, mutated)
                }
                Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                    visit_expr(expr, bound, outer_locals, seen, free, mutated)
                }
                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    visit_expr(condition, bound, outer_locals, seen, free, mutated);
                    visit_stmts(then_branch, bound, outer_locals, seen, free, mutated);
                    if let Some(eb) = else_branch {
                        visit_stmts(eb, bound, outer_locals, seen, free, mutated);
                    }
                }
                Stmt::Block { stmts, .. } => {
                    visit_stmts(stmts, bound, outer_locals, seen, free, mutated)
                }
                _ => {}
            }
        }
    }

    match body {
        ClosureBody::Expr(e) => {
            visit_expr(e, &bound, outer_locals, &mut seen, &mut free, &mut mutated)
        }
        ClosureBody::Block(stmts) => {
            visit_stmts(
                stmts,
                &mut bound,
                outer_locals,
                &mut seen,
                &mut free,
                &mut mutated,
            )
        }
    }
    ClosureFreeVars { free, mutated }
}
