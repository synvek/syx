//! Phase 50 ADR-070 M2: MacroParamId hygiene v2.
//! Phase 132 M22: declaration-site body rename (`macro-token-v3:decl-hygiene-mvp`).

use smol_str::SmolStr;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use syx_ast::{Decl, Expr, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

static EXPANSION_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroParamId(pub u32);

pub fn fresh_expansion_id() -> u32 {
    EXPANSION_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn hygienic_param_name(macro_name: &str, param: &str, expansion_id: u32) -> SmolStr {
    SmolStr::new(format!("__syx_mp_{macro_name}_{param}_{expansion_id}"))
}

/// Phase 58: `\name` intentional outer capture (ADR-070).
pub fn is_hygiene_escape_ident(name: &str) -> bool {
    name.starts_with('\\') && name.len() > 1
}

pub fn strip_hygiene_escape(name: &str) -> SmolStr {
    if is_hygiene_escape_ident(name) {
        SmolStr::new(&name[1..])
    } else {
        SmolStr::new(name)
    }
}

/// Bind macro parameters to fresh MacroParamId names at expansion site.
pub fn bind_macro_params(
    macro_name: &str,
    params: &[SmolStr],
    expansion_id: u32,
) -> Vec<(SmolStr, SmolStr)> {
    params
        .iter()
        .map(|p| (p.clone(), hygienic_param_name(macro_name, p.as_str(), expansion_id)))
        .collect()
}

fn collect_idents_expr(expr: &Expr, out: &mut HashSet<SmolStr>) {
    match expr {
        Expr::Ident { name, .. } => {
            if is_hygiene_escape_ident(name.as_str()) {
                return;
            }
            out.insert(name.clone());
        }
        Expr::Binary { left, right, .. } => {
            collect_idents_expr(left, out);
            collect_idents_expr(right, out);
        }
        Expr::Unary { expr, .. } => collect_idents_expr(expr, out),
        Expr::Call { callee, args, .. } => {
            collect_idents_expr(callee, out);
            for a in args {
                collect_idents_expr(&a.value, out);
            }
        }
        Expr::Member { object, .. } => collect_idents_expr(object, out),
        Expr::Group { expr, .. } => collect_idents_expr(expr, out),
        Expr::Index { object, index, .. } => {
            collect_idents_expr(object, out);
            collect_idents_expr(index, out);
        }
        Expr::MacroInvoke { args, .. } => {
            for a in args {
                collect_idents_expr(a, out);
            }
        }
        _ => {}
    }
}

fn collect_idents_stmts(stmts: &[Stmt], out: &mut HashSet<SmolStr>) {
    for stmt in stmts {
        match stmt {
            Stmt::Print { expr, .. } | Stmt::Expr { expr, .. } => collect_idents_expr(expr, out),
            Stmt::Return { value: Some(expr), .. } => collect_idents_expr(expr, out),
            Stmt::VarDecl { init: Some(expr), .. } => collect_idents_expr(expr, out),
            Stmt::Block { stmts, .. } => collect_idents_stmts(stmts, out),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                collect_idents_expr(condition, out);
                collect_idents_stmts(then_branch, out);
                if let Some(eb) = else_branch {
                    collect_idents_stmts(eb, out);
                }
            }
            Stmt::While { condition, body, .. } => {
                collect_idents_expr(condition, out);
                collect_idents_stmts(body, out);
            }
            _ => {}
        }
    }
}

/// Reject macro bodies that reference outer bindings (non-parameter idents at module scope).
pub fn check_macro_hygiene_capture(
    macro_name: &str,
    params: &[SmolStr],
    body: &[Stmt],
    module_bindings: &HashSet<SmolStr>,
) -> SyxResult<()> {
    let param_set: HashSet<_> = params.iter().cloned().collect();
    let mut used = HashSet::new();
    collect_idents_stmts(body, &mut used);
    reject_captured_idents(macro_name, &param_set, &used, module_bindings)
}

/// Phase 132: same capture check over declaration-site `#macro` bodies.
pub fn check_macro_decl_hygiene_capture(
    macro_name: &str,
    params: &[SmolStr],
    decl_body: &[Decl],
    module_bindings: &HashSet<SmolStr>,
) -> SyxResult<()> {
    let param_set: HashSet<_> = params.iter().cloned().collect();
    let mut used = HashSet::new();
    collect_idents_decls(decl_body, &mut used);
    reject_captured_idents(macro_name, &param_set, &used, module_bindings)
}

fn reject_captured_idents(
    macro_name: &str,
    param_set: &HashSet<SmolStr>,
    used: &HashSet<SmolStr>,
    module_bindings: &HashSet<SmolStr>,
) -> SyxResult<()> {
    for name in used {
        if param_set.contains(name) {
            continue;
        }
        if module_bindings.contains(name) {
            return Err(SyxDiagnostic::error(format!(
                "macro '{macro_name}' captures outer binding '{name}' (hygiene v2); use '\\{name}' to opt in"
            ))
            .with_code("E003"));
        }
    }
    Ok(())
}

fn collect_idents_decls(decls: &[Decl], out: &mut HashSet<SmolStr>) {
    for decl in decls {
        match decl {
            Decl::Function { body, .. }
            | Decl::Init { body, .. }
            | Decl::Deinit { body, .. }
            | Decl::OperatorFunc { body, .. } => collect_idents_stmts(body, out),
            Decl::VarGlobal {
                init,
                will_set,
                did_set,
                computed_getter,
                computed_setter,
                ..
            } => {
                if let Some(e) = init {
                    collect_idents_expr(e, out);
                }
                if let Some(ss) = will_set {
                    collect_idents_stmts(ss, out);
                }
                if let Some(ss) = did_set {
                    collect_idents_stmts(ss, out);
                }
                if let Some(ss) = computed_getter {
                    collect_idents_stmts(ss, out);
                }
                if let Some(ss) = computed_setter {
                    collect_idents_stmts(ss, out);
                }
            }
            Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                collect_idents_decls(members, out);
            }
            Decl::Extension { methods, .. } => collect_idents_decls(methods, out),
            Decl::TopLevelStmt { stmt, .. } => collect_idents_stmts(std::slice::from_ref(stmt), out),
            _ => {}
        }
    }
}

