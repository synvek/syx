//! ADR-299 / M3 W40: M3-D03 Gauge + labeled counter deepen.
// was: m3_d03_metrics_gauge_labels

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d03_metrics_gauge_labels_behavior() {
    let src = r#"
func main() {
  print(__syx_metrics_policy())
  print(__syx_metrics_gauge_set("temp", 42))
  print(__syx_metrics_gauge_get("temp"))
  print(__syx_metrics_counter_inc_labels("req", 1, { "m": "GET", "c": "200" }))
  print(__syx_metrics_counter_get_labels("req", { "m": "GET", "c": "200" }))
  print(__syx_metrics_snapshot())
}
"#;
    let out = run(src);
    assert!(
        out.contains("metrics-v1-gauge-labels"),
        "policy fingerprint required: {out}"
    );
    let lines: Vec<&str> = out.lines().collect();
    assert!(
        lines.iter().any(|l| *l == "42"),
        "gaugeSet/get must yield 42: {out}"
    );
    assert!(
        lines.iter().any(|l| *l == "1"),
        "incLabels/getLabels must yield 1: {out}"
    );
    let snap = lines
        .iter()
        .rev()
        .find(|l| l.contains("g:temp=") || l.contains("req{"))
        .expect("snapshot");
    assert!(
        snap.contains("g:temp=42"),
        "snapshot must include g:temp=42: {snap}"
    );
    assert!(
        snap.contains("req{c=200,m=GET}=1"),
        "snapshot must include labeled series with sorted keys: {snap}"
    );
}

#[test]
fn m3_d03_metrics_gauge_labels_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d03_metrics_gauge_labels"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d03_metrics_gauge_labels = \"signed-ADR-299\""),
        "inventory must note signed-ADR-299"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-299")
            || guide.contains("m3_d03_metrics_gauge_labels")
            || guide.contains("metrics-v1-gauge-labels"),
        "UsageGuide must document M3-D03 gauge+labels deepen"
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
            || (roadmap.contains("W40") && roadmap.contains("ADR-299")),
        "roadmap may advance W41 done / W42 = D04 thin start"
    );
    assert!(
        guide.contains("≠ OTel")
            || guide.contains("非 OTel")
            || d03.contains("≠ OTel")
            || d03.contains("≠ Histogram"),
        "docs must honestly exclude OTel/Prometheus GA and Histogram"
    );
}
