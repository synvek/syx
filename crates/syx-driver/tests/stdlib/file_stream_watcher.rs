//! 03 FileStream + FileWatcher lifecycle nail.
//! native/wasm behavioral nails for io.file.stream.
//! native/wasm behavioral nails for io.file.watch.
//! FileStream HostFn llvm=real (native AOT nail).
//! FileWatcher + FileCloseQuiet HostFn llvm=real (native AOT nails).
//! FileStream HostFn wasm=real (text 收官 + Fmt/print wasm8).
//! FileWatcher + FileCloseQuiet HostFn wasm=real (File 残余 wasm 收官).
// was: m2_s3_03_file_stream_watcher

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &opts()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W102 / ADR-446: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    let stream = include_str!("../../../../packages/io/file_stream.sy");
    let watcher = include_str!("../../../../packages/io/file_watcher.sy");
    format!("{fh}\n{stream}\n{watcher}\n{body}")
}

fn stream_facade_src(body: &str) -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    let stream = include_str!("../../../../packages/io/file_stream.sy");
    format!("{fh}\n{stream}\n{body}")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_m2_s3_03_{name}"))
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w204_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_native_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn facade_open_close_src(path_s: &str) -> String {
    stream_facade_src(&format!(
        r#"
func main() {{
  try {{
    let h = FileStream.open(path: "{path_s}", mode: "r")
    let c = FileStream.readChunk(handle: h, maxBytes: 64)
    print(c.count)
    FileStream.close(handle: h)
    print("FACADE_OK")
  }} catch (err) {{
    print("FACADE_ERR")
  }}
}}
"#
    ))
}

fn chunk_close_fail_src(path_s: &str) -> String {
    stream_facade_src(&format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  var nonempty = 0
  var total = 0
  var chunk = __syx_file_stream_read_chunk(h, 1024)
  while chunk.count > 0 {{
    nonempty = nonempty + 1
    total = total + chunk.count
    chunk = __syx_file_stream_read_chunk(h, 1024)
  }}
  __syx_file_stream_close(h)
  print("OK")
  print(nonempty)
  print(total)
  try {{
    let again = FileStream.readChunk(handle: h, maxBytes: 8)
    print("UNEXPECTED")
    print(again.count)
  }} catch (err) {{
    print("CLOSED_FAIL")
  }}
}}
"#
    ))
}

fn watcher_poll_close_src(path_s: &str) -> String {
    facade_src(&format!(
        r#"
func main() {{
  try {{
    let w = FileWatcher.open(path: "{path_s}")
    let ev = FileWatcher.poll(handle: w)
    print(ev.count)
    FileWatcher.close(handle: w)
    try {{
      let again = FileWatcher.poll(handle: w)
      print("UNEXPECTED")
      print(again.count)
    }} catch (err) {{
      print("CLOSED_FAIL")
    }}
  }} catch (err) {{
    print("ERR")
    print(err.message)
  }}
}}
"#
    ))
}

const BODY_LEN: usize = 4096;

#[test]
fn m2_s3_03_file_stream_chunk_and_close_fail() {
    let path = tmp_path("stream.bin");
    let body: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .cycle()
        .take(BODY_LEN)
        .collect();
    fs::write(&path, body.as_bytes()).expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");

    // Facade open/readChunk/close; bare-host path proven equivalent for session table.
    // Use bare open+read loop (facade throws wrapping is Hashable/try fragile in embed)
    // then facade readChunk after close to prove product close-then-fail.
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  var nonempty = 0
  var total = 0
  var chunk = __syx_file_stream_read_chunk(h, 1024)
  while chunk.count > 0 {{
    nonempty = nonempty + 1
    total = total + chunk.count
    chunk = __syx_file_stream_read_chunk(h, 1024)
  }}
  __syx_file_stream_close(h)
  print("OK")
  print(nonempty)
  print(total)
  try {{
    let again = FileStream.readChunk(handle: h, maxBytes: 8)
    print("UNEXPECTED")
    print(again.count)
  }} catch (err) {{
    print("CLOSED_FAIL")
  }}
}}
"#
    )));

    let _ = fs::remove_file(&path);
    // Wave A W71 / ADR-415 / A-03: FileStream chunk + close-fail product exact.
    assert_eq!(
        out, "OK\n4\n4096\nCLOSED_FAIL",
        "FileStream chunk read + close-fail must exact: {out}"
    );
}

