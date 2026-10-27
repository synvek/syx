//! static exclusive borrow thin start (stable S011).
// was: m4_b01_exclusive_borrow

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_b01_exclusive_borrow_pass_single_mut() {
    let src = r#"
func main() {
  var a: Array<Int> = [1]
  let p = &mut a
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("single &mut must compile and run")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print 1: {out}"
    );
}

#[test]
fn m4_b01_exclusive_borrow_fail_s011_double_mut() {
    let src = r#"
func main() {
  var a: Array<Int> = [1]
  let p = &mut a
  let q = &mut a
  print(p)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S011") => {
            assert!(
                e.message.contains("mutable")
                    || e.message.contains("borrow")
                    || e.message.contains("more than once")
                    || e.message.contains("mut"),
                "S011 message should mention mutable/borrow/more than once: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S011 double-mut failure, got: {e:?}"),
        Ok(r) => panic!("expected S011 double-mut failure, ran: {:?}", r.printed),
    }
}

#[test]
fn m4_b01_exclusive_borrow_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_b01_exclusive_borrow"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_b01_exclusive_borrow = \"signed-ADR-330\""),
        "inventory must note signed-ADR-330"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-330")
            || guide.contains("m4_b01_exclusive_borrow")
            || guide.contains("borrow-exclusive-v0:double-mut-s011"),
        "UsageGuide must document M4-B01 exclusive borrow thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-B01 |"))
        .expect("M4-B01 row");
    assert!(
        row.contains("| [~] |") || row.contains("| [x] |"),
        "M4-B01 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-330")
            || roadmap.contains("m4_b01_exclusive_borrow")
            || roadmap.contains("borrow-exclusive-v0:double-mut-s011")
            || row.contains("S011")
            || row.contains("thin"),
        "roadmap must footnote M4-B01 thin / ADR-330"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-B") || roadmap.contains("- [ ] EC-M4-B"),
        "EC-M4-B must remain documented (may be checked after ADR-337)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ NLL")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("≠ noncopyable")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-B / NLL / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W13")
            && (roadmap.contains("B02")
                || roadmap.contains("M4-B02")
                || roadmap.contains("noncopyable")
                || roadmap.contains("thin")))
            || (roadmap.contains("M4 W12")
                && (roadmap.contains("ADR-331")
                    || roadmap.contains("m4_b01_closeout")
                    || roadmap.contains("closeout")
                    || roadmap.contains("B01")
                    || roadmap.contains("M4-B01")))
            || (roadmap.contains("M4 W11")
                && (roadmap.contains("ADR-330")
                    || roadmap.contains("m4_b01_exclusive_borrow")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W13 = B02 thin (or record W12/W11 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-330")
            || gap.contains("m4_b01_exclusive_borrow")
            || gap.contains("ADR-331")
            || gap.contains("m4_b01_closeout")
            || gap.contains("borrow-exclusive-v0:double-mut-s011")
            || (gap.contains("M4-B01")
                && (gap.contains("S011") || gap.contains("thin") || gap.contains("叶子"))),
        "gap-analysis must reflect M4-B01 thin/closeout"
    );
}
