//! Phase 126 Macro M16 unit probes: protocol witness + advice chain.

use crate::service::{
    collect_services, expand_advice_interceptors, expand_service_macros, spec_to_meta_entry,
};

#[test]
fn p126_protocol_conformance_metadata_roundtrip() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Qualifier("fast")
@Service
class FastPing: Pingable {
  init() {}
  func ping() -> String { return "fast" }
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = services.iter().find(|s| s.type_name == "FastPing").unwrap();
    assert!(fast.protocol_conformances.iter().any(|p| p == "Pingable"));
    let entry = spec_to_meta_entry(fast);
    assert_eq!(entry.protocol_conformances, vec!["Pingable"]);
}

#[test]
fn p126_advice_chain_expands_multiple_around() {
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
    assert_eq!(dump.matches("__around__:").count(), 2, "{dump}");
}

#[test]
fn p126_before_after_expands_markers() {
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
    expand_advice_interceptors(&mut module).expect("advice");
    let dump = format!("{module:?}");
    assert!(dump.contains("__before__:"), "{dump}");
    assert!(dump.contains("__after__:"), "{dump}");
}

#[test]
fn p126_register_witness_codegen_in_boot() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Primary
@Service
class DefaultPing: Pingable { init() {} func ping() -> String { return "d" } }
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_register_witness"), "{dump}");
    assert!(dump.contains("Pingable"), "{dump}");
}

#[test]
fn p126_full_fixture_expands_boot_and_witness() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/protocol_witness.sy");
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert!(services.iter().any(|s| s.type_name == "App"), "{services:?}");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
    assert!(dump.contains("__syx_service_register_witness"), "{dump}");
}
