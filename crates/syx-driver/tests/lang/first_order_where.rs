//! first-order where constraint thin start (stable S010).
// was: m4_l01_first_order_where

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l01_first_order_where_pass_int_equatable() {
    let src = r#"
func idEq<T>(_ a: T) -> T where T: Equatable {
  return a
}
func main() {
  print(idEq(1))
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Int must satisfy where T: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print Int identity: {out}"
    );
}

#[test]
fn m4_l01_first_order_where_fail_s010() {
    let src = r#"
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func idEq<T>(_ a: T) -> T where T: Equatable {
  return a
}
func main() {
  let b = Box(1)
  print(idEq(b))
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S010") => {
            assert!(
                e.message.contains("where")
                    || e.message.contains("Equatable")
                    || e.message.contains("Box")
                    || e.message.contains("constraint"),
                "S010 message should mention where/Equatable/Box/constraint: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S010 where failure, got: {e:?}"),
        Ok(r) => panic!("expected S010 where failure, ran: {:?}", r.printed),
    }
}

#[test]
fn m4_l01_first_order_where_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_l01_first_order_where"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_l01_first_order_where = \"signed-ADR-320\""),
        "inventory must note signed-ADR-320"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-320")
            || guide.contains("m4_l01_first_order_where")
            || guide.contains("generic-where-v0:first-order-s010"),
        "UsageGuide must document M4-L01 first-order where thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-L01 |"))
        .expect("M4-L01 row");
    assert!(
        row.contains("| [x] |") || row.contains("| [~] |"),
        "M4-L01 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-320")
            || roadmap.contains("ADR-321")
            || roadmap.contains("m4_l01_first_order_where")
            || roadmap.contains("m4_l01_closeout")
            || roadmap.contains("generic-where-v0:first-order-s010"),
        "roadmap must footnote M4-L01 thin/closeout"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-L") || roadmap.contains("- [ ] EC-M4-L"),
        "EC-M4-L exit condition must remain documented (may be checked after ADR-329)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ L02")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("≠ associated")
            || roadmap.contains("总勾仍开")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-L / L02 / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W3")
            && (roadmap.contains("L02") || roadmap.contains("M4-L02") || roadmap.contains("thin")))
            || (roadmap.contains("M4 W2")
                && (roadmap.contains("ADR-321")
                    || roadmap.contains("m4_l01_closeout")
                    || roadmap.contains("closeout")
                    || roadmap.contains("L01")))
            || (roadmap.contains("M4 W1")
                && (roadmap.contains("ADR-320")
                    || roadmap.contains("m4_l01_first_order_where")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W3 = L02 thin (or record W2/W1 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-320")
            || gap.contains("ADR-321")
            || gap.contains("m4_l01_first_order_where")
            || gap.contains("m4_l01_closeout")
            || gap.contains("generic-where-v0:first-order-s010")
            || (gap.contains("M4-L01") && gap.contains("S010")),
        "gap-analysis must reflect M4-L01 thin/closeout evidence"
    );
}
