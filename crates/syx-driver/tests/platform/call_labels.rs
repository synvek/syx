//! Named parameters — external labels, `_` omit, required labels
//! (`lang-call-v1:external-label-mvp`, `lang-call-v1:omit-underscore-mvp`, `lang-call-v1:labeled-args-required`).

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn run_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    let (code, printed) = run_file_printed(&path, &options).expect("run fixture");
    assert_eq!(code, 0, "{printed:?}");
    printed
}

fn compile_fail(src: &str, code: &str) -> bool {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    let sem = syx_semantic::SemanticAnalyzer::analyze(&module);
    match sem {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn call_labels_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/call_labels.sy");
    assert!(
        printed.iter().any(|l| l.contains("hi SYX")),
        "greet(name:): {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "home"),
        "move(to:): {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "worker"),
        "accept positional: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "3"),
        "pair(1, second: 2): {printed:?}"
    );
}

#[test]
fn call_labels_required_labels_rejected() {
    assert!(compile_fail(
        r#"func f(x: Int) { } func main() { f(1) }"#,
        "S042"
    ));
}

#[test]
fn call_labels_underscore_rejects_label() {
    assert!(compile_fail(
        r#"func f(_ x: Int) { } func main() { f(x: 1) }"#,
        "S042"
    ));
}

#[test]
fn call_labels_external_not_internal() {
    assert!(compile_fail(
        r#"func f(to x: Int) { } func main() { f(x: 1) }"#,
        "S042"
    ));
    assert!(!compile_fail(
        r#"func f(to x: Int) { } func main() { f(to: 1) }"#,
        "S042"
    ));
}

#[test]
fn call_labels_fingerprint_strings() {
    let lib = include_str!("../../src/lib.rs");
    let surf = include_str!("../../src/lang_surface_v1.rs");
    assert!(surf.contains("lang-call-v1:external-label-mvp"));
    assert!(surf.contains("lang-call-v1:omit-underscore-mvp"));
    assert!(surf.contains("lang-call-v1:labeled-args-required"));
    assert!(surf.contains("lang-call-v1:docs-honesty"));
    assert!(lib.contains("CALL_EXTERNAL_LABEL_MVP"));
    assert!(lib.contains("lang_surface_v1"));
}
