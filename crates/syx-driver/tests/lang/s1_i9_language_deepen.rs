//! S1-I9: top-level await, Channel book stub, @resultBuilder struct, subscript thin.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i9_result_builder_struct_check_and_run() {
    let path = fixture("lang/result_builder_struct.sy");
    compile_file_with_options(&path, &opts()).expect("resultBuilder struct check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("resultBuilder struct run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("1") && out.contains("2") && out.contains("3"), "got {out:?}");
}

#[test]
fn s1_i9_labeled_break_still_checks() {
    let path = fixture("lang/labeled_break.sy");
    compile_file_with_options(&path, &opts()).expect("labeled break check");
}
