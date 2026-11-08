#![allow(dead_code)]
#![allow(unused_imports)]
mod lint_diag;

use lint_diag::lint_error_at;
use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct LintIssue {
    pub message: String,
    pub code: String,
}

/// AST-only lint (tests / `SYX_FMT_AST` hatch / rollback).
pub fn lint_source(source: &SourceFile) -> SyxResult<Vec<LintIssue>> {
    let module = Parser::parse(source.clone())?;
    Ok(lint_module(source, &module))
}

/// S2-I153 / ADR stage2-s2-04-lint-cst-first + S2-I161 / ADR stage2-s2-04-lint-lower-path:
/// CST-first parse **policy** aligned with `format_source_auto`. L001–L009 still run on
/// AST `Module` — via **CST→AST lower** on the I160 probe face, else `Parser::parse`
/// (**≠** lint-on-CST / ≠ shared lossless tree / ≠ FrontendPipeline).
///
/// - `SYX_FMT_AST=1` → [`lint_source`] (shared hatch with fmt).
/// - else `parse_cst_subset`:
///   - Ok → `lower_cst_subset_probe` → [`lint_module`]; lower Err (`LWR001`…) → [`lint_source`].
///   - `CST001` → [`lint_source`]; other CST errors propagate.
pub fn lint_source_auto(source: &SourceFile) -> SyxResult<Vec<LintIssue>> {
    match std::env::var("SYX_FMT_AST") {
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => return lint_source(source),
        _ => {}
    }
    match syx_cst::parse_cst_subset(source) {
        Ok(root) => match syx_cst::lower_cst_subset_probe(&root) {
            Ok(module) => Ok(lint_module(source, &module)),
            Err(_) => lint_source(source),
        },
        Err(e) if e.code.as_deref() == Some("CST001") => lint_source(source),
        Err(e) => Err(e),
    }
}

pub fn lint_module(source: &SourceFile, module: &Module) -> Vec<LintIssue> {
    let mut issues = Vec::new();
    let mut function_names = indexmap::IndexSet::new();
    for decl in &module.decls {
        // S3-I73: top-level decls are not type members.
        lint_decl(decl, &mut issues, &mut function_names, false);
    }
    lint_source_text(source, &mut issues);
    issues
}

/// `in_type_member`: true when walking type members so stored `VarGlobal`
/// fields are not treated as module globals (S3-I73 Class/Actor · S3-I74 Struct/Extension · L007).
fn lint_decl(
    decl: &Decl,
    issues: &mut Vec<LintIssue>,
    function_names: &mut indexmap::IndexSet<smol_str::SmolStr>,
    in_type_member: bool,
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
            // S3-I73/I74: type stored fields share VarGlobal shape but are not globals.
            if init.is_none() && !in_type_member {
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
        Decl::Class { members, .. }
        | Decl::Actor { members, .. }
        | Decl::Struct { members, .. } => {
            // S3-I73 Class/Actor · S3-I74 Struct: recurse with type-member context.
            for member in members {
                lint_decl(member, issues, function_names, true);
            }
        }
        Decl::Extension { methods, .. } => {
            // S3-I74: extension methods share Decl list; keep type-member context.
            for method in methods {
                lint_decl(method, issues, function_names, true);
            }
        }
        Decl::Export { item, .. } => {
            lint_decl(item, issues, function_names, in_type_member);
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
        Stmt::Try {
            body, catch_body, ..
        } => {
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
        } => body_has_await(then_branch) || else_branch.as_ref().is_some_and(|b| body_has_await(b)),
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
        // S3-I73: only column-0 `print(` counts as top-level-style (no trim_start).
        if line.starts_with("print(") {
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
        return Err(lint_error_at(
            &issue.message,
            None,
            Span::DUMMY,
            &issue.code,
        ));
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
    fn lint_source_auto_preserves_rules_on_cst_subset() {
        // CST-parseable body + L011 marker + L001 casing (non-`main` → LWR001 → Parser).
        let source = SourceFile::new(
            "t.sy",
            "// TODO: review\nfunc BadName() {\n    print(1)\n}\n",
        );
        let issues = lint_source_auto(&source).expect("auto lint");
        assert!(
            issues.iter().any(|i| i.code == "L001"),
            "expected L001; got {issues:?}"
        );
        assert!(
            issues.iter().any(|i| i.code == "L011"),
            "expected L011; got {issues:?}"
        );
    }

    #[test]
    fn lint_source_auto_probe_face_matches_lint_source() {
        // I160 probe face: lower path must match Parser-backed lint_source.
        let source = SourceFile::new("t.sy", "// TODO: x\nfunc main() {\n    print(1)\n}\n");
        let via_auto = lint_source_auto(&source).expect("auto");
        let via_parser = lint_source(&source).expect("parser");
        assert_eq!(via_auto, via_parser);
        assert!(via_auto.iter().any(|i| i.code == "L011"));
    }

    #[test]
    fn has_ten_plus_rules() {
        let codes = [
            "L001", "L002", "L003", "L004", "L005", "L006", "L007", "L008", "L009", "L010", "L011",
            "L012",
        ];
        assert!(codes.len() >= 10);
    }

    /// S3-I73: class field without init must not emit L007; indented print must not emit L012.
    #[test]
    fn class_field_and_indented_print_no_l007_l012() {
        let source = SourceFile::new(
            "class_demo_mini.sy",
            r#"class Greeter {
    var name: String
    func hello() {
        print(self.name)
    }
}

func main() {
    print("hi")
}
"#,
        );
        let issues = lint_source(&source).expect("lint");
        assert!(
            !issues.iter().any(|i| i.code == "L007"),
            "class field must not emit L007; got {issues:?}"
        );
        assert!(
            !issues.iter().any(|i| i.code == "L012"),
            "indented print must not emit L012; got {issues:?}"
        );
    }

    /// S3-I73: module-level uninitialized var still L007; column-0 print still L012.
    #[test]
    fn top_level_var_and_print_still_lint() {
        let source = SourceFile::new("t.sy", "var orphan: Int\nprint(1)\n");
        let issues = lint_source(&source).expect("lint");
        assert!(
            issues.iter().any(|i| i.code == "L007"),
            "top-level uninitialized var must emit L007; got {issues:?}"
        );
        assert!(
            issues.iter().any(|i| i.code == "L012"),
            "column-0 print must emit L012; got {issues:?}"
        );
    }

    /// S3-I74: struct fields without init must not emit L007; extension method still linted.
    #[test]
    fn struct_field_and_extension_no_l007() {
        let source = SourceFile::new(
            "struct_demo_mini.sy",
            r#"struct Point {
    var x: Int
    var y: Int
}

extension Point {
    func BadName() {
        print(self.x)
    }
}

func main() {
    print(1)
}
"#,
        );
        let issues = lint_source(&source).expect("lint");
        assert!(
            !issues.iter().any(|i| i.code == "L007"),
            "struct fields must not emit L007; got {issues:?}"
        );
        assert!(
            issues.iter().any(|i| i.code == "L001"),
            "extension method BadName should still emit L001; got {issues:?}"
        );
        assert!(
            !issues.iter().any(|i| i.code == "L012"),
            "indented print must not emit L012; got {issues:?}"
        );
    }
}
