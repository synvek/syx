//! Phase 51 ADR-070 M4: `#externalMacro` in-process expansion.

use indexmap::IndexMap;
use smol_str::SmolStr;
use std::sync::{LazyLock, Mutex};
use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::SyxResult;

use crate::external_protocol::{InProcessMacroExpander, MacroExpandRequest};

#[derive(Debug, Clone)]
pub struct ExternalMacroSpec {
    pub module: String,
    pub type_name: String,
}

static PACKAGE_MACROS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn register_package_macros(names: &[String]) {
    if let Ok(mut g) = PACKAGE_MACROS.lock() {
        *g = names.to_vec();
    }
}

pub fn package_macro_registered(name: &str) -> bool {
    PACKAGE_MACROS
        .lock()
        .ok()
        .map(|g| g.iter().any(|m| m == name))
        .unwrap_or(false)
}

static EXTERNAL_MACROS: Mutex<Option<IndexMap<SmolStr, ExternalMacroSpec>>> = Mutex::new(None);

pub fn collect_external_macros(module: &Module) {
    let mut map = IndexMap::new();
    for decl in &module.decls {
        if let Decl::MacroDecl {
            name,
            implementation: syx_ast::MacroImplementation::External { module, type_name },
            ..
        } = decl
        {
            map.insert(
                name.clone(),
                ExternalMacroSpec {
                    module: module.clone(),
                    type_name: type_name.clone(),
                },
            );
        }
    }
    if let Ok(mut g) = EXTERNAL_MACROS.lock() {
        *g = Some(map);
    }
}

pub fn expand_external_macros(module: &mut Module) -> SyxResult<()> {
    if crate::external_sandbox::wasm_external_macro_disabled() {
        if count_freestanding_external_invokes(module) > 0 {
            return Err(syx_diagnostics::SyxDiagnostic::error(
                "external macros disabled on WASM target",
            )
            .with_code("E004"));
        }
        return Ok(());
    }
    let expander = crate::external_sandbox::select_expander();
    expand_external_in_decls(&mut module.decls, &*expander)
}

fn count_freestanding_external_invokes(module: &Module) -> u32 {
    module.decls.iter().map(count_external_in_decl).sum()
}

fn count_external_in_decl(decl: &Decl) -> u32 {
    match decl {
        Decl::Function { body, .. }
        | Decl::Init { body, .. }
        | Decl::Deinit { body, .. } => count_external_in_stmts(body),
        Decl::Class { members, .. } | Decl::Actor { members, .. } => {
            members.iter().map(count_external_in_decl).sum()
        }
        Decl::Extension { methods, .. } => methods.iter().map(count_external_in_decl).sum(),
        Decl::MacroSiteInvoke { name, .. } => u32::from(lookup_external_macro(name).is_some()),
        _ => 0,
    }
}

fn count_external_in_stmts(stmts: &[Stmt]) -> u32 {
    let mut n = 0;
    for stmt in stmts {
        match stmt {
            Stmt::MacroInvoke { name, .. } => {
                if lookup_external_macro(name).is_some() {
                    n += 1;
                }
            }
            Stmt::Block { stmts, .. } => n += count_external_in_stmts(stmts),
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                n += count_external_in_stmts(then_branch);
                if let Some(eb) = else_branch {
                    n += count_external_in_stmts(eb);
                }
            }
            Stmt::While { body, .. } | Stmt::ForIn { body, .. } => n += count_external_in_stmts(body),
            Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                n += count_external_in_expr(expr);
            }
            Stmt::Return { value: Some(expr), .. } | Stmt::VarDecl { init: Some(expr), .. } => {
                n += count_external_in_expr(expr);
            }
            Stmt::Throw { expr, .. } => n += count_external_in_expr(expr),
            _ => {}
        }
    }
    n
}

