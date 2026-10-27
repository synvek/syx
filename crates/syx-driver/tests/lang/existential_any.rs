//! existential `any` binding thin start (stable S014).
// was: m4_l03_existential_any

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l03_existential_any_pass_int_equatable() {
    let src = r#"
func main() {
  let x: any Equatable = 1
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("Int must assign to any Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_l03_existential_any_fail_s014() {
    let src = r#"
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
func main() {
  let x: any Equatable = Box(1)
  print(1)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S014") => {
            assert!(
                e.message.contains("Equatable")
                    || e.message.contains("Box")
                    || e.message.contains("conform"),
                "S014 message should mention Equatable/Box/conform: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S014 existential-any failure, got: {e:?}"),
        Ok(r) => panic!(
            "expected S014 existential-any failure, ran: {:?}",
            r.printed
        ),
    }
}

#[test]
fn m4_l03_existential_any_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_l03_existential_any"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_l03_existential_any = \"signed-ADR-326\""),
        "inventory must note signed-ADR-326"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-326")
            || guide.contains("m4_l03_existential_any")
            || guide.contains("existential-any-v0:binding-s014"),
        "UsageGuide must document M4-L03 existential any thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-L03 |"))
        .expect("M4-L03 row");
    assert!(
        row.contains("| [~] |") || row.contains("| [x] |"),
        "M4-L03 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-326")
            || roadmap.contains("m4_l03_existential_any")
            || roadmap.contains("existential-any-v0:binding-s014"),
        "roadmap must footnote M4-L03 thin"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-L") || roadmap.contains("- [ ] EC-M4-L"),
        "EC-M4-L exit condition must remain documented (may be checked after ADR-329)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("≠ opening")
            || roadmap.contains("≠ full")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-L / full opening / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W9")
            && (roadmap.contains("EC-M4-L") || roadmap.contains("EC-M4")))
            || (roadmap.contains("M4 W8")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-327")
                    || roadmap.contains("m4_l03_closeout")
                    || roadmap.contains("L03")))
            || (roadmap.contains("M4 W7")
                && (roadmap.contains("ADR-326")
                    || roadmap.contains("m4_l03_existential_any")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W9 = EC-M4-L thin (or record W8/W7 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-326")
            || gap.contains("m4_l03_existential_any")
            || (gap.contains("M4-L03") && gap.contains("S014"))
            || (gap.contains("existential") && gap.contains("thin")),
        "gap-analysis must reflect M4-L03 thin"
    );
}
