//! task tree first nail (TaskGroup cancel → wait / C004).
// was: m3_a01_task_tree

use std::time::Instant;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a01_task_tree_cancel_wait_returns_promptly_and_join_c004() {
    // Product surface = Concurrency.taskGroup* (HostFn TaskGroupCancel); call hosts
    // directly to avoid embedding Concurrency prelude shadow (S023). Task.detached
    // matches production codegen (ConstStr body name → TaskDetach).
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

#[test]
fn m3_a01_task_tree_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a01_task_tree"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a01_task_tree = \"signed-ADR-260\""),
        "inventory must note signed-ADR-260"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-260") || guide.contains("M3-A01"),
        "UsageGuide must document M3-A01 first nail"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M3-A01")
            && (roadmap.contains("首钉")
                || roadmap.contains("first nail")
                || roadmap.contains("ADR-260")),
        "roadmap must record M3-A01 first nail"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    // W1 locked W2=silent-leak; after W2 signoff roadmap may advance to W3=A02.
    assert!(
        (roadmap.contains("W2")
            && (roadmap.contains("silent")
                || roadmap.contains("leak")
                || roadmap.contains("ADR-261")
                || roadmap.contains("done")))
            || (roadmap.contains("W3")
                && (roadmap.contains("A02") || roadmap.contains("deadline"))),
        "roadmap may advance W2 silent-leak done / W3 = A02"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("taskGroupCancel") && facade.contains("__syx_task_group_cancel"),
        "Concurrency.taskGroupCancel facade required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("TaskGroupCancel") && reg.contains("__syx_task_group_cancel"),
        "HostFnId::TaskGroupCancel must be registered"
    );
}
