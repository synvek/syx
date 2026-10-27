//! Phase 135 Macro M25 + Service DI v16 unit probes:
//! decl-site token subst deepen + `within(T) && args(U)` pointcut compose MVP.

use crate::expand_module;
use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_method, PointcutPattern, WithinWildcard,
};

#[test]
fn p135_and_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v16:pointcut-and-compose-mvp
    let and = parse_pointcut_pattern("within(Foo) && args(Int)").expect("parse");
    assert_eq!(
        and,
        PointcutPattern::And {
            left: Box::new(PointcutPattern::Within {
                type_name: smol_str::SmolStr::new("Foo"),
                wildcard: WithinWildcard::Exact,
            }),
            right: Box::new(PointcutPattern::Args {
                type_name: smol_str::SmolStr::new("Int"),
            }),
        }
    );
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
    assert!(pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Foo.bar",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Bar.bar",
        &[int_param.clone()]
    ));
    assert!(!pointcut_matches_method(
        "within(Foo) && args(Int)",
        "Foo.bar",
        &[syx_ast::Param {
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
        }]
    ));
    assert!(parse_pointcut_pattern("within(Foo) && within(Bar)").is_none());
    assert!(parse_pointcut_pattern("within(Foo) && args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v16:pointcut-and-compose-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p135_and_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Around("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  @After("within(Greeter) && args(Int)")
  func greet(x: Int) -> String {
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
fn p135_and_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  func greet(name: String) -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Before") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p135_decl_token_name_and_default_subst() {
    // Fingerprint: macro-token-v6:decl-token-subst-mvp
    let src = r#"
@freestanding(declaration)
#macro makeFunc(fnName, msg) {
  func fnName() {
    print(msg)
  }
}
@freestanding(declaration)
#macro makeDefault(base) {
  func compute(n: Int = base + 1) -> Int {
    return n
  }
}
#makeFunc(greet, "decl-token-ok")
#makeDefault(41)
func main() {
  greet()
  print(compute())
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-token-ok"), "{dump}");
    assert!(dump.contains("name: \"greet\"") || dump.contains("greet"), "{dump}");
    assert!(dump.contains("41"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v6:decl-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p135_and_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter) && args(Int)")
  @Before("within(Greeter) && args(Int)")
  @After("within(Greeter) && args(Int)")
  func greet(x: Int) -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet(x: 1))
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
}
