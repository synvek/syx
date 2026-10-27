//! Phase 137 Macro M27 + Service DI v18 unit probes:
//! decl-expand MVP + `!within(Type)` pointcut + DiscardOldest string pins.

use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_method, PointcutPattern, WithinWildcard,
};
use crate::user::{
    enumerate_decl_param_tokens, expand_decl_param_tokens, DeclExpandMode, MacroTokenKind,
};
use syx_ast::{Decl, Expr};
use syx_diagnostics::Span;

fn span() -> Span {
    Span::new(0, 0)
}

#[test]
fn p137_not_within_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v18:pointcut-not-within-mvp
    let not_within = parse_pointcut_pattern("!within(Excluded)").expect("parse");
    assert_eq!(
        not_within,
        PointcutPattern::Not {
            inner: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Excluded"),
                wildcard: WithinWildcard::Exact,
            }),
        }
    );
    assert!(pointcut_matches_method("!within(Excluded)", "Allowed.handle", &[]));
    assert!(!pointcut_matches_method("!within(Excluded)", "Excluded.handle", &[]));
    assert!(parse_pointcut_pattern("!within()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v18:pointcut-not-within-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p137_not_within_before_after_bind() {
    let src = r#"
@Service
class Allowed {
  init() {}
  @Pointcut("!within(Excluded)")
  @Before("!within(Excluded)")
  @After("!within(Excluded)")
  func handle() -> String {
    print("__body__:handle")
    return "allowed-ok"
  }
}
@Service
class Excluded {
  init() {}
  func handle() -> String {
    print("__body__:excluded")
    return "excluded-plain"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p137_decl_expand_rename_and_wrap() {
    // Fingerprint: macro-token-v8:decl-expand-mvp
    let tokens = vec![MacroTokenKind::Ident(smol_str::SmolStr::new("greet"))];
    let renamed = expand_decl_param_tokens(&tokens, DeclExpandMode::RenamePrefix("wrapped_".into()))
        .expect("rename expand");
    let wrapped = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("wrap expand");
    assert!(matches!(
        renamed,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrapped_greet"
    ));
    assert!(matches!(
        wrapped,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrap_greet"
    ));
    let dump = format!("{renamed:?}");
    assert!(dump.contains("decl-expand-rename"), "{dump}");
    let dump2 = format!("{wrapped:?}");
    assert!(dump2.contains("decl-expand-wrap"), "{dump2}");
    assert!(expand_decl_param_tokens(
        &[MacroTokenKind::Punct('+')],
        DeclExpandMode::RenamePrefix("x_".into())
    )
    .is_none());
    assert!(
        include_str!("user.rs").contains("macro-token-v8:decl-expand-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p137_token_stream_still_enumerates() {
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
}

#[test]
fn p137_not_within_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Allowed {
  init() {}
  @Pointcut("!within(Excluded)")
  @Before("!within(Excluded)")
  @After("!within(Excluded)")
  func handle() -> String {
    print("__body__:handle")
    return "ok"
  }
}
func main() {
  let a = Di.resolve("Allowed")
  print(a.handle())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p137_fingerprint_string_pins() {
    let service_rs = include_str!("service.rs");
    let user_rs = include_str!("user.rs");
    let runtime_rs = include_str!("../../syx-async/src/runtime.rs");
    let concurrency = include_str!("../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v18:pointcut-not-within-mvp"));
    assert!(user_rs.contains("macro-token-v8:decl-expand-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v5:discard-oldest-mvp"));
    assert!(concurrency.contains("discardOldest"));
}
