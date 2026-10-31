//! S1-I8: zipMap, Ch22/23 remainder, labeled break, opaque thin.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i8_zipmap_book_check_and_run() {
    let path = fixture("lang/zipmap_book.sy");
    compile_file_with_options(&path, &opts()).expect("zipMap check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("zipMap run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(
        out.contains("1-a") && out.contains("2-b") && out.contains("3-c"),
        "got {out:?}"
    );
}

#[test]
fn s1_i8_labeled_break_check() {
    let path = fixture("lang/labeled_break.sy");
    compile_file_with_options(&path, &opts()).expect("labeled break check");
}
