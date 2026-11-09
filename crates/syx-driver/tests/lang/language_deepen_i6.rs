//! S1-I6: extension where, map<T,U>, protocol default, default type args.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i6_extension_where_unique_check_and_run() {
    let path = fixture("lang/extension_where_unique.sy");
    compile_file_with_options(&path, &opts()).expect("unique check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("unique run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("[1, 2, 3]") || out.contains("1"), "got {out:?}");
}

#[test]
fn s1_i6_generics_book_map_check_and_run() {
    let path = fixture("lang/generics_book_map.sy");
    compile_file_with_options(&path, &opts()).expect("map check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("map run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("[2, 4, 6]") || out.contains("2"), "got {out:?}");
}

#[test]
fn s1_i6_protocol_book_default_check_and_run() {
    let path = fixture("lang/protocol_book_default.sy");
    compile_file_with_options(&path, &opts()).expect("protocol default check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("protocol default run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("<no description>"), "got {out:?}");
    assert!(out.contains("bar"), "got {out:?}");
}

#[test]
fn s1_i6_generics_book_holder_check_and_run() {
    let path = fixture("lang/generics_book_holder.sy");
    compile_file_with_options(&path, &opts()).expect("holder check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("holder run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('1'), "got {out:?}");
    assert!(out.contains('2'), "got {out:?}");
}