#[test]
fn m2_s3_03_file_stream_facade_open_close() {
    // Wave A W50 / ADR-394 / A-03: FileStream facade open/read/close product exact.
    let path = tmp_path("facade.bin");
    fs::write(&path, b"hello-stream").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let h = FileStream.open(path: "{path_s}", mode: "r")
    let c = FileStream.readChunk(handle: h, maxBytes: 64)
    print(c.count)
    FileStream.close(handle: h)
    print("FACADE_OK")
  }} catch (err) {{
    print("FACADE_ERR")
  }}
}}
"#
    )));
    let _ = fs::remove_file(&path);
    assert_eq!(out, "12\nFACADE_OK", "FileStream facade must exact: {out}");
}

#[test]
fn m2_s3_03_file_watcher_poll_and_close_fail() {
    let path = tmp_path("watch_target");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");

    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let w = FileWatcher.open(path: "{path_s}")
    let ev = FileWatcher.poll(handle: w)
    print(ev.count)
    FileWatcher.close(handle: w)
    try {{
      let again = FileWatcher.poll(handle: w)
      print("UNEXPECTED")
      print(again.count)
    }} catch (err) {{
      print("CLOSED_FAIL")
    }}
  }} catch (err) {{
    print("ERR")
    print(err.message)
  }}
}}
"#
    )));

    let _ = fs::remove_file(&path);
    // Wave A W72 / ADR-416 / A-03: FileWatcher poll + close-fail product exact.
    assert_eq!(
        out, "1\nCLOSED_FAIL",
        "FileWatcher poll + close-fail must exact: {out}"
    );
}

#[test]
fn m2_s3_03_docs_and_capability() {
    let stream = include_str!("../../../../packages/io/file_stream.sy");
    assert!(
        stream.contains("func open")
            && stream.contains("readChunk")
            && stream.contains("__syx_file_stream_open"),
        "FileStream facade required"
    );
    let watcher = include_str!("../../../../packages/io/file_watcher.sy");
    assert!(
        watcher.contains("FileWatcher")
            && watcher.contains("poll")
            && watcher.contains("__syx_file_watcher_open"),
        "FileWatcher facade required"
    );

    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("FILE_STREAM_SESSIONS")
            && hosts.contains("FILE_WATCH_SESSIONS")
            && hosts.contains("FileStream invalid handle")
            && hosts.contains("FileWatcher invalid handle"),
        "session tables + close-then-fail required"
    );

    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("FileStream")
            && guide.contains("FileWatcher")
            && guide.contains("ADR-240")
            && guide.contains("关闭后再操作必须失败"),
        "UsageGuide must document close-then-fail"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let stream_cap = product_block(caps, "io.file.stream");
    assert!(
        stream_cap.contains("status = \"implemented\""),
        "io.file.stream must be implemented: {stream_cap}"
    );
    let watch_cap = product_block(caps, "io.file.watch");
    assert!(
        watch_cap.contains("status = \"implemented\""),
        "io.file.watch must be implemented (ADR-555): {watch_cap}"
    );
    assert!(
        watch_cap.contains("mir = \"real\""),
        "io.file.watch mir must be real (ADR-555): {watch_cap}"
    );
    assert!(
        watch_cap.contains("m2_s3_03_file_stream_watcher")
            || watch_cap.contains("m2_s3_03_file_watcher_poll_close_fail_native")
            || watch_cap.contains("m2_s3_03_file_watcher_poll_close_fail_mir"),
        "tests must list nail: {watch_cap}"
    );
    assert!(
        stream_cap.contains("m2_s3_03_file_stream_watcher"),
        "stream tests must list nail: {stream_cap}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-240"),
        "inventory must note signed-ADR-240"
    );

    let io_mod = include_str!("../../../../packages/io/mod.sy");
    assert!(
        io_mod.contains("≠ FileWatcher") || io_mod.contains("lifecycle"),
        "File.watch must be documented as ≠ lifecycle"
    );
}


