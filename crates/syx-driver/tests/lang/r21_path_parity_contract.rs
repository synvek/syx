//! R-21 execution-path parity contract.
//!
//! `compile_and_run_embedded` (used by ~204 test files) and `run_file_printed` (used by
//! ~67) compile the same source through different pipelines. They used to disagree on
//! struct field reads: the embedded path returned `nil` for every field because a generic
//! zero-arg `{Type}_init` from `lower_class_init` shadowed the memberwise dataclass shell.
//!
//! Root cause: `merge_mir_functions` only appends when the name is absent, so the correct
//! shell was silently dropped. Fixed by `merge_mir_shell_functions`, which replaces.
//!
//! These tests pin that the two paths agree, so a future pipeline change cannot silently
//! reintroduce the divergence.

use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};

/// One source snippet exercised through both entry points.
const STRUCT_PROBE: &str = r#"
struct Point {
    var x: Int
    var y: Int
}

func main() {
    let p = Point(x: 3, y: 4)
    let q = Point(x: 3, y: 4)
    let r = Point(x: 9, y: 9)
    print(p.x)
    print(r.x)
    print(p == q)
    print(p == r)
    print(p != r)
}
"#;

/// Run `src` through the embedded pipeline and return its printed lines.
fn run_embedded(src: &str) -> Vec<String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("embedded run")
        .printed
}

/// Run `src` as a real file (the authoritative CLI path) and return its printed lines.
fn run_as_file(src: &str) -> Vec<String> {
    let dir = std::env::temp_dir().join(format!(
        "syx_r21_parity_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let file = dir.join("main.sy");
    std::fs::write(&file, src).expect("write source");
    let printed = run_file_printed(&file, &DriverOptions::default())
        .expect("file run")
        .1;
    let _ = std::fs::remove_dir_all(&dir);
    printed
}

#[test]
fn r21_struct_field_reads_agree_across_paths() {
    let embedded = run_embedded(STRUCT_PROBE);
    let as_file = run_as_file(STRUCT_PROBE);
    assert_eq!(
        embedded, as_file,
        "embedded and file pipelines must print identical lines\nembedded={embedded:?}\nfile={as_file:?}"
    );
    // And the shared answer must be the correct one, not both-wrong-but-equal.
    assert_eq!(
        embedded,
        vec!["3", "9", "true", "false", "true"],
        "struct fields must read back real values"
    );
}

#[test]
fn r21_data_class_members_agree_across_paths() {
    let src = r#"
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    let q = Point(3, 4)
    print(p.x)
    print(p == q)
    print(p.description)
}
"#;
    let embedded = run_embedded(src);
    let as_file = run_as_file(src);
    assert_eq!(
        embedded, as_file,
        "data class outputs must agree across paths\nembedded={embedded:?}\nfile={as_file:?}"
    );
    assert_eq!(embedded[0], "3", "data class field must read back");
}

#[test]
fn r21_shell_merge_replaces_generic_init() {
    // Source-level nail on the mechanism: shells must go through the replacing merge,
    // not the append-only one, or the zero-arg generic init wins again.
    let codegen = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("crates/syx-codegen/src/lib.rs"),
    )
    .expect("syx-codegen lib.rs");
    assert!(
        codegen
            .contains("merge_mir_shell_functions(&mut mir_work, syx_mir::lower_dataclass_shells"),
        "dataclass shells must use the replacing merge"
    );
    assert!(
        codegen.contains("merge_mir_shell_functions(&mut mir_work, syx_mir::lower_enum_shells"),
        "enum shells must use the replacing merge"
    );

    let shells = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("crates/syx-mir/src/mir_lower_shells.rs"),
    )
    .expect("mir_lower_shells.rs");
    assert!(
        shells.contains("pub fn merge_mir_shell_functions"),
        "the replacing merge helper must exist"
    );
    assert!(
        shells.contains("*existing = func"),
        "the replacing merge must overwrite the existing function"
    );
}
