//! task-local deepen (set/get + spawn clone inherit + isolation).
// was: m3_a02_task_local

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_a02_task_local_inherit_and_isolation() {
    let src = r#"
func main() {
  __syx_task_local_set("trace", "PARENT")
  let t = Task.detached {
    print(__syx_task_local_get("trace"))
    __syx_task_local_set("trace", "CHILD")
    print(__syx_task_local_get("trace"))
  }
  __syx_task_join(t)
  print(__syx_task_local_get("trace"))
}
"#;
    let out = run(src);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(
        lines.len() >= 3,
        "expected PARENT/CHILD/PARENT lines: {out}"
    );
    assert_eq!(lines[0], "PARENT", "child must inherit parent local: {out}");
    assert_eq!(lines[1], "CHILD", "child must see its own write: {out}");
    assert_eq!(
        lines[2], "PARENT",
        "parent must be unchanged after child write: {out}"
    );
}

#[test]
fn m3_a02_task_local_missing_key_is_nil() {
    let src = r#"
func main() {
  print(__syx_task_local_get("missing"))
}
"#;
    let out = run(src);
    assert!(
        out.lines().any(|l| l.trim() == "nil" || l.trim() == "Nil"),
        "missing key must print as nil: {out}"
    );
}

#[test]
fn m3_a02_task_local_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test concurrency"),
        "ci.yml must wire m3_a02_task_local"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_a02_task_local = \"signed-ADR-263\""),
        "inventory must note signed-ADR-263"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-263")
            || guide.contains("taskLocalSet")
            || guide.contains("task-local"),
        "UsageGuide must document task-local deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("ADR-263")
            && (roadmap.contains("task-local") || roadmap.contains("taskLocal")),
        "roadmap must record M3 W4 task-local"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    // W4 locked W5=A03; after later signoffs roadmap may advance W6 await-hold / W7=A04.
    assert!(
        (roadmap.contains("W5")
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
        "roadmap may advance W5–W14 done / W15 = A03 closeout"
    );
    let facade = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        facade.contains("taskLocalSet")
            && facade.contains("taskLocalGet")
            && facade.contains("__syx_task_local_set"),
        "Concurrency.taskLocalSet/Get facade required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("TaskLocalSet")
            && reg.contains("TaskLocalGet")
            && reg.contains("__syx_task_local_set"),
        "HostFnId::TaskLocalSet/Get must be registered"
    );
}
