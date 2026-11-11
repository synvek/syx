//! R-36: diagnostics quality — no Unknown leak / MIR001·C020·R003 spans / Error print E.bad.

use syx_driver::{
    check_file_with_options, compile_file_with_options, run_file_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn display_errs(errs: &[syx_diagnostics::SyxDiagnostic]) -> String {
    errs.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn s004_does_not_leak_unknown_token() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/diag_no_unknown_leak.sy");
    let errs = check_file_with_options(&path, &opts()).expect_err("expected S004");
    let text = display_errs(&errs);
    assert!(
        errs.iter().any(|e| e.code.as_deref() == Some("S004")),
        "expected S004, got {text}"
    );
    assert!(
        !text.contains("Unknown"),
        "S004 must not leak Unknown: {text}"
    );
}

#[test]
fn c020_display_has_file_line_col() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/diag_c020_has_span.sy");
    let err = compile_file_with_options(&path, &opts()).expect_err("expected C020");
    let text = err.to_string();
    assert_eq!(err.code.as_deref(), Some("C020"), "{text}");
    assert!(
        text.contains(".sy:") && text.contains(": [C020]"),
        "C020 Display must include file:line:col, got {text}"
    );
}

#[test]
fn mir001_display_has_file_line_col() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/diag_mir001_has_span.sy");
    let err = compile_file_with_options(&path, &opts()).expect_err("expected MIR001");
    let text = err.to_string();
    assert_eq!(err.code.as_deref(), Some("MIR001"), "{text}");
    assert!(
        text.contains(".sy:") && text.contains(": [MIR001]"),
        "MIR001 Display must include file:line:col, got {text}"
    );
}

#[test]
fn r003_display_has_file_line_col() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/diag_r003_has_span.sy");
    compile_file_with_options(&path, &opts()).expect("compile oob");
    let err = match run_file_printed(&path, &opts()) {
        Ok((code, lines)) => panic!("expected R003, exit={code} lines={lines:?}"),
        Err(e) => e,
    };
    let text = err.to_string();
    assert_eq!(err.code.as_deref(), Some("R003"), "{text}");
    assert!(
        text.contains(".sy:") && text.contains(": [R003]"),
        "R003 Display must include file:line:col, got {text}"
    );
}

#[test]
fn error_case_prints_as_type_dot_case() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/diag_error_print.sy");
    compile_file_with_options(&path, &opts()).expect("compile E.bad");
    let (code, lines) = run_file_printed(&path, &opts()).expect("run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(
        t.iter().any(|l| *l == "E.bad"),
        "expected E.bad, got {t:?}"
    );
    assert!(
        !t.iter().any(|l| l.contains("{case:") || l.contains("tag:")),
        "must not print internal bag, got {t:?}"
    );
}