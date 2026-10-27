//! Phase 131 Macro M21 unit probes: decl-param subst MVP + execution pointcut MVP.

use crate::service::{
    collect_pointcuts, expand_advice_interceptors, expand_service_macros, parse_pointcut_pattern,
    pointcut_matches_join, PointcutPattern,
};
use crate::expand_module;

#[test]
fn p131_decl_param_ident_literal_subst() {
    let src = r#"
@freestanding(declaration)
#macro makePrinter(msg) {
  func announce() {
    print(msg)
  }
}
#makePrinter("decl-ok")
func main() {
  announce()
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-ok"), "{dump}");
    assert!(
        dump.contains("name: \"announce\"") || dump.contains("announce"),
        "expanded announce fn missing: {dump}"
    );
    let fn_names: Vec<_> = module
        .decls
        .iter()
        .filter_map(|d| match d {
            syx_ast::Decl::Function { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        fn_names.iter().any(|n| *n == "announce"),
        "announce not in decls: {fn_names:?} dump={dump}"
    );
    assert!(
        include_str!("user.rs").contains("macro-token-v2:decl-param-subst-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p131_decl_param_sum_literals() {
    let src = r#"
@freestanding(declaration)
#macro makeSum(a, b) {
  func sum() -> Int {
    return a + b
  }
}
#makeSum(20, 22)
func main() {
  print(sum())
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
    assert!(dump.contains("22"), "{dump}");
}

#[test]
fn p131_execution_pointcut_parse_and_match() {
    let exact = parse_pointcut_pattern("execution(* Foo.bar(..))").expect("parse");
    assert_eq!(
        exact,
        PointcutPattern::Execution {
            type_name: smol_str::SmolStr::new("Foo"),
            method: Some(smol_str::SmolStr::new("bar")),
        }
    );
    let wild = parse_pointcut_pattern("execution(* Foo.*(..))").expect("parse");
    assert_eq!(
        wild,
        PointcutPattern::Execution {
            type_name: smol_str::SmolStr::new("Foo"),
            method: None,
        }
    );
    assert!(pointcut_matches_join(
        "execution(* Foo.bar(..))",
        "Foo.bar"
    ));
    assert!(pointcut_matches_join("execution(* Foo.*(..))", "Foo.baz"));
    assert!(!pointcut_matches_join(
        "execution(* Foo.bar(..))",
        "Foo.baz"
    ));
    assert!(!pointcut_matches_join(
        "execution(* Foo.*(..))",
        "Bar.baz"
    ));
    assert!(
        include_str!("service.rs").contains("macro-service-v12:pointcut-execution-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p131_execution_pointcut_collected_and_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Around("execution(* Greeter.greet(..))")
  func greet() -> String { return "hi" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let pcs = collect_pointcuts(&module);
    assert!(
        pcs.iter()
            .any(|p| p.as_str() == "execution(* Greeter.greet(..))"),
        "{pcs:?}"
    );

    let mut module = module;
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
}

#[test]
fn p131_execution_wildcard_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.*(..))")
  @Around("execution(* Greeter.*(..))")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}

#[test]
fn p131_execution_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.other(..))")
  @Around("execution(* Greeter.other(..))")
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
fn p131_execution_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Around("execution(* Greeter.greet(..))")
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
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
}
