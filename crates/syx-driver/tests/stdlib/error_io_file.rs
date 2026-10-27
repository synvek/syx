//! FileHandle open/close/read/write/seek/readAllBytes throw IOError.
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
fn m2_error_io_facade_source_has_ioerror_and_throws() {
    let src = include_str!("../../../../packages/io/file_handle.sy");
    assert!(src.contains("IOError"), "IOError type required");
    assert!(
        src.contains("-> Int throws") || src.contains("open(path: String, mode: String) -> Int throws"),
        "open must throws: {src}"
    );
    assert!(
        src.contains("func close(handle: Int) throws") || src.contains("close(handle: Int) throws"),
        "close must throws"
    );
    assert!(
        src.contains("-> Array<UInt8> throws")
            || src.contains("read(handle: Int, maxBytes: Int) -> Array<UInt8> throws"),
        "read must throws: {src}"
    );
    assert!(
        src.contains("write(handle: Int, bytes: Array<UInt8>) -> Int throws")
            || src.contains("bytes: Array<UInt8>) -> Int throws"),
        "write must throws: {src}"
    );
    assert!(
        src.contains("func seek(handle: Int, offset: Int) throws")
            || src.contains("seek(handle: Int, offset: Int) throws"),
        "seek must throws: {src}"
    );
    assert!(
        src.contains("readAllBytes(handle: Int) -> Array<UInt8> throws")
            || src.contains("func readAllBytes(handle: Int) -> Array<UInt8> throws"),
        "readAllBytes must throws: {src}"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    // ADR-149/152/153: read/write/seek/readAllBytes must not CallHost-bypass the facade.
    assert!(
        reg.contains("method: \"read\"") && reg.contains("method: \"write\""),
        "FileHandle read/write TypeMethod entries required"
    );
    // After open/close (host: None), the next FileHandle read/write/seek/readAllBytes blocks must also be host: None.
    let fh_idx = reg
        .find("type_name: \"FileHandle\"")
        .expect("FileHandle TypeMethod block");
    let slice = &reg[fh_idx..];
    assert!(
        slice.contains("method: \"read\"")
            && slice
                .split("method: \"read\"")
                .nth(1)
                .is_some_and(|s| s.contains("host: None")),
        "FileHandle.read must host: None: {slice}"
    );
    assert!(
        slice.contains("method: \"write\"")
            && slice
                .split("method: \"write\"")
                .nth(1)
                .is_some_and(|s| s.contains("host: None")),
        "FileHandle.write must host: None"
    );
    assert!(
        slice.contains("method: \"seek\"")
            && slice
                .split("method: \"seek\"")
                .nth(1)
                .is_some_and(|s| s.contains("host: None")),
        "FileHandle.seek must host: None"
    );
    assert!(
        slice.contains("method: \"readAllBytes\"")
            && slice
                .split("method: \"readAllBytes\"")
                .nth(1)
                .is_some_and(|s| s.contains("host: None")),
        "FileHandle.readAllBytes must host: None"
    );
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    let seek_fn = hosts
        .split("pub fn host_file_seek")
        .nth(1)
        .expect("host_file_seek");
    let seek_body = seek_fn.split("pub fn ").next().unwrap_or(seek_fn);
    assert!(
        seek_body.contains("with_code(\"R003\")")
            && !seek_body.contains("Value::Bool(ok)"),
        "host_file_seek must Err on I/O fail (no Bool(ok) sentinel): {seek_body}"
    );
}

#[test]
fn m2_error_io_open_missing_path_throws() {
    // Wave A W23 / ADR-367 / A-03: missing-path open product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
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
    // Facade wraps host Err as IOError; host Display is the stable R003 floor.
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
    // Facade wraps host Err as IOError; host Display is the stable R003 floor.
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
    // Facade wraps host Err as IOError; host Display is the stable R003 floor.
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
