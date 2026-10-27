//! Phase 134 Macro M24 + Service DI v15 unit probes:
//! expression-site token subst deepen + `args(Type)` pointcut name MVP.

use crate::expand_module;
use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_method, PointcutPattern,
};

#[test]
fn p134_expr_token_nested_call_subst() {
    // Fingerprint: macro-token-v5:expr-token-subst-mvp
    let src = r#"
#macro apply(f, x) {
  f(x) + f(x)
}
func helper(v: Int) -> Int {
  return v + 1
}
func main() {
  print(#apply(helper, 10))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("helper"), "{dump}");
    assert!(dump.contains("10"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v5:expr-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p134_expr_token_array_and_member_subst() {
    let src = r#"
#macro pair(x) {
  [x, x]
}
#macro pick(obj, field) {
  obj.field + 1
}
func main() {
  print(#pair(21))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("21"), "{dump}");
}

#[test]
fn p134_args_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v15:pointcut-args-name-mvp
    let args = parse_pointcut_pattern("args(Int)").expect("parse");
    assert_eq!(
        args,
        PointcutPattern::Args {
            type_name: smol_str::SmolStr::new("Int"),
        }
    );
    assert!(pointcut_matches_method(
        "args(Int)",
        "Foo.bar",
        &[syx_ast::Param {
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
        }]
    ));
    assert!(!pointcut_matches_method(
        "args(Int)",
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
    assert!(parse_pointcut_pattern("args()").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v15:pointcut-args-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p134_args_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Around("args(Int)")
  @Before("args(Int)")
  @After("args(Int)")
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
fn p134_args_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Before("args(Int)")
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
fn p134_args_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("args(Int)")
  @Before("args(Int)")
  @After("args(Int)")
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
