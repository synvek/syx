//! Mutex blocking deepen (lock_wait + cross-Task contention).
// was: m3_c01_mutex_blocking

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_c01_mutex_blocking_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_sync_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("mutex-raii-guard-v1-blocking"),
        "syncPolicy fingerprint after deepen: {out}"
    );
}

#[test]
fn m3_c01_mutex_blocking_try_lock_still_nonblocking() {
    let src = r#"
func main() {
  let m = __syx_mutex_create()
  print(__syx_mutex_lock(m))
  print(__syx_mutex_lock(m))
  print(__syx_mutex_unlock(m))
  print(__syx_mutex_lock(m))
}
"#;
    let out = run(src);
    let bools: Vec<&str> = out
        .lines()
        .filter(|l| *l == "true" || *l == "false")
        .collect();
    assert_eq!(
        bools,
        vec!["true", "false", "true", "true"],
        "try-lock must stay non-blocking Bool: {out}"
    );
}

#[test]
fn m3_c01_mutex_blocking_cross_task_contention() {
    // Task.detached closures cannot capture locals — pass mutex id via __syx_task_detach args.
    // Task A holds; Task B lock_wait blocks until A unlocks; print order proves wait.
    let src = r#"
func holdThenUnlock(_ mid: Object) {
  let ok = __syx_mutex_lock_wait(mid)
  print("A_HELD")
  print(ok)
  __syx_blocking_sleep(250)
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
  __syx_blocking_sleep(40)
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
        out.contains("A_HELD") && out.contains("A_UNLOCK") && out.contains("B_GOT") && out.contains("DONE"),
        "cross-Task wait path must complete: {out}"
    );
    let a_held = out.find("A_HELD").expect("A_HELD");
    let a_unlock = out.find("A_UNLOCK").expect("A_UNLOCK");
    let b_got = out.find("B_GOT").expect("B_GOT");
    assert!(
        a_held < a_unlock && a_unlock < b_got,
        "order must be A_HELD → A_UNLOCK → B_GOT: {out}"
    );
    assert!(
        elapsed.as_millis() >= 200,
        "B must have waited on held lock (elapsed too short={elapsed:?}): {out}"
    );
    assert!(
        elapsed.as_millis() < 5000,
        "contention wait must finish timely: elapsed={elapsed:?} out={out}"
    );
}

#[test]
fn m3_c01_mutex_blocking_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c01_mutex_blocking"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c01_mutex_blocking = \"signed-ADR-282\""),
        "inventory must note signed-ADR-282"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-282")
            || guide.contains("lock_wait")
            || guide.contains("m3_c01_mutex_blocking"),
        "UsageGuide must document M3-C01 blocking deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C01 |"))
        .expect("M3-C01 row");
    assert!(
        c01.contains("| [~] |") || c01.contains("| [x] |"),
        "M3-C01 may be [~] or [x] after later windows: {c01}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W25")
            && (roadmap.contains("C02")
                || roadmap.contains("RWLock")
                || roadmap.contains("Semaphore")))
            || (roadmap.contains("W24")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-283")
                    || roadmap.contains("C01")))
            || (roadmap.contains("W23") && roadmap.contains("ADR-282")),
        "roadmap may advance W23/W24 done / W25 = C02 thin start"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("__syx_mutex_lock_wait")
            && sync.contains("lockGuard"),
        "sync facade must route lock/lockGuard to wait"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-282")
            || book.contains("lock_wait")
            || book.contains("v1-blocking"),
        "18-concurrency must reflect blocking deepen"
    );
}
