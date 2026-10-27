//! conditional conformance thin start (Array Element where → stable S010).
// was: m4_conditional_conformance

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_conditional_conformance_pass_array_int() {
    let src = r#"
protocol Marker {
  func mark() -> Int { return 0 }
}
extension Array: Marker where Element: Equatable {
  func mark() -> Int { return 1 }
}
func takeMarker<T>(_ x: T) -> Int where T: Marker {
  return 1
}
func main() {
  let xs: [Int] = [1, 2]
  print(takeMarker(xs))
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Array<Int> must satisfy Marker where Element: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_conditional_conformance_fail_s010() {
    let src = r#"
protocol Marker {
  func mark() -> Int { return 0 }
}
extension Array: Marker where Element: Equatable {
  func mark() -> Int { return 1 }
}
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func takeMarker<T>(_ x: T) -> Int where T: Marker {
  return 1
}
func main() {
  let b = Box(1)
  let xs: [Box] = [b]
  print(takeMarker(xs))
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S010") => {
            assert!(
                e.message.contains("where")
                    || e.message.contains("Equatable")
                    || e.message.contains("Marker")
                    || e.message.contains("Element")
                    || e.message.contains("Box")
                    || e.message.contains("Array"),
                "S010 message should mention where/Equatable/Marker/Element/Box/Array: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S010 conditional-conformance failure, got: {e:?}"),
        Ok(r) => panic!(
            "expected S010 conditional-conformance failure, ran: {:?}",
            r.printed
        ),
    }
}

#[test]
fn m4_conditional_conformance_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_conditional_conformance"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_conditional_conformance = \"signed-ADR-324\""),
        "inventory must note signed-ADR-324"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-324")
            || guide.contains("m4_conditional_conformance")
            || guide.contains("conditional-conform-v0:array-element-s010"),
        "UsageGuide must document conditional conformance thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-L02 |"))
        .expect("M4-L02 row");
    assert!(
        row.contains("| [x] |"),
        "M4-L02 must remain [x] (assoc-only leaf); conditional is deepen: {row}"
    );
    assert!(
        roadmap.contains("ADR-324")
            || roadmap.contains("m4_conditional_conformance")
            || roadmap.contains("conditional-conform-v0:array-element-s010"),
        "roadmap must footnote conditional thin"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-L") || roadmap.contains("- [ ] EC-M4-L"),
        "EC-M4-L exit condition must remain documented (may be checked after ADR-329)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ L03")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("≠ conditional GA")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-L / L03 / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W7")
            && (roadmap.contains("L03")
                || roadmap.contains("existential")
                || roadmap.contains("opaque")))
            || (roadmap.contains("M4 W6")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-325")
                    || roadmap.contains("m4_conditional_closeout")
                    || roadmap.contains("conditional")))
            || (roadmap.contains("M4 W5")
                && (roadmap.contains("ADR-324")
                    || roadmap.contains("m4_conditional_conformance")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W7 = L03 thin (or record W6/W5 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-324")
            || gap.contains("m4_conditional_conformance")
            || (gap.contains("conditional") && gap.contains("thin")),
        "gap-analysis must reflect conditional conformance thin"
    );
}
