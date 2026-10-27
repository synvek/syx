//! EC-M2-3-F FileStream session live-count leak first nail.
// was: ec_m2_3_file_stream_leak

use std::fs;
use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{file_stream_force_close, file_stream_live_count};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_ec_m2_3_file_stream_leak_{name}"))
}

#[test]
fn ec_m2_3_file_stream_leak_open_close_returns_to_baseline() {
    // Wave A W49 / ADR-393 / A-03: FileStream open+close product exact (drop print(h)).
    let path = tmp_path("ok.bin");
    fs::write(&path, b"hi").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_stream_live_count();
    let out = run(&format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  print("OPENED")
  __syx_file_stream_close(h)
  print("CLOSED")
}}
"#
    ));
    assert_eq!(out, "OPENED\nCLOSED", "open+close must exact: {out}");
    assert_eq!(
        file_stream_live_count(),
        baseline,
        "open+close must restore file_stream_live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn ec_m2_3_file_stream_leak_open_without_close_increments() {
    let path = tmp_path("leak.bin");
    fs::write(&path, b"hi").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_stream_live_count();
    let out = run(&format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  print("LEAKED")
  print(h)
}}
"#
    ));
    assert!(out.contains("LEAKED"), "{out}");
    let after = file_stream_live_count();
    assert_eq!(after, baseline + 1, "baseline={baseline} after={after}");
    let id: i64 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("handle id");
    assert!(file_stream_force_close(id), "force_close {id}");
    assert_eq!(file_stream_live_count(), baseline);
    let _ = fs::remove_file(&path);
}
