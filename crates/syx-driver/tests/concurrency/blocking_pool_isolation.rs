//! isolation thin start (Task pool vs blocking_pool).
// was: m3_b01_isolation

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_b01_isolation_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_isolation_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("task-pool+blocking-pool-v1-thin")
            || out.contains("task-pool+blocking-pool-v1-sized"),
        "isolationPolicy fingerprint (thin or sized): {out}"
    );
}

#[test]
fn m3_b01_isolation_blocking_sleep_does_not_starve_short_task() {
    // Task pool size=1: long blockingSleep on blocking_pool must not starve a short Task.
    // Same-pool __syx_task_sleep would starve; help-while-wait + second pool must not.
    // Cancel the long Task after short joins so wall-clock reflects short-path latency.
    let src = r#"
func main() {
  __syx_task_set_worker_count(1)
  let longT = Task.detached {
    __syx_blocking_sleep(30000)
    print("LONG_DONE")
  }
  let shortT = Task.detached {
    print("SHORT_DONE")
  }
  __syx_task_join(shortT)
  print("SHORT_JOINED")
  __syx_task_cancel(longT)
  try {
    __syx_task_join(longT)
    print("LONG_JOIN_OK")
  } catch (err) {
    print("LONG_CANCELLED")
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("SHORT_DONE") && out.contains("SHORT_JOINED"),
        "short Task must complete under isolation: {out}"
    );
    assert!(
        elapsed.as_millis() < 1500,
        "short Task must not be starved by blockingSleep: elapsed={elapsed:?} out={out}"
    );
    assert!(
        !out.contains("LONG_DONE"),
        "long sleeper should be cancelled before finishing: {out}"
    );
}

#[test]
fn m3_b01_isolation_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_b01_isolation"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_b01_isolation = \"signed-ADR-275\""),
        "inventory must note signed-ADR-275"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-275")
            || guide.contains("blockingSleep")
            || guide.contains("M3-B01"),
        "UsageGuide must document M3-B01 isolation"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-B01 |"))
        .expect("M3-B01 row");
    assert!(
        b01.contains("| [~] |") || b01.contains("| [x] |"),
        "M3-B01 may be [~] after thin start or [x] after leaf closeout: {b01}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W19")
            && (roadmap.contains("B02")
                || roadmap.contains("adapter")
                || roadmap.contains("M3-B02")))
            || (roadmap.contains("W18")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-277")
                    || roadmap.contains("B01")))
            || (roadmap.contains("W17")
                && (roadmap.contains("deepen")
                    || roadmap.contains("ADR-276")
                    || roadmap.contains("B01")))
            || (roadmap.contains("W16") && roadmap.contains("ADR-275")),
        "roadmap may advance W16–W18 done / W19 = B02 thin start"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-275")
            || book.contains("blockingSleep")
            || book.contains("blocking pool")
            || book.contains("isolation"),
        "18-concurrency must document Task vs blocking pool thin isolation"
    );
}
