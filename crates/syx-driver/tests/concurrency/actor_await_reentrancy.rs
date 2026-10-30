//! await-hold deepen (no mid-await mailbox interleaving).
// was: m3_a03_await_reentrancy

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a03_await_hold_peek_sees_after_not_before() {
    // hold(): BEFORE → sleep → AFTER. Concurrent peek while sleeping must wait for
    // the handler to finish (await-hold), so peek observes AFTER — not BEFORE
    // (which would indicate Swift-style mid-await mailbox reentry).
    // Task.detached closures cannot capture locals — pass actor via detach args.
    let src = r#"
actor Hold {
  var phase: String
  init(phase: String) {
    self.phase = phase
  }
  func hold() {
    self.phase = "BEFORE"
    __syx_task_sleep(200)
    self.phase = "AFTER"
  }
  func peek() -> String {
    return self.phase
  }
}

func kickHold(_ a: Hold) {
  a.hold()
}

func main() {
  let a = Hold(phase: "INIT")
  let t = __syx_task_detach("kickHold", [a])
  __syx_task_sleep(80)
  print(a.peek())
  __syx_task_join(t)
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "await-hold peek must not hang: elapsed={elapsed:?} out={out}"
    );
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        !lines.is_empty(),
        "expected peek print: {out}"
    );
    assert_eq!(
        lines[0], "AFTER",
        "peek during hold-sleep must serialize to AFTER (await-hold), not BEFORE: {out}"
    );
    assert!(
        !out.contains("BEFORE"),
        "must not observe mid-await BEFORE from reentrant peek: {out}"
    );
}

#[test]
fn m3_a03_await_hold_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_actor_reentrancy_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("serial-no-reenter") && out.contains("await-hold"),
        "policy must be serial-no-reenter;await-hold: {out}"
    );
}

