//! Phase 56 Macro M5: fixed-point expansion driver.

use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::context::MacroContext;
use crate::external;
use crate::registry;

const MAX_ROUNDS: u32 = 16;

pub fn expand_to_fixed_point(mut module: Module) -> SyxResult<Module> {
    let mut ctx = MacroContext::new(module.name.as_str());
    external::collect_external_macros(&module);

    loop {
        let before = count_macro_invokes(&module);
        ctx.trace(format!("round start invokes={before}"));

        registry::expand_one_round(&mut module, &ctx)?;

        let after = count_macro_invokes(&module);
        ctx.trace(format!("round end invokes={after}"));
        ctx.round += 1;

        if after == 0 {
            return Ok(module);
        }
        if after == before {
            return Err(SyxDiagnostic::error("cyclic or stuck macro expansion").with_code("E002"));
        }
        if ctx.round >= MAX_ROUNDS {
            return Err(SyxDiagnostic::error("macro expansion depth exceeded").with_code("E002"));
        }
    }
}

pub fn count_macro_invokes(module: &Module) -> u32 {
    let mut n = 0u32;
    for decl in &module.decls {
        n += count_macro_invokes_decl(decl);
    }
    n
}

fn count_macro_invokes_decl(decl: &Decl) -> u32 {
    match decl {
        Decl::Function { body, .. }
        | Decl::Init { body, .. }
        | Decl::Deinit { body, .. } => count_macro_invokes_stmts(body),
        Decl::Class { members, .. } | Decl::Actor { members, .. } => members
            .iter()
            .map(count_macro_invokes_decl)
            .sum(),
        Decl::Extension { methods, .. } => methods.iter().map(count_macro_invokes_decl).sum(),
        Decl::MacroSiteInvoke { .. } => 1,
        _ => 0,
    }
}

fn count_macro_invokes_stmts(stmts: &[Stmt]) -> u32 {
    let mut n = 0u32;
    for stmt in stmts {
        match stmt {
            Stmt::MacroInvoke { .. } => n += 1,
            Stmt::Block { stmts, .. } => n += count_macro_invokes_stmts(stmts),
            Stmt::If {
                then_branch,
                else_branch,
                condition,
                ..
            } => {
                n += count_macro_invokes_expr(condition);
                n += count_macro_invokes_stmts(then_branch);
                if let Some(eb) = else_branch {
                    n += count_macro_invokes_stmts(eb);
                }
            }
            Stmt::While { body, condition, .. } => {
                n += count_macro_invokes_expr(condition);
                n += count_macro_invokes_stmts(body);
            }
            Stmt::ForIn { body, iterable, .. } => {
                n += count_macro_invokes_expr(iterable);
                n += count_macro_invokes_stmts(body);
            }
            Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => n += count_macro_invokes_expr(expr),
            Stmt::Return { value: Some(expr), .. } => n += count_macro_invokes_expr(expr),
            Stmt::VarDecl { init: Some(expr), .. } => n += count_macro_invokes_expr(expr),
            Stmt::Throw { expr, .. } => n += count_macro_invokes_expr(expr),
            _ => {}
        }
    }
    n
}

fn count_macro_invokes_expr(expr: &Expr) -> u32 {
    match expr {
        Expr::MacroInvoke { .. } => 1,
        Expr::Binary { left, right, .. } => {
            count_macro_invokes_expr(left) + count_macro_invokes_expr(right)
        }
        Expr::Unary { expr, .. } => count_macro_invokes_expr(expr),
        Expr::Call { callee, args, .. } => {
            count_macro_invokes_expr(callee)
                + args.iter().map(|a| count_macro_invokes_expr(&a.value)).sum::<u32>()
        }
        Expr::Member { object, .. } | Expr::Group { expr: object, .. } => {
            count_macro_invokes_expr(object)
        }
        Expr::Index { object, index, .. } => {
            count_macro_invokes_expr(object) + count_macro_invokes_expr(index)
        }
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => {
            count_macro_invokes_expr(value)
        }
        _ => 0,
    }
}
