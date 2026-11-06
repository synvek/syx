//! S2-I160 / S2-I162 S2-04 CST→AST lower probe contracts.
//! Nails: ADR + lower_cst_subset_probe (+ W791–797 expand) + Parser Module parity + LWR001 reject;
//! ≠ FrontendPipeline / ≠ full-subset lower / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-cst-lower-probe.md · docs/adr/stage2-s2-04-cst-lower-probe-expand.md.

use syx_ast::{Decl, Expr, Literal, Module, Stmt};
use syx_diagnostics::{SourceFile, Span};

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn zero_span() -> Span {
    Span::DUMMY
}

fn zero_expr(expr: &mut Expr) {
    match expr {
        Expr::Literal { span, .. }
        | Expr::Ident { span, .. }
        | Expr::Super { span }
        | Expr::Assign { span, .. }
        | Expr::CompoundAssign { span, .. }
        | Expr::MemberAssign { span, .. }
        | Expr::IndexAssign { span, .. } => {
            *span = zero_span();
        }
        Expr::Binary {
            span,
            left,
            right,
            ..
        } => {
            *span = zero_span();
            zero_expr(left);
            zero_expr(right);
        }
        Expr::Ternary {
            span,
            condition,
            then_branch,
            else_branch,
        } => {
            *span = zero_span();
            zero_expr(condition);
            zero_expr(then_branch);
            zero_expr(else_branch);
        }
        Expr::Unary { span, expr, .. } => {
            *span = zero_span();
            zero_expr(expr);
        }
        Expr::Call {
            span,
            callee,
            args,
        } => {
            *span = zero_span();
            zero_expr(callee);
            for a in args {
                zero_expr(&mut a.value);
            }
        }
        Expr::Member { span, object, .. } => {
            *span = zero_span();
            zero_expr(object);
        }
        Expr::Index { span, object, index } => {
            *span = zero_span();
            zero_expr(object);
            zero_expr(index);
        }
        Expr::Group { span, expr } => {
            *span = zero_span();
            zero_expr(expr);
        }
        Expr::Array { span, elements } => {
            *span = zero_span();
            for e in elements {
                zero_expr(e);
            }
        }
        other => {
            // Probe surface does not produce remaining variants; still zero known span fields via Debug fallback.
            let _ = other;
        }
    }
}

fn zero_stmt(stmt: &mut Stmt) {
    match stmt {
        Stmt::Print { span, expr } => {
            *span = zero_span();
            zero_expr(expr);
        }
        Stmt::Expr { span, expr } => {
            *span = zero_span();
            zero_expr(expr);
        }
        Stmt::Return { span, value } => {
            *span = zero_span();
            if let Some(v) = value {
                zero_expr(v);
            }
        }
        Stmt::VarDecl {
            span,
            init,
            ..
        } => {
            *span = zero_span();
            if let Some(v) = init {
                zero_expr(v);
            }
        }
        other => {
            let _ = other;
        }
    }
}

fn zero_module(module: &mut Module) {
    for decl in &mut module.decls {
        if let Decl::Function { span, body, .. } = decl {
            *span = zero_span();
            for stmt in body {
                zero_stmt(stmt);
            }
        }
    }
}

fn assert_probe_parity(label: &str, src: &str) {
    let file = SourceFile::new(label, src);
    let mut lowered = syx_cst::lower_cst_subset_probe_from_source(&file)
        .unwrap_or_else(|e| panic!("{label}: lower probe must Ok, got {e:?}"));
    let mut parsed = syx_parser::Parser::parse(file)
        .unwrap_or_else(|e| panic!("{label}: Parser::parse must Ok, got {e:?}"));
    zero_module(&mut lowered);
    zero_module(&mut parsed);
    assert_eq!(
        lowered, parsed,
        "{label}: span-zeroed Module must match Parser::parse\nlowered={lowered:#?}\nparsed={parsed:#?}"
    );
}

