//! S1-I5: protocol requirements, case let / where, compose mono.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i5_protocol_book_reqs_check_and_run() {
    let path = fixture("lang/protocol_book_reqs.sy");
    compile_file_with_options(&path, &opts()).expect("protocol reqs check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("protocol reqs run");
    assert_eq!(code, 0);
    assert!(lines.join("\n").contains("User"), "got {lines:?}");
}

#[test]
fn s1_i5_control_flow_case_let_check_and_run() {
    let path = fixture("lang/control_flow_case_let.sy");
    compile_file_with_options(&path, &opts()).expect("case let check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("case let run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("even"), "got {out:?}");
    assert!(out.contains("hi"), "got {out:?}");
    assert!(out.contains("42"), "got {out:?}");
}

#[test]
fn s1_i5_generics_book_compose_check_and_run() {
    let path = fixture("lang/generics_book_compose.sy");
    compile_file_with_options(&path, &opts()).expect("compose check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("compose run");
    assert_eq!(code, 0);
    assert!(lines.join("\n").contains('7'), "got {lines:?}");
}
