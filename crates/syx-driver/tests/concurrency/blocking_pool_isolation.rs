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

