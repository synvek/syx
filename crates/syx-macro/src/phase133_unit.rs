//! Phase 133 Macro M23 + Service DI v14 unit probes:
//! stmt-site token subst MVP + `within(Type)` pointcut name MVP.

use crate::expand_module;
use crate::service::{
    expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_join, PointcutPattern, WithinWildcard,
};

#[test]
fn p133_stmt_token_ident_literal_subst() {
    // Fingerprint: macro-token-v4:stmt-token-subst-mvp
    let src = r#"
#macro say(msg) {
  print(msg)
}
#macro logSum(a, b) {
  print(a + b)
}
func main() {
  #say("stmt-token-ok")
  #logSum(20, 22)
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("stmt-token-ok"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
    assert!(dump.contains("22"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v4:stmt-token-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p133_stmt_token_nested_expr_arg() {
    let src = r#"
#macro wrap(s) {
  "hi:" + s
}
#macro say(msg) {
  print(msg)
}
func main() {
  #say(#wrap("world"))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(
        dump.contains("hi:") || dump.contains("world"),
        "nested expr arg subst: {dump}"
    );
}

#[test]
fn p133_stmt_token_multi_stmt_body() {
    let src = r#"
#macro sayTwice(msg) {
  print(msg)
  print(msg)
}
func main() {
  #sayTwice("twice-ok")
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    let count = dump.matches("twice-ok").count();
    assert!(count >= 2, "expected two substituted prints: {dump}");
}

#[test]
fn p133_stmt_token_arity_mismatch() {
    let src = r#"
#macro say(msg) {
  print(msg)
}
func main() {
  #say("a", "b")
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_module(module).expect_err("arity");
    let msg = format!("{err}");
    assert!(
        msg.contains("arity") || msg.contains("mismatch"),
        "{msg}"
    );
}

#[test]
fn p133_within_pointcut_parse_and_match() {
    // Fingerprint: macro-service-v14:pointcut-within-name-mvp
    let within = parse_pointcut_pattern("within(Foo)").expect("parse");
    assert_eq!(
        within,
        PointcutPattern::Within {
            type_name: smol_str::SmolStr::new("Foo"),
            wildcard: WithinWildcard::Exact,
        }
    );
    assert!(pointcut_matches_join("within(Foo)", "Foo.bar"));
    assert!(pointcut_matches_join("within(Foo)", "Foo.baz"));
    assert!(!pointcut_matches_join("within(Foo)", "Bar.bar"));
    assert!(parse_pointcut_pattern("within()").is_none());
    assert!(parse_pointcut_pattern("within(Foo.Bar)").is_none());
    assert!(
        include_str!("service.rs").contains("macro-service-v14:pointcut-within-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p133_within_around_before_after_bind() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  @Around("within(Greeter)")
  @Before("within(Greeter)")
  @After("within(Greeter)")
  func greet() -> String {
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
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p133_within_hits_any_method_on_type() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  func marker() {}
  @Around("within(Greeter)")
  @Before("within(Greeter)")
  func alpha() -> String {
    print("__body__:alpha")
    return "a"
  }
  @After("within(Greeter)")
  func beta() -> String {
    print("__body__:beta")
    return "b"
  }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p133_within_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("within(Other)")
  @Around("within(Other)")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Around") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p133_within_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("within(Greeter)")
  @Before("within(Greeter)")
  @After("within(Greeter)")
  func greet() -> String {
    print("__body__:greet")
    return "ok"
  }
}
func main() {
  let g = Di.resolve("Greeter")
  print(g.greet())
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}
