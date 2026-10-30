//! 02b FileHandle Reader/Writer/Seekable + RAII/`with`.
//! native/wasm behavioral nails for io.file.raii.
//! FileWithOpenFail HostFn llvm=real (FileWithOpen deferred).
//! FileWithOpen HostFn llvm=real (MakeClosure fn_index).
// was: m2_s3_02b_filehandle_raii_with

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};
use syx_vm::{file_handle_force_close, file_handle_live_count};

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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w248_file_with_open_llvm_{}",
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
        Ok(r) => r.printed.join("\n"),
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let protocols = [
        include_str!("../../../../packages/core/protocols/reader.sy"),
        include_str!("../../../../packages/core/protocols/writer.sy"),
        include_str!("../../../../packages/core/protocols/seekable.sy"),
        include_str!("../../../../packages/core/protocols/closeable.sy"),
    ]
    .join("\n");
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!("{protocols}\n{fh}\n{body}")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_m2_s3_02b_raii_{name}"))
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w202_{}_{}",
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

fn run_native_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    match run_native_mir_file_printed(&path, &opts()) {
        Ok((code, lines)) if code == 0 => lines_joined(&lines),
        Ok((_, lines)) if !lines.is_empty() => lines_joined(&lines),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    }
}

fn run_wasm_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    match run_wasm_source_printed(&path, "main", &opts()) {
        Ok((code, lines)) if code == 0 => lines_joined(&lines),
        Ok((_, lines)) if !lines.is_empty() => lines_joined(&lines),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    }
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn success_src(path_s: &str) -> String {
    facade_src(&format!(
        r#"
func main() {{
  try {{
    let n = FileHandle.withOpen(path: "{path_s}", mode: "w", body: {{ (h: Int) -> Int in
      return __syx_file_write_chunk(h, [])
    }})
    print("OK")
  }} catch (err) {{
    print("caught")
  }}
}}
"#
    ))
}

fn throw_src(path_s: &str) -> String {
    facade_src(&format!(
        r#"
func main() {{
  try {{
    let n = FileHandle.withOpen(path: "{path_s}", mode: "w", body: {{ (h: Int) -> Int in
      return 0 - 1
    }})
    print("UNEXPECTED")
    print(n)
  }} catch (err) {{
    print("CAUGHT")
  }}
}}
"#
    ))
}

fn inject_src(path_s: &str) -> String {
    facade_src(&format!(
        r#"
func main() {{
  print(__syx_file_with_open_fail("{path_s}", "w"))
}}
"#
    ))
}

#[test]
fn m2_s3_02b_protocol_files_static() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../packages/core/protocols/");
    for name in ["reader.sy", "writer.sy", "seekable.sy", "closeable.sy"] {
        let path = format!("{root}{name}");
        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        assert!(
            src.contains("public protocol"),
            "{name} must declare a protocol"
        );
    }
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    assert!(
        fh.contains("class FileResource")
            && (fh.contains("Reader/Writer/Seekable/Closeable")
                || fh.contains("implements Reader")),
        "FileResource + IO protocol markers required: {fh}"
    );
    assert!(
        fh.contains("func withOpen(path: String, mode: String, body: Object)"),
        "withOpen must exist: {fh}"
    );
    assert!(
        fh.contains("openResource(path: String, mode: String)"),
        "openResource must exist"
    );
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("host_file_close_quiet")
            && hosts.contains("host_file_with_open")
            && hosts.contains("host_file_with_open_fail"),
        "quiet close + with_open + with_open_fail hosts required"
    );
}

