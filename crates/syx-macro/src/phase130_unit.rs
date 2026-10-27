//! Phase 130 Macro M20 unit probes: token expr MVP + `@Pointcut` method-name MVP.

use crate::service::{collect_pointcuts, expand_advice_interceptors, expand_service_macros};
use crate::expand_module;

#[test]
fn p130_token_expr_ident_literal_subst() {
    let src = r#"
#macro double(x) {
  x + x
}
func main() {
  print(#double(21))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("21"), "{dump}");
    assert!(
        include_str!("user.rs").contains("macro-token-v1:expr-token-mvp"),
        "fingerprint must live in user.rs"
    );
}

#[test]
fn p130_token_expr_return_body_and_nested() {
    let src = r#"
#macro inc(x) { return x + 1 }
#macro double(x) { x + x }
func main() {
  print(#double(#inc(20)))
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroInvoke"), "{dump}");
    assert!(dump.contains("20"), "{dump}");
}

#[test]
fn p130_pointcut_collected_and_around_binds() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("Greeter.greet")
  @Around("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let pcs = collect_pointcuts(&module);
    assert!(pcs.iter().any(|p| p.as_str() == "Greeter.greet"), "{pcs:?}");

    let mut module = module;
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(
        include_str!("service.rs").contains("macro-service-v11:pointcut-name-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p130_around_pointcut_requires_registration() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Pointcut") || msg.contains("@Around"),
        "{msg}"
    );
}

#[test]
fn p130_pointcut_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("Greeter.greet")
  @Around("Greeter.greet")
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
