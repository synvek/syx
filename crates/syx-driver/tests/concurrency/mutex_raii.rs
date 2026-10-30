//! Mutex RAII thin start (MutexGuard + forget-unlock).
// was: m3_c01_mutex_raii

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

/// Embed-safe Mutex RAII surface (full `sync/mod.sy` also has Channel/Select generics).
fn mutex_facade() -> &'static str {
    r#"
public class MutexGuard {
    public var mutexId: Int = -1

    deinit {
        __syx_mutex_unlock_quiet(self.mutexId)
    }
}

public class Mutex {
    public static func create() -> Object {
        return __syx_mutex_create()
    }

    public static func lock(mutex: Object) {
        __syx_mutex_lock(mutex)
    }

    public static func unlock(mutex: Object) {
        __syx_mutex_unlock(mutex)
    }

    public static func lockGuard(mutex: Object) -> MutexGuard {
        let ok = __syx_mutex_lock(mutex)
        let g = MutexGuard()
        if ok {
            g.mutexId = mutex
        }
        return g
    }
}
"#
}

fn facade_src(body: &str) -> String {
    format!("{}\n{body}", mutex_facade())
}

#[test]
fn m3_c01_mutex_raii_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_sync_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("mutex-raii-guard-v1-thin")
            || out.contains("mutex-raii-guard-v1-blocking"),
        "syncPolicy fingerprint (thin or deepen): {out}"
    );
}

#[test]
fn m3_c01_mutex_raii_forget_unlock_scope_exit() {
    // Helper return drops MutexGuard → deinit quiet unlock; no explicit unlock.
    let out = run(&facade_src(
        r#"
func holdThenDrop(_ m: Object) {
  let g = Mutex.lockGuard(mutex: m)
  print("HELD")
}

func main() {
  let m = __syx_mutex_create()
  let first = __syx_mutex_lock(m)
  print(first)
  __syx_mutex_unlock(m)
  holdThenDrop(m)
  let again = __syx_mutex_lock(m)
  print(again)
}
"#,
    ));
    assert!(
        out.contains("HELD"),
        "lockGuard path must run: {out}"
    );
    let lines: Vec<&str> = out.lines().collect();
    let bools: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| *l == "true" || *l == "false")
        .collect();
    assert!(
        bools.len() >= 2 && bools[0] == "true" && bools.last() == Some(&"true"),
        "initial lock + re-lock after forget-unlock must succeed: {out}"
    );
}

#[test]
fn m3_c01_mutex_raii_quiet_unlock_unknown_ok() {
    let out = run(
        r#"
func main() {
  print(__syx_mutex_unlock_quiet(0 - 1))
  print(__syx_mutex_unlock_quiet(999999))
}
"#,
    );
    assert!(
        out.contains("true"),
        "quiet unlock must silently succeed: {out}"
    );
}