fn count_external_in_expr(expr: &Expr) -> u32 {
    match expr {
        Expr::MacroInvoke { name, .. } => u32::from(lookup_external_macro(name).is_some()),
        Expr::Binary { left, right, .. } => {
            count_external_in_expr(left) + count_external_in_expr(right)
        }
        Expr::Unary { expr, .. } | Expr::Group { expr, .. } => count_external_in_expr(expr),
        Expr::Call { callee, args, .. } => {
            count_external_in_expr(callee)
                + args.iter().map(|a| count_external_in_expr(&a.value)).sum::<u32>()
        }
        Expr::Member { object, .. } => count_external_in_expr(object),
        Expr::Index { object, index, .. } => {
            count_external_in_expr(object) + count_external_in_expr(index)
        }
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
            count_external_in_expr(value)
        }
        _ => 0,
    }
}

fn expand_external_in_decls(
    decls: &mut Vec<Decl>,
    expander: &dyn crate::external_protocol::ExternalMacroExpander,
) -> SyxResult<()> {
    let mut out = Vec::new();
    for decl in decls.drain(..) {
        match decl {
            Decl::MacroSiteInvoke { span, name, args } => {
                if let Some(spec) = lookup_external_macro(&name) {
                    let type_name = args
                        .first()
                        .and_then(expr_string_lit)
                        .unwrap_or_else(|| "ExternalType".into());
                    let req = freestanding_request(&spec.type_name, &type_name, "declaration");
                    out.extend(crate::external_protocol::expand_decls_via_protocol(
                        expander, &req,
                    )?);
                    let _ = span;
                } else {
                    out.push(Decl::MacroSiteInvoke { span, name, args });
                }
            }
            mut d => {
                expand_external_in_decl(&mut d, expander)?;
                out.push(d);
            }
        }
    }
    *decls = out;
    Ok(())
}

