//! S2-I160 / S2-I162: CST→AST lower probe (≠ FrontendPipeline / ≠ full-subset lower).
//! Probe surface: zero-arg `func main() { 0..N` of
//!   print(<int|ident|string|call-simple>) |
//!   let/var <ident> = <simple> |
//!   return [<simple>]
//! `}` (W791–W797 simple-stmt band; S2-I162).

use smol_str::SmolStr;
use syx_ast::{AccessLevel, CallArg, Decl, Expr, Literal, Module, Stmt};
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_lexer::TokenKind;

use crate::{is_trivia, parse_cst_subset, CstRoot, GreenToken};

fn lwr001(message: impl Into<String>) -> SyxDiagnostic {
    SyxDiagnostic::error(message).with_code("LWR001")
}

fn significant_green(tokens: &[GreenToken]) -> Vec<&GreenToken> {
    tokens.iter().filter(|t| !is_trivia(&t.kind)).collect()
}

fn dummy() -> Span {
    Span::DUMMY
}

fn lower_simple_value(tok: &GreenToken) -> Option<Expr> {
    match &tok.kind {
        TokenKind::Integer(v, suffix) => Some(Expr::Literal {
            span: dummy(),
            value: Literal::Integer {
                value: *v,
                suffix: suffix.clone(),
            },
        }),
        TokenKind::Double(v, suffix) => Some(Expr::Literal {
            span: dummy(),
            value: Literal::Double {
                value: *v,
                suffix: suffix.clone(),
            },
        }),
        TokenKind::Bool(b) => Some(Expr::Literal {
            span: dummy(),
            value: Literal::Bool(*b),
        }),
        TokenKind::Nil => Some(Expr::Literal {
            span: dummy(),
            value: Literal::Nil,
        }),
        TokenKind::Identifier(name) => Some(Expr::Ident {
            span: dummy(),
            name: name.clone(),
        }),
        TokenKind::String(s) => Some(Expr::Literal {
            span: dummy(),
            value: Literal::String(s.clone()),
        }),
        _ => None,
    }
}

/// Call-simple for probe: `Ident ( )` or `Ident ( simple )` — no member chain / nested call.
/// Returns `(expr, next_index)`.
fn take_call_simple_probe(sig: &[&GreenToken], i: usize) -> Option<(Expr, usize)> {
    if i >= sig.len() {
        return None;
    }
    let TokenKind::Identifier(name) = &sig[i].kind else {
        return None;
    };
    if i + 1 >= sig.len() || sig[i + 1].kind != TokenKind::LParen {
        return None;
    }
    // Zero-arg: Ident ( )
    if i + 2 < sig.len() && sig[i + 2].kind == TokenKind::RParen {
        return Some((
            Expr::Call {
                span: dummy(),
                callee: Box::new(Expr::Ident {
                    span: dummy(),
                    name: name.clone(),
                }),
                args: vec![],
            },
            i + 3,
        ));
    }
    // One simple arg: Ident ( simple )
    if i + 3 >= sig.len() {
        return None;
    }
    let arg = lower_simple_value(sig[i + 2])?;
    // Reject Ident( followed by Ident( — that would be nested call, out of thin probe.
    if matches!(sig[i + 2].kind, TokenKind::Identifier(_))
        && i + 3 < sig.len()
        && sig[i + 3].kind == TokenKind::LParen
    {
        return None;
    }
    if sig[i + 3].kind != TokenKind::RParen {
        return None;
    }
    Some((
        Expr::Call {
            span: dummy(),
            callee: Box::new(Expr::Ident {
                span: dummy(),
                name: name.clone(),
            }),
            args: vec![CallArg {
                label: None,
                value: arg,
            }],
        },
        i + 4,
    ))
}

/// Print arg: call-simple | bare simple. Returns `(expr, next_index)`.
fn take_print_arg(sig: &[&GreenToken], i: usize) -> Option<(Expr, usize)> {
    if let Some((expr, next)) = take_call_simple_probe(sig, i) {
        return Some((expr, next));
    }
    if i >= sig.len() {
        return None;
    }
    // Bare Ident must not be start of call (handled above) or optional-chain / member.
    if matches!(sig[i].kind, TokenKind::Identifier(_))
        && i + 1 < sig.len()
        && matches!(
            sig[i + 1].kind,
            TokenKind::LParen | TokenKind::Question | TokenKind::Dot
        )
    {
        return None;
    }
    let expr = lower_simple_value(sig[i])?;
    Some((expr, i + 1))
}

