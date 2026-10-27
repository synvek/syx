//! Phase 122 Macro M12 unit probes: multi-package queue, scope, PreDestroy, RB nested fix.

use smol_str::SmolStr;

use crate::service::{collect_services, expand_service_macros, queue_services_from_module, spec_to_meta_entry};

#[test]
fn p122_service_scope_prototype_scan() {
    let src = r#"
@Scope("prototype")
@Service
class Worker {
  init() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].scope, SmolStr::new("prototype"));
}

#[test]
fn p122_service_predestroy_scan() {
    let src = r#"
@Service
class App {
  init() {}
  @PreDestroy
  func shutdown() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].pre_destroy.as_deref(), Some("shutdown"));
}

#[test]
fn p122_service_scope_request_accepted_migrated_p123() {
    let src = r#"
@Scope("request")
@Service
class App { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("request scope");
    assert_eq!(services[0].scope, SmolStr::new("request"));
}

#[test]
fn p122_service_di_multi_package_queue_append() {
    syx_meta::service_di_clear();
    let lib = r#"
@Service
class Logger { init() {} }
"#;
    let app = r#"
@Service
class App { init(@Inject logger: Logger) {} }
"#;
    syx_meta::service_di_set_scan_stem(Some("lib".to_string()));
    queue_services_from_module(
        &syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("lib.sy", lib)).expect("parse"),
    )
    .expect("queue lib");
    syx_meta::service_di_set_scan_stem(Some("main".to_string()));
    queue_services_from_module(
        &syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("main.sy", app)).expect("parse"),
    )
    .expect("queue app");
    syx_meta::service_di_set_scan_stem(None);
    let queued = syx_meta::service_di_peek_entries();
    assert_eq!(queued.len(), 2);
    assert!(queued.iter().any(|e| e.type_name == "Logger" && e.module_stem.as_deref() == Some("lib")));
    assert!(queued.iter().any(|e| e.type_name == "App" && e.module_stem.as_deref() == Some("main")));
    syx_meta::service_di_clear();
}

#[test]
fn p122_service_meta_entry_carries_scope_and_predestroy() {
    let src = r#"
@Scope("prototype")
@Service
class App {
  init() {}
  @PreDestroy
  func done() {}
}
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let entry = spec_to_meta_entry(&services[0]);
    assert_eq!(entry.scope, "prototype");
    assert_eq!(entry.pre_destroy.as_deref(), Some("done"));
}

#[test]
fn p122_resultbuilder_literal_for_in_expands() {
    let src = r#"
@resultBuilder
func Mixed() -> Array<String> {
    "x"
    for i in ["y"] {
        i
    }
}
func main() { print(1) }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let expanded = crate::expand_module(module).expect("expand");
    let dump = format!("{expanded:?}");
    assert!(
        dump.contains("__syx_result_builder_buildBlock"),
        "expected buildBlock fold for literal+for-in: {dump}"
    );
}

#[test]
fn p122_expand_service_macros_still_emits_boot() {
    let src = r#"
@Service
class Logger { init() {} }
func main() { print(1) }
"#;
    let mut module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    expand_service_macros(&mut module).expect("expand");
    let dump = format!("{module:?}");
    assert!(dump.contains("__syx_service_boot"), "{dump}");
}
