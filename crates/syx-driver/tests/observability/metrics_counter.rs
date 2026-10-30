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