/// One `print ( arg )` [`;`]. Returns next index or None.
fn take_print_stmt(sig: &[&GreenToken], i: usize) -> Option<(Stmt, usize)> {
    if i >= sig.len() {
        return None;
    }
    match &sig[i].kind {
        TokenKind::Identifier(name) if name.as_str() == "print" => {}
        _ => return None,
    }
    if i + 1 >= sig.len() || sig[i + 1].kind != TokenKind::LParen {
        return None;
    }
    let (arg, after_arg) = take_print_arg(sig, i + 2)?;
    if after_arg >= sig.len() || sig[after_arg].kind != TokenKind::RParen {
        return None;
    }
    let mut next = after_arg + 1;
    if next < sig.len() && sig[next].kind == TokenKind::Semicolon {
        next += 1;
    }
    Some((
        Stmt::Print {
            span: dummy(),
            expr: arg,
        },
        next,
    ))
}

/// `let|var <ident> = <simple>` [`;`] — untyped only (reject `: Type`).
fn take_bind_stmt(sig: &[&GreenToken], i: usize, mutable: bool) -> Option<(Stmt, usize)> {
    let kw = if mutable {
        TokenKind::Var
    } else {
        TokenKind::Let
    };
    if i >= sig.len() || sig[i].kind != kw {
        return None;
    }
    if i + 1 >= sig.len() {
        return None;
    }
    let TokenKind::Identifier(name) = &sig[i + 1].kind else {
        return None;
    };
    // Typed bind `let x: T = …` is out of I162 probe face.
    if i + 2 < sig.len() && sig[i + 2].kind == TokenKind::Colon {
        return None;
    }
    if i + 2 >= sig.len() || sig[i + 2].kind != TokenKind::Assign {
        return None;
    }
    if i + 3 >= sig.len() {
        return None;
    }
    // RHS must be bare simple (≠ call).
    if matches!(sig[i + 3].kind, TokenKind::Identifier(_))
        && i + 4 < sig.len()
        && matches!(
            sig[i + 4].kind,
            TokenKind::LParen | TokenKind::Question | TokenKind::Dot
        )
    {
        return None;
    }
    let init = lower_simple_value(sig[i + 3])?;
    let mut next = i + 4;
    if next < sig.len() && sig[next].kind == TokenKind::Semicolon {
        next += 1;
    }
    Some((
        Stmt::VarDecl {
            span: dummy(),
            mutable,
            name: name.clone(),
            ty: None,
            init: Some(init),
        },
        next,
    ))
}

fn take_let_stmt(sig: &[&GreenToken], i: usize) -> Option<(Stmt, usize)> {
    take_bind_stmt(sig, i, false)
}

fn take_var_stmt(sig: &[&GreenToken], i: usize) -> Option<(Stmt, usize)> {
    take_bind_stmt(sig, i, true)
}

/// `return` [`;`] | `return <simple>` [`;`].
fn take_return_stmt(sig: &[&GreenToken], i: usize) -> Option<(Stmt, usize)> {
    if i >= sig.len() || sig[i].kind != TokenKind::Return {
        return None;
    }
    let mut next = i + 1;
    if next >= sig.len()
        || sig[next].kind == TokenKind::RBrace
        || sig[next].kind == TokenKind::Semicolon
    {
        if next < sig.len() && sig[next].kind == TokenKind::Semicolon {
            next += 1;
        }
        return Some((
            Stmt::Return {
                span: dummy(),
                value: None,
            },
            next,
        ));
    }
    // return <simple> — reject call-simple RHS.
    if matches!(sig[next].kind, TokenKind::Identifier(_))
        && next + 1 < sig.len()
        && matches!(
            sig[next + 1].kind,
            TokenKind::LParen | TokenKind::Question | TokenKind::Dot
        )
    {
        return None;
    }
    let value = lower_simple_value(sig[next])?;
    next += 1;
    if next < sig.len() && sig[next].kind == TokenKind::Semicolon {
        next += 1;
    }
    Some((
        Stmt::Return {
            span: dummy(),
            value: Some(value),
        },
        next,
    ))
}

fn take_simple_stmt(sig: &[&GreenToken], i: usize) -> Option<(Stmt, usize)> {
    take_print_stmt(sig, i)
        .or_else(|| take_let_stmt(sig, i))
        .or_else(|| take_var_stmt(sig, i))
        .or_else(|| take_return_stmt(sig, i))
}

