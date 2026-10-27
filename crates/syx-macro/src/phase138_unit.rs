//! Phase 138 Macro M28 + Service DI v19 unit probes:
//! multi-token decl rewrite MVP + `within(*Service)` / `within(Foo*)` wildcard MVP.

use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_method, PointcutPattern, WithinWildcard,
};
use crate::user::{
    decl_base_name_from_tokens, enumerate_decl_param_tokens, expand_decl_param_tokens,
    DeclExpandMode, MacroTokenKind,
};
use syx_ast::{Decl, Expr};
use syx_diagnostics::Span;

fn span() -> Span {
    Span::new(0, 0)
}

#[test]
fn p138_within_wildcard_parse_and_match() {
    // Fingerprint: macro-service-v19:pointcut-within-wildcard-mvp
    let suffix = parse_pointcut_pattern("within(*Service)").expect("parse suffix");
    assert_eq!(
        suffix,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Service"),
            wildcard: WithinWildcard::Suffix,
        }
    );
    let prefix = parse_pointcut_pattern("within(Foo*)").expect("parse prefix");
    assert_eq!(
        prefix,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Foo"),
            wildcard: WithinWildcard::Prefix,
        }
    );
    assert!(pointcut_matches_method("within(*Service)", "OrderService.run", &[]));
    assert!(pointcut_matches_method("within(*Service)", "PaymentService.handle", &[]));
    assert!(!pointcut_matches_method("within(*Service)", "OrderRepo.run", &[]));
    assert!(pointcut_matches_method("within(Foo*)", "FooBar.run", &[]));
    assert!(pointcut_matches_method("within(Foo*)", "Foo.run", &[]));
    assert!(!pointcut_matches_method("within(Foo*)", "BarFoo.run", &[]));
    assert!(parse_pointcut_pattern("within(*)").is_none());
    assert!(parse_pointcut_pattern("within(*Foo*)").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v19:pointcut-within-wildcard-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p138_within_wildcard_before_bind() {
    let src = r#"
@Service
class OrderService {
  init() {}
  @Pointcut("within(*Service)")
  @Before("within(*Service)")
  func run() -> String {
    print("__body__:run")
    return "svc-ok"
  }
}
@Service
class OrderRepo {
  init() {}
  func run() -> String {
    print("__body__:repo")
    return "repo-plain"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
}

#[test]
fn p138_token_rewrite_multi_token_decl() {
    // Fingerprint: macro-token-v9:token-rewrite-mvp
    let tokens = vec![
        MacroTokenKind::Ident(smol_str::SmolStr::new("my")),
        MacroTokenKind::Punct('_'),
        MacroTokenKind::Ident(smol_str::SmolStr::new("fn")),
    ];
    assert_eq!(
        decl_base_name_from_tokens(&tokens).map(|s| s.to_string()),
        Some("my_fn".into())
    );
    let renamed =
        expand_decl_param_tokens(&tokens, DeclExpandMode::RenamePrefix("wrapped_".into()))
            .expect("multi-token rename");
    assert!(matches!(
        renamed,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "wrapped_my_fn"
    ));
    let wrapped = expand_decl_param_tokens(&tokens, DeclExpandMode::WrapPrefix("wrap_".into()))
        .expect("multi-token wrap");
    let dump = format!("{wrapped:?}");
    assert!(dump.contains("decl-rewrite-wrap"), "{dump}");
    let group_tokens = vec![MacroTokenKind::Group(vec![
        MacroTokenKind::Ident(smol_str::SmolStr::new("inner")),
        MacroTokenKind::Punct('_'),
        MacroTokenKind::Ident(smol_str::SmolStr::new("name")),
    ])];
    let grouped = expand_decl_param_tokens(&group_tokens, DeclExpandMode::WrapPrefix("g_".into()))
        .expect("group rewrite");
    assert!(matches!(
        grouped,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "g_inner_name"
    ));
    let expr = Expr::Binary {
        span: span(),
        left: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("a"),
        }),
        op: syx_ast::BinOp::Add,
        right: Box::new(Expr::Ident {
            span: span(),
            name: smol_str::SmolStr::new("b"),
        }),
    };
    let enumerated = enumerate_decl_param_tokens(&expr);
    let from_enum = expand_decl_param_tokens(&enumerated, DeclExpandMode::RenamePrefix("r_".into()))
        .expect("enum rewrite");
    assert!(matches!(
        from_enum,
        Decl::Function {
            ref name,
            ..
        } if name.as_str() == "r_a+b"
    ));
    assert!(expand_decl_param_tokens(&[], DeclExpandMode::WrapPrefix("x_".into())).is_none());
    assert!(
        include_str!("user.rs").contains("macro-token-v9:token-rewrite-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p138_within_wildcard_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class BillingService {
  init() {}
  @Pointcut("within(*Service)")
  @Before("within(*Service)")
  func handle() -> String {
    print("__body__:handle")
    return "ok"
  }
}
func main() {
  let s = Di.resolve("BillingService")
  print(s.handle())
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
fn p138_fingerprint_string_pins() {
    let service_rs = include_str!("service.rs");
    let user_rs = include_str!("user.rs");
    let runtime_rs = include_str!("../../syx-async/src/runtime.rs");
    let concurrency = include_str!("../../../packages/core/concurrency.sy");
    assert!(service_rs.contains("macro-service-v19:pointcut-within-wildcard-mvp"));
    assert!(user_rs.contains("macro-token-v9:token-rewrite-mvp"));
    assert!(runtime_rs.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
    assert!(concurrency.contains("setActorSharedPoolEnabled"));
}
