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

