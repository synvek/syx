//! S1-I4: book closures `{ x in }`, Equatable enum, dict/[K:V] probes.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

#[test]
fn s1_i4_closures_book_in_check_and_run() {
    let path = fixture("lang/closures_book_in.sy");
    compile_file_with_options(&path, &opts()).expect("closures book check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("closures book run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("42"), "got {out:?}");
    assert!(out.contains('4'), "got {out:?}");
    assert!(out.contains('3'), "got {out:?}");
    assert!(out.contains('6'), "got {out:?}");
}

#[test]
fn s1_i4_dict_book_kv_check_and_run() {
    let path = fixture("lang/dict_book_kv.sy");
    compile_file_with_options(&path, &opts()).expect("dict [K:V] check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("dict run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains('1'), "got {out:?}");
    assert!(out.contains('0'), "got {out:?}");
}

#[test]
fn s1_i4_equatable_book_enum_check_and_run() {
    let path = fixture("lang/equatable_book_enum.sy");
    compile_file_with_options(&path, &opts()).expect("Equatable enum check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("Equatable enum run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("false"), "got {out:?}");
    assert!(out.contains("true"), "got {out:?}");
}

#[test]
fn s1_i4_generics_book_id_eq_and_first() {
    let path = fixture("lang/generics_book_id.sy");
    compile_file_with_options(&path, &opts()).expect("generics id/eq/first check");
    let (code, lines) = run_file_printed(&path, &opts()).expect("generics run");
    assert_eq!(code, 0);
    let out = lines.join("\n");
    assert!(out.contains("42"), "got {out:?}");
    assert!(out.contains("true"), "got {out:?}");
}
