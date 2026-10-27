//! EC-M2-3-F FileWatcher session live-count leak first nail.
// was: ec_m2_3_file_watch_leak

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{file_watch_force_close, file_watch_live_count};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn ec_m2_3_file_watch_leak_open_close_returns_to_baseline() {
    // Wave A W50 / ADR-394 / A-03: FileWatcher open+close product exact (drop print(h)).
    let baseline = file_watch_live_count();
    let out = run(
        r#"
func main() {
  let h = __syx_file_watcher_open("/tmp/syx_watch_ok")
  print("OPENED")
  __syx_file_watcher_close(h)
  print("CLOSED")
}
"#,
    );
    assert_eq!(out, "OPENED\nCLOSED", "open+close must exact: {out}");
    assert_eq!(
        file_watch_live_count(),
        baseline,
        "open+close must restore file_watch_live_count"
    );
}

#[test]
fn ec_m2_3_file_watch_leak_open_without_close_increments() {
    let baseline = file_watch_live_count();
    let out = run(
        r#"
func main() {
  let h = __syx_file_watcher_open("/tmp/syx_watch_leak")
  print("LEAKED")
  print(h)
}
"#,
    );
    assert!(out.contains("LEAKED"), "{out}");
    let after = file_watch_live_count();
    assert_eq!(after, baseline + 1, "baseline={baseline} after={after}");
    let id: i64 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("handle id");
    assert!(file_watch_force_close(id), "force_close {id}");
    assert_eq!(file_watch_live_count(), baseline);
}
