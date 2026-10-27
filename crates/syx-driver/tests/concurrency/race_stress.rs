//! race/stress CI sampling thin start.
// was: m3_c04_race_stress

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_c04_race_stress_mutex_cross_task_smoke() {
    // C04-owned sampling smoke: short cross-Task Mutex contention (Rounds=1).
    let src = r#"
func holdThenUnlock(_ mid: Object) {
  let ok = __syx_mutex_lock_wait(mid)
  print("A_HELD")
  print(ok)
  __syx_blocking_sleep(120)
  print("A_UNLOCK")
  __syx_mutex_unlock(mid)
}

func waitThenUnlock(_ mid: Object) {
  let ok = __syx_mutex_lock_wait(mid)
  print("B_GOT")
  print(ok)
  __syx_mutex_unlock(mid)
}

func main() {
  __syx_task_set_worker_count(2)
  let m = __syx_mutex_create()
  let a = __syx_task_detach("holdThenUnlock", [m])
  __syx_blocking_sleep(30)
  let b = __syx_task_detach("waitThenUnlock", [m])
  __syx_task_join(a)
  __syx_task_join(b)
  print("DONE")
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("A_HELD")
            && out.contains("A_UNLOCK")
            && out.contains("B_GOT")
            && out.contains("DONE"),
        "C04 stress smoke must complete: {out}"
    );
    let a_held = out.find("A_HELD").expect("A_HELD");
    let a_unlock = out.find("A_UNLOCK").expect("A_UNLOCK");
    let b_got = out.find("B_GOT").expect("B_GOT");
    assert!(
        a_held < a_unlock && a_unlock < b_got,
        "order must be A_HELD → A_UNLOCK → B_GOT: {out}"
    );
    assert!(
        elapsed.as_millis() >= 80,
        "B must have waited on held lock (elapsed={elapsed:?}): {out}"
    );
    assert!(
        elapsed.as_millis() < 5000,
        "sampling smoke must finish timely: elapsed={elapsed:?} out={out}"
    );
}

#[test]
fn m3_c04_race_stress_ci_inventory_samples_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c04_race_stress"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c04_race_stress = \"signed-ADR-289\""),
        "inventory must note signed-ADR-289"
    );
    let samples = include_str!("../../../../packages/internal/race_stress_samples.toml");
    for id in [
        "m3_c01_mutex_blocking",
        "m3_c02_semaphore",
        "m3_c04_race_stress",
    ] {
        assert!(
            samples.contains(&format!("id = \"{id}\"")),
            "sample corpus must list {id}"
        );
    }
    let sample_count = samples.matches("[[samples]]").count();
    assert!(
        sample_count >= 3,
        "sample corpus must have ≥3 entries, got {sample_count}"
    );
    assert!(
        samples.contains("kind = \"stress\""),
        "samples must include kind=stress"
    );
    // After deepen, corpus may also list kind=chaos (ADR-290).
    let _ = samples.contains("kind = \"chaos\"");
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-289")
            || guide.contains("m3_c04_race_stress")
            || guide.contains("race/stress"),
        "UsageGuide must document M3-C04 race/stress sampling"
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
            || (roadmap.contains("W31")
                && (roadmap.contains("deepen")
                    || roadmap.contains("chaos")
                    || roadmap.contains("ADR-290")
                    || roadmap.contains("C04")))
            || (roadmap.contains("W30") && roadmap.contains("ADR-289")),
        "roadmap may advance W30–W32 done / W33 = D01 thin start"
    );
    // Honesty denials like 「**非** TSAN GA」 may appear in UsageGuide; forbid bare product claims only.
    let guide_denies_tsan_ga = guide.contains("非** TSAN GA")
        || guide.contains("非 TSAN GA")
        || guide.contains("≠ TSAN GA")
        || guide.contains("非 ThreadSanitizer");
    assert!(
        (!guide.contains("TSAN GA") || guide_denies_tsan_ga) && !c04.contains("TSAN GA"),
        "must not claim TSAN GA"
    );
}
