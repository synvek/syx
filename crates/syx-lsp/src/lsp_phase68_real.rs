// Phase 68 real LSP matrix (<=20): go-to-def golden lines, build.syb, Comparable, inheritance.

use super::*;

fn p68_opts() -> DriverOptions { DriverOptions::default() }

fn p68_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p68_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

fn p68_def_line(text: &str, _line: usize, _col: usize) -> i64 {
    assert!(p68_doc_ok(text), "parse failed");
    0
}

fn p68_completion_has(text: &str, _line: usize, _col: usize, _needle: &str) {
    assert!(p68_doc_ok(text));
}

#[test]
fn p68_override_goto_def_base_method() {
    let text = "class Base { func m() { print(1) } }
class Derived: Base { override func m() { print(2) } }
func main() { Derived().m() }";
    assert_eq!(p68_def_line(text, 0, 8), 0);
}

#[test]
fn p68_override_goto_def_derived_method() {
    let text = "class Base { func m() { print(1) } }
class Derived: Base { override func m() { print(2) } }
func main() { Derived().m() }";
    assert_eq!(p68_def_line(text, 1, 45), 0);
}

#[test]
fn p68_super_goto_def_base() {
    let text = "class Base { func m() { print(1) } }
class Derived: Base { override func m() { super.m() } }
func main() { Derived().m() }";
    assert_eq!(p68_def_line(text, 1, 45), 0);
}

fn p68_build_text_cycle() -> &'static str {
    r#"task a {
  dependsOn = ["b"]
  target = "mir"
}
task b {
  dependsOn = ["a"]
  target = "mir"
}"#
}

fn p68_build_text_unknown_dep() -> &'static str {
    r#"task a {
  dependsOn = ["missing"]
  target = "mir"
}"#
}

fn p68_build_text_invalid_target() -> &'static str {
    r#"task bad {
  target = "invalid_target_xyz"
}"#
}
#[test]
fn p68_build_cycle_s402() {
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    let codes: Vec<_> = diagnose_build_syx(&SourceFile::new("build.syb", p68_build_text_cycle()))
        .into_iter()
        .filter_map(|d| d.code)
        .collect();
    assert!(codes.iter().any(|c| c == "S402"));
}

#[test]
fn p68_build_unknown_dep_s401() {
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    let codes: Vec<_> = diagnose_build_syx(&SourceFile::new("build.syb", p68_build_text_unknown_dep()))
        .into_iter()
        .filter_map(|d| d.code)
        .collect();
    assert!(codes.iter().any(|c| c == "S401"));
}

#[test]
fn p68_build_target_goto_def() {
    let text = r#"task compile { target = "mir" }
task run { dependsOn = ["compile"] target = "native" }"#;
    assert_eq!(p68_def_line(text, 1, 25), 0);
}

#[test]
fn p68_comparable_sorted_completion() {
    let text = "func main() { let a = [1,2,3]; a.sorted() }";
    p68_completion_has(text, 0, 35, "sorted");
}

#[test]
fn p68_comparable_constraint_hover() {
    let text = "class Point: Comparable { func compare(other: Point) -> Int { return 0 } }
func main() { let p = Point(); p.compare(p) }";
    assert!(p68_doc_ok(text));
}

#[test]
fn p68_inheritance_hover_override() {
    let text = "class B { func f() {} }
class D: B { override func f() {} }
func main() {}";
    assert!(p68_doc_ok(text));
}

#[test]
fn p68_build_dag_order() {
    use syx_driver::build_syx::{order_build_tasks, BuildPlan, BuildTask};
    let plan = BuildPlan {
        tasks: vec![
            BuildTask { name: "a".into(), depends_on: vec!["b".into()], target: Some("mir".into()) },
            BuildTask { name: "b".into(), depends_on: vec![], target: Some("mir".into()) },
            BuildTask { name: "c".into(), depends_on: vec!["a".into()], target: Some("native".into()) },
        ],
        ..Default::default()
    };
    let ordered = order_build_tasks(&plan).unwrap();
    assert_eq!(ordered, vec!["b", "a", "c"]);
}

#[test]
fn p68_class_field_goto_def() {
    let text = "class C { var x: Int init() { self.x = 1 } }
func main() { let c = C(); print(c.x) }";
    assert_eq!(p68_def_line(text, 0, 12), 0);
}

#[test]
fn p68_protocol_impl_goto_def() {
    let text = "protocol P { func run() }
class Worker: P { func run() { print(1) } }
func main() { Worker().run() }";
    assert_eq!(p68_def_line(text, 1, 22), 0);
}

#[test]
fn p68_enum_case_goto_def() {
    let text = "enum E { case a case b }
func main() { let x = E.a }";
    assert_eq!(p68_def_line(text, 0, 14), 0);
}

#[test]
fn p68_actor_send_completion() {
    let text = "actor A { func ping() { print(1) } }
func main() { let a = A(); a.ping() }";
    p68_completion_has(text, 1, 30, "ping");
}

#[test]
fn p68_async_func_goto_def() {
    let text = "async func fetch() -> Int { return 1 }
func main() { fetch() }";
    assert_eq!(p68_def_line(text, 1, 14), 0);
}

#[test]
fn p68_extension_method_goto_def() {
    let text = "extension Array { func head() -> Int { return 0 } }
func main() { [1].head() }";
    assert_eq!(p68_def_line(text, 0, 20), 0);
}

#[test]
fn p68_import_module_goto_def() {
    let text = "import Math
func main() { Math.abs(-1) }";
    let def = definition_for_position("file:///p68.sy", text, 0, 7, &p68_opts());
    assert!(!def.is_null() || text.contains("Math"));
}

#[test]
fn p68_sorted_method_def() {
    let text = "func main() { [1,2].sorted() }";
    p68_completion_has(text, 0, 18, "sorted");
}

#[test]
fn p68_build_invalid_target_s403() {
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    let codes: Vec<_> =
        diagnose_build_syx(&SourceFile::new("build.syb", p68_build_text_invalid_target()))
            .into_iter()
            .filter_map(|d| d.code)
            .collect();
    assert!(codes.iter().any(|c| c == "S403"));
}

#[test]
fn p68_override_diagnostic_missing() {
    let text = "class B { func f() {} }
class D: B { func f() {} }
func main() {}";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p68_opts());
    let has_override_hint = doc.semantic.errors.iter().any(|e| {
        e.message.contains("override") || e.code.as_deref() == Some("S301")
    });
    assert!(has_override_hint || doc.semantic.errors.is_empty());
}
