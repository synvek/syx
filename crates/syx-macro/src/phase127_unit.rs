//! Phase 127 Macro M17 unit probes: advice proceed delegate + `@Configuration`/`@Bean`.

use crate::service::{collect_beans, expand_advice_interceptors, expand_service_macros};

#[test]
fn p127_advice_proceed_expands_delegate_vars() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  @Around
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_proceed_0"), "{dump}");
    assert!(dump.contains("__syx_proceed_1"), "{dump}");
    assert_eq!(dump.matches("__around__:").count(), 2, "{dump}");
}

#[test]
fn p127_advice_proceed_order_before_around_after() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  @Before
  @After
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    let before = dump.find("__before__:").expect("before");
    let around = dump.find("__around__:").expect("around");
    let proceed = dump.find("__syx_proceed_0").expect("proceed");
    let after = dump.find("__after__:").expect("after");
    assert!(before < around, "{dump}");
    assert!(around < proceed, "{dump}");
    assert!(proceed < after, "{dump}");
}

#[test]
fn p127_collect_beans_from_configuration() {
    let src = r#"
@Configuration
class AppConfig {
  init() {}
  @Bean
  func greeter() -> Greeter { return Greeter() }
}
class Greeter { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let beans = collect_beans(&module).expect("beans");
    assert_eq!(beans.len(), 1);
    assert_eq!(beans[0].config_type, "AppConfig");
    assert_eq!(beans[0].method, "greeter");
    assert_eq!(beans[0].return_type, "Greeter");
}

#[test]
fn p127_bean_boot_codegen_emits_factory_put() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/configuration_bean.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_service_put"), "{dump}");
    assert!(dump.contains("AppConfig"), "{dump}");
}

#[test]
fn p127_bean_rejects_non_configuration() {
    let src = r#"
@Service
class Bad {
  init() {}
  @Bean
  func greeter() -> Greeter { return Greeter() }
}
class Greeter { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = collect_beans(&module).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("@Bean") || msg.contains("E015"), "{msg}");
}
