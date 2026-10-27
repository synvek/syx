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

#[test]
fn m3_c02_semaphore_raii_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_c02_semaphore_raii"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_c02_semaphore_raii = \"signed-ADR-285\""),
        "inventory must note signed-ADR-285"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-285")
            || guide.contains("SemaphoreGuard")
            || guide.contains("m3_c02_semaphore_raii"),
        "UsageGuide must document M3-C02 SemaphoreGuard RAII"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let c02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-C02 |"))
        .expect("M3-C02 row");
    assert!(
        c02.contains("| [~] |") || c02.contains("| [x] |"),
        "M3-C02 may be [~] or [x] after later windows: {c02}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        (roadmap.contains("W28")
            && (roadmap.contains("C03")
                || roadmap.contains("Atomic")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W27")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-286")
                    || roadmap.contains("C02")))
            || (roadmap.contains("W26") && roadmap.contains("ADR-285")),
        "roadmap may advance W26–W27 done / W28 = C03 thin start"
    );
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(
        sync.contains("class SemaphoreGuard")
            && sync.contains("waitGuard")
            && sync.contains("__syx_semaphore_signal_quiet"),
        "sync facade must expose SemaphoreGuard RAII"
    );
    let book = include_str!("../../../../books/language-guide/18-concurrency.md");
    assert!(
        book.contains("ADR-285")
            || book.contains("SemaphoreGuard")
            || book.contains("semaphore-v1-raii"),
        "18-concurrency must reflect SemaphoreGuard RAII deepen"
    );
}
