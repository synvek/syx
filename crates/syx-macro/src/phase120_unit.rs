//! Phase 120 Macro M10 unit probes: Service DI scan / E006 / E007 / registry codegen.

use smol_str::SmolStr;
use syx_ast::{AccessLevel, Attribute, Decl};

use crate::service::{collect_services, expand_service_macros, reject_service_on_non_types};

fn diag_span() -> syx_diagnostics::Span {
    syx_diagnostics::Span {
        start: 0,
        len: 1,
    }
}

#[test]
fn p120_service_e006_on_function() {
    let decls = vec![Decl::Function {
        span: diag_span(),
        access: AccessLevel::Public,
        attrs: vec![Attribute {
            name: SmolStr::new("Service"),
            args: vec![],
        }],
        name: SmolStr::new("bad"),
        type_params: vec![],
        params: vec![],
        return_type: None,
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body: vec![],
        is_async: false,
        is_override: false,
        is_final: false,
    }];
    let err = reject_service_on_non_types(&decls).expect_err("E006");
    assert_eq!(err.code.as_deref(), Some("E006"));
}

#[test]
fn p120_service_scan_and_boot_codegen() {
    let src = r#"
@Service
class Logger {
  init() {}
}

@Service
class App {
  init(@Inject logger: Logger) {}
}

func main() {}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    expand_service_macros(&mut module).expect("expand service");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services.len(), 2);
    let app = services.iter().find(|s| s.type_name == "App").unwrap();
    assert_eq!(app.deps, vec![SmolStr::new("Logger")]);
    assert!(module.decls.iter().any(|d| matches!(
        d,
        Decl::Function { name, .. } if name.as_str() == "__syx_service_boot_impl"
            || name.as_str() == "__syx_service_boot"
    )));
}

#[test]
fn p120_service_cycle_e007() {
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
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let err = expand_service_macros(&mut module).expect_err("cycle");
    assert_eq!(err.code.as_deref(), Some("E007"));
}

#[test]
fn p120_service_missing_dep_e008() {
    let src = r#"
@Service
class App {
  init(@Inject logger: Logger) {}
}
"#;
    let file = syx_diagnostics::SourceFile::new("t.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let err = expand_service_macros(&mut module).expect_err("missing");
    assert_eq!(err.code.as_deref(), Some("E008"));
}

#[test]
fn p120_macros_exports_schema_fingerprint() {
    let manifest = include_str!("../../syx-pkg/src/manifest.rs");
    assert!(manifest.contains("MacrosField"));
    assert!(manifest.contains("exports"));
}

#[test]
fn p120_expand_example_service_di_has_boot() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/service_di/main.sy"
    );
    let src = std::fs::read_to_string(path).expect("read example");
    let file = syx_diagnostics::SourceFile::new("main.sy", src);
    let mut module = syx_parser::Parser::parse(file).expect("parse");
    let services = crate::service::collect_services(&module).expect("collect");
    assert!(
        services.len() >= 2,
        "expected App + loggers (+ optional lazy/cycle), got {services:?}"
    );
    module = crate::expand_module(module).expect("expand");
    assert!(
        module.decls.iter().any(|d| {
            matches!(
                d,
                Decl::Function { name, .. }
                    if name.as_str() == "__syx_service_boot"
                        || name.as_str() == "__syx_service_boot_impl"
            )
        }),
        "boot missing after expand"
    );
}

