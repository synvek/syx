//! Phase 128 Macro M18 unit probes: `@Lazy` + cycle-break allowance.

use crate::service::{check_service_cycles, collect_services, expand_service_macros};

#[test]
fn p128_lazy_attr_collected_on_service() {
    let src = r#"
@Lazy
@Service
class LazyBox {
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services.len(), 1);
    assert!(services[0].lazy, "{services:?}");
}

#[test]
fn p128_lazy_cycle_allowed_at_expand() {
    let src = r#"
@Lazy
@Service
class A {
  init(@Inject b: B) {}
}
@Service
class B {
  init(@Inject a: A) {}
}
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("lazy cycle should expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_lazy_proxy") || dump.contains("Lazy"), "{dump}");
}

#[test]
fn p128_cycle_without_lazy_still_e007_unit() {
    let src = r#"
@Service
class A {
  init(@Inject b: B) {}
}
@Service
class B {
  init(@Inject a: A) {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let err = check_service_cycles(&services).expect_err("E007");
    assert_eq!(err.code.as_deref(), Some("E007"));
}
