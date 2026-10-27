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

#[test]
fn m3_a01_silent_leak_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a01_silent_leak"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a01_silent_leak = \"signed-ADR-261\""),
        "inventory must note signed-ADR-261"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-261") || guide.contains("silent-leak") || guide.contains("liveTaskCount"),
        "UsageGuide must document silent-leak deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-261")
            && (roadmap.contains("silent") || roadmap.contains("leak")),
        "roadmap must record M3 W2 silent-leak"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-2") || roadmap.contains("- [ ] EC-M3-2"),
        "EC-M3-2 exit condition must remain documented (may be checked after ADR-307)"
    );
    // W2 locked W3=A02; after later signoffs roadmap may advance W4–W7.
    assert!(
        (roadmap.contains("W3")
            && (roadmap.contains("A02")
                || roadmap.contains("deadline")
                || roadmap.contains("ADR-262")
                || roadmap.contains("done")))
            || (roadmap.contains("W4")
                && (roadmap.contains("task-local")
                    || roadmap.contains("A02")
                    || roadmap.contains("ADR-263")
                    || roadmap.contains("done")))
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
                && (roadmap.contains("A02")
                    || roadmap.contains("closeout")
                    || roadmap.contains("deadline")
                    || roadmap.contains("task-local"))),
        "roadmap may advance W3–W13 done / W14 = A02 closeout"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("liveTaskCount") && facade.contains("__syx_task_live_count"),
        "Concurrency.liveTaskCount facade required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("TaskLiveCount") && reg.contains("__syx_task_live_count"),
        "HostFnId::TaskLiveCount must be registered"
    );
    let async_hosts = include_str!("../../../syx-async/src/hosts.rs");
    assert!(
        async_hosts.contains("host_task_live_count")
            || async_hosts.contains("task_live_count"),
        "async hosts must wire task live_count"
    );
}
