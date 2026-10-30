//! chaos sampling deepen (TaskGroup cancel → C004).
// was: m3_c04_chaos

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_c04_chaos_task_group_cancel_interrupts_long_sleep() {
    // C04-owned chaos micro-sample: cancel mid-flight long sleep → C004 promptly.
    let src = r#"
func main() {
  let g = __syx_task_group_create()
  let t = Task.detached {
    __syx_task_sleep(30000)
    print("SLEPT")
  }
  __syx_task_group_add(g, t)
  __syx_task_group_cancel(g)
  __syx_task_group_wait(g)
  print("WAITED")
  try {
    __syx_task_join(t)
    print("JOIN_OK")
  } catch (err) {
    print("JOIN_CANCEL")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "cancel+wait must not hang for full sleep: elapsed={elapsed:?} out={out}"
    );
    assert!(out.contains("WAITED"), "wait must complete: {out}");
    assert!(
        !out.contains("SLEPT"),
        "cancelled sleeper must not finish sleep: {out}"
    );
    assert!(
        out.contains("JOIN_CANCEL") && out.contains("C004"),
        "cancelled child join must surface C004: {out}"
    );
    assert!(!out.contains("JOIN_OK"), "join must not succeed: {out}");
}

