use smol_str::SmolStr;
use syx_ast::{BinOp, CompoundAssignOp, Decl, Expr, Literal, Stmt};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};

use crate::warnings::push_macro_warning;

    fn macro_message_arg(
        args: Vec<Expr>,
        span: syx_diagnostics::Span,
        kind: &str,
    ) -> SyxResult<String> {
        let expr = args.into_iter().next().ok_or_else(|| {
            SyxDiagnostic::error(format!("#{kind} requires a message argument"))
        })?;
        match expr {
            Expr::Literal {
                value: Literal::String(s),
                ..
            } => Ok(s),
            _ => Err(SyxDiagnostic::error(format!(
                "#{kind} message must be a string literal"
            ))
            .with_span(
                &SourceFile::new("macro.sy", ""),
                span,
                "message",
            )),
        }
    }

    fn eval_const_expr(expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::Literal { span, value } => Some(Expr::Literal {
                span: *span,
                value: value.clone(),
            }),
            Expr::Unary {
                span,
                op: syx_ast::UnaryOp::Neg,
                expr,
            } => {
                if let Some(Expr::Literal {
                    value: Literal::Integer { value, suffix },
                    ..
                }) = eval_const_expr(expr)
                {
                    Some(Expr::Literal {
                        span: *span,
                        value: Literal::Integer {
                            value: -value,
                            suffix: suffix.clone(),
                        },
                    })
                } else {
                    None
                }
            }
            Expr::Unary {
                span,
                op: syx_ast::UnaryOp::Not,
                expr,
            } => {
                if let Some(Expr::Literal {
                    value: Literal::Bool(b),
                    ..
                }) = eval_const_expr(expr)
                {
                    Some(Expr::Literal {
                        span: *span,
                        value: Literal::Bool(!b),
                    })
                } else {
                    None
                }
            }
            Expr::Binary { span, left, op, right } => {
                let l = eval_const_expr(left)?;
                let r = eval_const_expr(right)?;
                match (&l, &r) {
                    (
                        Expr::Literal {
                            value: Literal::Integer { value: a, .. },
                            ..
                        },
                        Expr::Literal {
                            value: Literal::Integer { value: b, .. },
                            ..
                        },
                    ) => {
                        let value = match op {
                            BinOp::Add => a + b,
                            BinOp::Sub => a - b,
                            BinOp::Mul => a * b,
                            BinOp::Div if *b != 0 => a / b,
                            BinOp::Eq => {
                                return Some(Expr::Literal {
                                    span: *span,
                                    value: Literal::Bool(a == b),
                                })
                            }
                            BinOp::Lt => {
                                return Some(Expr::Literal {
                                    span: *span,
                                    value: Literal::Bool(a < b),
                                })
                            }
                            _ => return None,
                        };
                        Some(Expr::Literal {
                            span: *span,
                            value: Literal::Integer {
                                value,
                                suffix: None,
                            },
                        })
                    }
                    _ => None,
                }
            }
            Expr::Call { span, callee, args } => {
                if matches!(callee.as_ref(), Expr::Ident { name, .. } if name.as_str() == "print") {
                    let folded_args: Option<Vec<syx_ast::CallArg>> = args
                        .iter()
                        .map(|a| {
                            eval_const_expr(&a.value).map(|e| syx_ast::CallArg {
                                label: a.label.clone(),
                                value: e,
                            })
                        })
                        .collect();
                    folded_args.map(|args| Expr::Call {
                        span: *span,
                        callee: callee.clone(),
                        args,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn macro_arg_to_stmt(span: syx_diagnostics::Span, e: Expr) -> Stmt {
        if let Expr::Call {
            callee,
            args: print_args,
            ..
        } = &e
        {
            if matches!(callee.as_ref(), Expr::Ident { name, .. } if name.as_str() == "print")
            {
                return Stmt::Print {
                    span,
                    expr: print_args
                        .first()
                        .map(|a| a.value.clone())
                        .unwrap_or(Expr::Literal {
                            span,
                            value: Literal::String(String::new()),
                        }),
                };
            }
        }
        Stmt::Expr { span, expr: e }
    }

    fn stringify_repr(expr: &Expr) -> String {
        match expr {
            Expr::Ident { name, .. } => name.to_string(),
            Expr::Literal {
                value: Literal::Integer { value, .. },
                ..
            } => value.to_string(),
            Expr::Literal {
                value: Literal::String(s),
                ..
            } => format!("\"{s}\""),
            Expr::Literal {
                value: Literal::Bool(b),
                ..
            } => b.to_string(),
            Expr::Binary { left, op, right, .. } => format!(
                "({} {} {})",
                stringify_repr(left),
                match op {
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Mul => "*",
                    BinOp::Div => "/",
                    BinOp::Mod => "%",
                    BinOp::Eq => "==",
                    BinOp::NotEq => "!=",
                    BinOp::Lt => "<",
                    BinOp::LtEq => "<=",
                    BinOp::Gt => ">",
                    BinOp::GtEq => ">=",
                    BinOp::And => "&&",
                    BinOp::Or => "||",
                    _ => "?",
                },
                stringify_repr(right)
            ),
            Expr::Unary { op, expr, .. } => format!(
                "{}{}",
                match op {
                    syx_ast::UnaryOp::Neg => "-",
                    syx_ast::UnaryOp::Not => "!",
                    syx_ast::UnaryOp::BitNot => "~",
                    syx_ast::UnaryOp::Try => "?",
                },
                stringify_repr(expr)
            ),
            Expr::Group { expr, .. } => stringify_repr(expr),
            _ => "?".into(),
        }
    }

    fn expand_stringify_in_expr(expr: &mut Expr) -> SyxResult<()> {
        match expr {
            Expr::MacroInvoke { span, name, args } if name.as_str() == "stringify" => {
                if args.len() != 1 {
                    return Err(SyxDiagnostic::error("#stringify requires exactly one argument"));
                }
                let text = stringify_repr(&args[0]);
                *expr = Expr::Literal {
                    span: *span,
                    value: Literal::String(text),
                };
                Ok(())
            }
            Expr::Binary { left, right, .. } => {
                expand_stringify_in_expr(left)?;
                expand_stringify_in_expr(right)
            }
            Expr::Unary { expr: inner, .. } | Expr::Group { expr: inner, .. } => {
                expand_stringify_in_expr(inner)
            }
            Expr::Call { callee, args, .. } => {
                expand_stringify_in_expr(callee)?;
                for a in args {
                    expand_stringify_in_expr(&mut a.value)?;
                }
                Ok(())
            }
            Expr::Member { object, .. } => expand_stringify_in_expr(object),
            Expr::Index { object, index, .. } => {
                expand_stringify_in_expr(object)?;
                expand_stringify_in_expr(index)
            }
            _ => Ok(()),
        }
    }

    fn expand_stringify_in_stmt(stmt: &mut Stmt) -> SyxResult<()> {
        match stmt {
            Stmt::Print { expr, .. } | Stmt::Expr { expr, .. } => expand_stringify_in_expr(expr),
            Stmt::VarDecl { init: Some(expr), .. } => expand_stringify_in_expr(expr),
            Stmt::Return { value: Some(expr), .. } => expand_stringify_in_expr(expr),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                expand_stringify_in_expr(condition)?;
                for s in then_branch.iter_mut() {
                    expand_stringify_in_stmt(s)?;
                }
                if let Some(eb) = else_branch {
                    for s in eb.iter_mut() {
                        expand_stringify_in_stmt(s)?;
                    }
                }
                Ok(())
            }
            Stmt::Block { stmts, .. } | Stmt::While { body: stmts, .. } => {
                for s in stmts.iter_mut() {
                    expand_stringify_in_stmt(s)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn expand_stmts(stmts: &mut Vec<Stmt>) -> SyxResult<()> {
        let mut out = Vec::new();
        for mut stmt in std::mem::take(stmts) {
            expand_stringify_in_stmt(&mut stmt)?;
            match stmt {
                Stmt::MacroInvoke { span, name, args, .. } => {
                    if name.as_str() == "assert" {
                        let cond = args.into_iter().next().ok_or_else(|| {
                            SyxDiagnostic::error("#assert requires one argument")
                        })?;
                        let fail = Expr::Literal {
                            span,
                            value: Literal::String("assertion failed".into()),
                        };
                        out.push(Stmt::If {
                            span,
                            condition: Expr::Unary {
                                span,
                                op: syx_ast::UnaryOp::Not,
                                expr: Box::new(cond),
                            },
                            then_branch: vec![Stmt::Throw { span, expr: fail }],
                            else_branch: None,
                        });
                    } else if name.as_str() == "if" {
                        let mut arg_iter = args.into_iter();
                        let cond = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#if requires condition and body statements")
                        })?;
                        let then_stmts: Vec<Stmt> = arg_iter
                            .map(|e| macro_arg_to_stmt(span, e))
                            .collect();
                        if then_stmts.is_empty() {
                            return Err(SyxDiagnostic::error(
                                "#if requires at least one body statement after condition",
                            ));
                        }
                        out.push(Stmt::If {
                            span,
                            condition: cond,
                            then_branch: then_stmts,
                            else_branch: None,
                        });
                    } else if name.as_str() == "assert_eq" {
                        let mut arg_iter = args.into_iter();
                        let left = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#assert_eq requires two arguments")
                        })?;
                        let right = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#assert_eq requires two arguments")
                        })?;
                        let cond = Expr::Binary {
                            span,
                            left: Box::new(left),
                            op: BinOp::Eq,
                            right: Box::new(right),
                        };
                        let fail = Expr::Literal {
                            span,
                            value: Literal::String("assert_eq failed".into()),
                        };
                        out.push(Stmt::If {
                            span,
                            condition: Expr::Unary {
                                span,
                                op: syx_ast::UnaryOp::Not,
                                expr: Box::new(cond),
                            },
                            then_branch: vec![Stmt::Throw { span, expr: fail }],
                            else_branch: None,
                        });
                    } else if name.as_str() == "assert_ne" {
                        let mut arg_iter = args.into_iter();
                        let left = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#assert_ne requires two arguments")
                        })?;
                        let right = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#assert_ne requires two arguments")
                        })?;
                        let cond = Expr::Binary {
                            span,
                            left: Box::new(left),
                            op: BinOp::Eq,
                            right: Box::new(right),
                        };
                        let fail = Expr::Literal {
                            span,
                            value: Literal::String("assert_ne failed".into()),
                        };
                        out.push(Stmt::If {
                            span,
                            condition: cond,
                            then_branch: vec![Stmt::Throw { span, expr: fail }],
                            else_branch: None,
                        });
                    } else if name.as_str() == "debug_assert" {
                        let expr = args.into_iter().next().ok_or_else(|| {
                            SyxDiagnostic::error("#debug_assert requires one argument")
                        })?;
                        let fail = Expr::Literal {
                            span,
                            value: Literal::String("debug_assert failed".into()),
                        };
                        out.push(Stmt::If {
                            span,
                            condition: Expr::Unary {
                                span,
                                op: syx_ast::UnaryOp::Not,
                                expr: Box::new(expr),
                            },
                            then_branch: vec![Stmt::Throw { span, expr: fail }],
                            else_branch: None,
                        });
                    } else if name.as_str() == "line" || name.as_str() == "file" {
                        // Phase 16: freestanding source location hints (no runtime effect).
                    } else if name.as_str() == "while" {
                        let mut arg_iter = args.into_iter();
                        let cond = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#while requires condition and body statements")
                        })?;
                        let body_stmts: Vec<Stmt> = arg_iter
                            .map(|e| macro_arg_to_stmt(span, e))
                            .collect();
                        if body_stmts.is_empty() {
                            return Err(SyxDiagnostic::error(
                                "#while requires at least one body statement after condition",
                            ));
                        }
                        out.push(Stmt::While {
                            span,
                            condition: cond,
                            body: body_stmts,
                            label: None,
                        });
                    } else if name.as_str() == "compileTime" {
                        if args.is_empty() {
                            return Err(SyxDiagnostic::error(
                                "#compileTime requires a constant expression or block body",
                            ));
                        }
                        for arg in args {
                            let stmt = macro_arg_to_stmt(span, arg);
                            match stmt {
                                Stmt::Print { expr, .. } => {
                                    let folded = eval_const_expr(&expr).ok_or_else(|| {
                                        SyxDiagnostic::error(
                                            "#compileTime block statement must be compile-time constant",
                                        )
                                    })?;
                                    if let Expr::Call {
                                        callee,
                                        args: print_args,
                                        ..
                                    } = &folded
                                    {
                                        if matches!(
                                            callee.as_ref(),
                                            Expr::Ident { name, .. } if name.as_str() == "print"
                                        ) {
                                            out.push(Stmt::Print {
                                                span,
                                                expr: print_args
                                                    .first()
                                                    .map(|a| a.value.clone())
                                                    .unwrap_or(folded),
                                            });
                                            continue;
                                        }
                                    }
                                    out.push(Stmt::Expr { span, expr: folded });
                                }
                                Stmt::Expr { expr, .. } => {
                                    let folded = eval_const_expr(&expr).ok_or_else(|| {
                                        SyxDiagnostic::error(
                                            "#compileTime expression must be a compile-time constant",
                                        )
                                    })?;
                                    out.push(Stmt::Expr { span, expr: folded });
                                }
                                _ => {
                                    return Err(SyxDiagnostic::error(
                                        "#compileTime block supports only constant expressions and print",
                                    ));
                                }
                            }
                        }
                    } else if name.as_str() == "line" {
                        out.push(Stmt::Expr {
                            span,
                            expr: Expr::Literal {
                                span,
                                value: Literal::Integer {
                                    value: 1,
                                    suffix: None,
                                },
                            },
                        });
                    } else if name.as_str() == "file" {
                        out.push(Stmt::Expr {
                            span,
                            expr: Expr::Literal {
                                span,
                                value: Literal::String("syx.sy".into()),
                            },
                        });
                    } else if name.as_str() == "for" {
                        let mut arg_iter = args.into_iter();
                        let count = arg_iter.next().ok_or_else(|| {
                            SyxDiagnostic::error("#for requires count and body statements")
                        })?;
                        let body_stmts: Vec<Stmt> = arg_iter
                            .map(|e| macro_arg_to_stmt(span, e))
                            .collect();
                        if body_stmts.is_empty() {
                            return Err(SyxDiagnostic::error(
                                "#for requires at least one body statement after count",
                            ));
                        }
                        let counter = SmolStr::new("__syx_for_i");
                        out.push(Stmt::VarDecl {
                            span,
                            mutable: true,
                            name: counter.clone(),
                            ty: None,
                            init: Some(Expr::Literal {
                                span,
                                value: Literal::Integer {
                                    value: 0,
                                    suffix: None,
                                },
                            }),
                        });
                        let condition = Expr::Binary {
                            span,
                            left: Box::new(Expr::Ident {
                                span,
                                name: counter.clone(),
                            }),
                            op: BinOp::Lt,
                            right: Box::new(count),
                        };
                        let mut while_body = body_stmts;
                        while_body.push(Stmt::Expr {
                            span,
                            expr: Expr::CompoundAssign {
                                span,
                                name: counter,
                                op: CompoundAssignOp::Add,
                                value: Box::new(Expr::Literal {
                                    span,
                                    value: Literal::Integer {
                                        value: 1,
                                        suffix: None,
                                    },
                                }),
                            },
                        });
                        out.push(Stmt::While {
                            span,
                            condition,
                            body: while_body,
                            label: None,
                        });
                    } else if name.as_str() == "warning" {
                        let msg = macro_message_arg(args, span, "warning")?;
                        push_macro_warning(
                            SyxDiagnostic::warning(msg).with_code("W001"),
                        );
                    } else if name.as_str() == "error" {
                        let msg = macro_message_arg(args, span, "error")?;
                        return Err(SyxDiagnostic::error(msg).with_code("E001"));
                    } else {
                        out.push(Stmt::MacroInvoke {
                            span,
                            name: name.clone(),
                            args: args.clone(),
                        });
                    }
                }
                Stmt::Block { span, mut stmts, .. } => {
                    expand_stmts(&mut stmts)?;
                    out.push(Stmt::Block { span, stmts });
                }
                Stmt::If {
                    span,
                    condition,
                    mut then_branch,
                    else_branch,
                    ..
                } => {
                    expand_stmts(&mut then_branch)?;
                    let else_branch = if let Some(mut branch) = else_branch {
                        expand_stmts(&mut branch)?;
                        Some(branch)
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
                    label,
                    ..
                } => {
                    expand_stmts(&mut body)?;
                    out.push(Stmt::While {
                        span,
                        condition,
                        body,
                        label,
                    });
                }
                Stmt::ForIn {
                    span,
                    name,
                    iterable,
                    mut body,
                    is_async_iter,
                    label,
                    ..
                } => {
                    expand_stmts(&mut body)?;
                    out.push(Stmt::ForIn {
                        span,
                        name,
                        iterable,
                        body,
                        is_async_iter,
                        label,
                    });
                }
                Stmt::Try {
                    span,
                    mut body,
                    catch_name,
                    catch_type,
                    mut catch_body,
                    ..
                } => {
                    expand_stmts(&mut body)?;
                    expand_stmts(&mut catch_body)?;
                    out.push(Stmt::Try {
                        span,
                        body,
                        catch_name,
                        catch_type,
                        catch_body,
                    });
                }
                Stmt::Switch {
                    span,
                    subject,
                    mut cases,
                    mut default,
                    ..
                } => {
                    for case in &mut cases {
                        expand_stmts(&mut case.body)?;
                    }
                    if let Some(ref mut body) = default {
                        expand_stmts(body)?;
                    }
                    out.push(Stmt::Switch {
                        span,
                        subject,
                        cases,
                        default,
                    });
                }
                other => out.push(other),
            }
        }
        *stmts = out;
        Ok(())
    }

    pub(crate) fn expand_decls(decls: &mut [Decl]) -> SyxResult<()> {
        for decl in decls.iter_mut() {
            match decl {
                syx_ast::Decl::Function { body, .. }
                | syx_ast::Decl::Init { body, .. }
                | syx_ast::Decl::Deinit { body, .. } => {
                    expand_stmts(body)?;
                }
                syx_ast::Decl::Class { members, .. } | syx_ast::Decl::Actor { members, .. } => {
                    expand_decls(members)?;
                }
                syx_ast::Decl::Extension { methods, .. } => {
                    expand_decls(methods)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