/// Lower a CST root on the I160/I162 probe surface into a `Module`.
///
/// Accepted: zero-arg `func main() { 0..N` print | untyped let/var | return `}`.
/// Anything else → `LWR001` (even if `parse_cst_subset` would accept a wider subset).
pub fn lower_cst_subset_probe(root: &CstRoot) -> SyxResult<Module> {
    let sig = significant_green(&root.green.tokens);
    if sig.is_empty() {
        return Err(lwr001("lower probe: empty CST"));
    }

    let mut i = 0usize;
    if sig[i].kind != TokenKind::Func {
        return Err(lwr001(
            "lower probe: expected bare `func main()` (no export/access/attrs)",
        ));
    }
    i += 1;
    let TokenKind::Identifier(fname) = &sig
        .get(i)
        .ok_or_else(|| lwr001("lower probe: missing func name"))?
        .kind
    else {
        return Err(lwr001("lower probe: expected Identifier after func"));
    };
    if fname.as_str() != "main" {
        return Err(lwr001("lower probe: function name must be `main`"));
    }
    i += 1;
    if i >= sig.len() || sig[i].kind != TokenKind::LParen {
        return Err(lwr001("lower probe: expected `()` after main"));
    }
    i += 1;
    if i >= sig.len() || sig[i].kind != TokenKind::RParen {
        return Err(lwr001("lower probe: params not allowed (zero-arg only)"));
    }
    i += 1;
    if i >= sig.len() || sig[i].kind != TokenKind::LBrace {
        return Err(lwr001(
            "lower probe: expected `{` (no return/throws/requires/where)",
        ));
    }
    i += 1;

    let mut body = Vec::new();
    while i < sig.len() && sig[i].kind != TokenKind::RBrace {
        let (stmt, next) = take_simple_stmt(&sig, i).ok_or_else(|| {
            lwr001(
                "lower probe: body allows print | untyped let/var | return [<simple>]",
            )
        })?;
        body.push(stmt);
        i = next;
    }
    if i >= sig.len() || sig[i].kind != TokenKind::RBrace {
        return Err(lwr001("lower probe: missing closing `}`"));
    }
    i += 1;
    if i != sig.len() {
        return Err(lwr001(
            "lower probe: trailing tokens after `func main` (single-func only)",
        ));
    }

    Ok(Module {
        name: SmolStr::new("main"),
        decls: vec![Decl::Function {
            span: dummy(),
            access: AccessLevel::Internal,
            attrs: vec![],
            name: SmolStr::new("main"),
            type_params: vec![],
            params: vec![],
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body,
            is_async: false,
            is_override: false,
            is_final: false,
        }],
    })
}

/// Parse CST subset then run the lower probe.
pub fn lower_cst_subset_probe_from_source(source: &SourceFile) -> SyxResult<Module> {
    let root = parse_cst_subset(source)?;
    lower_cst_subset_probe(&root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;

    #[test]
    fn probe_print_int_ok() {
        let file = SourceFile::new("t.sy", "func main() {\n    print(1)\n}\n");
        let m = lower_cst_subset_probe_from_source(&file).expect("lower");
        assert_eq!(m.decls.len(), 1);
        match &m.decls[0] {
            Decl::Function { name, body, .. } => {
                assert_eq!(name.as_str(), "main");
                assert_eq!(body.len(), 1);
                assert!(matches!(
                    &body[0],
                    Stmt::Print {
                        expr: Expr::Literal {
                            value: Literal::Integer { value: 1, .. },
                            ..
                        },
                        ..
                    }
                ));
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn probe_let_print_ok() {
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n    let x = 1\n    print(x)\n}\n",
        );
        let m = lower_cst_subset_probe_from_source(&file).expect("lower");
        match &m.decls[0] {
            Decl::Function { body, .. } => {
                assert_eq!(body.len(), 2);
                assert!(matches!(
                    &body[0],
                    Stmt::VarDecl {
                        mutable: false,
                        name,
                        ty: None,
                        ..
                    } if name.as_str() == "x"
                ));
                assert!(matches!(&body[1], Stmt::Print { .. }));
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn probe_var_return_ok() {
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n    var y = true\n    return y\n}\n",
        );
        let m = lower_cst_subset_probe_from_source(&file).expect("lower");
        match &m.decls[0] {
            Decl::Function { body, .. } => {
                assert_eq!(body.len(), 2);
                assert!(matches!(
                    &body[0],
                    Stmt::VarDecl { mutable: true, .. }
                ));
                assert!(matches!(
                    &body[1],
                    Stmt::Return {
                        value: Some(Expr::Ident { .. }),
                        ..
                    }
                ));
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn probe_rejects_while_with_lwr001() {
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n    while x {\n        print(1)\n    }\n}\n",
        );
        let err = lower_cst_subset_probe_from_source(&file).expect_err("while must LWR001");
        assert_eq!(err.code.as_deref(), Some("LWR001"));
    }

    #[test]
    fn probe_rejects_typed_let_with_lwr001() {
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n    let x: Int = 1\n    print(x)\n}\n",
        );
        let err = lower_cst_subset_probe_from_source(&file).expect_err("typed let must LWR001");
        assert_eq!(err.code.as_deref(), Some("LWR001"));
    }
}
