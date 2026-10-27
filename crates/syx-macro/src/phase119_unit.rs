//! Phase 119 macro M9 unit probes (role E003, stringify, attached E005).

use smol_str::SmolStr;
use syx_ast::{AccessLevel, Attribute, AttributeArg, Decl, Module, TypeExpr};

use crate::attached::expand_attached_macros;
use crate::roles::{role_mismatch_diag, user_macro_roles, FreestandingRole};

fn diag_span() -> syx_diagnostics::Span {
    syx_diagnostics::Span {
        start: 0,
        len: 1,
    }
}

#[test]
fn p119_role_e003_code() {
    let diag = role_mismatch_diag("badMacro", FreestandingRole::Expression);
    assert_eq!(diag.code.as_deref(), Some("E003"));
    assert!(diag.message.contains("expression"));
}

#[test]
fn p119_empty_freestanding_roles_fail_closed() {
    let attrs = vec![Attribute {
        name: SmolStr::new("freestanding"),
        args: vec![],
    }];
    let roles = user_macro_roles(&SmolStr::new("m"), &attrs);
    assert!(roles.is_empty());
}

#[test]
fn p119_stringify_expands_ident() {
    let src = "func main() { print(#stringify(hello)) }";
    let mut module = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src))
        .expect("parse stringify probe");
    module = crate::expand_module(module).expect("expand stringify");
    let body = match &module.decls[0] {
        Decl::Function { body, .. } => body,
        other => panic!("expected function decl, got {other:?}"),
    };
    match &body[0] {
        syx_ast::Stmt::Print { expr, .. } => match expr {
            syx_ast::Expr::Literal {
                value: syx_ast::Literal::String(s),
                ..
            } => assert_eq!(s.as_str(), "hello"),
            other => panic!("expected string literal, got {other:?}"),
        },
        other => panic!("expected print stmt, got {other:?}"),
    }
}

#[test]
fn p119_attached_unknown_conformance_e005() {
    let mut module = Module {
        name: SmolStr::new("test"),
        decls: vec![Decl::DataClass {
            attrs: vec![Attribute {
                name: SmolStr::new("attached"),
                args: vec![
                    AttributeArg::Ident(SmolStr::new("conformance")),
                    AttributeArg::Ident(SmolStr::new("CustomProto")),
                ],
            }],
            access: AccessLevel::Public,
            name: SmolStr::new("Box"),
            span: diag_span(),
            fields: vec![(
                SmolStr::new("n"),
                TypeExpr::Named {
                    name: SmolStr::new("Int"),
                    generics: vec![],
                },
            )],
            type_params: vec![],
            where_clause: vec![],
        }],
    };
    let err = expand_attached_macros(&mut module).expect_err("unknown conformance");
    assert_eq!(err.code.as_deref(), Some("E005"));
    assert!(err.message.contains("CustomProto"));
}

#[test]
fn p119_macro_sandbox_mode_exported() {
    assert!(!crate::macro_sandbox_mode().is_empty());
}

#[test]
fn p119_resultbuilder_desugar_builds_block() {
    let src = r#"
@resultBuilder
func pair() -> Array<String> {
    "one"
    "two"
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let expanded = crate::expand_module(module).expect("expand");
    let dump = format!("{expanded:?}");
    assert!(
        dump.contains("buildBlock") || dump.contains("__syx_result_builder_buildBlock"),
        "expected buildBlock in expanded AST, got {dump}"
    );
}


