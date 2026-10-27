//! Semaphore thin start (create/wait/tryWait/signal).
// was: m3_c02_semaphore

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_c02_semaphore_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_semaphore_policy())
  print(__syx_sync_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("semaphore-v1-thin") || out.contains("semaphore-v1-raii"),
        "semaphorePolicy fingerprint (thin or raii): {out}"
    );
    assert!(
        out.contains("mutex-raii-guard-v1-blocking"),
        "C01 syncPolicy must remain unchanged: {out}"
    );
}

#[test]
fn m3_c02_semaphore_try_wait_and_count() {
    let src = r#"
func main() {
  let s = __syx_semaphore_create(1)
  print(__syx_semaphore_wait(s))
  print(__syx_semaphore_try_wait(s))
  print(__syx_semaphore_signal(s))
  print(__syx_semaphore_try_wait(s))
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
        "wait then tryWait false; signal restores tryWait: {out}"
    );
}

#[test]
fn m3_c02_semaphore_cross_task_wait_signal() {
    // Task.detached cannot capture locals — pass sem id via __syx_task_detach.
    let src = r#"
func waiter(_ sid: Object) {
  print("W_WAIT")
  let ok = __syx_semaphore_wait(sid)
  print("W_GOT")
  print(ok)
}

func signaller(_ sid: Object) {
  __syx_blocking_sleep(200)
  print("S_SIGNAL")
  __syx_semaphore_signal(sid)
}

func main() {
  __syx_task_set_worker_count(2)
  let s = __syx_semaphore_create(0)
  let w = __syx_task_detach("waiter", [s])
  __syx_blocking_sleep(40)
  let sig = __syx_task_detach("signaller", [s])
  __syx_task_join(w)
  __syx_task_join(sig)
  print("DONE")
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("W_WAIT")
            && out.contains("S_SIGNAL")
            && out.contains("W_GOT")
            && out.contains("DONE"),
        "cross-Task wait/signal must complete: {out}"
    );
    let w_wait = out.find("W_WAIT").expect("W_WAIT");
    let s_signal = out.find("S_SIGNAL").expect("S_SIGNAL");
    let w_got = out.find("W_GOT").expect("W_GOT");
    assert!(
        w_wait < s_signal && s_signal < w_got,
        "order must be W_WAIT → S_SIGNAL → W_GOT: {out}"
    );
    assert!(
        elapsed.as_millis() >= 150,
        "waiter must have blocked until signal (elapsed={elapsed:?}): {out}"
    );
    assert!(
        elapsed.as_millis() < 5000,
        "wait/signal must finish timely: elapsed={elapsed:?} out={out}"
    );
}

#[test]
fn m3_c02_semaphore_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c02_semaphore"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c02_semaphore = \"signed-ADR-284\""),
        "inventory must note signed-ADR-284"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-284")
            || guide.contains("Semaphore")
            || guide.contains("m3_c02_semaphore"),
        "UsageGuide must document M3-C02 Semaphore"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C02 |"))
        .expect("M3-C02 row");
    assert!(
        c02.contains("| [~] |") || c02.contains("| [x] |"),
        "M3-C02 may be [~] or [x] after later windows: {c02}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W28")
            && (roadmap.contains("C03")
                || roadmap.contains("Atomic")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W27")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-286")
                    || roadmap.contains("C02")))
            || (roadmap.contains("W26")
                && (roadmap.contains("deepen")
                    || roadmap.contains("ADR-285")
                    || roadmap.contains("C02")))
            || (roadmap.contains("W25") && roadmap.contains("ADR-284")),
        "roadmap may advance W25–W27 done / W28 = C03 thin start"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("class Semaphore")
            && sync.contains("__syx_semaphore_wait")
            && sync.contains("tryWait"),
        "sync facade must expose Semaphore"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-284")
            || book.contains("Semaphore")
            || book.contains("semaphore-v1"),
        "18-concurrency must reflect Semaphore thin nail"
    );
}
