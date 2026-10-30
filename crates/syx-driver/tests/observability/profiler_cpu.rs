//! ADR-302 / M3 W43: M3-D04 process CPU section deepen.
// was: m3_d04_profiler_cpu

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d04_profiler_cpu_behavior() {
    let src = r#"
func main() {
  print(__syx_profiler_policy())
  print(__syx_profiler_reset("work"))
  print(__syx_profiler_cpu_elapsed("work"))
  print(__syx_profiler_cpu_start("work"))
  print(__syx_profiler_cpu_stop("work"))
  print(__syx_profiler_cpu_elapsed("work"))
  let _ = __syx_profiler_start("work")
  let _w = __syx_profiler_stop("work")
  print(__syx_profiler_snapshot())
}
"#;
    let out = run(src);
    assert!(
        out.contains("profiler-v1-cpu"),
        "policy fingerprint required: {out}"
    );
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.get(2).copied(), Some("0"), "cpuElapsed after reset: {out}");
    assert_eq!(lines.get(3).copied(), Some("true"), "cpuStart: {out}");
    let stop_ms: i64 = lines
        .get(4)
        .and_then(|l| l.parse().ok())
        .expect("cpuStop must print int");
    assert!(stop_ms >= 0, "cpuStop ms must be >= 0: {out}");
    let elapsed_ms: i64 = lines
        .get(5)
        .and_then(|l| l.parse().ok())
        .expect("cpuElapsed must print int");
    assert!(elapsed_ms >= 0, "cpuElapsed ms must be >= 0: {out}");
    let snap = lines.last().copied().unwrap_or("");
    assert!(
        snap.contains("c:work="),
        "snapshot must include c:work=: {snap}"
    );
    let has_wall = snap
        .split_whitespace()
        .any(|p| p.starts_with("work=") && !p.starts_with("c:"));
    assert!(has_wall, "snapshot must include wall work=: {snap}");
    let c_pos = snap.find("c:work=").expect("c:work");
    let wall_pos = snap
        .split_whitespace()
        .find(|p| p.starts_with("work=") && !p.starts_with("c:"))
        .and_then(|p| snap.find(p))
        .expect("wall part");
    assert!(
        c_pos < wall_pos,
        "snapshot keys must be sorted (c: before wall): {snap}"
    );
}

