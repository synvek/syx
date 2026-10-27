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

#[test]
fn m3_c01_mutex_raii_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c01_mutex_raii"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c01_mutex_raii = \"signed-ADR-281\""),
        "inventory must note signed-ADR-281"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-281")
            || guide.contains("MutexGuard")
            || guide.contains("m3_c01_mutex_raii"),
        "UsageGuide must document M3-C01 Mutex RAII"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C01 |"))
        .expect("M3-C01 row");
    assert!(
        c01.contains("| [~] |") || c01.contains("| [x] |"),
        "M3-C01 may be [~] or [x] after later windows: {c01}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W25")
            && (roadmap.contains("C02")
                || roadmap.contains("RWLock")
                || roadmap.contains("Semaphore")))
            || (roadmap.contains("W24")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-283")
                    || roadmap.contains("C01")
                    || roadmap.contains("Mutex")))
            || (roadmap.contains("W23")
                && (roadmap.contains("ADR-282")
                    || roadmap.contains("deepen")
                    || roadmap.contains("C01")))
            || (roadmap.contains("W22") && roadmap.contains("ADR-281")),
        "roadmap may advance W22–W24 done / W25 = C02 thin start"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("class MutexGuard")
            && sync.contains("lockGuard")
            && sync.contains("__syx_mutex_unlock_quiet"),
        "sync facade must expose MutexGuard RAII"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-281")
            || book.contains("MutexGuard")
            || book.contains("mutex-raii"),
        "18-concurrency must reflect Mutex RAII thin nail"
    );
}
