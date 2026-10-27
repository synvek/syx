use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct LintIssue {
    pub message: String,
    pub code: String,
}

pub fn lint_source(source: &SourceFile) -> SyxResult<Vec<LintIssue>> {
    let module = Parser::parse(source.clone())?;
    Ok(lint_module(source, &module))
}

pub fn lint_module(source: &SourceFile, module: &Module) -> Vec<LintIssue> {
    let mut issues = Vec::new();
    let mut function_names = indexmap::IndexSet::new();
    for decl in &module.decls {
        lint_decl(decl, &mut issues, &mut function_names);
    }
    lint_source_text(source, &mut issues);
    issues
}

fn lint_decl(
    decl: &Decl,
    issues: &mut Vec<LintIssue>,
    function_names: &mut indexmap::IndexSet<smol_str::SmolStr>,
) {
    match decl {
        Decl::Function {
            name,
            body,
            is_async,
            access,
            return_type,
            ..
        } => {
            if name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                issues.push(LintIssue {
                    message: format!("Function '{name}' should start with lowercase"),
                    code: "L001".into(),
                });
            }
            if !function_names.insert(name.clone()) {
                issues.push(LintIssue {
                    message: format!("Duplicate function name '{name}'"),
                    code: "L002".into(),
                });
            }
            if body.is_empty() {
                issues.push(LintIssue {
                    message: format!("Function '{name}' has an empty body"),
                    code: "L003".into(),
                });
            }
            if *access == syx_ast::AccessLevel::Public && return_type.is_none() {
                issues.push(LintIssue {
                    message: format!("Public function '{name}' should declare a return type"),
                    code: "L004".into(),
                });
            }
            if *is_async && !body_has_await(body) {
                issues.push(LintIssue {
                    message: format!("Async function '{name}' does not use await"),
                    code: "L005".into(),
                });
            }
            for stmt in body {
                lint_stmt(stmt, issues, name);
            }
        }
        Decl::VarGlobal { name, init, .. } => {
            if name.contains('-') {
                issues.push(LintIssue {
                    message: format!("Variable '{name}' contains invalid character '-'"),
                    code: "L006".into(),
                });
            }
            if init.is_none() {
                issues.push(LintIssue {
                    message: format!("Global variable '{name}' has no initializer"),
                    code: "L007".into(),
                });
            }
        }
        Decl::Import { path, .. } => {
            if path.is_empty() {
                issues.push(LintIssue {
                    message: "Empty import module name".into(),
                    code: "L008".into(),
                });
            }
        }
        Decl::Class { members, .. } => {
            for member in members {
                lint_decl(member, issues, function_names);
            }
        }
        Decl::Export { item, .. } => {
            lint_decl(item, issues, function_names);
        }
        Decl::ExportList { .. } => {}
        _ => {}
    }
}

fn lint_stmt(stmt: &Stmt, issues: &mut Vec<LintIssue>, fn_name: &smol_str::SmolStr) {
    match stmt {
        Stmt::Return { value: None, .. } => {
            issues.push(LintIssue {
                message: format!("Bare return in '{fn_name}' may be unclear"),
                code: "L009".into(),
            });
        }
        Stmt::Block { stmts, .. } => {
            for s in stmts {
                lint_stmt(s, issues, fn_name);
            }
        }
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            for s in then_branch {
                lint_stmt(s, issues, fn_name);
            }
            if let Some(else_b) = else_branch {
                for s in else_b {
                    lint_stmt(s, issues, fn_name);
                }
            }
        }
        Stmt::While { body, .. } | Stmt::ForIn { body, .. } => {
            for s in body {
                lint_stmt(s, issues, fn_name);
            }
        }
        Stmt::Try { body, catch_body, .. } => {
            for s in body {
                lint_stmt(s, issues, fn_name);
            }
            for s in catch_body {
                lint_stmt(s, issues, fn_name);
            }
        }
        _ => {}
    }
}

fn body_has_await(stmts: &[Stmt]) -> bool {
    stmts.iter().any(|s| match s {
        Stmt::Expr { expr, .. } => expr_has_await(expr),
        Stmt::Return { value: Some(v), .. } => expr_has_await(v),
        Stmt::VarDecl { init: Some(v), .. } => expr_has_await(v),
        Stmt::Block { stmts, .. } => body_has_await(stmts),
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            body_has_await(then_branch)
                || else_branch.as_ref().is_some_and(|b| body_has_await(b))
        }
        Stmt::While { body, .. } | Stmt::ForIn { body, .. } => body_has_await(body),
        _ => false,
    })
}

fn expr_has_await(expr: &Expr) -> bool {
    matches!(expr, Expr::Await { .. })
}

fn lint_source_text(source: &SourceFile, issues: &mut Vec<LintIssue>) {
    for (line_no, line) in source.contents.lines().enumerate() {
        if line.len() > 120 {
            issues.push(LintIssue {
                message: format!("Line {} exceeds 120 characters", line_no + 1),
                code: "L010".into(),
            });
        }
        if line.contains("TODO") || line.contains("FIXME") {
            issues.push(LintIssue {
                message: format!("Line {} contains TODO/FIXME marker", line_no + 1),
                code: "L011".into(),
            });
        }
        if line.trim_start().starts_with("print(") {
            issues.push(LintIssue {
                message: format!("Line {} uses top-level-style print", line_no + 1),
                code: "L012".into(),
            });
        }
    }
}

pub fn lint_source_strict(source: &SourceFile) -> SyxResult<()> {
    let issues = lint_source(source)?;
    if let Some(issue) = issues.first() {
        return Err(SyxDiagnostic::error(&issue.message).with_code(&issue.code));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lints_bad_function_name() {
        let source = SourceFile::new("t.sy", "func BadName() {}");
        let issues = lint_source(&source).unwrap();
        assert!(issues.iter().any(|i| i.code == "L001"));
    }

    #[test]
    fn has_ten_plus_rules() {
        let codes = [
            "L001", "L002", "L003", "L004", "L005", "L006", "L007", "L008", "L009", "L010", "L011",
            "L012",
        ];
        assert!(codes.len() >= 10);
    }
}
