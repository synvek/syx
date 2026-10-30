//! 02b Child Stdio/Pipe streaming nail.
//! ChildWait + ChildStdout + ChildPipeReadChunk + ChildPipeClose HostFn llvm=real.
//! ChildStdin + ChildPipeWrite HostFn llvm=real.
//! ChildStderr HostFn llvm=real.
// was: m2_s6_02b_child_stdio_pipe

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w245_child_stdio_pipe_llvm_{}",
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
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r.printed.join("\n"),
        Err(e) => panic!("compile/run failed: {e:?}\n---src---\n{src}"),
    }
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W104 / ADR-448: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let proc = include_str!("../../../../packages/process/mod.sy");
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    format!("{proc}\n{pb}\n{body}")
}

const BODY_LEN: usize = 4096;

#[test]
fn m2_s6_02b_stdout_stream_chunks() {
    // Bare hosts for the read loop (facade throws wrapping is fragile in embed).
    // Produce >=4KB via python for portable chunking.
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let b = __syx_process_builder_create("/usr/bin/python3", ["-c", "import sys; sys.stdout.buffer.write(b'A'*{BODY_LEN})"])
  let h = __syx_process_builder_spawn(b)
  let p = __syx_child_stdout(h)
  var nonempty = 0
  var total = 0
  var chunk = __syx_child_pipe_read_chunk(p, 1024)
  while chunk.count > 0 {{
    nonempty = nonempty + 1
    total = total + chunk.count
    chunk = __syx_child_pipe_read_chunk(p, 1024)
  }}
  __syx_child_pipe_close(p)
  let code = __syx_child_wait(h)
  print("OK")
  print(nonempty)
  print(total)
  print(code)
}}
"#
    )));
    // Wave A W71 / ADR-415 / A-03: ChildPipe stdout stream chunk product exact.
    assert_eq!(
        out, "OK\n4\n4096\n0",
        "stdout stream chunks must exact: {out}"
    );
}

#[test]
fn m2_s6_02b_stdin_echo() {
    // Wave A W57 / ADR-401 / A-03: ChildPipe stdin echo product exact.
    let out = run(&facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/bin/cat", [])
  let h = __syx_process_builder_spawn(b)
  let sin = __syx_child_stdin(h)
  let sout = __syx_child_stdout(h)
  let n = __syx_child_pipe_write(sin, [72u8, 105u8, 10u8])
  __syx_child_pipe_close(sin)
  var total = 0
  var chunk = __syx_child_pipe_read_chunk(sout, 64)
  while chunk.count > 0 {
    total = total + chunk.count
    chunk = __syx_child_pipe_read_chunk(sout, 64)
  }
  __syx_child_pipe_close(sout)
  let code = __syx_child_wait(h)
  print("OK")
  print(n)
  print(total)
  print(code)
}
"#,
    ));
    assert_eq!(out, "OK\n3\n3\n0", "stdin echo must exact: {out}");
}

#[test]
fn m2_s6_02b_close_then_read_fails() {
    // Wave A W56 / ADR-400 / A-03: ChildPipe close-then-read product exact (drop print(e1)).
    let out = run(&facade_src(
        r#"
func main() {
  var p = -1
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    p = Child.stdout(handle: h)
    ChildPipe.close(pipe: p)
    let code = Child.wait(handle: h)
    print("CLOSED")
    print(code)
  } catch (e1) {
    print("SETUP_ERR")
  }
  try {
    let again = ChildPipe.readChunk(pipe: p, maxBytes: 8)
    print("UNEXPECTED")
    print(again.count)
  } catch (e2) {
    print("READ_FAIL")
  }
}
"#,
    ));
    assert_eq!(
        out, "CLOSED\n0\nREAD_FAIL",
        "close then read must exact: {out}"
    );
}

#[test]
fn m2_s6_02b_untaken_wait_no_hang() {
    // Wave A W57 / ADR-401 / A-03: ChildPipe untaken wait product exact (drop print(e1)).
    // Measured: large untaken stdout → SIGPIPE wait 141 (128+13), not 0.
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let b = ProcessBuilder.create(command: "/bin/sh", args: ["-c", "printf '%0{BODY_LEN}d' 0 | tr '0' 'B'"])
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("OK")
    print(code)
  }} catch (e1) {{
    print("ERR")
  }}
}}
"#
    )));
    assert_eq!(out, "OK\n141", "untaken wait must exact: {out}");
}

