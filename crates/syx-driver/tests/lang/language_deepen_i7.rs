//! S1-I7: closures stored/ternary, T: A & B, extension deepen, Iterator for-in.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i7_closures_book_stored_check_and_run() {
    let path = fixture("lang/closures_book_stored.sy");
    compile_file_with_options(&path, &opts()).expect("closures stored check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("closures stored run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("42") || out.contains('5'), "got {out:?}");
    assert!(out.contains("later") || out.contains('5'), "got {out:?}");
}

#[test]
fn s1_i7_protocol_compose_ab_check_and_run() {
    let path = fixture("lang/protocol_compose_ab.sy");
    compile_file_with_options(&path, &opts()).expect("compose A&B check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("compose A&B run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('a') && out.contains('b'), "got {out:?}");
}

#[test]
fn s1_i7_protocol_iterator_forin_check_and_run() {
    let path = fixture("lang/protocol_iterator_forin.sy");
    compile_file_with_options(&path, &opts()).expect("iterator for-in check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("iterator for-in run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('6'), "got {out:?}");
}

#[test]
fn s1_i7_extension_int_doubled_times_check_and_run() {
    let path = fixture("lang/extension_int_doubled_times.sy");
    compile_file_with_options(&path, &opts()).expect("extension Int check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("extension Int run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('6'), "got {out:?}");
    assert!(out.matches("hi").count() >= 3, "got {out:?}");
}

#[test]
fn s1_i7_extension_point_convenience_init_check_and_run() {
    let path = fixture("lang/extension_point_init.sy");
    compile_file_with_options(&path, &opts()).expect("extension Point init check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("extension Point init run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('0'), "got {out:?}");
}

#[test]
fn s1_i7_control_if_expr_check_and_run() {
    let path = fixture("lang/control_if_expr.sy");
    compile_file_with_options(&path, &opts()).expect("if-expr check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("if-expr run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("pos"), "got {out:?}");
}

#[test]
fn s1_i7_control_throws_sig_check() {
    let path = fixture("lang/control_throws_sig.sy");
    compile_file_with_options(&path, &opts()).expect("throws -> T check");
}
