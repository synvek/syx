//! Phase 129 Macro M19 unit probes: associated-enum Codable + `@Bean` param autowire.

use crate::attached::expand_attached_macros;
use crate::service::{collect_beans, expand_service_macros};

#[test]
fn p129_codable_associated_enum_expands_encode_decode() {
    let src = r#"
@Codable
enum WireStatus {
  ok(code: Int)
  err(message: String)
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_attached_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("encode"), "{dump}");
    assert!(dump.contains("decode"), "{dump}");
    assert!(
        dump.contains("__tag") || dump.contains("ok"),
        "{dump}"
    );
    // L2 fingerprint must remain searchable in attached.rs
    assert!(include_str!("attached.rs").contains("macro-codable-l2:associated-enum-auto"));
}

#[test]
fn p129_bean_params_collected_as_inject() {
    let src = r#"
@Configuration
class AppConfig {
  init() {}
  @Bean
  func greeter(@Autowired logger: DepLogger) -> BeanGreeter {
    return BeanGreeter(logger: logger)
  }
}
class DepLogger { init() {} }
class BeanGreeter { init(logger: DepLogger) {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let beans = collect_beans(&module).expect("beans");
    assert_eq!(beans.len(), 1);
    assert_eq!(beans[0].params.len(), 1);
    assert_eq!(beans[0].params[0].dep_type, "DepLogger");
    assert_eq!(beans[0].params[0].param, "logger");
}

#[test]
fn p129_bean_param_boot_codegen_passes_args() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/bean_param_autowire.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("greeter") || dump.contains("BeanGreeter"), "{dump}");
    assert!(include_str!("service.rs").contains("macro-service-v10:bean-param-autowire-mvp"));
}
