//! EC-M2-3 Process child live-count leak first nail.
// was: ec_m2_3_process_leak

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{process_child_force_close, process_child_live_count};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let proc = include_str!("../../../../packages/process/mod.sy");
    format!("{proc}\n{body}")
}

#[test]
fn ec_m2_3_process_leak_facade_source_has_spawn_wait() {
    let src = include_str!("../../../../packages/process/mod.sy");
    assert!(
        src.contains("func spawn(command: String, args: Array<String>) -> Int")
            || (src.contains("spawn") && src.contains("Array")),
        "spawn required: {src}"
    );
    assert!(
        src.contains("func wait(handle: Int) -> Int") || src.contains("wait(handle: Int)"),
        "wait required: {src}"
    );
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(
        hosts.contains("pub fn process_child_live_count"),
        "process_child_live_count must be exported for nails"
    );
    let wait_fn = hosts
        .split("pub fn host_process_wait")
        .nth(1)
        .expect("host_process_wait");
    let body = wait_fn.split("pub fn ").next().unwrap_or(wait_fn);
    assert!(
        body.contains("remove"),
        "host_process_wait must remove from CHILD_TABLE: {body}"
    );
}

#[test]
fn ec_m2_3_process_leak_spawn_wait_returns_to_baseline() {
    // Wave A W52 / ADR-396 / A-03: Process spawn+wait product exact (drop print(h)).
    let baseline = process_child_live_count();

    let out = run(&facade_src(
        r#"
func main() {
  let h = Process.spawn(command: "/usr/bin/true", args: [])
  print("SPAWNED")
  let code = Process.wait(handle: h)
  print("WAITED")
  print(code)
}
"#,
    ));
    assert_eq!(out, "SPAWNED\nWAITED\n0", "spawn+wait must exact: {out}");
    assert_eq!(
        process_child_live_count(),
        baseline,
        "spawn+wait must return live count to baseline"
    );
}

#[test]
fn ec_m2_3_process_leak_spawn_without_wait_increments() {
    let baseline = process_child_live_count();

    let out = run(&facade_src(
        r#"
func main() {
  let h = Process.spawn(command: "/usr/bin/true", args: [])
  print("LEAKED")
  print(h)
}
"#,
    ));
    assert!(out.contains("LEAKED"), "spawn without wait must succeed: {out}");

    let after = process_child_live_count();
    assert_eq!(
        after,
        baseline + 1,
        "spawn without wait must leave live_count == baseline + 1 (baseline={baseline}, after={after})"
    );

    let id: i64 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("printed handle id");
    assert!(id >= 0, "handle must be >= 0, got {id}");
    assert!(
        process_child_force_close(id),
        "force_close must drop leaked child {id}"
    );
    assert_eq!(
        process_child_live_count(),
        baseline,
        "cleanup must restore baseline"
    );
}