fn expand_external_in_decl(
    decl: &mut Decl,
    expander: &dyn crate::external_protocol::ExternalMacroExpander,
) -> SyxResult<()> {
    match decl {
        Decl::Function { body, .. }
        | Decl::Init { body, .. }
        | Decl::Deinit { body, .. } => expand_external_in_stmts(body, expander)?,
        Decl::Class { members, .. } | Decl::Actor { members, .. } => {
            for m in members {
                expand_external_in_decl(m, expander)?;
            }
        }
        Decl::Extension { methods, .. } => {
            for m in methods {
                expand_external_in_decl(m, expander)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn expand_external_in_stmts(
    stmts: &mut Vec<Stmt>,
    expander: &dyn crate::external_protocol::ExternalMacroExpander,
) -> SyxResult<()> {
    let mut out = Vec::new();
    for stmt in stmts.drain(..) {
        match stmt {
            Stmt::MacroInvoke { span, name, args } => {
                if let Some(spec) = lookup_external_macro(&name) {
                    let type_name = args
                        .first()
                        .and_then(expr_string_lit)
                        .unwrap_or_else(|| "Freestanding".into());
                    let req = freestanding_request(&spec.type_name, &type_name, "statement");
                    out.extend(crate::external_protocol::expand_via_protocol(expander, &req)?);
                    let _ = span;
                } else {
                    out.push(Stmt::MacroInvoke { span, name, args });
                }
            }
            Stmt::Block { span, mut stmts } => {
                expand_external_in_stmts(&mut stmts, expander)?;
                out.push(Stmt::Block { span, stmts });
            }
            Stmt::If {
                span,
                mut condition,
                mut then_branch,
                else_branch,
            } => {
                expand_external_in_expr(&mut condition, expander)?;
                expand_external_in_stmts(&mut then_branch, expander)?;
                let else_branch = if let Some(mut eb) = else_branch {
                    expand_external_in_stmts(&mut eb, expander)?;
                    Some(eb)
                } else {
                    None
                };
                out.push(Stmt::If {
                    span,
                    condition,
                    then_branch,
                    else_branch,
                });
            }
            Stmt::While { span, mut condition, mut body } => {
                expand_external_in_expr(&mut condition, expander)?;
                expand_external_in_stmts(&mut body, expander)?;
                out.push(Stmt::While { span, condition, body });
            }
            Stmt::ForIn {
                span,
                name,
                mut iterable,
                mut body,
                is_async_iter,
            } => {
                expand_external_in_expr(&mut iterable, expander)?;
                expand_external_in_stmts(&mut body, expander)?;
                out.push(Stmt::ForIn {
                    span,
                    name,
                    iterable,
                    body,
                    is_async_iter,
                });
            }
            Stmt::Expr { span, mut expr } => {
                expand_external_in_expr(&mut expr, expander)?;
                out.push(Stmt::Expr { span, expr });
            }
            Stmt::Print { span, mut expr } => {
                expand_external_in_expr(&mut expr, expander)?;
                out.push(Stmt::Print { span, expr });
            }
            Stmt::Return { span, value } => {
                let value = if let Some(mut e) = value {
                    expand_external_in_expr(&mut e, expander)?;
                    Some(e)
                } else {
                    None
                };
                out.push(Stmt::Return { span, value });
            }
            Stmt::VarDecl {
                span,
                mutable,
                name,
                ty,
                init,
            } => {
                let init = if let Some(mut e) = init {
                    expand_external_in_expr(&mut e, expander)?;
                    Some(e)
                } else {
                    None
                };
                out.push(Stmt::VarDecl {
                    span,
                    mutable,
                    name,
                    ty,
                    init,
                });
            }
            Stmt::Throw { span, mut expr } => {
                expand_external_in_expr(&mut expr, expander)?;
                out.push(Stmt::Throw { span, expr });
            }
            other => out.push(other),
        }
    }
    *stmts = out;
    Ok(())
}

fn expand_external_in_expr(
    expr: &mut Expr,
    expander: &dyn crate::external_protocol::ExternalMacroExpander,
) -> SyxResult<()> {
    match expr {
        Expr::MacroInvoke { .. } => {
            if let Expr::MacroInvoke { span, name, args } = std::mem::replace(
                expr,
                Expr::Literal {
                    span: syx_diagnostics::Span::DUMMY,
                    value: syx_ast::Literal::Integer {
                        value: 0,
                        suffix: None,
                    },
                },
            ) {
                if lookup_external_macro(&name).is_some() {
                    let spec = lookup_external_macro(&name).unwrap();
                    let type_name = args
                        .first()
                        .and_then(expr_string_lit)
                        .unwrap_or_else(|| "Freestanding".into());
                    let req = freestanding_request(&spec.type_name, &type_name, "expression");
                    let stmts = crate::external_protocol::expand_via_protocol(expander, &req)?;
                    if let Some(Stmt::Expr { expr: inner, .. }) = stmts.first() {
                        *expr = inner.clone();
                    } else if let Some(Stmt::Print { expr: inner, .. }) = stmts.first() {
                        *expr = inner.clone();
                    } else {
                        *expr = Expr::Literal {
                            span,
                            value: syx_ast::Literal::String("external".into()),
                        };
                    }
                } else {
                    *expr = Expr::MacroInvoke { span, name, args };
                }
            }
        }
        Expr::Binary { left, right, .. } => {
            expand_external_in_expr(left, expander)?;
            expand_external_in_expr(right, expander)?;
        }
        Expr::Unary { expr: e, .. } => expand_external_in_expr(e, expander)?,
        Expr::Group { expr: e, .. } => expand_external_in_expr(e, expander)?,
        Expr::Call { callee, args, .. } => {
            expand_external_in_expr(callee, expander)?;
            for a in args {
                expand_external_in_expr(&mut a.value, expander)?;
            }
        }
        Expr::Member { object, .. } => expand_external_in_expr(object, expander)?,
        Expr::Index { object, index, .. } => {
            expand_external_in_expr(object, expander)?;
            expand_external_in_expr(index, expander)?;
        }
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
            expand_external_in_expr(value, expander)?;
        }
        _ => {}
    }
    Ok(())
}

fn freestanding_request(macro_type: &str, type_name: &str, role: &str) -> MacroExpandRequest {
    MacroExpandRequest {
        api_version: 1,
        role: role.into(),
        attachment: macro_type.into(),
        type_name: type_name.into(),
        members: vec!["externalMember".into()],
        syntax: None,
    }
}

fn expr_string_lit(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Literal {
            value: syx_ast::Literal::String(s),
            ..
        } => Some(s.clone()),
        _ => None,
    }
}

pub fn lookup_external_macro(name: &SmolStr) -> Option<ExternalMacroSpec> {
    EXTERNAL_MACROS
        .lock()
        .ok()
        .and_then(|g| g.as_ref().and_then(|m| m.get(name).cloned()))
        .or_else(|| {
            IMPORTED_EXTERNAL
                .lock()
                .ok()
                .and_then(|g| g.get(name).cloned())
        })
}

static IMPORTED_EXTERNAL: LazyLock<Mutex<IndexMap<SmolStr, ExternalMacroSpec>>> =
    LazyLock::new(|| Mutex::new(IndexMap::new()));

pub fn register_imported_external_macros(decls: &[Decl]) {
    if let Ok(mut g) = IMPORTED_EXTERNAL.lock() {
        for decl in decls {
            if let Decl::MacroDecl {
                name,
                implementation: syx_ast::MacroImplementation::External { module, type_name },
                ..
            } = decl
            {
                g.insert(
                    name.clone(),
                    ExternalMacroSpec {
                        module: module.clone(),
                        type_name: type_name.clone(),
                    },
                );
            }
        }
    }
}

/// Phase 58: register all imported macro decl kinds (user + external).
pub fn register_imported_macros(decls: &[Decl]) {
    register_imported_external_macros(decls);
    crate::user::register_imported_user_macros(decls);
}

/// Clear imported macro registries (LSP re-analyze / avoid cross-document leakage).
pub fn clear_imported_macros() {
    if let Ok(mut g) = IMPORTED_EXTERNAL.lock() {
        g.clear();
    }
    crate::user::clear_imported_user_macros();
}

/// Expand attached member body for external macro on a type (Phase 52 JSON protocol).
pub fn expand_external_type_macro(
    macro_type: &str,
    type_name: &SmolStr,
    member_name: &str,
    field_names: &[SmolStr],
) -> SyxResult<Vec<Stmt>> {
    let expander = InProcessMacroExpander;
    let req = MacroExpandRequest {
        api_version: 1,
        role: "member".into(),
        attachment: macro_type.into(),
        type_name: type_name.to_string(),
        members: if field_names.is_empty() {
            vec![member_name.to_string()]
        } else {
            field_names.iter().map(|f| f.to_string()).collect()
        },
        syntax: None,
    };
    let decls = crate::external_protocol::expand_decls_via_protocol(&expander, &req)?;
    if macro_type == "CodableMacro" {
        let mut stmts = Vec::new();
        for decl in decls {
            if let Decl::Function { body, .. } = decl {
                stmts.extend(body);
            }
        }
        return Ok(stmts);
    }
    if let Some(Decl::Function { body, .. }) = decls.into_iter().next() {
        return Ok(body);
    }
    Ok(vec![])
}

/// Expand external macro into extension methods (Codable returns multiple decls).
pub fn expand_external_type_decls(
    macro_type: &str,
    type_name: &SmolStr,
    member_name: &str,
    field_names: &[SmolStr],
) -> SyxResult<Vec<Decl>> {
    let expander = InProcessMacroExpander;
    let req = MacroExpandRequest {
        api_version: 1,
        role: "member".into(),
        attachment: macro_type.into(),
        type_name: type_name.to_string(),
        members: if macro_type == "CodableMacro" {
            field_names.iter().map(|f| f.to_string()).collect()
        } else {
            vec![member_name.to_string()]
        },
        syntax: None,
    };
    crate::external_protocol::expand_decls_via_protocol(&expander, &req)
}