#[test]
fn m2_error_file_stream_open_missing_path_display_throws() {
    // Wave A W136 / ADR-480 / A-03: missing-path open Display product exact.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = FileStream.open(path: "/tmp/syx_m2_w136_missing_no_such_file_xyz.bin", mode: "r")
}
"#,
    ));
    assert_eq!(
        err, "[R007] FileStream open failed",
        "missing open Display must exact: {err}"
    );
}

#[test]
fn m2_error_file_stream_read_after_close_display_throws() {
    // Wave A W136 / ADR-480 / A-03: read-after-close Display product exact.
    let path = tmp_path("display_read_after_close.bin");
    fs::write(&path, b"x").expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let err = run_err(&facade_src(&format!(
        r#"
func main() {{
  let h = FileStream.open(path: "{path_s}", mode: "r")
  let c1 = FileStream.readChunk(handle: h, maxBytes: 8)
  print(c1.count)
  FileStream.close(handle: h)
  let _ = FileStream.readChunk(handle: h, maxBytes: 8)
}}
"#
    )));
    let _ = fs::remove_file(&path);
    assert_eq!(
        err, "[R007] FileStream read failed",
        "read-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_file_stream_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_stream_close(999999)
}
"#,
    );
    // Wave A W102 / ADR-446 / A-03: FileStream invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] FileStream invalid handle 999999",
        "FileStream invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_file_stream_read_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_stream_read_chunk(999999, 8)
}
"#,
    );
    // Wave A W105 / ADR-449 / A-03: FileStream readChunk invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] FileStream invalid handle 999999",
        "FileStream readChunk invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_file_watcher_close_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_watcher_close(999999)
}
"#,
    );
    // Wave A W103 / ADR-447 / A-03: FileWatcher close invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] FileWatcher invalid handle 999999",
        "FileWatcher close invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_file_watcher_poll_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_file_watcher_poll(999999)
}
"#,
    );
    // Wave A W103 / ADR-447 / A-03: FileWatcher poll invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] FileWatcher invalid handle 999999",
        "FileWatcher poll invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_file_watcher_poll_after_close_display_throws() {
    // Wave A W138 / ADR-482 / A-03: FileWatcher poll-after-close Display product exact.
    let path = tmp_path("display_poll_after_close");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let err = run_err(&facade_src(&format!(
        r#"
func main() {{
  let w = FileWatcher.open(path: "{path_s}")
  FileWatcher.close(handle: w)
  let _ = FileWatcher.poll(handle: w)
}}
"#
    )));
    let _ = fs::remove_file(&path);
    assert_eq!(
        err, "[R007] FileWatcher poll failed",
        "poll-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_file_watcher_close_after_close_display_throws() {
    // Wave A W138 / ADR-482 / A-03: FileWatcher close-after-close Display product exact.
    let path = tmp_path("display_close_after_close");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let err = run_err(&facade_src(&format!(
        r#"
func main() {{
  let w = FileWatcher.open(path: "{path_s}")
  FileWatcher.close(handle: w)
  FileWatcher.close(handle: w)
}}
"#
    )));
    let _ = fs::remove_file(&path);
    assert_eq!(
        err, "[R007] FileWatcher close failed",
        "close-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_s3_03_file_stream_facade_native() {
    let path = tmp_path("facade_native.bin");
    fs::write(&path, b"hello-stream").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_native_src("facade_n", &facade_open_close_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "12\nFACADE_OK",
        "native FileStream facade must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_stream_facade_wasm() {
    let path = tmp_path("facade_wasm.bin");
    fs::write(&path, b"hello-stream").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_wasm_src("facade_w", &facade_open_close_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "12\nFACADE_OK",
        "wasm FileStream facade must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_stream_chunk_close_fail_native() {
    let path = tmp_path("chunk_native.bin");
    let body: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .cycle()
        .take(BODY_LEN)
        .collect();
    fs::write(&path, body.as_bytes()).expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_native_src("chunk_n", &chunk_close_fail_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "OK\n4\n4096\nCLOSED_FAIL",
        "native FileStream chunk+close-fail must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_stream_chunk_close_fail_wasm() {
    let path = tmp_path("chunk_wasm.bin");
    let body: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .cycle()
        .take(BODY_LEN)
        .collect();
    fs::write(&path, body.as_bytes()).expect("write temp");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_wasm_src("chunk_w", &chunk_close_fail_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "OK\n4\n4096\nCLOSED_FAIL",
        "wasm FileStream chunk+close-fail must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_watcher_poll_close_fail_mir() {
    let path = tmp_path("watch_mir.bin");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_native_src("watch_mir", &watcher_poll_close_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "1\nCLOSED_FAIL",
        "mir-run FileWatcher poll+close-fail must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_watcher_poll_close_fail_native() {
    let path = tmp_path("watch_native.bin");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_native_src("watch_n", &watcher_poll_close_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "1\nCLOSED_FAIL",
        "native FileWatcher poll+close-fail must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_file_watcher_poll_close_fail_wasm() {
    let path = tmp_path("watch_wasm.bin");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let lines = run_wasm_src("watch_w", &watcher_poll_close_src(&path_s));
    let _ = fs::remove_file(&path);
    assert_eq!(
        lines_joined(&lines),
        "1\nCLOSED_FAIL",
        "wasm FileWatcher poll+close-fail must exact: {lines:?}"
    );
}

#[test]
fn m2_s3_03_product_watch_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.watch");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "io.file.watch native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "io.file.watch mir must be real: {block}"
    );
    assert!(
        block.contains("ADR-555") || block.contains("ADR-550"),
        "notes must cite ADR-555 or ADR-550: {block}"
    );
    assert!(
        block.contains("m2_s3_03_file_watcher_poll_close_fail_native"),
        "tests must list native nail: {block}"
    );
}

#[test]
fn m2_s3_03_product_watch_mir_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.watch");
    assert!(
        block.contains("status = \"implemented\""),
        "io.file.watch status must be implemented: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "io.file.watch mir must be real: {block}"
    );
    assert!(
        block.contains("ADR-555"),
        "notes must cite ADR-555: {block}"
    );
    assert!(
        block.contains("m2_s3_03_file_watcher_poll_close_fail_mir"),
        "tests must list mir nail: {block}"
    );
}

#[test]
fn m2_s3_03_product_stream_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.stream");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "io.file.stream native/wasm must be real: {block}"
    );
    assert!(
        block.contains("ADR-548"),
        "notes must cite ADR-548: {block}"
    );
    assert!(
        block.contains("m2_s3_03_file_stream_facade_native"),
        "tests must list native nail: {block}"
    );
    let watch = product_block(caps, "io.file.watch");
    assert!(
        watch.contains("native = \"real\"") && watch.contains("wasm = \"real\""),
        "io.file.watch native/wasm must be real: {watch}"
    );
    assert!(
        watch.contains("mir = \"real\"") && watch.contains("status = \"implemented\""),
        "io.file.watch mir/status must be real/implemented: {watch}"
    );
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w241_file_stream_llvm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn m2_s3_03_file_stream_read_chunk_native_llvm() {
    let dir = std::env::temp_dir().join(format!(
        "syx_w241_fs_data_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("w241.txt");
    fs::write(&path, b"hi").unwrap();
    let path_s = path.display().to_string().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  let c = __syx_file_stream_read_chunk(h, 64)
  print(c.count)
  __syx_file_stream_close(h)
}}
"#
    );
    let (code, stdout) = run_native_strict(&src);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "2", "native llvm FileStream read_chunk hi length must exact: {out}");
}

#[test]
fn m2_s3_03_file_stream_read_chunk_wasm() {
    let dir = std::env::temp_dir().join(format!(
        "syx_w267_fs_data_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("w267.txt");
    fs::write(&path, b"hi").unwrap();
    let path_s = path.display().to_string().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let h = __syx_file_stream_open("{path_s}", "r")
  let c = __syx_file_stream_read_chunk(h, 64)
  print(c.count)
  __syx_file_stream_close(h)
}}
"#
    );
    let out = lines_joined(&run_wasm_src("fs_chunk_host_wasm", &src));
    assert_eq!(out, "2", "wasm FileStream read_chunk hi length must exact: {out}");
}