#[test]
fn m2_s6_02b_docs_and_capability() {
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    assert!(
        pb.contains("ChildPipe")
            && pb.contains("__syx_child_stdout")
            && pb.contains("__syx_child_pipe_read_chunk"),
        "ChildPipe facade required"
    );
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(
        hosts.contains("CHILD_PIPE_SESSIONS")
            && hosts.contains("host_child_pipe_read_chunk")
            && hosts.contains("ChildPipe invalid handle"),
        "pipe sessions + fail-after required"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ChildPipe")
            && guide.contains("ADR-243")
            && guide.contains("关闭后再读写必须失败"),
        "UsageGuide must document ChildPipe fail-after"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "os.process.stdio");
    // Honesty: wasm=unsupported keeps overall status=partial (≠ product implemented).
    assert!(
        block.contains("status = \"partial\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"unsupported\""),
        "os.process.stdio must stay partial/native=real/wasm=unsupported: {block}"
    );
    assert!(
        (block.contains("ADR-602") || block.contains("ADR-243")) && block.contains("W258"),
        "os.process.stdio notes must cite ADR-602: {block}"
    );
    assert!(
        block.contains("m2_s6_02b_child_stdio_pipe"),
        "tests must list nail"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s6_02b_child_stdio_pipe_tier = \"signed-ADR-243\""),
        "inventory must note signed-ADR-243"
    );
}


#[test]
fn m2_error_child_pipe_close_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_pipe_close(999999)
}
"#,
    );
    // Wave A W104 / ADR-448 / A-03: ChildPipe close invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ChildPipe invalid handle 999999",
        "ChildPipe close invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_close_invalid_handle_display_throws() {
    // Wave A W176 / ADR-520 / A-03 — ChildPipe.close invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  ChildPipe.close(pipe: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe close failed",
        "close invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_close_after_close_display_throws() {
    // Wave A W176 / ADR-520 / A-03 — ChildPipe close-after-close facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
  let h = ProcessBuilder.spawn(builder: b)
  let p = Child.stdout(handle: h)
  ChildPipe.close(pipe: p)
  ChildPipe.close(pipe: p)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe close failed",
        "close-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_read_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_pipe_read_chunk(999999, 8)
}
"#,
    );
    // Wave A W104 / ADR-448 / A-03: ChildPipe readChunk invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ChildPipe invalid handle 999999",
        "ChildPipe readChunk invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_read_invalid_handle_display_throws() {
    // Wave A W187 / ADR-531 / A-03 — ChildPipe.readChunk invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = ChildPipe.readChunk(pipe: 999999, maxBytes: 8)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe read failed",
        "ChildPipe.readChunk invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_read_after_close_display_throws() {
    // Wave A W187 / ADR-531 / A-03 — ChildPipe.readChunk after-close facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
  let h = ProcessBuilder.spawn(builder: b)
  let p = Child.stdout(handle: h)
  ChildPipe.close(pipe: p)
  let _ = ChildPipe.readChunk(pipe: p, maxBytes: 8)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe read failed",
        "ChildPipe.readChunk-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_write_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_pipe_write(999999, [1u8])
}
"#,
    );
    // Wave A W105 / ADR-449 / A-03: ChildPipe write invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ChildPipe invalid handle 999999",
        "ChildPipe write invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_write_invalid_handle_display_throws() {
    // Wave A W188 / ADR-532 / A-03 — ChildPipe.write invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = ChildPipe.write(pipe: 999999, bytes: [1u8])
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe write failed",
        "ChildPipe.write invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_pipe_write_after_close_display_throws() {
    // Wave A W188 / ADR-532 / A-03 — ChildPipe.write after-close facade Display (stdin).
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
  let h = ProcessBuilder.spawn(builder: b)
  let p = Child.stdin(handle: h)
  ChildPipe.close(pipe: p)
  let _ = ChildPipe.write(pipe: p, bytes: [1u8])
}
"#,
    ));
    assert_eq!(
        err, "[R007] ChildPipe write failed",
        "ChildPipe.write-after-close Display must exact: {err}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s6_02b_stdout_stream_native_llvm() {
    // Wave A W245 / ADR-589 — bare hosts (avoid facade nested try under SYX_NATIVE_STRICT).
    let src = facade_src(&format!(
        r#"
func main() {{
  let b = __syx_process_builder_create("/usr/bin/python3", ["-c", "import sys; sys.stdout.buffer.write(b'A'*{BODY_LEN})"])
  let h = __syx_process_builder_spawn(b)
  let p = __syx_child_stdout(h)
  var nonempty = 0
  var total = 0
  var chunk = __syx_child_pipe_read_chunk(p, 1024)
  while chunk.count > 0 {{
    nonempty = nonempty + 1
    total = total + chunk.count
    chunk = __syx_child_pipe_read_chunk(p, 1024)
  }}
  __syx_child_pipe_close(p)
  let code = __syx_child_wait(h)
  print("OK")
  print(nonempty)
  print(total)
  print(code)
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
        out, "OK\n4\n4096\n0",
        "native llvm Child stdout/pipe stream must exact: {out}"
    );
}

#[test]
fn m2_s6_02b_child_stdout_pipe_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "ChildWait",
        "ChildStdout",
        "ChildPipeReadChunk",
        "ChildPipeClose",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-589): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"unsupported\""),
            "{name} must keep wasm=unsupported: {block}"
        );
        assert!(
            block.contains("ADR-589"),
            "{name} notes must cite ADR-589: {block}"
        );
        if name == "ChildWait" {
            assert!(
                block.contains("ADR-242"),
                "ChildWait notes must cite ADR-242: {block}"
            );
        } else {
            assert!(
                block.contains("ADR-243"),
                "{name} notes must cite ADR-243: {block}"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn m2_s6_02b_stdin_echo_native_llvm() {
    // Wave A W246 / ADR-590 — bare hosts (stdin + PipeWrite).
    let src = facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/bin/cat", [])
  let h = __syx_process_builder_spawn(b)
  let sin = __syx_child_stdin(h)
  let sout = __syx_child_stdout(h)
  let n = __syx_child_pipe_write(sin, [72u8, 105u8, 10u8])
  __syx_child_pipe_close(sin)
  var total = 0
  var chunk = __syx_child_pipe_read_chunk(sout, 64)
  while chunk.count > 0 {
    total = total + chunk.count
    chunk = __syx_child_pipe_read_chunk(sout, 64)
  }
  __syx_child_pipe_close(sout)
  let code = __syx_child_wait(h)
  print("OK")
  print(n)
  print(total)
  print(code)
}
"#,
    );
    let (code, stdout) = run_native_strict(&src);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "OK\n3\n3\n0",
        "native llvm ChildStdin+PipeWrite echo must exact: {out}"
    );
}