#[test]
fn s2_04_cst_lower_probe_adr_locked() {
    let adr = repo_file("docs/adr/stage2-s2-04-cst-lower-probe.md");
    assert!(adr.contains("S2-I160"), "lower probe ADR must exist");
    assert!(
        adr.contains("lower_cst_subset_probe"),
        "ADR must name lower_cst_subset_probe"
    );
    assert!(
        adr.contains("LWR001"),
        "ADR must name LWR001 reject code"
    );
    assert!(
        adr.contains("≠") && adr.contains("FrontendPipeline"),
        "ADR must Non-claim FrontendPipeline"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("full-subset"),
        "ADR must Non-claim full-subset lower"
    );

    let expand = repo_file("docs/adr/stage2-s2-04-cst-lower-probe-expand.md");
    assert!(expand.contains("S2-I162"), "expand ADR must exist");
    assert!(
        expand.contains("let") && expand.contains("var") && expand.contains("return"),
        "expand ADR must name let/var/return"
    );
    assert!(
        expand.contains("≠") && expand.contains("while"),
        "expand ADR must Non-claim while"
    );

    let frontend_toml = repo_file("crates/syx-frontend/Cargo.toml");
    assert!(
        !frontend_toml.contains("syx-cst"),
        "Frontend must not gain syx-cst dep in probe window"
    );
    let cst_toml = repo_file("crates/syx-cst/Cargo.toml");
    assert!(
        cst_toml.contains("syx-ast"),
        "syx-cst must depend on syx-ast for lower probe"
    );
    assert!(
        !cst_toml.contains("syx-parser"),
        "syx-cst must not depend on syx-parser"
    );
}

#[test]
fn s2_04_cst_lower_probe_parity_fixtures() {
    assert_probe_parity("print_int", "func main() {\n    print(1)\n}\n");
    assert_probe_parity(
        "multi_print",
        "func main() {\n    print(1)\n    print(x)\n    print(\"hi\")\n}\n",
    );
    assert_probe_parity("print_call_simple", "func main() {\n    print(f())\n}\n");
    assert_probe_parity(
        "trivia",
        "// ahead\nfunc main() {\n    // inside\n    print(1)\n}\n/* trail */\n",
    );
    assert_probe_parity("empty_body", "func main() {\n}\n");
    assert_probe_parity(
        "print_call_simple_arg",
        "func main() {\n    print(g(1))\n}\n",
    );
    // S2-I162 simple-stmt expand.
    assert_probe_parity(
        "let_then_print",
        "func main() {\n    let x = 1\n    print(x)\n}\n",
    );
    assert_probe_parity(
        "var_bool",
        "func main() {\n    var y = true\n    print(y)\n}\n",
    );
    assert_probe_parity(
        "return_simple",
        "func main() {\n    return 1\n}\n",
    );
    assert_probe_parity("bare_return", "func main() {\n    return\n}\n");
    assert_probe_parity(
        "mixed_let_var_return",
        "func main() {\n    let a = 1\n    var b = false\n    print(a)\n    return b\n}\n",
    );
    assert_probe_parity(
        "let_nil_double",
        "func main() {\n    let n = nil\n    let d = 1.5\n    print(n)\n}\n",
    );
}

#[test]
fn s2_04_cst_lower_probe_rejects_out_of_face() {
    let cases = [
        ("while_loop", "func main() {\n    while x {\n        print(1)\n    }\n}\n"),
        (
            "multi_func",
            "func main() {\n    print(1)\n}\nfunc other() {\n    print(2)\n}\n",
        ),
        ("named_other", "func foo() {\n    print(1)\n}\n"),
        ("typed_param", "func main(x: Int) {\n    print(x)\n}\n"),
        (
            "typed_let",
            "func main() {\n    let x: Int = 1\n    print(x)\n}\n",
        ),
    ];
    for (label, src) in cases {
        let file = SourceFile::new(label, src);
        // Wider CST may still Ok; probe must LWR001.
        let err = syx_cst::lower_cst_subset_probe_from_source(&file)
            .expect_err(&format!("{label}: must LWR001"));
        assert_eq!(
            err.code.as_deref(),
            Some("LWR001"),
            "{label}: expected LWR001, got {err:?}"
        );
    }
}

#[test]
fn s2_04_cst_lower_probe_smoke_shapes() {
    let file = SourceFile::new("smoke.sy", "func main() {\n    print(1)\n}\n");
    let m = syx_cst::lower_cst_subset_probe_from_source(&file).expect("lower");
    match &m.decls[0] {
        Decl::Function { body, .. } => match &body[0] {
            Stmt::Print {
                expr: Expr::Literal {
                    value: Literal::Integer { value: 1, .. },
                    ..
                },
                ..
            } => {}
            other => panic!("expected Stmt::Print(1), got {other:?}"),
        },
        other => panic!("expected Function, got {other:?}"),
    }
}
