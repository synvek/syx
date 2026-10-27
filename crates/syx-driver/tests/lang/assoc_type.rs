//! associated-type typealias witness + stable S012 thin start.
// was: m4_l02_assoc_type

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn m4_l02_assoc_type_pass_int_equatable() {
    let src = r#"
protocol P {
  associatedtype Item: Equatable
}
class C: P {
  typealias Item = Int
  init() {}
}
func main() {
  let _ = C()
  print(1)
  return 0
}
"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("typealias Item = Int must satisfy Item: Equatable")
        .printed
        .join("\n");
    assert!(
        out.contains('1'),
        "pass path must print: {out}"
    );
}

#[test]
fn m4_l02_assoc_type_fail_s012() {
    let src = r#"
protocol P {
  associatedtype Item: Equatable
}
class Box {
  var x: Int
  init(_ x: Int) { self.x = x }
}
class C: P {
  typealias Item = Box
  init() {}
}
func main() {
  let _ = C()
  print(1)
  return 0
}
"#;
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Err(e) if e.code.as_deref() == Some("S012") => {
            assert!(
                e.message.contains("Associated type")
                    || e.message.contains("Item")
                    || e.message.contains("Equatable")
                    || e.message.contains("Box"),
                "S012 message should mention Associated type/Item/Equatable/Box: {}",
                e.message
            );
        }
        Err(e) => panic!("expected S012 assoc-type failure, got: {e:?}"),
        Ok(r) => panic!("expected S012 assoc-type failure, ran: {:?}", r.printed),
    }
}

#[test]
fn m4_l02_assoc_type_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test lang"),
        "ci.yml must wire m4_l02_assoc_type"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m4_l02_assoc_type = \"signed-ADR-322\""),
        "inventory must note signed-ADR-322"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-322")
            || guide.contains("m4_l02_assoc_type")
            || guide.contains("assoc-type-v0:typealias-s012"),
        "UsageGuide must document M4-L02 assoc-type thin start"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let row = roadmap
        .lines()
        .find(|l| l.starts_with("| M4-L02 |"))
        .expect("M4-L02 row");
    assert!(
        row.contains("| [~] |") || row.contains("| [x] |"),
        "M4-L02 must be [~] after thin or [x] after closeout: {row}"
    );
    assert!(
        roadmap.contains("ADR-322")
            || roadmap.contains("m4_l02_assoc_type")
            || roadmap.contains("assoc-type-v0:typealias-s012"),
        "roadmap must footnote M4-L02 thin"
    );
    assert!(
        roadmap.contains("- [x] EC-M4-L") || roadmap.contains("- [ ] EC-M4-L"),
        "EC-M4-L exit condition must remain documented (may be checked after ADR-329)"
    );
    assert!(
        roadmap.contains("≠ EC-M4")
            || roadmap.contains("≠ conditional")
            || roadmap.contains("≠ M4 退出")
            || roadmap.contains("assoc-only")
            || row.contains("≠"),
        "roadmap must note honest ≠ EC-M4-L / conditional / M4 exit"
    );
    assert!(
        (roadmap.contains("M4 W5")
            && (roadmap.contains("conditional")
                || roadmap.contains("condition")
                || roadmap.contains("thin")))
            || (roadmap.contains("M4 W4")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-323")
                    || roadmap.contains("m4_l02_closeout")
                    || roadmap.contains("L02")))
            || (roadmap.contains("M4 W3")
                && (roadmap.contains("ADR-322")
                    || roadmap.contains("m4_l02_assoc_type")
                    || roadmap.contains("thin"))),
        "roadmap must lock next M4 W5 = conditional thin (or record W4/W3 done)"
    );
    let gap = include_str!("../../../../plan/archive/syx-language-stdlib-gap-analysis.md");
    assert!(
        gap.contains("ADR-322")
            || gap.contains("m4_l02_assoc_type")
            || (gap.contains("M4-L02") && (gap.contains("thin") || gap.contains("[~]"))),
        "gap-analysis must reflect M4-L02 assoc-type thin"
    );
}
