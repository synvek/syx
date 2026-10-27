//! Phase 125 Macro M15 unit probes: polymorphic inject, field Qualifier, @Around.

use crate::service::{
    collect_services, expand_around_interceptors, expand_service_macros, pick_binding,
    spec_to_meta_entry,
};

#[test]
fn p125_field_qualifier_metadata_roundtrip() {
    let src = r#"
@Service
class Logger { init() {} }
@Service
class App {
  @Autowired @Qualifier("fast")
  var logger: Logger
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.field_autowires.len(), 1);
    assert_eq!(
        app.field_autowires[0].qualifier.as_deref(),
        Some("fast")
    );
    let entry = spec_to_meta_entry(app);
    assert_eq!(entry.field_qualifiers.len(), 1);
    assert_eq!(entry.field_qualifiers[0].as_deref(), Some("fast"));
}

#[test]
fn p125_pick_binding_rewrites_to_concrete_on_expand() {
    let src = r#"
class Logger { init() {} func tag() -> String { return "?" } }
@Primary
@Service
class DefaultLogger: Logger { override func tag() -> String { return "default" } }
@Qualifier("fast")
@Service
class FastLogger: Logger { override func tag() -> String { return "fast" } }
@Service
class App {
  init(@Inject @Qualifier("fast") logger: Logger) {}
}
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_di_FastLogger"), "{dump}");
    assert!(!dump.contains("__syx_di_Logger"), "{dump}");
}

#[test]
fn p125_around_expands_delegate_body() {
    let src = r#"
@Service
class Greeter {
  init() {}
  @Around
  func greet(name: String) -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_around_interceptors(&mut module).expect("around");
    let dump = format!("{module:?}");
    assert!(dump.contains("__around__:"), "{dump}");
    assert!(dump.contains("print"), "{dump}");
    assert!(dump.contains("hi"), "{dump}");
}

#[test]
fn p125_around_rejects_non_service() {
    let src = r#"
class Greeter {
  @Around
  func greet() -> String { return "hi" }
}
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let err = expand_around_interceptors(&mut module).expect_err("E014");
    let msg = format!("{err:?}");
    assert!(msg.contains("E014") || msg.contains("@Around"), "{msg}");
}

#[test]
fn p125_protocol_binding_alias_pick() {
    let src = r#"
protocol Pingable { func ping() -> String { return "?" } }
@Primary
@Service
class DefaultPing: Pingable { func ping() -> String { return "d" } }
@Qualifier("fast")
@Service
class FastPing: Pingable { func ping() -> String { return "f" } }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = pick_binding(&services, "Pingable", Some("fast")).expect("fast");
    assert_eq!(fast.type_name, "FastPing");
    assert!(fast.binding_aliases.iter().any(|a| a == "Pingable"));
}

#[test]
fn p125_field_autowire_boot_contains_fast_logger_assign() {
    let src = include_str!("../../syx-driver/tests/fixtures/macros/field_qualifier.sy");
    let mut module = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_di_FastLogger"), "{dump}");
    assert!(dump.contains("logger"), "{dump}");
    assert!(dump.contains("__syx_service_boot_impl"), "{dump}");
}

#[test]
fn p125_binding_alias_includes_superclass() {
    let src = r#"
class Logger { init() {} }
@Qualifier("fast")
@Service
class FastLogger: Logger { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let fast = services.iter().find(|s| s.type_name == "FastLogger").unwrap();
    assert!(fast.binding_aliases.iter().any(|a| a == "Logger"));
    let picked = pick_binding(&services, "Logger", Some("fast")).expect("pick");
    assert_eq!(picked.type_name, "FastLogger");
}
