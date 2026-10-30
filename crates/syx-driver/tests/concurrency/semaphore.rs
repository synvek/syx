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

