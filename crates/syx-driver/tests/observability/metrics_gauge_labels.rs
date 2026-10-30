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

