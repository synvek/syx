//! Phase 123 Macro M13 unit probes: request scope, ceremony sugar, typed ident resolve, RB HostCall+Closure.

use smol_str::SmolStr;

use crate::service::{collect_services, expand_service_macros, spec_to_meta_entry};

#[test]
fn p123_service_scope_request_accepted() {
    let src = r#"
@Scope("request")
@Service
class App { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    assert_eq!(services[0].scope, SmolStr::new("request"));
}

#[test]
fn p123_service_meta_entry_request_scope() {
    let src = r#"
@Scope("request")
@Service
class App { init() {} }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let services = collect_services(&module).expect("collect");
    let entry = spec_to_meta_entry(&services[0]);
    assert_eq!(entry.scope, "request");
}

#[test]
fn p123_expand_service_macros_emits_resolve_sugar() {
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
    assert!(dump.contains("resolve"), "ceremony sugar resolve wrapper: {dump}");
}

#[test]
fn p123_resultbuilder_hostcall_closure_keeps_body_in_closure() {
    let src = r#"
@resultBuilder
func Mixed() -> Array<String> {
    "lead"
    for x in ["a"] {
        __syx_str_length(x)
        x
    }
}
func main() { print(1) }
"#;
    let module =
        syx_parser::Parser::parse(syx_diagnostics::SourceFile::new("t.sy", src)).expect("parse");
    let expanded = crate::expand_module(module).expect("expand");
    let dump = format!("{expanded:?}");
    assert!(
        dump.contains("__syx_str_length") && dump.contains("Closure"),
        "HostCall+Closure must stay inside flat_map closure: {dump}"
    );
}

#[test]
fn p123_resultbuilder_literal_for_in_still_expands() {
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
