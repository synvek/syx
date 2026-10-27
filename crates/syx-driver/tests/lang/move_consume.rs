//! move-consume ownership thin start (stable S011).
// was: m4_b02_move_consume

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_b02_move_consume_pass_once() {
    let src = r#"
func take(_ s: String) {
  print(1)
}
func main() {
  let s = "hi"
  take(s)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("String by-value consume-once must compile and run")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print 1: {out}"
    );
}

#[test]
fn m4_b02_move_consume_fail_s011_use_after_move() {
    let src = r#"
func take(_ s: String) {
  print(1)
}
func main() {
  let s = "hi"
  take(s)
  print(s)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S011") => {
            assert!(
                e.message.contains("moved")
                    || e.message.contains("Use of moved")
                    || e.message.to_lowercase().contains("moved value"),
                "S011 message should mention moved value: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S011 use-after-move failure, got: {e:?}"),
        Ok(r) => panic!("expected S011 use-after-move failure, ran: {:?}", r.printed),
    }
}

#[test]
fn m4_b02_move_consume_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_b02_move_consume"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_b02_move_consume = \"signed-ADR-332\""),
        "inventory must note signed-ADR-332"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-332")
            || guide.contains("m4_b02_move_consume")
            || guide.contains("move-consume-v0:string-s011"),
        "UsageGuide must document M4-B02 move-consume thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-B02 |"))
        .expect("M4-B02 row");
    assert!(
        row.contains("| [~] |") || row.contains("| [x] |"),
        "M4-B02 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-332")
            || roadmap.contains("m4_b02_move_consume")
            || roadmap.contains("move-consume-v0:string-s011")
            || row.contains("S011")
            || row.contains("thin")
            || row.contains("move"),
        "roadmap must footnote M4-B02 thin / ADR-332"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-B") || roadmap.contains("- [ ] EC-M4-B"),
        "EC-M4-B must remain documented (may be checked after ADR-337)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ ~Copyable")
            || roadmap.contains("≠ Copyable")
            || roadmap.contains("≠ NLL")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("move-consume")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-B / ~Copyable / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W15")
            && (roadmap.contains("B03")
                || roadmap.contains("M4-B03")
                || roadmap.contains("language-complete")
                || roadmap.contains("thin")))
            || (roadmap.contains("M4 W14")
                && (roadmap.contains("ADR-333")
                    || roadmap.contains("m4_b02_closeout")
                    || roadmap.contains("closeout")
                    || roadmap.contains("B02")
                    || roadmap.contains("M4-B02")))
            || (roadmap.contains("M4 W13")
                && (roadmap.contains("ADR-332")
                    || roadmap.contains("m4_b02_move_consume")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W15 = B03 thin (or record W14/W13 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-332")
            || gap.contains("m4_b02_move_consume")
            || gap.contains("ADR-333")
            || gap.contains("m4_b02_closeout")
            || gap.contains("move-consume-v0:string-s011")
            || (gap.contains("M4-B02")
                && (gap.contains("S011")
                    || gap.contains("thin")
                    || gap.contains("move")
                    || gap.contains("叶子"))),
        "gap-analysis must reflect M4-B02 thin/closeout"
    );
}
