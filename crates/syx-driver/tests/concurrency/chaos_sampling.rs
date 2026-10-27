//! chaos sampling deepen (TaskGroup cancel → C004).
// was: m3_c04_chaos

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_c04_chaos_task_group_cancel_interrupts_long_sleep() {
    // C04-owned chaos micro-sample: cancel mid-flight long sleep → C004 promptly.
    let src = r#"
func main() {
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_cancel(g)
  __syx_task_group_wait(g)
  print("WAITED")
  try {
    __syx_task_join(t)
    print("JOIN_OK")
  } catch (err) {
    print("JOIN_CANCEL")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "cancel+wait must not hang for full sleep: elapsed={elapsed:?} out={out}"
    );
    assert!(out.contains("WAITED"), "wait must complete: {out}");
    assert!(
        !out.contains("SLEPT"),
        "cancelled sleeper must not finish sleep: {out}"
    );
    assert!(
        out.contains("JOIN_CANCEL") && out.contains("C004"),
        "cancelled child join must surface C004: {out}"
    );
    assert!(!out.contains("JOIN_OK"), "join must not succeed: {out}");
}

#[test]
fn m3_c04_chaos_ci_inventory_samples_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c04_chaos"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c04_chaos = \"signed-ADR-290\""),
        "inventory must note signed-ADR-290"
    );
    let samples = include_str!("../../../../packages/internal/race_stress_samples.toml");
    assert!(
        samples.contains("id = \"m3_c04_chaos\"")
            && samples.contains("kind = \"chaos\""),
        "sample corpus must list C04-owned chaos entry"
    );
    assert!(
        samples.contains("id = \"m3_a01_task_tree\""),
        "sample corpus must register m3_a01_task_tree as chaos"
    );
    let sample_count = samples.matches("[[samples]]").count();
    assert!(
        sample_count >= 5,
        "sample corpus must have ≥5 entries after deepen, got {sample_count}"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-290")
            || guide.contains("m3_c04_chaos")
            || guide.contains("chaos"),
        "UsageGuide must document M3-C04 chaos deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c04 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C04 |"))
        .expect("M3-C04 row");
    assert!(
        c04.contains("| [~] |") || c04.contains("| [x] |"),
        "M3-C04 may be [~] or [x] after later windows: {c04}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W33")
            && (roadmap.contains("D01")
                || roadmap.contains("logging")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W32")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-291")
                    || roadmap.contains("C04")))
            || (roadmap.contains("W31") && roadmap.contains("ADR-290")),
        "roadmap may advance W31–W32 done / W33 = D01 thin start"
    );
}
