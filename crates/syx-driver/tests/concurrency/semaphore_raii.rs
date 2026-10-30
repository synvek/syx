//! SemaphoreGuard RAII deepen (forget-signal → scope exit).
// was: m3_c02_semaphore_raii

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

/// Embed-safe Semaphore RAII surface (full `sync/mod.sy` also has Channel/Select generics).
fn semaphore_facade() -> &'static str {
    r#"
public class SemaphoreGuard {
    public var semId: Int = -1

    deinit {
        __syx_semaphore_signal_quiet(self.semId)
    }
}

public class Semaphore {
    public static func create(permits: Int) -> Object {
        return __syx_semaphore_create(permits)
    }

    public static func wait(sem: Object) {
        __syx_semaphore_wait(sem)
    }

    public static func tryWait(sem: Object) -> Bool {
        return __syx_semaphore_try_wait(sem)
    }

    public static func signal(sem: Object) {
        __syx_semaphore_signal(sem)
    }

    public static func waitGuard(sem: Object) -> SemaphoreGuard {
        let ok = __syx_semaphore_wait(sem)
        let g = SemaphoreGuard()
        if ok {
            g.semId = sem
        }
        return g
    }
}
"#
}

fn facade_src(body: &str) -> String {
    format!("{}\n{body}", semaphore_facade())
}

#[test]
fn m3_c02_semaphore_raii_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_semaphore_policy())
  print(__syx_sync_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("semaphore-v1-raii"),
        "semaphorePolicy fingerprint must be v1-raii: {out}"
    );
    assert!(
        out.contains("mutex-raii-guard-v1-blocking"),
        "C01 syncPolicy must remain unchanged: {out}"
    );
}

#[test]
fn m3_c02_semaphore_raii_forget_signal_scope_exit() {
    // create(0) → signal once → waitGuard in helper (no explicit signal) → tryWait succeeds.
    let out = run(&facade_src(
        r#"
func holdThenDrop(_ s: Object) {
  let g = Semaphore.waitGuard(sem: s)
  print("HELD")
}

func main() {
  let s = __syx_semaphore_create(0)
  __syx_semaphore_signal(s)
  holdThenDrop(s)
  let again = __syx_semaphore_try_wait(s)
  print(again)
}
"#,
    ));
    assert!(
        out.contains("HELD"),
        "waitGuard path must run: {out}"
    );
    assert!(
        out.lines().any(|l| l == "true"),
        "tryWait after forget-signal scope exit must succeed: {out}"
    );
}

#[test]
fn m3_c02_semaphore_raii_quiet_signal_unknown_ok() {
    let out = run(
        r#"
func main() {
  print(__syx_semaphore_signal_quiet(0 - 1))
  print(__syx_semaphore_signal_quiet(999999))
}
"#,
    );
    assert!(
        out.contains("true"),
        "quiet signal must silently succeed: {out}"
    );
}

