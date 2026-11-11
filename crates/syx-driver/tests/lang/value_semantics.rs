//! R-31: struct / Array / Dict / Set value semantics (assign isolation).

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn run(rel: &str) -> (i32, Vec<String>) {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    run_file_printed(&path, &options).expect("run")
}

fn lines_trimmed(lines: &[String]) -> Vec<String> {
    lines.iter().map(|l| l.trim().to_string()).collect()
}

#[test]
fn struct_assign_isolates_field_mutation() {
    let (code, lines) = run("lang/value_semantics_struct.sy");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t = lines_trimmed(&lines);
    assert!(
        t.windows(2).any(|w| w == ["1", "9"]),
        "expected p.x=1 then q.x=9, got {t:?}"
    );
}

#[test]
fn array_assign_isolates_append() {
    let (code, lines) = run("lang/value_semantics_array.sy");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t = lines_trimmed(&lines);
    assert!(
        t.windows(2).any(|w| w == ["1", "2"]),
        "expected a.count=1 then b.count=2, got {t:?}"
    );
}

#[test]
fn dict_assign_isolates_remove() {
    let (code, lines) = run("lang/value_semantics_dict.sy");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t = lines_trimmed(&lines);
    assert!(
        t.windows(2).any(|w| w == ["1", "0"]),
        "expected a.count=1 then b.count=0, got {t:?}"
    );
}

#[test]
fn set_assign_isolates_insert() {
    let (code, lines) = run("lang/value_semantics_set.sy");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t = lines_trimmed(&lines);
    assert!(
        t.windows(2).any(|w| w == ["1", "2"]),
        "expected a.count=1 then b.count=2, got {t:?}"
    );
}
