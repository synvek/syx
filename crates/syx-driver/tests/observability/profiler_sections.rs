//! ADR-301 / M3 W42: M3-D04 section wall-clock profiler thin start.
// was: m3_d04_profiler_sections

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d04_profiler_sections_start_stop_snapshot() {
    let src = r#"
func main() {
  print(__syx_profiler_policy())
  let r = __syx_profiler_reset("work")
  print(r)
  print(__syx_profiler_elapsed("work"))
  let s = __syx_profiler_start("work")
  print(s)
  let ms = __syx_profiler_stop("work")
  print(ms)
  print(__syx_profiler_elapsed("work"))
  let _ = __syx_profiler_reset("alpha")
  let _a = __syx_profiler_start("alpha")
  let _b = __syx_profiler_stop("alpha")
  print(__syx_profiler_snapshot())
}
"#;
    let out = run(src);
    assert!(
        out.contains("profiler-v1-thin") || out.contains("profiler-v1-cpu"),
        "policy fingerprint required (thin or deepen): {out}"
    );
    assert!(out.contains("true"), "reset/start must succeed: {out}");
    let lines: Vec<&str> = out.lines().collect();
    // policy, reset true, elapsed 0, start true, stop ms, elapsed ms, snapshot
    assert_eq!(lines.get(2).copied(), Some("0"), "elapsed after reset: {out}");
    let stop_ms: i64 = lines
        .get(4)
        .and_then(|l| l.parse().ok())
        .expect("stop must print int");
    assert!(stop_ms >= 0, "stop ms must be >= 0: {out}");
    let elapsed_ms: i64 = lines
        .get(5)
        .and_then(|l| l.parse().ok())
        .expect("elapsed must print int");
    assert!(elapsed_ms >= 0, "elapsed ms must be >= 0: {out}");
    let snap = lines.last().copied().unwrap_or("");
    assert!(
        snap.contains("work="),
        "snapshot must include work=: {snap}"
    );
    assert!(
        snap.contains("alpha="),
        "snapshot must include alpha=: {snap}"
    );
    let a_pos = snap.find("alpha=").expect("alpha");
    let w_pos = snap.find("work=").expect("work");
    assert!(a_pos < w_pos, "snapshot keys must be sorted: {snap}");
    for part in snap.split_whitespace() {
        let (_k, v) = part.split_once('=').expect("name=ms");
        let n: i64 = v.parse().expect("ms int");
        assert!(n >= 0, "snapshot ms must be >= 0: {part}");
    }
}

#[test]
fn m3_d04_profiler_sections_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d04_profiler_sections"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d04_profiler_sections = \"signed-ADR-301\""),
        "inventory must note signed-ADR-301"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-301")
            || guide.contains("m3_d04_profiler_sections")
            || guide.contains("profiler-v1-thin"),
        "UsageGuide must document M3-D04 profiler thin"
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
            || (roadmap.contains("W43") && roadmap.contains("ADR-302"))
            || (roadmap.contains("W42") && roadmap.contains("ADR-301")),
        "roadmap may advance W44 done / W45 = EC-M3-4 thin start"
    );
    assert!(
        guide.contains("≠ 采样")
            || guide.contains("非 采样")
            || d04.contains("≠ 采样")
            || d04.contains("≠ CPU"),
        "docs must honestly exclude sampling/CPU GA"
    );
}
