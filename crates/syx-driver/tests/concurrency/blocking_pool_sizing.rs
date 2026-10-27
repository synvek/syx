//! isolation deepen (tunable blocking_pool size).
// was: m3_b01_pool_sizing

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_b01_pool_sizing_fingerprint_and_getter() {
    let src = r#"
func main() {
  print(__syx_isolation_policy())
  let set = __syx_blocking_pool_set_size(3)
  print(set)
  print(__syx_blocking_pool_size())
}
"#;
    let out = run(src);
    assert!(
        out.contains("task-pool+blocking-pool-v1-sized"),
        "isolationPolicy must be v1-sized: {out}"
    );
    assert!(
        out.contains("3"),
        "setBlockingPoolSize/blockingPoolSize must report 3: {out}"
    );
}

#[test]
fn m3_b01_pool_sizing_parallel_blocking_sleeps() {
    // size=2: two concurrent blockingSleep(800) should finish near one duration, not sum.
    let src = r#"
func main() {
  __syx_task_set_worker_count(2)
  __syx_blocking_pool_set_size(2)
  let a = Task.detached {
    __syx_blocking_sleep(800)
    print("A_DONE")
  }
  let b = Task.detached {
    __syx_blocking_sleep(800)
    print("B_DONE")
  }
  __syx_task_join(a)
  __syx_task_join(b)
  print("BOTH_JOINED")
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("A_DONE") && out.contains("B_DONE") && out.contains("BOTH_JOINED"),
        "both blockingSleep tasks must finish: {out}"
    );
    assert!(
        elapsed.as_millis() < 1500,
        "size=2 must allow parallel blockingSleep: elapsed={elapsed:?} out={out}"
    );
}

#[test]
fn m3_b01_pool_sizing_queued_blocking_does_not_starve_short_task() {
    // size=1 + two long blockingSleep: blocking queue backs up, but short Task must still run.
    let src = r#"
func main() {
  __syx_task_set_worker_count(2)
  __syx_blocking_pool_set_size(1)
  let longA = Task.detached {
    __syx_blocking_sleep(30000)
    print("LONG_A")
  }
  let longB = Task.detached {
    __syx_blocking_sleep(30000)
    print("LONG_B")
  }
  let shortT = Task.detached {
    print("SHORT_DONE")
  }
  __syx_task_join(shortT)
  print("SHORT_JOINED")
  __syx_task_cancel(longA)
  __syx_task_cancel(longB)
  try { __syx_task_join(longA) } catch (e1) { print("A_CANCELLED") }
  try { __syx_task_join(longB) } catch (e2) { print("B_CANCELLED") }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("SHORT_DONE") && out.contains("SHORT_JOINED"),
        "short Task must complete under size=1 blocking queue: {out}"
    );
    assert!(
        elapsed.as_millis() < 2000,
        "blocking queue must not starve Task pool: elapsed={elapsed:?} out={out}"
    );
    assert!(
        !out.contains("LONG_A") && !out.contains("LONG_B"),
        "long sleepers should be cancelled: {out}"
    );
}

#[test]
fn m3_b01_pool_sizing_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_b01_pool_sizing"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_b01_pool_sizing = \"signed-ADR-276\""),
        "inventory must note signed-ADR-276"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-276")
            || guide.contains("setBlockingPoolSize")
            || guide.contains("pool_sizing"),
        "UsageGuide must document M3-B01 deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let b01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-B01 |"))
        .expect("M3-B01 row");
    assert!(
        b01.contains("| [~] |") || b01.contains("| [x] |"),
        "M3-B01 may be [~] after deepen or [x] after leaf closeout: {b01}"
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
            || (roadmap.contains("W17") && roadmap.contains("ADR-276")),
        "roadmap may advance W17/W18 done / W19 = B02 thin start"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-276")
            || book.contains("setBlockingPoolSize")
            || book.contains("blockingPoolSize"),
        "18-concurrency must document tunable blocking pool"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("setBlockingPoolSize")
            && facade.contains("__syx_blocking_pool_set_size"),
        "concurrency.sy must expose setBlockingPoolSize"
    );
}
