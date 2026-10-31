//! Phase 40+ user `#macro` expansion.
//! Phase 130 M20: expression-position token/ident/literal substitution MVP.
//! Fingerprint: `macro-token-v1:expr-token-mvp`
//! Phase 131 M21: declaration-site `#macro` param substitution MVP.
//! Fingerprint: `macro-token-v2:decl-param-subst-mvp`
//! Phase 132 M22: declaration-site hygiene MVP.
//! Fingerprint: `macro-token-v3:decl-hygiene-mvp`
//! Phase 133 M23: statement-site `#macro` ident/literal token subst deepen MVP.
//! Fingerprint: `macro-token-v4:stmt-token-subst-mvp`
//! Phase 134 M24: expression-site `#macro` param subst deepen.
//! Fingerprint: `macro-token-v5:expr-token-subst-mvp`
//! Phase 135 M25: declaration-site `#macro` param subst deepen.
//! Fingerprint: `macro-token-v6:decl-token-subst-mvp`
//! Phase 136 M26: declaration-site `#macro` token-stream enumerate MVP.
//! Fingerprint: `macro-token-v7:token-stream-mvp`
//! Phase 137 M27: declaration-site `#macro` token expand MVP.
//! Fingerprint: `macro-token-v8:decl-expand-mvp`
//! Phase 138 M28: multi-token decl rewrite MVP.
//! Fingerprint: `macro-token-v9:token-rewrite-mvp`

