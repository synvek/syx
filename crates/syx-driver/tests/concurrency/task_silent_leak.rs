//! silent-leak deepen (TaskRegistry live_count / wait reap).
// was: m3_a01_silent_leak

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a01_silent_leak_wait_returns_to_baseline() {
    let src = r#"
func main() {
  let baseline = __syx_task_live_count()
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(10)
    print("DONE")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_wait(g)
  let after = __syx_task_live_count()
  if after == baseline {
    print("BASELINE_OK")
  } else {
    print("BASELINE_FAIL")
    print(baseline)
    print(after)
  }
}
"#;
    let out = run(src);
    assert!(out.contains("DONE"), "child must run: {out}");
    assert!(
        out.contains("BASELINE_OK"),
        "taskGroupWait must reap to baseline: {out}"
    );
    assert!(!out.contains("BASELINE_FAIL"), "{out}");
}

#[test]
fn m3_a01_silent_leak_cancel_wait_returns_to_baseline() {
    let src = r#"
func main() {
  let baseline = __syx_task_live_count()
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_cancel(g)
  __syx_task_group_wait(g)
  let after = __syx_task_live_count()
  if after == baseline {
    print("CANCEL_BASELINE_OK")
  } else {
    print("CANCEL_BASELINE_FAIL")
    print(baseline)
    print(after)
  }
}
"#;
    let out = run(src);
    assert!(
        !out.contains("SLEPT"),
        "cancelled sleeper must not finish: {out}"
    );
    assert!(
        out.contains("CANCEL_BASELINE_OK"),
        "cancel+wait must reap to baseline: {out}"
    );
    assert!(!out.contains("CANCEL_BASELINE_FAIL"), "{out}");
}

#[test]
fn m3_a01_silent_leak_orphan_increments_live_count() {
    let src = r#"
func main() {
  let baseline = __syx_task_live_count()
  let t = Task.detached {
    print("ORPHAN_BODY")
  }
  let after = __syx_task_live_count()
  if after == baseline + 1 {
    print("LEAKED")
  } else {
    print("LEAK_FAIL")
    print(baseline)
    print(after)
  }
  // Teardown: join reaps so later tests stay isolated.
  try {
    __syx_task_join(t)
  } catch (err) {
    print("JOIN_TEARDOWN")
  }
  let cleaned = __syx_task_live_count()
  if cleaned == baseline {
    print("CLEANED")
  } else {
    print("CLEAN_FAIL")
    print(cleaned)
  }
}
"#;
    let out = run(src);
    assert!(out.contains("LEAKED"), "unreaped detach must elevate live_count: {out}");
    assert!(!out.contains("LEAK_FAIL"), "{out}");
    assert!(
        out.contains("CLEANED"),
        "join teardown must restore baseline: {out}"
    );
    assert!(!out.contains("CLEAN_FAIL"), "{out}");
}