#[test]
fn m2_s3_03_file_stream_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileStreamOpen", "FileStreamReadChunk", "FileStreamClose"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-585): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-611): {block}"
        );
        assert!(
            block.contains("ADR-611") && block.contains("ADR-585"),
            "{name} notes must cite ADR-611 + ADR-585: {block}"
        );
    }
}

#[test]
fn m2_s3_03_file_watcher_native_llvm() {
    let path = tmp_path("watch_llvm.bin");
    let _ = fs::remove_file(&path);
    fs::write(&path, b"x").expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    // Bare hosts + single try: facade nested try/catch breaks LLVM IR under SYX_NATIVE_STRICT.
    let src = format!(
        r#"
func main() {{
  let w = __syx_file_watcher_open("{path_s}")
  let ev = __syx_file_watcher_poll(w)
  print(ev.count)
  __syx_file_watcher_close(w)
  try {{
    let again = __syx_file_watcher_poll(w)
    print("UNEXPECTED")
    print(again.count)
  }} catch (err) {{
    print("CLOSED_FAIL")
  }}
}}
"#
    );
    let (code, stdout) = run_native_strict(&src);
    let _ = fs::remove_file(&path);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "1\nCLOSED_FAIL",
        "native llvm FileWatcher poll+close-fail must exact: {out}"
    );
}