#[test]
fn m2_s3_02b_with_open_success_live_count() {
    // Wave A W49 / ADR-393 / A-03: withOpen success product exact (drop print(n)).
    let path = tmp_path("ok.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let out = run(&success_src(&path_s));
    assert_eq!(out, "OK", "withOpen success must exact: {out}");
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "withOpen success must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_with_open_success_live_count_native() {
    let path = tmp_path("ok_native.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let lines = run_native_src("raii_ok", &success_src(&path_s));
    assert_eq!(
        lines_joined(&lines),
        "OK",
        "native withOpen success must exact: {lines:?}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "native withOpen success must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_with_open_success_live_count_wasm() {
    let path = tmp_path("ok_wasm.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let lines = run_wasm_src("raii_ok", &success_src(&path_s));
    assert_eq!(
        lines_joined(&lines),
        "OK",
        "wasm withOpen success must exact: {lines:?}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "wasm withOpen success must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_with_open_throw_live_count() {
    // Wave A W48 / ADR-392 / A-03: withOpen body-fail product exact.
    let path = tmp_path("fail.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let out = run(&throw_src(&path_s));
    assert_eq!(out, "CAUGHT", "body failure must exact: {out}");
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "withOpen throw path must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_with_open_throw_live_count_native() {
    let path = tmp_path("fail_native.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let lines = run_native_src("raii_throw", &throw_src(&path_s));
    assert_eq!(
        lines_joined(&lines),
        "CAUGHT",
        "native withOpen throw must exact: {lines:?}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "native withOpen throw must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_with_open_throw_live_count_wasm() {
    let path = tmp_path("fail_wasm.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let lines = run_wasm_src("raii_throw", &throw_src(&path_s));
    assert_eq!(
        lines_joined(&lines),
        "CAUGHT",
        "wasm withOpen throw must exact: {lines:?}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "wasm withOpen throw must restore live_count"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_host_inject_fail_live_count() {
    let path = tmp_path("inject.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let err = run_err(&inject_src(&path_s));
    // Wave A W91 / ADR-435 / A-03: withOpen host inject fail product exact.
    assert_eq!(
        err, "[R001] withOpen body failed (host inject)",
        "host inject must exact: {err}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "host inject fail must not leak (baseline={baseline})"
    );
    // Defensive cleanup if any leftover.
    while file_handle_live_count() > baseline {
        let _ = file_handle_force_close(file_handle_live_count() as i32);
        break;
    }
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_host_inject_fail_live_count_native() {
    let path = tmp_path("inject_native.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let err = run_native_err("raii_inject", &inject_src(&path_s));
    assert_eq!(
        err, "[R001] withOpen body failed (host inject)",
        "native host inject must exact: {err}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "native host inject fail must not leak"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_host_inject_fail_live_count_wasm() {
    let path = tmp_path("inject_wasm.bin");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let baseline = file_handle_live_count();

    let err = run_wasm_err("raii_inject", &inject_src(&path_s));
    assert_eq!(
        err, "[R001] withOpen body failed (host inject)",
        "wasm host inject must exact: {err}"
    );
    assert_eq!(
        file_handle_live_count(),
        baseline,
        "wasm host inject fail must not leak"
    );
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_product_raii_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.raii");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "io.file.raii native/wasm must be real: {block}"
    );
    assert!(
        block.contains("ADR-546"),
        "notes must cite ADR-546: {block}"
    );
    assert!(
        block.contains("m2_s3_02b_with_open_success_live_count_native"),
        "tests must list native nail: {block}"
    );
}

#[test]
fn m2_s3_02b_docs_and_capability() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-234")
            && usage.contains("withOpen")
            && (usage.contains("Reader") || usage.contains("Closeable")),
        "UsageGuide must document withOpen / IO protocols + ADR-234"
    );
    assert!(
        usage.contains("M2-S3-03") || usage.contains("mmap") || usage.contains("EC-M2-3"),
        "UsageGuide must fence stream/mmap/EC-M2-3"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.raii");
    assert!(
        block.contains("status = \"implemented\""),
        "io.file.raii must be implemented: {block}"
    );
    assert!(
        block.contains("ADR-234") || block.contains("M2-S3-02b") || block.contains("ADR-546"),
        "notes must cite ADR-234 or ADR-546: {block}"
    );
    assert!(
        block.contains("m2_s3_02b_filehandle_raii_with"),
        "tests must list nail: {block}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-234") || inv.contains("ADR-234"),
        "inventory must note ADR-234"
    );
    let debt = inv
        .split("id = \"io.file_handle\"")
        .nth(1)
        .unwrap_or("")
        .split("[[debt]]")
        .next()
        .unwrap_or("");
    assert!(
        !debt.contains("instance RAII remaining"),
        "debt must drop instance RAII remaining: {debt}"
    );
}


#[cfg(unix)]
#[test]
fn m2_s3_02b_with_open_fail_native_llvm() {
    // Bare inject host + single-layer try (avoid nested facade LLVM IR pit).
    let path = tmp_path("with_open_fail_llvm");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let src = facade_src(&format!(
        r#"
func main() {{
  try {{
    print(__syx_file_with_open_fail("{path_s}", "w"))
  }} catch (err) {{
    print(err.code)
    print(err.message)
  }}
}}
"#
    ));
    let (code, stdout) = run_native_strict(&src);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "R001\nwithOpen body failed (host inject)",
        "native llvm FileWithOpenFail must exact: {out}"
    );
    let _ = fs::remove_file(&path);
}

#[cfg(unix)]
#[test]
fn m2_s3_02b_with_open_native_llvm() {
    // Wave A W248 / ADR-592: bare FileWithOpen + Closure body under SYX_NATIVE_STRICT.
    let path = tmp_path("with_open_ok_llvm");
    let _ = fs::remove_file(&path);
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  try {{
    let n = __syx_file_with_open("{path_s}", "w", {{ (h: Int) -> Int in
      return __syx_file_write_chunk(h, [])
    }})
    print("OK")
  }} catch (err) {{
    print("caught")
    print(err.code)
    print(err.message)
  }}
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
    assert_eq!(out, "OK", "native llvm FileWithOpen must exact: {out}");
    let _ = fs::remove_file(&path);
}

#[test]
fn m2_s3_02b_file_with_open_fail_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "FileWithOpenFail");
    assert!(
        block.contains("llvm = \"real\""),
        "FileWithOpenFail must declare llvm=real (ADR-591): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "FileWithOpenFail must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "FileWithOpenFail must declare wasm=real (ADR-613): {block}"
    );
    assert!(
        block.contains("ADR-613") && block.contains("ADR-591"),
        "FileWithOpenFail notes must cite ADR-613 + ADR-591: {block}"
    );
}

#[test]
fn m2_s3_02b_file_with_open_host_llvm_real() {
    // Wave A W248 / ADR-592 llvm + Wave A W269 / ADR-613 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let open = host_block(toml, "FileWithOpen");
    assert!(
        open.contains("llvm = \"real\""),
        "FileWithOpen must declare llvm=real (ADR-592): {open}"
    );
    assert!(
        open.contains("status = \"partial\""),
        "FileWithOpen must keep status=partial: {open}"
    );
    assert!(
        open.contains("wasm = \"real\""),
        "FileWithOpen must declare wasm=real (ADR-613): {open}"
    );
    assert!(
        open.contains("ADR-613") && open.contains("ADR-592"),
        "FileWithOpen notes must cite ADR-613 + ADR-592: {open}"
    );
}
