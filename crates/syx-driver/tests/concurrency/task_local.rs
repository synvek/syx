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

