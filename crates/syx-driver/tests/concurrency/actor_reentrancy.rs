//! actor reentrancy thin start (serial-no-reenter / C009).
// was: m3_a03_reentrancy

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a03_reentrancy_sync_self_call_fails_c009() {
    let src = r#"
actor Counter {
  var value: Int
  init(value: Int) {
    self.value = value
  }
  func inc() {
    self.value = self.value + 1
  }
  func bump() {
    self.inc()
  }
}

func main() {
  let c = Counter(value: 0)
  try {
    c.bump()
    print("OK")
  } catch (err) {
    print("DENIED")
    print(err.code)
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "same-actor sync reenter must fail fast (no hang): elapsed={elapsed:?} out={out}"
    );
    assert!(
        out.contains("DENIED") && out.contains("C009"),
        "sync self-call must surface C009: {out}"
    );
    assert!(!out.contains("OK"), "reentrant call must not succeed: {out}");
}

#[test]
fn m3_a03_reentrancy_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_actor_reentrancy_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("serial-no-reenter"),
        "policy HostFn must return serial-no-reenter: {out}"
    );
}

