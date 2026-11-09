//! Book / language-surface deepen: compile (+ run) fixture probes.

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn check_and_run(rel: &str, label: &str) -> (i32, String) {
    let path = fixture(rel);
    compile_file_with_options(&path, &opts()).unwrap_or_else(|e| panic!("{label} check: {e:?}"));
    let (code, lines) = run_file_printed(&path, &opts()).unwrap_or_else(|e| panic!("{label} run: {e:?}"));
    (code, lines.join("\n"))
}

fn check_only(rel: &str, label: &str) {
    let path = fixture(rel);
    compile_file_with_options(&path, &opts()).unwrap_or_else(|e| panic!("{label} check: {e:?}"));
}

#[test]
fn closure_shorthand_dollar_check_and_run() {
    let (code, out) = check_and_run("lang/closure_shorthand_dollar.sy", "$0");
    assert_eq!(code, 0);
    assert!(out.contains('2') || out.contains('6') || out.contains("15"), "got {out:?}");
}

#[test]
fn enum_status_ok_check() {
    check_only("lang/enum_book_status_check.sy", "Status.ok");
}

#[test]
fn generics_book_id_check_and_run() {
    let (code, out) = check_and_run("lang/generics_book_id.sy", "id<T>");
    assert_eq!(code, 0);
    assert!(out.contains("42"), "got {out:?}");
    assert!(out.contains("hi"), "got {out:?}");
    assert!(out.contains('7'), "got {out:?}");
    assert!(out.contains("true"), "got {out:?}");
}

#[test]
fn enum_switch_associated_patterns_check_and_run() {
    let (code, out) = check_and_run("lang/enum_book_switch_patterns.sy", "switch patterns");
    assert_eq!(code, 0);
    assert!(out.contains("OK"), "got {out:?}");
    assert!(out.contains("Not Found"), "got {out:?}");
}

#[test]
fn closures_book_in_check_and_run() {
    let (code, out) = check_and_run("lang/closures_book_in.sy", "closures book");
    assert_eq!(code, 0);
    assert!(out.contains("42"), "got {out:?}");
    assert!(out.contains('4'), "got {out:?}");
    assert!(out.contains('3'), "got {out:?}");
    assert!(out.contains('6'), "got {out:?}");
}

#[test]
fn dict_book_kv_check_and_run() {
    let (code, out) = check_and_run("lang/dict_book_kv.sy", "dict [K:V]");
    assert_eq!(code, 0);
    assert!(out.contains('1'), "got {out:?}");
    assert!(out.contains('0'), "got {out:?}");
}

#[test]
fn equatable_book_enum_check_and_run() {
    let (code, out) = check_and_run("lang/equatable_book_enum.sy", "Equatable enum");
    assert_eq!(code, 0);
    assert!(out.contains("false"), "got {out:?}");
    assert!(out.contains("true"), "got {out:?}");
}

#[test]
fn protocol_book_reqs_check_and_run() {
    let (code, out) = check_and_run("lang/protocol_book_reqs.sy", "protocol reqs");
    assert_eq!(code, 0);
    assert!(out.contains("User"), "got {out:?}");
}

#[test]
fn control_flow_case_let_check_and_run() {
    let (code, out) = check_and_run("lang/control_flow_case_let.sy", "case let");
    assert_eq!(code, 0);
    assert!(out.contains("even"), "got {out:?}");
    assert!(out.contains("hi"), "got {out:?}");
    assert!(out.contains("42"), "got {out:?}");
}

#[test]
fn generics_book_compose_check_and_run() {
    let (code, out) = check_and_run("lang/generics_book_compose.sy", "compose");
    assert_eq!(code, 0);
    assert!(out.contains('7'), "got {out:?}");
}

#[test]
fn extension_where_unique_check_and_run() {
    let (code, out) = check_and_run("lang/extension_where_unique.sy", "unique");
    assert_eq!(code, 0);
    assert!(out.contains("[1, 2, 3]") || out.contains("1"), "got {out:?}");
}

#[test]
fn generics_book_map_check_and_run() {
    let (code, out) = check_and_run("lang/generics_book_map.sy", "map");
    assert_eq!(code, 0);
    assert!(out.contains("[2, 4, 6]") || out.contains("2"), "got {out:?}");
}

#[test]
fn protocol_book_default_check_and_run() {
    let (code, out) = check_and_run("lang/protocol_book_default.sy", "protocol default");
    assert_eq!(code, 0);
    assert!(out.contains("<no description>"), "got {out:?}");
    assert!(out.contains("bar"), "got {out:?}");
}

#[test]
fn generics_book_holder_check_and_run() {
    let (code, out) = check_and_run("lang/generics_book_holder.sy", "holder");
    assert_eq!(code, 0);
    assert!(out.contains('1'), "got {out:?}");
    assert!(out.contains('2'), "got {out:?}");
}

#[test]
fn closures_book_stored_check_and_run() {
    let (code, out) = check_and_run("lang/closures_book_stored.sy", "closures stored");
    assert_eq!(code, 0);
    assert!(out.contains("42") || out.contains('5'), "got {out:?}");
    assert!(out.contains("later") || out.contains('5'), "got {out:?}");
}

#[test]
fn protocol_compose_ab_check_and_run() {
    let (code, out) = check_and_run("lang/protocol_compose_ab.sy", "compose A&B");
    assert_eq!(code, 0);
    assert!(out.contains('a') && out.contains('b'), "got {out:?}");
}

#[test]
fn protocol_iterator_forin_check_and_run() {
    let (code, out) = check_and_run("lang/protocol_iterator_forin.sy", "iterator for-in");
    assert_eq!(code, 0);
    assert!(out.contains('6'), "got {out:?}");
}

#[test]
fn extension_int_doubled_times_check_and_run() {
    let (code, out) = check_and_run("lang/extension_int_doubled_times.sy", "extension Int");
    assert_eq!(code, 0);
    assert!(out.contains('6'), "got {out:?}");
    assert!(out.matches("hi").count() >= 3, "got {out:?}");
}

#[test]
fn extension_point_convenience_init_check_and_run() {
    let (code, out) = check_and_run("lang/extension_point_init.sy", "extension Point init");
    assert_eq!(code, 0);
    assert!(out.contains('0'), "got {out:?}");
}

#[test]
fn control_if_expr_check_and_run() {
    let (code, out) = check_and_run("lang/control_if_expr.sy", "if-expr");
    assert_eq!(code, 0);
    assert!(out.contains("pos"), "got {out:?}");
}

#[test]
fn control_throws_sig_check() {
    check_only("lang/control_throws_sig.sy", "throws -> T");
}

#[test]
fn zipmap_book_check_and_run() {
    let (code, out) = check_and_run("lang/zipmap_book.sy", "zipMap");
    assert_eq!(code, 0);
    assert!(
        out.contains("1-a") && out.contains("2-b") && out.contains("3-c"),
        "got {out:?}"
    );
}

#[test]
fn labeled_break_check() {
    check_only("lang/labeled_break.sy", "labeled break");
}

#[test]
fn result_builder_struct_check_and_run() {
    let (code, out) = check_and_run("lang/result_builder_struct.sy", "resultBuilder struct");
    assert_eq!(code, 0);
    assert!(out.contains("1") && out.contains("2") && out.contains("3"), "got {out:?}");
}
