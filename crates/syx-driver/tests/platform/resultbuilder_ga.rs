//! Phase 102 Track C: `@resultBuilder` GA — multi-stmt/nested if/for gate.

use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn p102_resultbuilder_ga_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("lang-resultbuilder-ga-v1:multistmt-nested"));
}

#[test]
fn p102_resultbuilder_ga_build_protocol() {
    let macro_src = include_str!("../../../syx-macro/src/result_builder.rs");
    assert!(macro_src.contains("__syx_result_builder_buildEither"));
    assert!(macro_src.contains("__syx_result_builder_buildBlock"));
    assert!(macro_src.contains("__syx_result_builder_buildArray"));
    assert!(macro_src.contains("__syx_result_builder_buildOptional"));
    assert!(
        include_str!("../../../syx-macro/src/lib.rs").contains("desugar_result_builders"),
        "resultBuilder must be wired into syx-macro expand_module pipeline"
    );
}

#[test]
fn p103_resultbuilder_build_optional_stdout() {
    // Wave A W2 / ADR-346 / A-03: exact lines (was contains "42"/"99")
    let src = std::fs::read_to_string(fixture("tests/fixtures/lang/resultbuilder_build_optional.sy")).unwrap();
    let out = run(&src);
    assert_eq!(out, "2\n42\n99", "buildOptional exact stdout: {out}");
}

#[test]
fn p102_resultbuilder_ga_multistmt_nested_stdout() {
    let src = std::fs::read_to_string(fixture("tests/fixtures/language/resultbuilder_ga.sy")).unwrap();
    let out = run(&src);
    assert_eq!(
        out,
        "root\nnested-a\nloop\nloop\nrb-ga-done",
        "stable VStack builder stdout: {out}"
    );
}

#[test]
fn p102_resultbuilder_ga_two_expr_flat() {
    let out = run(
        r#"@resultBuilder
func pair() -> Array<String> {
    "one"
    "two"
}
func main() {
    let a = pair()
    print(a[0])
    print(a[1])
}"#,
    );
    assert_eq!(out, "one\ntwo");
}

#[test]
fn p102_resultbuilder_ga_map_host_smoke() {
    let out = run(
        r#"func main() {
    let r = __syx_array_flat_map([10, 20], { (x: Int) -> Array in return [x] })
    print(r.count)
    print(r[0])
}"#,
    );
    assert_eq!(out, "2\n10");
}

#[test]
fn p102_resultbuilder_ga_nested_if_branch() {
    let out = run(
        r#"@resultBuilder
func pick(flag: Bool, inner: Bool) -> Array<String> {
    if flag {
        if inner {
            "yes-yes"
        } else {
            "yes-no"
        }
    } else {
        "no"
    }
}
func main() {
    let a = pick(flag: true, inner: false)
    print(a[0])
}"#,
    );
    assert_eq!(out, "yes-no");
}

#[test]
fn p102_resultbuilder_ga_for_in_collect() {
    let out = run(
        r#"@resultBuilder
func rows() -> Array<Int> {
    for x in [10, 20] {
        x
    }
}
func main() {
    let r = rows()
    print(r[0])
    print(r[1])
}"#,
    );
    assert_eq!(out, "10\n20");
}

#[test]
fn p102_resultbuilder_host_direct() {
    let out = run(
        r#"func main() {
    let a = __syx_result_builder_buildBlock(["one"], ["two"])
    print(a.count)
    print(a[0])
    print(a[1])
}"#,
    );
    println!("HOST_DIRECT={out}");
    assert_eq!(out, "2\none\ntwo");
}

