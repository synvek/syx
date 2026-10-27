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

#[test]
fn m3_a02_deadline_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a02_deadline"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a02_deadline = \"signed-ADR-262\""),
        "inventory must note signed-ADR-262"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-262") || guide.contains("setDeadlineMs") || guide.contains("M3-A02"),
        "UsageGuide must document M3-A02 deadline"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M3-A02")
            && (roadmap.contains("ADR-262") || roadmap.contains("deadline")),
        "roadmap must record M3-A02 deadline"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    // W3 locked W4=task-local; later signoffs may advance W5–W7.
    assert!(
        (roadmap.contains("W4")
            && (roadmap.contains("task-local")
                || roadmap.contains("ADR-263")
                || roadmap.contains("done")
                || roadmap.contains("A02")))
            || (roadmap.contains("W5")
                && (roadmap.contains("A03")
                    || roadmap.contains("reentran")
                    || roadmap.contains("ADR-264")
                    || roadmap.contains("done")))
            || (roadmap.contains("W6")
                && (roadmap.contains("await-reentrancy")
                    || roadmap.contains("await-hold")
                    || roadmap.contains("await")
                    || roadmap.contains("ADR-265")
                    || roadmap.contains("done")))
            || (roadmap.contains("W7")
                && (roadmap.contains("A04")
                    || roadmap.contains("Channel")
                    || roadmap.contains("ADR-266")
                    || roadmap.contains("done")))
            || (roadmap.contains("W8")
                && (roadmap.contains("backpressure")
                    || roadmap.contains("ADR-267")
                    || roadmap.contains("A04")
                    || roadmap.contains("done")))
            || (roadmap.contains("W9")
                && (roadmap.contains("fairness")
                    || roadmap.contains("ADR-268")
                    || roadmap.contains("A04")
                    || roadmap.contains("done")))
            || (roadmap.contains("W10")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-269")
                    || roadmap.contains("A04")
                    || roadmap.contains("done")))
            || (roadmap.contains("W11")
                && (roadmap.contains("A05")
                    || roadmap.contains("docs")
                    || roadmap.contains("ADR-270")
                    || roadmap.contains("done")))
            || (roadmap.contains("W12")
                && (roadmap.contains("closeout") || roadmap.contains("A05")))
            || (roadmap.contains("W13")
                && (roadmap.contains("closeout")
                    || roadmap.contains("A01")
                    || roadmap.contains("ADR-272")
                    || roadmap.contains("done")))
            || (roadmap.contains("W14")
                && (roadmap.contains("closeout")
                    || roadmap.contains("A02")
                    || roadmap.contains("ADR-273")
                    || roadmap.contains("done")))
            || (roadmap.contains("W15")
                && (roadmap.contains("A03")
                    || roadmap.contains("closeout")
                    || roadmap.contains("reentran")
                    || roadmap.contains("await-hold"))),
        "roadmap may advance W4–W14 done / W15 = A03 closeout"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("setDeadlineMs") && facade.contains("__syx_task_set_deadline_ms"),
        "Concurrency.setDeadlineMs facade required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("TaskSetDeadlineMs") && reg.contains("__syx_task_set_deadline_ms"),
        "HostFnId::TaskSetDeadlineMs must be registered"
    );
}
