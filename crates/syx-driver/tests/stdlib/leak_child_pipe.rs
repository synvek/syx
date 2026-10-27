//! EC-M2-3-F ChildPipe session live-count leak first nail.
// was: ec_m2_3_child_pipe_leak

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{child_pipe_force_close, child_pipe_live_count, process_child_force_close};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    format!("{pb}\n{body}")
}

#[test]
fn ec_m2_3_child_pipe_leak_take_close_returns_to_baseline() {
    // Wave A W53 / ADR-397 / A-03: ChildPipe take+close product exact (drop print(p)).
    let baseline = child_pipe_live_count();
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let p = Child.stdout(handle: h)
    print("TAKEN")
    ChildPipe.close(pipe: p)
    let code = Child.wait(handle: h)
    print("CLOSED")
    print(code)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "TAKEN\nCLOSED\n0", "take+close must exact: {out}");
    assert_eq!(
        child_pipe_live_count(),
        baseline,
        "take+close must restore child_pipe_live_count"
    );
}

#[test]
fn ec_m2_3_child_pipe_leak_take_without_close_increments() {
    let baseline = child_pipe_live_count();
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let p = Child.stdout(handle: h)
    print("LEAKED")
    print(p)
    print("CHILD")
    print(h)
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    ));
    assert!(out.contains("LEAKED"), "{out}");
    assert!(!out.contains("caught"), "{out}");
    let after = child_pipe_live_count();
    assert_eq!(after, baseline + 1, "baseline={baseline} after={after}");
    let mut nums = out
        .lines()
        .filter_map(|l| l.trim().parse::<i64>().ok());
    let pipe_id = nums.next().expect("pipe id");
    let child_id = nums.next().expect("child id");
    assert!(child_pipe_force_close(pipe_id), "force_close pipe {pipe_id}");
    assert_eq!(child_pipe_live_count(), baseline);
    // Drop child so later tests stay isolated.
    let _ = process_child_force_close(child_id);
}
