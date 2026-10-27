//! Channel select fairness product nail (RR 10/10).
// was: m3_a04_fairness

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{reset_select_fair_cursor, select_fair_fingerprint};

fn run(src: &str) -> Vec<String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
}

#[test]
fn m3_a04_fairness_select_rr_even_split() {
    assert_eq!(
        select_fair_fingerprint(),
        "select-fair-v1:round-robin",
        "Phase 107 RR fingerprint must remain"
    );
    reset_select_fair_cursor(0);
    let lines = run(
        r#"
func main() {
  let ch1 = __syx_channel_create(1)
  let ch2 = __syx_channel_create(1)
  var i = 0
  while i < 20 {
    __syx_channel_try_recv(ch1)
    __syx_channel_try_recv(ch2)
    __syx_channel_send(ch1, 11)
    __syx_channel_send(ch2, 22)
    let r = __syx_channel_select([ch1, ch2])
    print(r[1])
    i = i + 1
  }
}
"#,
    );
    let c11 = lines.iter().filter(|l| l.contains("11")).count();
    let c22 = lines.iter().filter(|l| l.contains("22")).count();
    assert_eq!(
        (c11, c22),
        (10, 10),
        "fair RR from seed 0 must split evenly: 11={c11} 22={c22} lines={lines:?}"
    );
}

#[test]
fn m3_a04_fairness_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a04_fairness"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a04_fairness = \"signed-ADR-268\""),
        "inventory must note signed-ADR-268"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-268")
            || guide.contains("m3_a04_fairness")
            || guide.contains("fairness"),
        "UsageGuide must document Channel fairness product nail"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-268")
            && (roadmap.contains("fairness")
                || roadmap.contains("m3_a04_fairness")
                || roadmap.contains("round-robin")
                || roadmap.contains("RR")),
        "roadmap must record M3 W9 fairness"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W10")
            && (roadmap.contains("closeout")
                || roadmap.contains("ADR-269")
                || roadmap.contains("A04")
                || roadmap.contains("done")))
            || (roadmap.contains("W11")
                && (roadmap.contains("A05")
                    || roadmap.contains("docs")
                    || roadmap.contains("ADR-270")
                    || roadmap.contains("done")))
            || (roadmap.contains("W12")
                && (roadmap.contains("closeout") || roadmap.contains("A05"))),
        "roadmap may advance W10/W11 done / W12 = A05 closeout"
    );
    let a04 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-A04 |"))
        .expect("M3-A04 row");
    assert!(
        a04.contains("| [~] |") || a04.contains("| [x] |"),
        "M3-A04 may be [~] pre-closeout or [x] after W10: {a04}"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("class Select")
            && sync.contains("__syx_channel_select")
            && sync.contains("select"),
        "Sync.Select facade required"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-268")
            || book.contains("round-robin")
            || book.contains("fairness"),
        "18-concurrency must document select fairness thin nail"
    );
}
