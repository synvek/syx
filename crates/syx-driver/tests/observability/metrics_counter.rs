//! ADR-298 / M3 W39: M3-D03 Metrics Counter thin start.
// was: m3_d03_metrics_counter

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d03_metrics_counter_inc_get_snapshot() {
    let src = r#"
func main() {
  print(__syx_metrics_policy())
  let r = __syx_metrics_counter_reset("hits")
  print(r)
  print(__syx_metrics_counter_get("hits"))
  print(__syx_metrics_counter_inc("hits", 2))
  print(__syx_metrics_counter_get("hits"))
  print(__syx_metrics_counter_inc("hits", 3))
  print(__syx_metrics_counter_get("hits"))
  let _ = __syx_metrics_counter_inc("alpha", 1)
  print(__syx_metrics_snapshot())
}
"#;
    let out = run(src);
    assert!(
        out.contains("metrics-v1-thin") || out.contains("metrics-v1-gauge-labels"),
        "policy fingerprint required (thin or deepen): {out}"
    );
    assert!(out.contains("true"), "reset must succeed: {out}");
    let lines: Vec<&str> = out.lines().collect();
    // policy, reset true, get 0, inc 2, get 2, inc 5, get 5, snapshot
    assert!(
        lines.iter().any(|l| *l == "0"),
        "get after reset must be 0: {out}"
    );
    assert!(
        lines.iter().any(|l| *l == "2"),
        "inc(+2) must return 2: {out}"
    );
    assert!(
        lines.iter().any(|l| *l == "5"),
        "inc(+3) must accumulate to 5: {out}"
    );
    let snap = lines
        .iter()
        .rev()
        .find(|l| l.contains('=') && l.contains("hits="))
        .expect("snapshot");
    assert!(
        snap.contains("hits=5"),
        "snapshot must include hits=5: {snap}"
    );
    assert!(
        snap.contains("alpha=1"),
        "snapshot must include alpha=1: {snap}"
    );
    let a_pos = snap.find("alpha=").expect("alpha");
    let h_pos = snap.find("hits=").expect("hits");
    assert!(a_pos < h_pos, "snapshot keys must be sorted: {snap}");
}

#[test]
fn m3_d03_metrics_counter_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d03_metrics_counter"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d03_metrics_counter = \"signed-ADR-298\""),
        "inventory must note signed-ADR-298"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-298")
            || guide.contains("m3_d03_metrics_counter")
            || guide.contains("metrics-v1-thin"),
        "UsageGuide must document M3-D03 metrics thin"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let d03 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-D03 |"))
        .expect("M3-D03 row");
    assert!(
        d03.contains("| [~] |") || d03.contains("| [x] |"),
        "M3-D03 may be [~] or [x] after later windows: {d03}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-4") || roadmap.contains("- [ ] EC-M3-4"),
        "EC-M3-4 exit condition must remain documented (may be checked after ADR-305)"
    );
    assert!(
        (roadmap.contains("W42")
            && (roadmap.contains("D04")
                || roadmap.contains("profiler")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W41")
                && (roadmap.contains("closeout")
                    || roadmap.contains("D03")
                    || roadmap.contains("leaf")
                    || roadmap.contains("ADR-300")))
            || (roadmap.contains("W40") && roadmap.contains("ADR-299"))
            || (roadmap.contains("W39") && roadmap.contains("ADR-298")),
        "roadmap may advance W41 done / W42 = D04 thin start"
    );
    assert!(
        guide.contains("≠ OTel")
            || guide.contains("非 OTel")
            || d03.contains("≠ OTel"),
        "docs must honestly exclude OTel/Prometheus GA"
    );
}
