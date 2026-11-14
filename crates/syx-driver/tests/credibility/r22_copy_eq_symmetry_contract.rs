//! R-22 copy-return-type + equality symmetry contract.
//!
//! MIR binary ops dispatch on the **left** operand type only (`Call("{T}_==")`).
//! Before R-22, `T.copy()` had no return-type inference, so `p.copy() == p` fell
//! through to `ValueEq` (identity) and printed `false`, while `p == p.copy()`
//! called `{T}_==` and printed `true`. Bound copies also lost `.description`
//! (`ObjectGet("description")` → `nil`).
//!
//! Fix: `dataclass_shell_method_return_type` treats `{T}.copy()` as returning `T`
//! for struct / data class receivers. This contract pins symmetry + describe labels
//! on both execution paths, and nails the mir_unify replacing-merge wiring.

use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};

const SYMMETRY_PROBE: &str = r#"
struct Point {
    var x: Int
    var y: Int
}

func main() {
    let p = Point(x: 3, y: 4)
    let c = p.copy()
    print(p == c)
    print(c == p)
    print(p == p.copy())
    print(p.copy() == p)
    print(p.description)
    print(c.description)
}
"#;

fn run_embedded(src: &str) -> Vec<String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("embedded run")
        .printed
}

fn run_as_file(src: &str) -> Vec<String> {
    // Unique temp path so incremental cache from earlier probes cannot shadow results.
    let dir = std::env::temp_dir().join(format!(
        "syx_r22_symmetry_{}",
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

fn expect_symmetry(lines: &[String]) {
    assert_eq!(
        lines,
        &[
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
            "Point(x: 3, y: 4)".to_string(),
            "Point(x: 3, y: 4)".to_string(),
        ],
        "copy equality must be symmetric and description must label field 0\ngot={lines:?}"
    );
}

#[test]
fn r22_copy_eq_symmetric_embedded() {
    expect_symmetry(&run_embedded(SYMMETRY_PROBE));
}

#[test]
fn r22_copy_eq_symmetric_file_path() {
    let embedded = run_embedded(SYMMETRY_PROBE);
    let as_file = run_as_file(SYMMETRY_PROBE);
    assert_eq!(
        embedded, as_file,
        "embedded and file paths must agree\nembedded={embedded:?}\nfile={as_file:?}"
    );
    expect_symmetry(&as_file);
}

#[test]
fn r22_copy_return_type_infer_wired() {
    let infer = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("crates/syx-mir/src/mir_type_infer_helpers.rs"),
    )
    .expect("mir_type_infer_helpers.rs");
    assert!(
        infer.contains("fn dataclass_shell_method_return_type"),
        "copy return-type helper must exist"
    );
    assert!(
        infer.contains("method.as_str() == \"copy\"")
            && infer.contains("dataclass_names.contains(recv)"),
        "helper must recognize dataclass/struct .copy() as returning the receiver type"
    );
    assert!(
        infer.matches("dataclass_shell_method_return_type(").count() >= 3,
        "helper must be used from both infer_expr_init_type and infer_object_type_name"
    );
}

#[test]
fn r22_mir_unify_shells_use_replacing_merge() {
    let unify = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/mir_unify.rs"),
    )
    .expect("mir_unify.rs");
    // Six shell merges: 3 dataclass + 3 enum.
    let dc_shells = unify
        .matches("merge_mir_shell_functions(&mut mir, syx_mir::lower_dataclass_shells")
        .count()
        + unify
            .matches("merge_mir_shell_functions(mir, syx_mir::lower_dataclass_shells")
            .count();
    let enum_shells = unify
        .matches("merge_mir_shell_functions(&mut mir, syx_mir::lower_enum_shells")
        .count()
        + unify
            .matches("merge_mir_shell_functions(mir, syx_mir::lower_enum_shells")
            .count();
    // Driver mir_unify hosts the multi-module / mono sites; syx-mir/codegen own the other shell merges.
    assert!(
        dc_shells >= 2,
        "mir_unify must replace-merge dataclass shells (got {dc_shells})"
    );
    assert!(
        enum_shells >= 2,
        "mir_unify must replace-merge enum shells (got {enum_shells})"
    );
    assert!(
        !unify.contains("merge_mir_functions(mir, syx_mir::lower_dataclass_shells")
            && !unify.contains("merge_mir_functions(&mut mir, syx_mir::lower_dataclass_shells")
            && !unify.contains("merge_mir_functions(mir, syx_mir::lower_enum_shells")
            && !unify.contains("merge_mir_functions(&mut mir, syx_mir::lower_enum_shells"),
        "no skip-style merge remains for dataclass/enum shells"
    );
}

#[test]
fn r22_describe_labels_first_field() {
    let shells = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("crates/syx-mir/src/mir_lower_shells.rs"),
    )
    .expect("mir_lower_shells.rs");
    assert!(
        shells.contains("format!(\"{field}: \")") || shells.contains("format!(\"{field}:\")"),
        "describe shell must emit a label for field 0"
    );
    // Prefer the exact R-22 form used in source.
    assert!(
        shells.contains("format!(\"{field}: \")"),
        "describe field-0 prefix must be `{{field}}: `"
    );
}