#[test]
fn m2_s6_02b_child_stdin_write_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["ChildStdin", "ChildPipeWrite"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-590): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"unsupported\""),
            "{name} must keep wasm=unsupported: {block}"
        );
        assert!(
            block.contains("ADR-590") && block.contains("ADR-243"),
            "{name} notes must cite ADR-590 + ADR-243: {block}"
        );
    }
}

#[cfg(unix)]
#[test]
fn m2_s6_02b_stderr_stream_native_llvm() {
    // Wave A W247 / ADR-591 — bare hosts (stderr stream).
    let src = facade_src(&format!(
        r#"
func main() {{
  let b = __syx_process_builder_create("/usr/bin/python3", ["-c", "import sys; sys.stderr.buffer.write(b'B'*{BODY_LEN})"])
  let h = __syx_process_builder_spawn(b)
  let p = __syx_child_stderr(h)
  var nonempty = 0
  var total = 0
  var chunk = __syx_child_pipe_read_chunk(p, 1024)
  while chunk.count > 0 {{
    nonempty = nonempty + 1
    total = total + chunk.count
    chunk = __syx_child_pipe_read_chunk(p, 1024)
  }}
  __syx_child_pipe_close(p)
  let code = __syx_child_wait(h)
  print("OK")
  print(nonempty)
  print(total)
  print(code)
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
        out, "OK\n4\n4096\n0",
        "native llvm ChildStderr stream must exact: {out}"
    );
}

#[test]
fn m2_s6_02b_child_stderr_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "ChildStderr");
    assert!(
        block.contains("llvm = \"real\""),
        "ChildStderr must declare llvm=real (ADR-591): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "ChildStderr must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"unsupported\""),
        "ChildStderr must keep wasm=unsupported: {block}"
    );
    assert!(
        block.contains("ADR-591") && block.contains("ADR-243"),
        "ChildStderr notes must cite ADR-591 + ADR-243: {block}"
    );
}