use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{AccessLevel, CallArg, Decl, Expr, Literal, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::attached_config::macro_hygiene_enabled;
use crate::hygiene::{
    bind_macro_params, check_macro_decl_hygiene_capture, check_macro_hygiene_capture,
    check_macro_param_shadow, fresh_expansion_id, rename_macro_body, rename_macro_decls,
};
use crate::roles::{macro_has_role, role_mismatch_diag, user_macro_roles, FreestandingRole};
use std::sync::{LazyLock, Mutex};

static IMPORTED_USER_MACROS: LazyLock<Mutex<IndexMap<SmolStr, UserMacroDef>>> =
    LazyLock::new(|| Mutex::new(IndexMap::new()));

/// Phase 58: register `#macro` defs from imported dependency modules.
pub fn register_imported_user_macros(decls: &[Decl]) {
    if let Ok(mut g) = IMPORTED_USER_MACROS.lock() {
        for decl in decls {
            if let Decl::MacroDecl {
                name,
                attrs,
                implementation: syx_ast::MacroImplementation::User {
                    params,
                    body,
                    decl_body,
                },
                ..
            } = decl
            {
                g.insert(
                    name.clone(),
                    UserMacroDef {
                        params: params.clone(),
                        body: body.clone(),
                        decl_body: decl_body.clone(),
                        roles: user_macro_roles(name, attrs),
                    },
                );
            }
        }
    }
}

pub(crate) fn clear_imported_user_macros() {
    if let Ok(mut g) = IMPORTED_USER_MACROS.lock() {
        g.clear();
    }
}

#[derive(Clone)]
struct UserMacroDef {
    params: Vec<SmolStr>,
    body: Vec<Stmt>,
    decl_body: Vec<Decl>,
    roles: Vec<FreestandingRole>,
}

fn collect_module_bindings(module: &syx_ast::Module) -> HashSet<SmolStr> {
    let mut out = HashSet::new();
    for decl in &module.decls {
        if let Decl::VarGlobal { name, .. } = decl {
            out.insert(name.clone());
        }
    }
    out
}

fn expr_to_string_repr(expr: &Expr) -> String {
    match expr {
        Expr::Literal {
            value: Literal::Integer { value, .. },
            ..
        } => value.to_string(),
        Expr::Literal {
            value: Literal::String(s),
            ..
        } => s.clone(),
        Expr::Binary { left, op, right, .. } => {
            format!(
                "{}{}{}",
                expr_to_string_repr(left),
                match op {
                    syx_ast::BinOp::Add => "+",
                    syx_ast::BinOp::Sub => "-",
                    syx_ast::BinOp::Mul => "*",
                    syx_ast::BinOp::Div => "/",
                    _ => "?",
                },
                expr_to_string_repr(right)
            )
        }
        Expr::Unary { op, expr, .. } => format!(
            "{}{}",
            match op {
                syx_ast::UnaryOp::Neg => "-",
                syx_ast::UnaryOp::Not => "!",
                syx_ast::UnaryOp::BitNot => "~",
                syx_ast::UnaryOp::Try => "?",
            },
            expr_to_string_repr(expr)
        ),
        Expr::Group { expr, .. } => expr_to_string_repr(expr),
        _ => "?".into(),
    }
}

/// Phase 130/131/133: substitute a macro parameter token (ident) with the call-site
/// argument expression (ident or literal AST node). Narrow MVP — not full token streams.
/// Phase 133: deepen to match expression-token precision for statement-site bodies
/// (`macro-token-v4:stmt-token-subst-mvp`).
fn subst_param(expr: &Expr, param: &str, arg: &Expr) -> Expr {
    match expr {
        Expr::Ident { name, span } if name.as_str() == param => arg.clone(),
        Expr::Binary { span, left, op, right } => Expr::Binary {
            span: *span,
            left: Box::new(subst_param(left, param, arg)),
            op: *op,
            right: Box::new(subst_param(right, param, arg)),
        },
        Expr::Unary { span, op, expr } => Expr::Unary {
            span: *span,
            op: *op,
            expr: Box::new(subst_param(expr, param, arg)),
        },
        Expr::Call { span, callee, args } => Expr::Call {
            span: *span,
            callee: Box::new(subst_param(callee, param, arg)),
            args: args
                .iter()
                .map(|a| syx_ast::CallArg {
                    label: a.label.clone(),
                    value: subst_param(&a.value, param, arg),
                })
                .collect(),
        },
        Expr::Member { span, object, name } => Expr::Member {
            span: *span,
            object: Box::new(subst_param(object, param, arg)),
            name: name.clone(),
        },
        Expr::Group { span, expr } => Expr::Group {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
        },
        Expr::Index { span, object, index } => Expr::Index {
            span: *span,
            object: Box::new(subst_param(object, param, arg)),
            index: Box::new(subst_param(index, param, arg)),
        },
        Expr::Array { span, elements } => Expr::Array {
            span: *span,
            elements: elements.iter().map(|e| subst_param(e, param, arg)).collect(),
        },
        Expr::Cast { span, expr, target } => Expr::Cast {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
            target: target.clone(),
        },
        Expr::OptionalCast { span, expr, target } => Expr::OptionalCast {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
            target: target.clone(),
        },
        Expr::Assign { span, name, value } => Expr::Assign {
            span: *span,
            name: name.clone(),
            value: Box::new(subst_param(value, param, arg)),
        },
        Expr::CompoundAssign {
            span,
            name,
            op,
            value,
        } => Expr::CompoundAssign {
            span: *span,
            name: name.clone(),
            op: *op,
            value: Box::new(subst_param(value, param, arg)),
        },
        Expr::MemberAssign {
            span,
            object,
            name,
            value,
        } => Expr::MemberAssign {
            span: *span,
            object: Box::new(subst_param(object, param, arg)),
            name: name.clone(),
            value: Box::new(subst_param(value, param, arg)),
        },
        Expr::IndexAssign {
            span,
            object,
            index,
            value,
        } => Expr::IndexAssign {
            span: *span,
            object: Box::new(subst_param(object, param, arg)),
            index: Box::new(subst_param(index, param, arg)),
            value: Box::new(subst_param(value, param, arg)),
        },
        Expr::Await { span, expr } => Expr::Await {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
        },
        Expr::OptionalChain { span, object, name } => Expr::OptionalChain {
            span: *span,
            object: Box::new(subst_param(object, param, arg)),
            name: name.clone(),
        },
        Expr::ForceUnwrap { span, expr } => Expr::ForceUnwrap {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
        },
        Expr::Is { span, expr, target } => Expr::Is {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
            target: target.clone(),
        },
        Expr::Borrow { span, expr, mutable } => Expr::Borrow {
            span: *span,
            expr: Box::new(subst_param(expr, param, arg)),
            mutable: *mutable,
        },
        Expr::MacroInvoke { span, name, args } => Expr::MacroInvoke {
            span: *span,
            name: name.clone(),
            args: args.iter().map(|a| subst_param(a, param, arg)).collect(),
        },
        Expr::Closure {
            span,
            params: closure_params,
            return_type,
            body,
        } => Expr::Closure {
            span: *span,
            params: closure_params.clone(),
            return_type: return_type.clone(),
            body: match body {
                syx_ast::ClosureBody::Expr(inner) => syx_ast::ClosureBody::Expr(Box::new(subst_param(
                    inner, param, arg,
                ))),
                syx_ast::ClosureBody::Block(stmts) => syx_ast::ClosureBody::Block(
                    stmts.iter().map(|s| subst_stmt(s, param, arg)).collect(),
                ),
            },
        },
        Expr::TaskBlock {
            span,
            detached,
            body,
        } => Expr::TaskBlock {
            span: *span,
            detached: *detached,
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        _ => expr.clone(),
    }
}

/// Phase 134 M24: apply all macro param bindings via expression-tree walk.
/// Fingerprint: `macro-token-v5:expr-token-subst-mvp`
fn subst_params_into_expr(expr: &Expr, bindings: &[(&str, &Expr)]) -> Expr {
    let mut out = expr.clone();
    for (param, arg) in bindings {
        out = subst_param(&out, param, arg);
    }
    out
}

/// Phase 133: statement-site param walk aligned with expr-token subst precision.
fn subst_stmt(stmt: &Stmt, param: &str, arg: &Expr) -> Stmt {
    match stmt {
        Stmt::Print { span, expr } => Stmt::Print {
            span: *span,
            expr: subst_param(expr, param, arg),
        },
        Stmt::Expr { span, expr } => Stmt::Expr {
            span: *span,
            expr: subst_param(expr, param, arg),
        },
        Stmt::Throw { span, expr } => Stmt::Throw {
            span: *span,
            expr: subst_param(expr, param, arg),
        },
        Stmt::VarDecl {
            span,
            mutable,
            name,
            ty,
            init,
        } => Stmt::VarDecl {
            span: *span,
            mutable: *mutable,
            name: name.clone(),
            ty: ty.clone(),
            init: init.as_ref().map(|e| subst_param(e, param, arg)),
        },
        Stmt::Return { span, value } => Stmt::Return {
            span: *span,
            value: value.as_ref().map(|e| subst_param(e, param, arg)),
        },
        Stmt::Block { span, stmts } => Stmt::Block {
            span: *span,
            stmts: stmts.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Stmt::If {
            span,
            condition,
            then_branch,
            else_branch,
        } => Stmt::If {
            span: *span,
            condition: subst_param(condition, param, arg),
            then_branch: then_branch.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            else_branch: else_branch
                .as_ref()
                .map(|b| b.iter().map(|s| subst_stmt(s, param, arg)).collect()),
        },
        Stmt::While {
            span,
            condition,
            body,
        } => Stmt::While {
            span: *span,
            condition: subst_param(condition, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Stmt::RepeatWhile {
            span,
            body,
            condition,
        } => Stmt::RepeatWhile {
            span: *span,
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            condition: subst_param(condition, param, arg),
        },
        Stmt::ForIn {
            span,
            name,
            iterable,
            body,
            is_async_iter,
        } => Stmt::ForIn {
            span: *span,
            name: name.clone(),
            iterable: subst_param(iterable, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            is_async_iter: *is_async_iter,
        },
        Stmt::Try {
            span,
            body,
            catch_name,
            catch_type,
            catch_body,
        } => Stmt::Try {
            span: *span,
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            catch_name: catch_name.clone(),
            catch_type: catch_type.clone(),
            catch_body: catch_body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Stmt::Switch {
            span,
            subject,
            cases,
            default,
        } => Stmt::Switch {
            span: *span,
            subject: subst_param(subject, param, arg),
            cases: cases
                .iter()
                .map(|c| syx_ast::SwitchCase {
                    pattern: subst_param(&c.pattern, param, arg),
                    guard: c.guard.as_ref().map(|g| subst_param(g, param, arg)),
                    body: c.body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
                })
                .collect(),
            default: default
                .as_ref()
                .map(|b| b.iter().map(|s| subst_stmt(s, param, arg)).collect()),
        },
        Stmt::IfCase {
            span,
            pattern,
            subject,
            body,
            else_branch,
        } => Stmt::IfCase {
            span: *span,
            pattern: subst_param(pattern, param, arg),
            subject: subst_param(subject, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            else_branch: else_branch
                .as_ref()
                .map(|b| b.iter().map(|s| subst_stmt(s, param, arg)).collect()),
        },
        Stmt::IfLet {
            span,
            name,
            pattern,
            body,
            else_branch,
        } => Stmt::IfLet {
            span: *span,
            name: name.clone(),
            pattern: subst_param(pattern, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            else_branch: else_branch
                .as_ref()
                .map(|b| b.iter().map(|s| subst_stmt(s, param, arg)).collect()),
        },
        Stmt::GuardLet {
            span,
            name,
            pattern,
            body,
            else_branch,
        } => Stmt::GuardLet {
            span: *span,
            name: name.clone(),
            pattern: subst_param(pattern, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            else_branch: else_branch.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Stmt::MacroInvoke { span, name, args } => Stmt::MacroInvoke {
            span: *span,
            name: name.clone(),
            args: args.iter().map(|a| subst_param(a, param, arg)).collect(),
        },
        other => other.clone(),
    }
}

/// Phase 136 M26: narrow token-stream kinds for `#macro` decl params.
/// Fingerprint: `macro-token-v7:token-stream-mvp`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroTokenKind {
    Ident(SmolStr),
    Punct(char),
    Group(Vec<MacroTokenKind>),
}

fn binop_punct(op: syx_ast::BinOp) -> char {
    match op {
        syx_ast::BinOp::Add => '+',
        syx_ast::BinOp::Sub => '-',
        syx_ast::BinOp::Mul => '*',
        syx_ast::BinOp::Div => '/',
        _ => '?',
    }
}

fn unary_punct(op: syx_ast::UnaryOp) -> char {
    match op {
        syx_ast::UnaryOp::Neg => '-',
        syx_ast::UnaryOp::Not => '!',
        syx_ast::UnaryOp::BitNot => '~',
        syx_ast::UnaryOp::Try => '?',
    }
}

/// Enumerate a declaration-site `#macro` argument as a minimal token stream
/// (`Ident` / `Punct` / `Group`). Not a full proc-macro TokenStream / user SYX expand.
/// Fingerprint: `macro-token-v7:token-stream-mvp`
pub fn enumerate_decl_param_tokens(expr: &Expr) -> Vec<MacroTokenKind> {
    match expr {
        Expr::Ident { name, .. } => vec![MacroTokenKind::Ident(name.clone())],
        Expr::Literal {
            value: Literal::Integer { value, .. },
            ..
        } => vec![MacroTokenKind::Ident(SmolStr::new(value.to_string()))],
        Expr::Literal {
            value: Literal::Double { value, .. },
            ..
        } => vec![MacroTokenKind::Ident(SmolStr::new(value.to_string()))],
        Expr::Literal {
            value: Literal::String(s),
            ..
        } => vec![MacroTokenKind::Ident(SmolStr::new(s.as_str()))],
        Expr::Literal {
            value: Literal::Bool(b),
            ..
        } => vec![MacroTokenKind::Ident(SmolStr::new(if *b {
            "true"
        } else {
            "false"
        }))],
        Expr::Group { expr, .. } => {
            vec![MacroTokenKind::Group(enumerate_decl_param_tokens(expr))]
        }
        Expr::Binary {
            left, op, right, ..
        } => {
            let mut out = enumerate_decl_param_tokens(left);
            out.push(MacroTokenKind::Punct(binop_punct(*op)));
            out.extend(enumerate_decl_param_tokens(right));
            out
        }
        Expr::Unary { op, expr, .. } => {
            let mut out = vec![MacroTokenKind::Punct(unary_punct(*op))];
            out.extend(enumerate_decl_param_tokens(expr));
            out
        }
        Expr::Call { callee, args, .. } => {
            let mut out = enumerate_decl_param_tokens(callee);
            let mut inner = Vec::new();
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    inner.push(MacroTokenKind::Punct(','));
                }
                inner.extend(enumerate_decl_param_tokens(&a.value));
            }
            out.push(MacroTokenKind::Group(inner));
            out
        }
        _ => vec![MacroTokenKind::Ident(SmolStr::new("?"))],
    }
}

/// Phase 137 M27: narrow decl-site token expand — single `Ident` → transformed function decl.
/// Phase 138 M28: multi-token `Ident`/`Punct`/`Group` streams → transformed decl (hygiene preserved).
/// Fingerprint: `macro-token-v8:decl-expand-mvp`, `macro-token-v9:token-rewrite-mvp`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclExpandMode {
    /// Prefix the ident name (rename).
    RenamePrefix(SmolStr),
    /// Wrap: emit `func {prefix}{name}()` with a marker print.
    WrapPrefix(SmolStr),
}

/// Flatten declaration-site token stream (`Ident` / `Punct` / `Group`) into a base name string.
/// Fingerprint: `macro-token-v9:token-rewrite-mvp`
pub fn decl_base_name_from_tokens(tokens: &[MacroTokenKind]) -> Option<SmolStr> {
    let mut out = String::new();
    let mut has_ident = false;
    fn append(out: &mut String, has_ident: &mut bool, tok: &MacroTokenKind) -> bool {
        match tok {
            MacroTokenKind::Ident(s) => {
                *has_ident = true;
                out.push_str(s.as_str());
                true
            }
            MacroTokenKind::Punct(c) => {
                out.push(*c);
                true
            }
            MacroTokenKind::Group(inner) => inner.iter().all(|t| append(out, has_ident, t)),
        }
    }
    for t in tokens {
        if !append(&mut out, &mut has_ident, t) {
            return None;
        }
    }
    if out.is_empty() || !has_ident {
        return None;
    }
    if !out
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || "+-*/?".contains(c))
    {
        return None;
    }
    Some(SmolStr::new(out))
}

/// Expand a declaration-site `#macro` token argument into a minimal function decl (rename/wrap MVP).
/// Phase 138: accepts multi-token streams (`Ident`+`Punct`+`Group`), not only a lone `Ident`.
/// Fingerprint: `macro-token-v8:decl-expand-mvp`, `macro-token-v9:token-rewrite-mvp`
pub fn expand_decl_param_tokens(tokens: &[MacroTokenKind], mode: DeclExpandMode) -> Option<Decl> {
    let base = decl_base_name_from_tokens(tokens)?;
    let span = syx_diagnostics::Span::new(0, 0);
    let (name, marker) = match &mode {
        DeclExpandMode::RenamePrefix(prefix) => (
            SmolStr::new(format!("{prefix}{base}")),
            SmolStr::new(if tokens.len() > 1 {
                "decl-rewrite-rename"
            } else {
                "decl-expand-rename"
            }),
        ),
        DeclExpandMode::WrapPrefix(prefix) => (
            SmolStr::new(format!("{prefix}{base}")),
            SmolStr::new(if tokens.len() > 1 {
                "decl-rewrite-wrap"
            } else {
                "decl-expand-wrap"
            }),
        ),
    };
    Some(Decl::Function {
        span,
        access: AccessLevel::Internal,
        attrs: vec![],
        name,
        type_params: vec![],
        params: vec![],
        return_type: None,
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body: vec![Stmt::Expr {
            span,
            expr: Expr::Call {
                span,
                callee: Box::new(Expr::Ident {
                    span,
                    name: SmolStr::new("print"),
                }),
                args: vec![CallArg {
                    label: None,
                    value: Expr::Literal {
                        span,
                        value: Literal::String(marker.to_string()),
                    },
                }],
            },
        }],
        is_async: false,
        is_override: false,
        is_final: false,
    })
}

/// Phase 135 M25: substitute ident param into declaration names (function/enum case).
/// Fingerprint: `macro-token-v6:decl-token-subst-mvp`
fn subst_decl_name(name: &smol_str::SmolStr, param: &str, arg: &Expr) -> smol_str::SmolStr {
    if name.as_str() != param {
        return name.clone();
    }
    if let Expr::Ident { name: id, .. } = arg {
        return id.clone();
    }
    name.clone()
}

fn subst_params_list(params: &[syx_ast::Param], param: &str, arg: &Expr) -> Vec<syx_ast::Param> {
    params
        .iter()
        .map(|p| syx_ast::Param {
            default: p.default.as_ref().map(|e| subst_param(e, param, arg)),
            ..p.clone()
        })
        .collect()
}

/// Phase 131 M21: walk declaration macro bodies and substitute param idents/literals.
/// Phase 135 M25: deepen to function names, param defaults, enum payloads
/// (`macro-token-v6:decl-token-subst-mvp`).
fn subst_decl(decl: &Decl, param: &str, arg: &Expr) -> Decl {
    match decl {
        Decl::Function {
            span,
            access,
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
            is_override,
            is_final,
        } => Decl::Function {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            name: subst_decl_name(name, param, arg),
            type_params: type_params.clone(),
            params: subst_params_list(params, param, arg),
            return_type: return_type.clone(),
            throws_type: throws_type.clone(),
            effect: effect.clone(),
            where_clause: where_clause.clone(),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
            is_async: *is_async,
            is_override: *is_override,
            is_final: *is_final,
        },
        Decl::VarGlobal {
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
        } => Decl::VarGlobal {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            mutable: *mutable,
            name: name.clone(),
            ty: ty.clone(),
            init: init.as_ref().map(|e| subst_param(e, param, arg)),
            will_set: will_set
                .as_ref()
                .map(|ss| ss.iter().map(|s| subst_stmt(s, param, arg)).collect()),
            did_set: did_set
                .as_ref()
                .map(|ss| ss.iter().map(|s| subst_stmt(s, param, arg)).collect()),
            computed_getter: computed_getter
                .as_ref()
                .map(|ss| ss.iter().map(|s| subst_stmt(s, param, arg)).collect()),
            computed_setter: computed_setter
                .as_ref()
                .map(|ss| ss.iter().map(|s| subst_stmt(s, param, arg)).collect()),
            is_override: *is_override,
            is_lazy: *is_lazy,
        },
        Decl::Class {
            span,
            access,
            attrs,
            name,
            type_params,
            type_param_defaults,
            superclass,
            conformances,
            members,
        } => Decl::Class {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            name: name.clone(),
            type_params: type_params.clone(),
            type_param_defaults: type_param_defaults.clone(),
            superclass: superclass.clone(),
            conformances: conformances.clone(),
            members: members.iter().map(|d| subst_decl(d, param, arg)).collect(),
        },
        Decl::Actor {
            span,
            access,
            name,
            type_params,
            type_param_defaults,
            conformances,
            members,
        } => Decl::Actor {
            span: *span,
            access: *access,
            name: name.clone(),
            type_params: type_params.clone(),
            type_param_defaults: type_param_defaults.clone(),
            conformances: conformances.clone(),
            members: members.iter().map(|d| subst_decl(d, param, arg)).collect(),
        },
        Decl::Extension {
            span,
            type_name,
            conformances,
            where_clause,
            methods,
        } => Decl::Extension {
            span: *span,
            type_name: type_name.clone(),
            conformances: conformances.clone(),
            where_clause: where_clause.clone(),
            methods: methods.iter().map(|d| subst_decl(d, param, arg)).collect(),
        },
        Decl::Init {
            span,
            access,
            params,
            body,
        } => Decl::Init {
            span: *span,
            access: *access,
            params: subst_params_list(params, param, arg),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Decl::Deinit { span, access, body } => Decl::Deinit {
            span: *span,
            access: *access,
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Decl::TopLevelStmt { span, stmt } => Decl::TopLevelStmt {
            span: *span,
            stmt: subst_stmt(stmt, param, arg),
        },
        Decl::OperatorFunc {
            span,
            access,
            attrs,
            op,
            params,
            return_type,
            body,
        } => Decl::OperatorFunc {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            op: op.clone(),
            params: subst_params_list(params, param, arg),
            return_type: return_type.clone(),
            body: body.iter().map(|s| subst_stmt(s, param, arg)).collect(),
        },
        Decl::Enum {
            span,
            access,
            attrs,
            name,
            type_params,
            cases,
        } => Decl::Enum {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            name: name.clone(),
            type_params: type_params.clone(),
            cases: cases
                .iter()
                .map(|(case_name, payload, fields)| {
                    (
                        subst_decl_name(case_name, param, arg),
                        payload.as_ref().map(|e| subst_param(e, param, arg)),
                        fields.clone(),
                    )
                })
                .collect(),
        },
        other => other.clone(),
    }
}

fn expand_site_invokes(
    decls: Vec<Decl>,
    defs: &IndexMap<smol_str::SmolStr, UserMacroDef>,
) -> SyxResult<Vec<Decl>> {
    let mut out = Vec::new();
    for decl in decls {
        match decl {
            Decl::MacroSiteInvoke { name, args, .. } => {
                if let Some(def) = defs.get(&name) {
                    if !macro_has_role(&def.roles, FreestandingRole::Declaration) {
                        return Err(role_mismatch_diag(name.as_str(), FreestandingRole::Declaration));
                    }
                    if def.params.len() != args.len() {
                        return Err(SyxDiagnostic::error(format!(
                            "macro '#{name}' arity mismatch"
                        )));
                    }
                    // Phase 131: substitute ident/literal args into decl_body
                    // (macro-token-v2:decl-param-subst-mvp).
                    // Phase 132: hygienic param rename before subst
                    // (macro-token-v3:decl-hygiene-mvp).
                    // Phase 135 M25: decl-site token subst deepen
                    // (macro-token-v6:decl-token-subst-mvp).
                    let mut expanded = def.decl_body.clone();
                    if macro_hygiene_enabled() {
                        let eid = fresh_expansion_id();
                        let renames = bind_macro_params(name.as_str(), &def.params, eid);
                        rename_macro_decls(&mut expanded, &renames);
                        for ((_, hyg), arg_expr) in renames.iter().zip(args.iter()) {
                            expanded = expanded
                                .into_iter()
                                .map(|d| subst_decl(&d, hyg.as_str(), arg_expr))
                                .collect();
                        }
                    } else {
                        for (param, arg_expr) in def.params.iter().zip(args.iter()) {
                            expanded = expanded
                                .into_iter()
                                .map(|d| subst_decl(&d, param.as_str(), arg_expr))
                                .collect();
                        }
                    }
                    out.extend(expanded);
                    continue;
                }
                return Err(SyxDiagnostic::error(format!("Unknown macro '#{name}'")));
            }
            other => out.push(other),
        }
    }
    Ok(out)
}

pub(crate) fn expand_user_macros(module: &mut syx_ast::Module) -> SyxResult<()> {
    use indexmap::IndexMap;
    use syx_ast::{Decl, Expr, Stmt};

    let module_bindings = collect_module_bindings(module);
    let mut defs: IndexMap<smol_str::SmolStr, UserMacroDef> = IndexMap::new();
    let mut kept_decls = Vec::new();
    for decl in module.decls.drain(..) {
        if let Decl::MacroDecl {
            name,
            attrs,
            implementation: syx_ast::MacroImplementation::User {
                params,
                body,
                decl_body,
            },
            ..
        } = decl
        {
            if macro_hygiene_enabled() {
                check_macro_hygiene_capture(name.as_str(), &params, &body, &module_bindings)?;
                check_macro_decl_hygiene_capture(
                    name.as_str(),
                    &params,
                    &decl_body,
                    &module_bindings,
                )?;
                check_macro_param_shadow(name.as_str(), &params, &module_bindings)?;
            }
            defs.insert(
                name.clone(),
                UserMacroDef {
                    params,
                    body,
                    decl_body,
                    roles: user_macro_roles(&name, &attrs),
                },
            );
        } else {
            kept_decls.push(decl);
        }
    }
    if let Ok(g) = IMPORTED_USER_MACROS.lock() {
        for (name, def) in g.iter() {
            defs.entry(name.clone()).or_insert_with(|| def.clone());
        }
    }
    module.decls = expand_site_invokes(kept_decls, &defs)?;

    /// Extract a single expression from an expression-role macro body after param subst.
    fn expr_from_macro_body(name: &str, expanded: &[Stmt]) -> SyxResult<Expr> {
        match expanded {
            [Stmt::Expr { expr, .. } | Stmt::Print { expr, .. }] => Ok(expr.clone()),
            [Stmt::Return {
                value: Some(expr), ..
            }] => Ok(expr.clone()),
            [Stmt::Block { stmts, .. }] => expr_from_macro_body(name, stmts),
            _ => Err(SyxDiagnostic::error(format!(
                "expression-position macro '#{name}' must expand to a single expression (ident/literal token subst MVP)"
            ))
            .with_code("E002")),
        }
    }

    fn expand_expr_user(
        expr: &mut Expr,
        defs: &IndexMap<smol_str::SmolStr, UserMacroDef>,
        depth: u32,
        stack: &mut Vec<smol_str::SmolStr>,
    ) -> SyxResult<()> {
        match expr {
            Expr::MacroInvoke { span, name, args } => {
                // `#stringify` is handled in the builtin pass (Phase 119 M9).
                // Phase 130: expand nested token args first so `#double(#inc(20))` works.
                for a in args.iter_mut() {
                    expand_expr_user(a, defs, depth, stack)?;
                }
                if let Some(def) = defs.get(name) {
                    if !macro_has_role(&def.roles, FreestandingRole::Expression) {
                        return Err(role_mismatch_diag(name.as_str(), FreestandingRole::Expression));
                    }
                    let params = &def.params;
                    let body = &def.body;
                    if params.len() != args.len() {
                        return Err(SyxDiagnostic::error(format!(
                            "macro '#{name}' arity mismatch"
                        )));
                    }
                    if depth >= 16 {
                        return Err(SyxDiagnostic::error("macro expansion depth exceeded").with_code("E002"));
                    }
                    if stack.iter().any(|n| n == name) {
                        return Err(SyxDiagnostic::error("cyclic macro expansion").with_code("E002"));
                    }
                    stack.push(name.clone());
                    // Phase 134 M24: expression-site param subst deepen
                    // (macro-token-v5:expr-token-subst-mvp).
                    let mut body_for_template = body.clone();
                    let bindings: Vec<(String, Expr)> = if macro_hygiene_enabled() {
                        let eid = fresh_expansion_id();
                        let renames = bind_macro_params(name.as_str(), params, eid);
                        rename_macro_body(&mut body_for_template, &renames);
                        renames
                            .into_iter()
                            .zip(args.iter().cloned())
                            .map(|((_, hyg), arg_expr)| (hyg.to_string(), arg_expr))
                            .collect()
                    } else {
                        params
                            .iter()
                            .zip(args.iter().cloned())
                            .map(|(p, a)| (p.to_string(), a))
                            .collect()
                    };
                    let binding_refs: Vec<(&str, &Expr)> = bindings
                        .iter()
                        .map(|(p, a)| (p.as_str(), a))
                        .collect();
                    let replaced =
                        if let Ok(template) = expr_from_macro_body(name.as_str(), &body_for_template) {
                            subst_params_into_expr(&template, &binding_refs)
                        } else {
                        let mut expanded = body.clone();
                        if macro_hygiene_enabled() {
                            let eid = fresh_expansion_id();
                            let renames = bind_macro_params(name.as_str(), params, eid);
                            rename_macro_body(&mut expanded, &renames);
                            for ((_, hyg), arg_expr) in renames.iter().zip(args.iter()) {
                                expanded = expanded
                                    .into_iter()
                                    .map(|s| subst_stmt(&s, hyg.as_str(), arg_expr))
                                    .collect();
                            }
                        } else {
                            for (param, arg_expr) in params.iter().zip(args.iter()) {
                                expanded = expanded
                                    .into_iter()
                                    .map(|s| subst_stmt(&s, param.as_str(), arg_expr))
                                    .collect();
                            }
                        }
                        expand_stmts_user(&mut expanded, defs, depth + 1, stack)?;
                        expr_from_macro_body(name.as_str(), &expanded)?
                    };
                    stack.pop();
                    *expr = replaced;
                    // Recurse in case body produced nested macro invokes.
                    expand_expr_user(expr, defs, depth + 1, stack)?;
                } else if crate::external::lookup_external_macro(name).is_none()
                    && name.as_str() != "stringify"
                {
                    let _ = span;
                    return Err(SyxDiagnostic::error(format!("Unknown macro '#{name}'")));
                }
            }
            Expr::Binary { left, right, .. } => {
                expand_expr_user(left, defs, depth, stack)?;
                expand_expr_user(right, defs, depth, stack)?;
            }
            Expr::Unary { expr: inner, .. } => expand_expr_user(inner, defs, depth, stack)?,
            Expr::Group { expr: inner, .. } => expand_expr_user(inner, defs, depth, stack)?,
            Expr::Call { callee, args, .. } => {
                expand_expr_user(callee, defs, depth, stack)?;
                for a in args {
                    expand_expr_user(&mut a.value, defs, depth, stack)?;
                }
            }
            Expr::Member { object, .. } => expand_expr_user(object, defs, depth, stack)?,
            Expr::Index { object, index, .. } => {
                expand_expr_user(object, defs, depth, stack)?;
                expand_expr_user(index, defs, depth, stack)?;
            }
            Expr::Array { elements, .. } => {
                for e in elements {
                    expand_expr_user(e, defs, depth, stack)?;
                }
            }
            Expr::Cast { expr: inner, .. } | Expr::OptionalCast { expr: inner, .. } => {
                expand_expr_user(inner, defs, depth, stack)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn expand_exprs_in_stmt(
        stmt: &mut Stmt,
        defs: &IndexMap<smol_str::SmolStr, UserMacroDef>,
        depth: u32,
        stack: &mut Vec<smol_str::SmolStr>,
    ) -> SyxResult<()> {
        match stmt {
            Stmt::Print { expr, .. }
            | Stmt::Expr { expr, .. }
            | Stmt::Throw { expr, .. } => {
                expand_expr_user(expr, defs, depth, stack)?;
            }
            Stmt::VarDecl { init: Some(expr), .. } => {
                expand_expr_user(expr, defs, depth, stack)?;
            }
            Stmt::Return { value: Some(expr), .. } => {
                expand_expr_user(expr, defs, depth, stack)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                expand_expr_user(condition, defs, depth, stack)?;
                for s in then_branch.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
                if let Some(eb) = else_branch {
                    for s in eb.iter_mut() {
                        expand_exprs_in_stmt(s, defs, depth, stack)?;
                    }
                }
            }
            Stmt::While {
                condition, body, ..
            }
            | Stmt::RepeatWhile {
                condition, body, ..
            } => {
                expand_expr_user(condition, defs, depth, stack)?;
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
            }
            Stmt::ForIn {
                iterable, body, ..
            } => {
                expand_expr_user(iterable, defs, depth, stack)?;
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
            }
            Stmt::Try {
                body, catch_body, ..
            } => {
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
                for s in catch_body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
            }
            Stmt::Switch {
                subject,
                cases,
                default,
                ..
            } => {
                expand_expr_user(subject, defs, depth, stack)?;
                for c in cases.iter_mut() {
                    expand_expr_user(&mut c.pattern, defs, depth, stack)?;
                    if let Some(g) = &mut c.guard {
                        expand_expr_user(g, defs, depth, stack)?;
                    }
                    for s in c.body.iter_mut() {
                        expand_exprs_in_stmt(s, defs, depth, stack)?;
                    }
                }
                if let Some(d) = default {
                    for s in d.iter_mut() {
                        expand_exprs_in_stmt(s, defs, depth, stack)?;
                    }
                }
            }
            Stmt::IfCase {
                pattern,
                subject,
                body,
                else_branch,
                ..
            } => {
                expand_expr_user(pattern, defs, depth, stack)?;
                expand_expr_user(subject, defs, depth, stack)?;
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
                if let Some(eb) = else_branch {
                    for s in eb.iter_mut() {
                        expand_exprs_in_stmt(s, defs, depth, stack)?;
                    }
                }
            }
            Stmt::IfLet {
                pattern,
                body,
                else_branch,
                ..
            } => {
                expand_expr_user(pattern, defs, depth, stack)?;
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
                if let Some(eb) = else_branch {
                    for s in eb.iter_mut() {
                        expand_exprs_in_stmt(s, defs, depth, stack)?;
                    }
                }
            }
            Stmt::GuardLet {
                pattern,
                body,
                else_branch,
                ..
            } => {
                expand_expr_user(pattern, defs, depth, stack)?;
                for s in body.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
                for s in else_branch.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
            }
            Stmt::Block { stmts, .. } => {
                for s in stmts.iter_mut() {
                    expand_exprs_in_stmt(s, defs, depth, stack)?;
                }
            }
            Stmt::MacroInvoke { args, .. } => {
                // Phase 133: expand nested token args before statement-site subst
                // (align with expr-token precision).
                for a in args.iter_mut() {
                    expand_expr_user(a, defs, depth, stack)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn expand_stmts_user(
        stmts: &mut Vec<Stmt>,
        defs: &IndexMap<smol_str::SmolStr, UserMacroDef>,
        depth: u32,
        stack: &mut Vec<smol_str::SmolStr>,
    ) -> SyxResult<()> {
        let mut out = Vec::new();
        for mut stmt in stmts.drain(..) {
            expand_exprs_in_stmt(&mut stmt, defs, depth, stack)?;
            if let Stmt::MacroInvoke { name, args, .. } = &stmt {
                if let Some(def) = defs.get(name) {
                    if !macro_has_role(&def.roles, FreestandingRole::Statement) {
                        return Err(role_mismatch_diag(name.as_str(), FreestandingRole::Statement));
                    }
                    let params = &def.params;
                    let body = &def.body;
                    if params.len() != args.len() {
                        return Err(SyxDiagnostic::error(format!(
                            "macro '#{name}' arity mismatch"
                        )));
                    }
                    if depth >= 16 {
                        return Err(SyxDiagnostic::error("macro expansion depth exceeded").with_code("E002"));
                    }
                    if stack.iter().any(|n| n == name) {
                        return Err(SyxDiagnostic::error("cyclic macro expansion").with_code("E002"));
                    }
                    stack.push(name.clone());
                    // Phase 133 M23: stmt-site ident/literal token subst
                    // (macro-token-v4:stmt-token-subst-mvp).
                    let mut expanded = body.clone();
                    if macro_hygiene_enabled() {
                        let eid = fresh_expansion_id();
                        let renames = bind_macro_params(name.as_str(), params, eid);
                        rename_macro_body(&mut expanded, &renames);
                        for ((_, hyg), arg_expr) in renames.iter().zip(args.iter()) {
                            expanded = expanded
                                .into_iter()
                                .map(|s| subst_stmt(&s, hyg.as_str(), arg_expr))
                                .collect();
                        }
                    } else {
                        for (param, arg_expr) in params.iter().zip(args.iter()) {
                            expanded = expanded
                                .into_iter()
                                .map(|s| subst_stmt(&s, param.as_str(), arg_expr))
                                .collect();
                        }
                    }
                    expand_stmts_user(&mut expanded, defs, depth + 1, stack)?;
                    out.extend(expanded);
                    stack.pop();
                    continue;
                }
            }
            match stmt {
                Stmt::Block { span, mut stmts, .. } => {
                    expand_stmts_user(&mut stmts, defs, depth, stack)?;
                    out.push(Stmt::Block { span, stmts });
                }
                Stmt::If {
                    span,
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let mut then_b = then_branch;
                    expand_stmts_user(&mut then_b, defs, depth, stack)?;
                    let else_b = if let Some(mut eb) = else_branch {
                        expand_stmts_user(&mut eb, defs, depth, stack)?;
                        Some(eb)
                    } else {
                        None
                    };
                    out.push(Stmt::If {
                        span,
                        condition,
                        then_branch: then_b,
                        else_branch: else_b,
                    });
                }
                Stmt::While {
                    span,
                    condition,
                    mut body,
                } => {
                    expand_stmts_user(&mut body, defs, depth, stack)?;
                    out.push(Stmt::While {
                        span,
                        condition,
                        body,
                    });
                }
                Stmt::RepeatWhile {
                    span,
                    mut body,
                    condition,
                } => {
                    expand_stmts_user(&mut body, defs, depth, stack)?;
                    out.push(Stmt::RepeatWhile {
                        span,
                        body,
                        condition,
                    });
                }
                Stmt::ForIn {
                    span,
                    name,
                    iterable,
                    mut body,
                    is_async_iter,
                } => {
                    expand_stmts_user(&mut body, defs, depth, stack)?;
                    out.push(Stmt::ForIn {
                        span,
                        name,
                        iterable,
                        body,
                        is_async_iter,
                    });
                }
                Stmt::Try {
                    span,
                    mut body,
                    catch_name,
                    catch_type,
                    mut catch_body,
                } => {
                    expand_stmts_user(&mut body, defs, depth, stack)?;
                    expand_stmts_user(&mut catch_body, defs, depth, stack)?;
                    out.push(Stmt::Try {
                        span,
                        body,
                        catch_name,
                        catch_type,
                        catch_body,
                    });
                }
                other => out.push(other),
            }
        }
        *stmts = out;
        Ok(())
    }

    fn expand_decls_user(
        decls: &mut [Decl],
        defs: &IndexMap<smol_str::SmolStr, UserMacroDef>,
    ) -> SyxResult<()> {
        let mut stack = Vec::new();
        for decl in decls {
            match decl {
                Decl::Function { body, .. }
                | Decl::Init { body, .. }
                | Decl::Deinit { body, .. } => {
                    expand_stmts_user(body, defs, 0, &mut stack)?;
                }
                Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                    expand_decls_user(members, defs)?;
                }
                Decl::Extension { methods, .. } => expand_decls_user(methods, defs)?,
                _ => {}
            }
        }
        Ok(())
    }

    expand_decls_user(&mut module.decls, &defs)?;
    fn reject_unknown_macros(stmts: &mut Vec<Stmt>) -> SyxResult<()> {
        let mut out = Vec::new();
        for stmt in stmts.drain(..) {
            if let Stmt::MacroInvoke { name, .. } = &stmt {
                if crate::external::lookup_external_macro(name).is_none() {
                    return Err(SyxDiagnostic::error(format!("Unknown macro '#{name}'")));
                }
                out.push(stmt);
                continue;
            }
            match stmt {
                Stmt::Block { span, mut stmts, .. } => {
                    reject_unknown_macros(&mut stmts)?;
                    out.push(Stmt::Block { span, stmts });
                }
                Stmt::If {
                    span,
                    condition,
                    mut then_branch,
                    else_branch,
                } => {
                    reject_unknown_macros(&mut then_branch)?;
                    let else_branch = if let Some(mut eb) = else_branch {
                        reject_unknown_macros(&mut eb)?;
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
                Stmt::While {
                    span,
                    condition,
                    mut body,
                } => {
                    reject_unknown_macros(&mut body)?;
                    out.push(Stmt::While {
                        span,
                        condition,
                        body,
                    });
                }
                other => out.push(other),
            }
        }
        *stmts = out;
        Ok(())
    }
    fn reject_in_decls(decls: &mut [Decl]) -> SyxResult<()> {
        for decl in decls {
            match decl {
                Decl::Function { body, .. }
                | Decl::Init { body, .. }
                | Decl::Deinit { body, .. } => reject_unknown_macros(body)?,
                Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                    reject_in_decls(members)?
                }
                Decl::Extension { methods, .. } => reject_in_decls(methods)?,
                _ => {}
            }
        }
        Ok(())
    }
    reject_in_decls(&mut module.decls)
}
