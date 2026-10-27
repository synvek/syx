//! Phase 136 Macro M26 + Service DI v17 unit probes:
//! token-stream enumerate MVP + `within(T) || args(U)` pointcut compose MVP.

use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_method, PointcutPattern, WithinWildcard,
};
use crate::user::{enumerate_decl_param_tokens, MacroTokenKind};
use syx_ast::Expr;
use syx_diagnostics::Span;

fn span() -> Span {
    Span::new(0, 0)
}

#[test]
fn p136_or_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v17:pointcut-or-compose-mvp
    let or = parse_pointcut_pattern("within(Foo) || args(Int)").expect("parse");
    assert_eq!(
        or,
        PointcutPattern::Or {
            left: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Foo"),
                wildcard: WithinWildcard::Exact,
            }),
            right: Box::new(PointcutPattern::Args {
                type_name: smol_str::SmolStr::new("Int"),
            }),
        }
    );
    let swapped = parse_pointcut_pattern("args(Int) || within(Foo)").expect("parse swapped");
    assert!(matches!(swapped, PointcutPattern::Or { .. }));
    let int_param = syx_ast::Param {
        name: smol_str::SmolStr::new("x"),
        external_label: syx_ast::ParamLabel::Same,
        ty: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("Int"),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    };
    let str_param = syx_ast::Param {
        name: smol_str::SmolStr::new("s"),
        external_label: syx_ast::ParamLabel::Same,
        ty: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("String"),
            generics: vec![],
        }),
        default: None,
        isolated: false,
        sending: false,
        attrs: vec![],
    };
    assert!(pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Foo.bar",
        &[str_param.clone()]
    ));
    assert!(pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Bar.baz",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) || args(Int)",
        "Bar.baz",
        &[str_param]
    ));
    assert!(parse_pointcut_pattern("within(Foo) || within(Bar)").is_none());
    assert!(parse_pointcut_pattern("within(Foo) || args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v17:pointcut-or-compose-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p136_or_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) || args(Int)")
  @Around("within(Greeter) || args(Int)")
  @Before("within(Greeter) || args(Int)")
  @After("within(Greeter) || args(Int)")
  func greet(name: String) -> String {
    print("__body__:greet")
    return "hi"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p136_token_stream_enumerate_ident_punct_group() {
    // Fingerprint: macro-token-v7:token-stream-mvp
    let expr = Expr::Binary {
        span: span(),
        left: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("a"),
        }),
        op: syx_ast::BinOp::Add,
        right: Box::new(Expr::Group {
            span: span(),
            expr: Box::new(Expr::Ident {
                span: span(),
                name: smol_str::SmolStr::new("b"),
            }),
        }),
    };
    let tokens = enumerate_decl_param_tokens(&expr);
    assert_eq!(
        tokens,
        vec![
            MacroTokenKind::Ident(smol_str::SmolStr::new("a")),
            MacroTokenKind::Punct('+'),
            MacroTokenKind::Group(vec![MacroTokenKind::Ident(smol_str::SmolStr::new("b"))]),
        ]
    );
    assert!(
        include_str!("user.rs").contains("macro-token-v7:token-stream-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p136_or_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) || args(Int)")
  @Before("within(Greeter) || args(Int)")
  @After("within(Greeter) || args(Int)")
  func greet(name: String) -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet(name: "x"))
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}