/// Reject macro parameters that shadow module-level bindings at definition site.
pub fn check_macro_param_shadow(
    macro_name: &str,
    params: &[SmolStr],
    module_bindings: &HashSet<SmolStr>,
) -> SyxResult<()> {
    for p in params {
        if module_bindings.contains(p) {
            return Err(SyxDiagnostic::error(format!(
                "macro '{macro_name}' parameter '{p}' shadows outer binding (hygiene v2)"
            ))
            .with_code("E003"));
        }
    }
    Ok(())
}

pub fn rename_macro_body(stmts: &mut [Stmt], renames: &[(SmolStr, SmolStr)]) {
    for stmt in stmts {
        rename_stmt(stmt, renames);
    }
}

/// Phase 132 M22: rename hygienic params inside declaration-site `#macro` bodies.
pub fn rename_macro_decls(decls: &mut [Decl], renames: &[(SmolStr, SmolStr)]) {
    for decl in decls {
        rename_decl(decl, renames);
    }
}

fn rename_decl(decl: &mut Decl, renames: &[(SmolStr, SmolStr)]) {
    match decl {
        Decl::Function {
            name,
            params,
            body,
            ..
        } => {
            rename_smolstr(name, renames);
            for p in params.iter_mut() {
                if let Some(d) = p.default.as_mut() {
                    rename_expr(d, renames);
                }
            }
            rename_macro_body(body, renames);
        }
        Decl::OperatorFunc { params, body, .. } => {
            for p in params.iter_mut() {
                if let Some(d) = p.default.as_mut() {
                    rename_expr(d, renames);
                }
            }
            rename_macro_body(body, renames);
        }
        Decl::Init { params, body, .. } => {
            for p in params.iter_mut() {
                if let Some(d) = p.default.as_mut() {
                    rename_expr(d, renames);
                }
            }
            rename_macro_body(body, renames);
        }
        Decl::Deinit { body, .. } => rename_macro_body(body, renames),
        Decl::VarGlobal {
            init,
            will_set,
            did_set,
            computed_getter,
            ..
        } => {
            if let Some(e) = init {
                rename_expr(e, renames);
            }
            if let Some(ss) = will_set {
                rename_macro_body(ss, renames);
            }
            if let Some(ss) = did_set {
                rename_macro_body(ss, renames);
            }
            if let Some(ss) = computed_getter {
                rename_macro_body(ss, renames);
            }
        }
        Decl::Class { members, .. } | Decl::Actor { members, .. } => {
            rename_macro_decls(members, renames);
        }
        Decl::Extension { methods, .. } => rename_macro_decls(methods, renames),
        Decl::Enum { cases, .. } => {
            for (case_name, payload, _) in cases.iter_mut() {
                rename_smolstr(case_name, renames);
                if let Some(e) = payload {
                    rename_expr(e, renames);
                }
            }
        }
        Decl::TopLevelStmt { stmt, .. } => rename_stmt(stmt, renames),
        _ => {}
    }
}

fn rename_smolstr(name: &mut SmolStr, renames: &[(SmolStr, SmolStr)]) {
    for (from, to) in renames {
        if name.as_str() == from.as_str() {
            *name = to.clone();
            return;
        }
    }
}

fn rename_stmt(stmt: &mut Stmt, renames: &[(SmolStr, SmolStr)]) {
    match stmt {
        Stmt::Print { expr, .. } | Stmt::Expr { expr, .. } => rename_expr(expr, renames),
        Stmt::Return { value: Some(expr), .. } => rename_expr(expr, renames),
        Stmt::VarDecl { init: Some(expr), .. } => rename_expr(expr, renames),
        Stmt::Block { stmts, .. } => rename_macro_body(stmts, renames),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            rename_expr(condition, renames);
            rename_macro_body(then_branch, renames);
            if let Some(eb) = else_branch {
                rename_macro_body(eb, renames);
            }
        }
        Stmt::While { condition, body, .. } => {
            rename_expr(condition, renames);
            rename_macro_body(body, renames);
        }
        _ => {}
    }
}

fn rename_expr(expr: &mut Expr, renames: &[(SmolStr, SmolStr)]) {
    match expr {
        Expr::Ident { name, .. } => {
            if is_hygiene_escape_ident(name.as_str()) {
                *name = strip_hygiene_escape(name.as_str());
                return;
            }
            for (from, to) in renames {
                if name == from {
                    *name = to.clone();
                    break;
                }
            }
        }
        Expr::Binary { left, right, .. } => {
            rename_expr(left, renames);
            rename_expr(right, renames);
        }
        Expr::Unary { expr, .. } => rename_expr(expr, renames),
        Expr::Call { callee, args, .. } => {
            rename_expr(callee, renames);
            for a in args {
                rename_expr(&mut a.value, renames);
            }
        }
        Expr::Member { object, .. } => rename_expr(object, renames),
        Expr::Group { expr, .. } => rename_expr(expr, renames),
        Expr::Index { object, index, .. } => {
            rename_expr(object, renames);
            rename_expr(index, renames);
        }
        Expr::Array { elements, .. } => {
            for e in elements {
                rename_expr(e, renames);
            }
        }
        Expr::MacroInvoke { args, .. } => {
            for a in args {
                rename_expr(a, renames);
            }
        }
        _ => {}
    }
}
