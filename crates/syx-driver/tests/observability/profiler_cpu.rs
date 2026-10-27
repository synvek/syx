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

#[test]
fn m3_d04_profiler_cpu_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d04_profiler_cpu"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d04_profiler_cpu = \"signed-ADR-302\""),
        "inventory must note signed-ADR-302"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-302")
            || guide.contains("m3_d04_profiler_cpu")
            || guide.contains("profiler-v1-cpu"),
        "UsageGuide must document M3-D04 CPU deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let d04 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-D04 |"))
        .expect("M3-D04 row");
    assert!(
        d04.contains("| [~] |") || d04.contains("| [x] |"),
        "M3-D04 may be [~] or [x] after later windows: {d04}"
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
        (roadmap.contains("W45")
            && (roadmap.contains("EC-M3-4")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W44")
                && (roadmap.contains("closeout")
                    || roadmap.contains("D04")
                    || roadmap.contains("leaf")
                    || roadmap.contains("ADR-303")))
            || (roadmap.contains("W43") && roadmap.contains("ADR-302")),
        "roadmap may advance W44 done / W45 = EC-M3-4 thin start"
    );
    assert!(
        guide.contains("≠ 采样")
            || guide.contains("非 采样")
            || d04.contains("≠ 采样")
            || d04.contains("≠ pprof"),
        "docs must honestly exclude sampling/pprof GA"
    );
}
