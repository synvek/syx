//! R-20 struct / data class operator contract.
//!
//! Nails two things that were silently broken before this window:
//!   1. `==` / `!=` on a value type compile and produce structural-equality results.
//!   2. An operator that is *not* synthesized reports an actionable diagnostic instead
//!      of a bare `[C020] Unknown function 'Point_+'`.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn run(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r.printed.join("\n"),
        Err(e) => panic!("compile/run failed: {e}\n---src---\n{src}"),
    }
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        Err(e) => format!("{e}"),
    }
}

#[test]
fn r20_struct_equality_is_structural() {
    // R-21: the two pipelines now agree (see `r21_path_parity_contract`), so all four
    // cases are asserted on the embedded path directly.
    let out = run(
        r#"
struct Point {
    var x: Int
    var y: Int
}

func main() {
    let p = Point(x: 3, y: 4)
    let q = Point(x: 3, y: 4)
    let r = Point(x: 9, y: 9)
    print(p == q)
    print(p != q)
    print(p == r)
    print(p != r)
}
"#,
    );
    assert_eq!(
        out, "true\nfalse\nfalse\ntrue",
        "struct == / != must be structural: {out}"
    );
}

#[test]
fn r20_dataclass_equality_is_structural() {
    let out = run(
        r#"
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    let q = Point(3, 4)
    let r = p.copy()
    print(p == q)
    print(p == r)
}
"#,
    );
    assert_eq!(
        out, "true\ntrue",
        "data class == must be structural, including after copy(): {out}"
    );
}

#[test]
fn r20_dataclass_fixture_runs() {
    // The parity fixture that used to fail with `[C020] Unknown function 'Point_=='`.
    // Run it on the file path so it matches the CLI semantics asserted above.
    let path = repo_path("crates/syx-driver/tests/fixtures/parity/struct_eq.sy");
    let out = syx_driver::run_file_printed(&path, &DriverOptions::default())
        .expect("run struct_eq fixture")
        .1
        .join("\n");
    assert_eq!(out, "true\nfalse\nfalse\ntrue", "fixture output: {out}");
}

#[test]
fn r20_unsynthesized_operator_has_actionable_diagnostic() {
    let err = run_err(
        r#"
data class P(x: Int)

func main() {
    let a = P(x: 1)
    let b = P(x: 2)
    print(a + b)
}
"#,
    );
    assert!(
        err.contains("has no '+' operator"),
        "expected an actionable operator diagnostic, got: {err}"
    );
    assert!(
        err.contains("only '==' and '!=' are synthesized"),
        "diagnostic must state which operators exist, got: {err}"
    );
    // The bare form must not leak through.
    assert!(
        !err.contains("Unknown function 'P_+'"),
        "bare Unknown function diagnostic must be replaced, got: {err}"
    );
}

#[test]
fn r20_equality_shells_are_synthesized() {
    // Source-level nail: the shells must exist and be wired into the shell list, so a
    // future refactor cannot silently drop them again.
    let shells = std::fs::read_to_string(repo_path("crates/syx-mir/src/mir_lower_shells.rs"))
        .expect("mir_lower_shells.rs");
    assert!(
        shells.contains("fn lower_dataclass_eq_shell"),
        "== shell must exist"
    );
    assert!(
        shells.contains("fn lower_dataclass_ne_shell"),
        "!= shell must exist"
    );
    assert!(
        shells.contains("out.push(lower_dataclass_eq_shell(name, fields));"),
        "== shell must be wired into lower_dataclass_shells"
    );
    assert!(
        shells.contains("out.push(lower_dataclass_ne_shell(name, fields));"),
        "!= shell must be wired into lower_dataclass_shells"
    );
}
