//! Phase 121 Macro M11 unit probes: field `@Autowired`, `@PostConstruct`, import-visible append.

use smol_str::SmolStr;
use syx_ast::Decl;

use crate::service::{collect_services, expand_service_macros};

#[test]
fn p121_service_field_autowired_scan() {
    let src = r#"
@Service
class Logger {
  init() {}
}

@Service
class App {
  @Autowired
  var logger: Logger
  init() {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.deps, vec![SmolStr::new("Logger")]);
    assert_eq!(app.field_autowires.len(), 1);
    assert_eq!(app.field_autowires[0].field, "logger");
    assert_eq!(app.field_autowires[0].type_name, "Logger");
}

#[test]
fn p121_service_postconstruct_scan() {
    let src = r#"
@Service
class App {
  init() {}
  @PostConstruct
  func onReady() {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let services = collect_services(&module).expect("collect");
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.post_construct.as_deref(), Some("onReady"));
}

#[test]
fn p121_service_di_append_import_visible() {
    syx_meta::service_di_clear();
    let src_a = r#"
@Service
class Logger {
  init() {}
}
"#;
    let src_b = r#"
@Service
class App {
  init(@Inject logger: Logger) {}
}
"#;
    let mut mod_a =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("a.sy", src_a)).expect("parse");
    expand_service_macros(&mut mod_a).expect("expand a");
    let mut mod_b =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("b.sy", src_b)).expect("parse");
    expand_service_macros(&mut mod_b).expect("expand b");
    let queued = syx_meta::service_di_take();
    assert_eq!(queued.len(), 2);
    assert!(queued.iter().any(|e| e.type_name == "Logger"));
    assert!(queued.iter().any(|e| e.type_name == "App"));
    syx_meta::service_di_clear();
}

#[test]
fn p121_boot_codegen_emits_service_put() {
    let src = r#"
@Service
class Logger { init() {} }
@Service
class App { init(@Inject logger: Logger) {} }
func main() { print(1) }
"#;
    let module = syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let expanded = crate::expand_module(module).expect("expand");
    let dump = format!("{expanded:?}");
    assert!(dump.contains("__syx_service_put"), "boot must construct+put: {dump}");
    assert!(dump.contains("__syx_di_Logger"), "{dump}");
}

#[test]
fn p121_resultbuilder_nested_hostcall_e009() {
    let src = r#"
@resultBuilder
func Bad() -> Array<String> {
    "x"
    for i in ["y"] {
        i
    }
}
func main() { print(1) }
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let expanded = crate::expand_module(module);
    assert!(expanded.is_ok(), "Phase 122 RB nested stack fix should expand: {expanded:?}");
}
