//! FileHandle open/close/read/write/seek/readAllBytes throw SystemError.
// was: m2_error_io_file

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W100 / ADR-444: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!("{fh}\n{body}")
}

#[test]
fn m2_error_io_open_missing_path_throws() {
    // Wave A W23 / ADR-367 / A-03: missing-path open product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let h = FileHandle.open(path: "/tmp/syx_m2_w3_missing_no_such_file_xyz.bin", mode: "r")
    print("UNEXPECTED_OK")
    print(h)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "missing open must exact-catch: {out}");
}

#[test]
fn m2_error_io_open_missing_path_display_throws() {
    // Wave A W135 / ADR-479 / A-03: missing-path open Display product exact.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = FileHandle.open(path: "/tmp/syx_m2_w3_missing_no_such_file_xyz.bin", mode: "r")
}
"#,
    ));
    assert_eq!(
        err, "[R007] file open failed",
        "missing open Display must exact: {err}"
    );
}

#[test]
fn m2_error_io_close_invalid_handle_throws() {
    // Facade wraps host Err as SystemError; host Display is the stable R003 floor.
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_close(999999)
}
"#,
    );
    // Wave A W101 / ADR-445 / A-03: FileHandle close invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Invalid file handle 999999",
        "close invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_io_read_invalid_handle_throws() {
    // Facade wraps host Err as SystemError; host Display is the stable R003 floor.
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_read_chunk(999999, 16)
}
"#,
    );
    // Wave A W100 / ADR-444 / A-03: FileHandle read invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Invalid file handle 999999",
        "read invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_io_write_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_write_chunk(999999, [1u8, 2u8])
}
"#,
    );
    // Wave A W100 / ADR-444 / A-03: FileHandle write invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Invalid file handle 999999",
        "write invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_io_seek_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_seek(999999, 0)
}
"#,
    );
    // Wave A W101 / ADR-445 / A-03: FileHandle seek invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Invalid file handle 999999",
        "seek invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_io_read_all_bytes_invalid_handle_throws() {
    // Facade wraps host Err as SystemError; host Display is the stable R003 floor.
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_read_all_bytes(999999)
}
"#,
    );
    // Wave A W102 / ADR-446 / A-03: FileHandle readAllBytes invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Invalid file handle 999999",
        "readAllBytes invalid-handle must exact: {err}"
    );
}
