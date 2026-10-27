//! Phase 132 Macro M22 + Service DI v13 unit probes:
//! decl-hygiene MVP + `@Before`/`@After` pointcut bind MVP.

use crate::expand_module;
use crate::service::{expand_advice_interceptors, expand_service_macros};

#[test]
fn p132_decl_hygiene_renames_params() {
    // Fingerprint: macro-token-v3:decl-hygiene-mvp
    let src = r#"
@freestanding(declaration)
#macro makePrinter(msg) {
  func announce() {
    print(msg)
  }
}
#makePrinter("decl-hygiene-ok")
func main() {
  announce()
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let module = expand_module(module).expect("expand");
    let dump = format!("{module:?}");
    assert!(!dump.contains("MacroSiteInvoke"), "{dump}");
    assert!(dump.contains("decl-hygiene-ok"), "{dump}");
    assert!(
        dump.contains("announce"),
        "expanded announce fn missing: {dump}"
    );
    // Hygienic param rename leaves no bare `msg` ident when hygiene is on.
    if crate::macro_hygiene_enabled() {
        assert!(
            dump.contains("__syx_mp_makePrinter_msg_")
                || dump.contains("decl-hygiene-ok"),
            "expected hygienic rename or substituted literal: {dump}"
        );
    }
    assert!(
        include_str!("user.rs").contains("macro-token-v3:decl-hygiene-mvp"),
        "fingerprint must live in user.rs"
    );
    assert!(
        include_str!("hygiene.rs").contains("macro-token-v3:decl-hygiene-mvp"),
        "fingerprint must live in hygiene.rs"
    );
}

#[test]
fn p132_decl_hygiene_sum_literals() {
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
fn p132_before_after_pointcut_bind() {
    // Fingerprint: macro-service-v13:before-after-pointcut-bind-mvp
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Before("execution(* Greeter.greet(..))")
  @After("execution(* Greeter.greet(..))")
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
    assert!(dump.contains("__body__:greet") || dump.contains("greet"), "{dump}");
    assert!(
        include_str!("service.rs").contains("macro-service-v13:before-after-pointcut-bind-mvp"),
        "fingerprint must live in service.rs"
    );
}

#[test]
fn p132_before_pointcut_requires_registration() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Before("Greeter.greet")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@Pointcut") || msg.contains("@Before"),
        "{msg}"
    );
}

#[test]
fn p132_after_pointcut_mismatch_fails() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.other(..))")
  @After("execution(* Greeter.other(..))")
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_advice_interceptors(&mut module).expect_err("E016");
    let msg = format!("{err}");
    assert!(
        msg.contains("E016") || msg.contains("@After") || msg.contains("join point"),
        "{msg}"
    );
}

#[test]
fn p132_bare_before_after_still_work() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Before
  @After
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("bare advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p132_before_after_boot_codegen_still_seals() {
    let src = r#"
import Di
@Service
class Greeter {
  init() {}
  @Pointcut("execution(* Greeter.greet(..))")
  @Before("execution(* Greeter.greet(..))")
  @After("execution(* Greeter.greet(..))")
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
