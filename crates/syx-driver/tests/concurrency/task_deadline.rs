//! deadline thin start (setDeadlineMs → inherit → C004).
// was: m3_a02_deadline

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a02_deadline_child_sleep_joins_c004_promptly() {
    let src = r#"
func main() {
  __syx_task_set_deadline_ms(50)
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  try {
    __syx_task_join(t)
    print("JOIN_OK")
  } catch (err) {
    print("JOIN_DEADLINE")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "deadline must interrupt long sleep: elapsed={elapsed:?} out={out}"
    );
    assert!(
        !out.contains("SLEPT"),
        "deadline-cancelled sleeper must not finish: {out}"
    );
    assert!(
        out.contains("JOIN_DEADLINE") && out.contains("C004"),
        "join must surface C004 on deadline: {out}"
    );
    assert!(!out.contains("JOIN_OK"), "join must not succeed: {out}");
}

#[test]
fn m3_a02_deadline_group_child_inherits() {
    let src = r#"
func main() {
  __syx_task_set_deadline_ms(50)
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_wait(g)
  print("WAITED")
  try {
    __syx_task_join(t)
    print("JOIN_OK")
  } catch (err) {
    print("JOIN_DEADLINE")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "group wait under deadline must not hang: elapsed={elapsed:?} out={out}"
    );
    assert!(out.contains("WAITED"), "wait must complete: {out}");
    assert!(!out.contains("SLEPT"), "child must not finish sleep: {out}");
    assert!(
        out.contains("JOIN_DEADLINE") && out.contains("C004"),
        "late join must still surface C004: {out}"
    );
}

