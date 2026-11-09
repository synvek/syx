//! S1-I3: `<T:P>`, `$0` trailing, generics id<T>, enum switch patterns.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_03_closure_shorthand_dollar_check_and_run() {
    let path = fixture("lang/closure_shorthand_dollar.sy");
    compile_file_with_options(&path, &opts()).expect("$0 check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("$0 run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('2') || out.contains('6') || out.contains("15"), "got {out:?}");
}

#[test]
fn s1_03_enum_status_ok_check() {
    let path = fixture("lang/enum_book_status_check.sy");
    compile_file_with_options(&path, &opts()).expect("Status.ok check");
}

#[test]
fn s1_05_generics_book_id_check_and_run() {
    let path = fixture("lang/generics_book_id.sy");
    compile_file_with_options(&path, &opts()).expect("id<T> check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("id<T> run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("42"), "got {out:?}");
    assert!(out.contains("hi"), "got {out:?}");
    assert!(out.contains('7'), "got {out:?}");
}

#[test]
fn s1_i3_enum_switch_associated_patterns_check_and_run() {
    let path = fixture("lang/enum_book_switch_patterns.sy");
    compile_file_with_options(&path, &opts()).expect("switch patterns check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("switch patterns run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("OK"), "got {out:?}");
    assert!(out.contains("Not Found"), "got {out:?}");
}
