//! EC-M2-3 FileHandle live-count leak first nail.
// was: ec_m2_3_file_leak

use std::fs;
use std::path::PathBuf;
use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{file_handle_force_close, file_handle_live_count};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!("{fh}\n{body}")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_ec_m2_3_file_leak_{name}"))
}

#[test]
fn ec_m2_3_file_leak_facade_source_has_open_close_throws() {
    let src = include_str!("../../../../packages/io/file_handle.sy");
    assert!(
        src.contains("open(path: String, mode: String) -> Int throws")
            || src.contains("-> Int throws"),
        "open must throws: {src}"
    );
    assert!(
        src.contains("func close(handle: Int) throws") || src.contains("close(handle: Int) throws"),
        "close must throws"
    );
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("pub fn file_handle_live_count"),
        "file_handle_live_count must be exported for nails"
    );
}

#[test]
fn ec_m2_3_file_leak_open_close_returns_to_baseline() {
    // Wave A W48 / ADR-392 / A-03: FileHandle open+close product exact (drop print(h)).
    let path = tmp_path("ok.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let h = FileHandle.open(path: "{path_s}", mode: "w")
    print("OPENED")
    FileHandle.close(handle: h)
    print("CLOSED")
  }} catch (err) {{
    print("caught")
  }}
}}
"#
    )));
    assert_eq!(out, "OPENED\nCLOSED", "open+close must exact: {out}");
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "open+close must return live count to baseline"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn ec_m2_3_file_leak_open_without_close_increments() {
    let path = tmp_path("leak.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let h = FileHandle.open(path: "{path_s}", mode: "w")
    print("LEAKED")
    print(h)
  }} catch (err) {{
    print("caught")
    print(err.message)
  }}
}}
"#
    )));
    assert!(out.contains("LEAKED"), "open without close must succeed: {out}");
    assert!(!out.contains("caught"), "open must not throw: {out}");

    let after = file_handle_live_count();
    assert_eq!(
        after,
        baseline + 1,
        "open without close must leave live_count == baseline + 1 (baseline={baseline}, after={after})"
    );

    // Nail cleanup: force-close the leaked id so later tests stay isolated (--test-threads=1).
    let id: i32 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("printed handle id");
    assert!(
        file_handle_force_close(id),
        "force_close must drop leaked handle {id}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "cleanup must restore baseline"
    );
    let _ = fs::remove_file(&path);
}