#[test]
fn m2_s3_03_file_close_quiet_native_llvm() {
    let dir = std::env::temp_dir().join(format!(
        "syx_w244_close_quiet_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("w244.txt");
    let path_s = path.display().to_string().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let h = __syx_file_open("{path_s}", "w")
  __syx_file_close_quiet(h)
  print("OK")
}}
"#
    );
    let (code, stdout) = run_native_strict(&src);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm close_quiet exit {code}: {stdout}");
    assert_eq!(out, "OK", "native llvm FileCloseQuiet must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m2_s3_03_file_close_quiet_wasm() {
    // Wave A W269 / ADR-613 — FileCloseQuiet HostFn wasm=real
    let dir = std::env::temp_dir().join(format!(
        "syx_w269_close_quiet_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("w269.txt");
    let path_s = path.display().to_string().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let h = __syx_file_open("{path_s}", "w")
  __syx_file_close_quiet(h)
  print("OK")
}}
"#
    );
    let out = lines_joined(&run_wasm_src("close_quiet_wasm", &src));
    assert_eq!(out, "OK", "wasm FileCloseQuiet must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m2_s3_03_file_watcher_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileWatcherOpen", "FileWatcherPoll", "FileWatcherClose"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-588): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-613): {block}"
        );
        assert!(
            block.contains("ADR-613") && block.contains("ADR-588"),
            "{name} notes must cite ADR-613 + ADR-588: {block}"
        );
    }
}

#[test]
fn m2_s3_03_file_close_quiet_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "FileCloseQuiet");
    assert!(
        block.contains("llvm = \"real\""),
        "FileCloseQuiet must declare llvm=real (ADR-588): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "FileCloseQuiet must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "FileCloseQuiet must declare wasm=real (ADR-613): {block}"
    );
    assert!(
        block.contains("ADR-613") && block.contains("ADR-588"),
        "FileCloseQuiet notes must cite ADR-613 + ADR-588: {block}"
    );
}
