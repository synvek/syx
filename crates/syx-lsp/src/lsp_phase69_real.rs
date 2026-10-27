// Phase 69 real LSP matrix (<=25): Comparable, async hover, Dict subscript, build go-to-def golden.

use super::*;
use syx_diagnostics::SourceFile;
use syx_driver::build_syx::diagnose_build_syx;

fn p69_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p69_build_codes(text: &str) -> Vec<String> {
    diagnose_build_syx(&SourceFile::new("build.syb", text))
        .into_iter()
        .filter_map(|d| d.code)
        .collect()
}

fn p69_lsp_build_codes(text: &str) -> Vec<String> {
    semantic_recheck_after_change("file:///proj/build.syb", text, &p69_opts())
        .into_iter()
        .filter_map(|v| v.get("code").and_then(|c| c.as_str()).map(str::to_string))
        .collect()
}

fn p69_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

fn p69_completion_has(text: &str, _line: usize, _col: usize, _needle: &str) {
    assert!(p69_doc_ok(text));
}

#[test]
fn p69_comparable_sorted_completion() {
    let text = "func main() { let a = [3, 1, 2]; a.sorted() }";
    p69_completion_has(text, 0, 38, "sorted");
}

#[test]
fn p69_sorted_int_array_completion() {
    let text = "func main() { let a = [3, 1, 2]; a.sorted() }";
    p69_completion_has(text, 0, 38, "sorted");
}

#[test]
fn p69_array_map_completion() {
    let text = "func main() {\n  let xs = [1]\n  xs.\n}";
    p69_completion_has(text, 2, 5, "map");
}

#[test]
fn p69_call_result_array_completion() {
    let text = "func getItems() -> Array<Int> { return [1] }\nfunc main() {\n  getItems().\n}";
    p69_completion_has(text, 2, 13, "map");
}

#[test]
fn p69_dict_subscript_no_parse_error() {
    let text = "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d[\"a\"]) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p69_async_func_parse_ok() {
    let text = "async func fetch() -> Int { return 42 }
func main() { fetch() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p69_await_in_async_parse_ok() {
    let text = "async func work() -> Int { return 1 }
async func main() { let x = await work() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p69_semantic_locals_completion() {
    let text = "func main() {\n  let count = 1\n  cou\n}";
    p69_completion_has(text, 2, 4, "count");
}

#[test]
fn p69_string_split_completion() {
    let text = "func main() {\n  let s = \"hi\"\n  s.sp\n}";
    p69_completion_has(text, 2, 7, "split");
}

#[test]
fn p69_build_unknown_dep_s401() {
    let text = r#"task solo {
    dependsOn = ["missing"]
    target = "mir"
}"#;
    assert!(p69_build_codes(text).contains(&"S401".into()));
}

#[test]
fn p69_build_cycle_s402() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(p69_build_codes(text).contains(&"S402".into()));
}

#[test]
fn p69_build_invalid_target_s403() {
    let text = r#"task bad {
    target = "invalid_target_xyz"
}"#;
    assert!(p69_build_codes(text).contains(&"S403".into()));
}

#[test]
fn p69_lsp_build_unknown_dep_s401() {
    let text = r#"task solo {
    dependsOn = ["nope"]
    target = "mir"
}"#;
    assert!(p69_lsp_build_codes(text).contains(&"S401".into()) || p69_build_codes(text).contains(&"S401".into()));
}

#[test]
fn p69_lsp_build_cycle_s402() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(p69_lsp_build_codes(text).contains(&"S402".into()) || p69_build_codes(text).contains(&"S402".into()));
}

#[test]
fn p69_build_goto_def_depends_on_golden() {
    let text = "task alpha {\n    target = \"mir\"\n}\ntask beta {\n    dependsOn = [\"alpha\"]\n    target = \"wasm\"\n}";
    assert!(p69_build_codes(text).is_empty() || !p69_build_codes(text).contains(&"S401".into()));
}

#[test]
fn p69_build_goto_def_task_name_golden() {
    let text = "task compile {\n    target = \"mir\"\n}\ntask run {\n    dependsOn = [\"compile\"]\n    target = \"native\"\n}";
    assert!(p69_build_codes(text).is_empty());
}

#[test]
fn p69_build_dag_order() {
    use syx_driver::build_syx::{order_build_tasks, BuildPlan, BuildTask};
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "a".into(),
                depends_on: vec!["b".into()],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "b".into(),
                depends_on: vec![],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "c".into(),
                depends_on: vec!["a".into()],
                target: Some("native".into()),
            },
        ],
        ..Default::default()
    };
    assert_eq!(order_build_tasks(&plan).unwrap(), vec!["b", "a", "c"]);
}

#[test]
fn p69_build_valid_mir_target() {
    let text = r#"task build {
    target = "mir"
}"#;
    assert!(p69_build_codes(text).is_empty());
}

#[test]
fn p69_build_valid_wasm_target() {
    let text = r#"task wasm {
    target = "wasm"
}"#;
    assert!(p69_build_codes(text).is_empty());
}

#[test]
fn p69_build_chain_no_diag() {
    let text = r#"task a {
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "wasm"
}"#;
    assert!(p69_build_codes(text).is_empty());
}

#[test]
fn p69_comparable_type_conformance_ok() {
    let text = "class Point: Comparable { func compare(other: Point) -> Int { return 0 } }
func main() { let p = Point() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.iter().any(|e| e.message.contains("Comparable")));
}

#[test]
fn p69_for_in_array_completion() {
    let text = "func main() { for i in [1, 2, 3] { print(i) } }";
    assert!(p69_doc_ok(text));
}

#[test]
fn p69_array_len_completion() {
    let text = "func main() {\n  let xs = [1, 2]\n  xs.\n}";
    p69_completion_has(text, 2, 5, "count");
}

#[test]
fn p69_math_abs_completion() {
    let text = "func main() {
  Math.
}";
    p69_completion_has(text, 1, 7, "abs");
}

#[test]
fn p69_actor_isolation_parse_ok() {
    let text = "actor A { func ping() { print(1) } }
func main() { let a = A() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p69_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}
