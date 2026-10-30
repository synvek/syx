//! 01 ProcessBuilder cwd/env/kill/timeout nail.
//! ProcessBuilder HostFn llvm=real (native AOT nail).
//! ChildKill + ChildWaitTimeout HostFn llvm=real.
// was: m2_s6_01_process_builder

use std::fs;
use std::path::PathBuf;
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
        "syx_w243_process_builder_llvm_{}",
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
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W106 / ADR-450: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let proc = include_str!("../../../../packages/process/mod.sy");
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    format!("{proc}\n{pb}\n{body}")
}

fn tmp_dir(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("syx_m2_s6_01_{name}"));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).expect("mkdir");
    p
}

#[test]
fn m2_s6_01_cwd_overlay() {
    // Wave A W54 / ADR-398 / A-03: ProcessBuilder cwd product exact (drop print(err)).
    let dir = tmp_dir("cwd");
    let marker = dir.join("marker.txt");
    let dir_s = dir.to_string_lossy().replace('\\', "\\\\");
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let b = ProcessBuilder.create(command: "/bin/sh", args: ["-c", "touch marker.txt"])
    ProcessBuilder.cwd(builder: b, path: "{dir_s}")
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("OK")
    print(code)
  }} catch (err) {{
    print("ERR")
  }}
}}
"#
    )));
    assert_eq!(out, "OK\n0", "cwd overlay must exact: {out}");
    assert!(
        marker.exists(),
        "child must create marker in cwd {dir:?}: {out}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m2_s6_01_env_overlay() {
    // Wave A W54 / ADR-398 / A-03: ProcessBuilder env product exact (drop print(err)).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/bin/sh", args: ["-c", "test \"$SYX_PB_KEY\" = \"v1\""])
    ProcessBuilder.env(builder: b, key: "SYX_PB_KEY", value: "v1")
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("OK")
    print(code)
  } catch (err) {
    print("ERR")
  }
}
"#,
    ));
    assert_eq!(out, "OK\n0", "env overlay must exact: {out}");
}

#[test]
fn m2_s6_01_kill_and_fail_after() {
    // Wave A W55 / ADR-399 / A-03: ProcessBuilder kill fail-after product exact (drop print(e1)).
    let out = run(&facade_src(
        r#"
func main() {
  var h = -1
  try {
    let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
    h = ProcessBuilder.spawn(builder: b)
    Child.kill(handle: h)
    print("KILLED")
  } catch (e1) {
    print("KILL_ERR")
  }
  try {
    Child.wait(handle: h)
    print("UNEXPECTED_WAIT")
  } catch (e2) {
    print("WAIT_FAIL")
  }
  try {
    Child.kill(handle: h)
    print("UNEXPECTED_KILL")
  } catch (e3) {
    print("KILL_FAIL")
  }
}
"#,
    ));
    assert_eq!(
        out, "KILLED\nWAIT_FAIL\nKILL_FAIL",
        "kill fail-after must exact: {out}"
    );
}

#[test]
fn m2_s6_01_wait_timeout_kills() {
    // Wave A W56 / ADR-400 / A-03: ProcessBuilder waitTimeout kills product exact.
    let out = run(&facade_src(
        r#"
func main() {
  var h = -1
  try {
    let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
    h = ProcessBuilder.spawn(builder: b)
    let code = Child.waitTimeout(handle: h, timeoutMs: 200)
    print("UNEXPECTED")
    print(code)
  } catch (e1) {
    print("TIMEOUT")
  }
  try {
    Child.wait(handle: h)
    print("UNEXPECTED_WAIT")
  } catch (e2) {
    print("GONE")
  }
}
"#,
    ));
    assert_eq!(out, "TIMEOUT\nGONE", "waitTimeout kills must exact: {out}");
}

#[test]
fn m2_s6_01_wait_timeout_success() {
    // Wave A W55 / ADR-399 / A-03: ProcessBuilder waitTimeout success product exact (drop print(err)).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.waitTimeout(handle: h, timeoutMs: 5000)
    print("OK")
    print(code)
  } catch (err) {
    print("ERR")
  }
}
"#,
    ));
    assert_eq!(out, "OK\n0", "waitTimeout success must exact: {out}");
}

#[test]
fn m2_s6_01_docs_and_capability() {
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    assert!(
        pb.contains("ProcessBuilder")
            && pb.contains("Child")
            && pb.contains("__syx_process_builder_create")
            && pb.contains("__syx_child_wait_timeout"),
        "ProcessBuilder/Child facade required"
    );

    let hosts = include_str!("../../../syx-vm/src/stdlib_phase84_hosts.rs");
    assert!(
        hosts.contains("PROCESS_BUILDER_SESSIONS")
            && hosts.contains("host_child_kill")
            && hosts.contains("host_child_wait_timeout")
            && hosts.contains("Child invalid handle"),
        "builder sessions + fail-after hosts required"
    );

    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ProcessBuilder")
            && guide.contains("ADR-242")
            && guide.contains("wait/kill 后再操作必须失败"),
        "UsageGuide must document fail-after"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "os.process.builder");
    // Honesty: wasm=unsupported keeps overall status=partial (≠ product implemented).
    assert!(
        block.contains("status = \"partial\"")
            && block.contains("native = \"real\"")
            && block.contains("wasm = \"unsupported\""),
        "os.process.builder must stay partial/native=real/wasm=unsupported: {block}"
    );
    assert!(
        (block.contains("ADR-602") || block.contains("ADR-242")) && block.contains("W258"),
        "os.process.builder notes must cite ADR-602: {block}"
    );
    assert!(
        block.contains("m2_s6_01_process_builder"),
        "tests must list nail"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s6_01_process_builder_tier = \"signed-ADR-242\""),
        "inventory must note signed-ADR-242"
    );
}


#[test]
fn m2_error_child_wait_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_wait(999999)
}
"#,
    );
    // Wave A W106 / ADR-450 / A-03: Child wait invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child wait invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_wait_invalid_handle_display_throws() {
    // Wave A W181 / ADR-525 / A-03 — Child.wait invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Child.wait(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child wait failed",
        "Child.wait invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_wait_after_kill_display_throws() {
    // Wave A W181 / ADR-525 / A-03 — Child.wait after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  let _ = Child.wait(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child wait failed",
        "Child.wait-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_kill_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_kill(999999)
}
"#,
    );
    // Wave A W106 / ADR-450 / A-03: Child kill invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child kill invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_kill_invalid_handle_display_throws() {
    // Wave A W180 / ADR-524 / A-03 — Child.kill invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  Child.kill(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child kill failed",
        "Child.kill invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_kill_after_kill_display_throws() {
    // Wave A W180 / ADR-524 / A-03 — Child.kill after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  Child.kill(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child kill failed",
        "Child.kill-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_create_empty_command_throws() {
    // Wave A W191 / ADR-535 / A-03 — ProcessBuilder create empty-command bare R003.
    let err = run_err(
        r#"
func main() {
  let _ = __syx_process_builder_create("", [])
}
"#,
    );
    assert_eq!(
        err, "[R003] ProcessBuilder create: empty command",
        "ProcessBuilder create empty-command bare must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_create_empty_command_display_throws() {
    // Wave A W191 / ADR-535 / A-03 — ProcessBuilder.create empty-command facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = ProcessBuilder.create(command: "", args: [])
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder create failed",
        "ProcessBuilder.create empty-command Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_cwd_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_process_builder_cwd(999999, "/tmp")
}
"#,
    );
    // Wave A W107 / ADR-451 / A-03: ProcessBuilder cwd invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ProcessBuilder invalid handle 999999",
        "ProcessBuilder cwd invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_cwd_invalid_handle_display_throws() {
    // Wave A W191 / ADR-535 / A-03 — ProcessBuilder.cwd invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  ProcessBuilder.cwd(builder: 999999, path: "/tmp")
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder cwd failed",
        "ProcessBuilder.cwd invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_env_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_process_builder_env(999999, "K", "V")
}
"#,
    );
    // Wave A W107 / ADR-451 / A-03: ProcessBuilder env invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ProcessBuilder invalid handle 999999",
        "ProcessBuilder env invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_env_invalid_handle_display_throws() {
    // Wave A W190 / ADR-534 / A-03 — ProcessBuilder.env invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  ProcessBuilder.env(builder: 999999, key: "K", value: "V")
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder env failed",
        "ProcessBuilder.env invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_env_empty_key_display_throws() {
    // Wave A W190 / ADR-534 / A-03 — ProcessBuilder.env empty-key facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "true", args: [])
  ProcessBuilder.env(builder: b, key: "", value: "v")
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder env failed",
        "ProcessBuilder.env empty-key Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_spawn_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_process_builder_spawn(999999)
}
"#,
    );
    // Wave A W108 / ADR-452 / A-03: ProcessBuilder spawn invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] ProcessBuilder invalid handle 999999",
        "ProcessBuilder spawn invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_spawn_invalid_handle_display_throws() {
    // Wave A W189 / ADR-533 / A-03 — ProcessBuilder.spawn invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = ProcessBuilder.spawn(builder: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder spawn failed",
        "ProcessBuilder.spawn invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_process_builder_spawn_missing_cmd_display_throws() {
    // Wave A W189 / ADR-533 / A-03 — ProcessBuilder.spawn missing-cmd facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/nonexistent/syx_qs1_missing_cmd", args: [])
  let _ = ProcessBuilder.spawn(builder: b)
}
"#,
    ));
    assert_eq!(
        err, "[R007] ProcessBuilder spawn failed",
        "ProcessBuilder.spawn missing-cmd Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_signal_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_signal(999999, "TERM")
}
"#,
    );
    // Wave A W108 / ADR-452 / A-03: Child signal invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child signal invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_signal_invalid_handle_display_throws() {
    // Wave A W182 / ADR-526 / A-03 — Child.signal invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  Child.signal(handle: 999999, name: "TERM")
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child signal failed",
        "Child.signal invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_signal_after_kill_display_throws() {
    // Wave A W182 / ADR-526 / A-03 — Child.signal after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  Child.signal(handle: h, name: "TERM")
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child signal failed",
        "Child.signal-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_wait_timeout_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_wait_timeout(999999, 100)
}
"#,
    );
    // Wave A W109 / ADR-453 / A-03: Child waitTimeout invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child waitTimeout invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_wait_timeout_invalid_handle_display_throws() {
    // Wave A W183 / ADR-527 / A-03 — Child.waitTimeout invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Child.waitTimeout(handle: 999999, timeoutMs: 100)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child waitTimeout failed",
        "Child.waitTimeout invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_wait_timeout_after_kill_display_throws() {
    // Wave A W183 / ADR-527 / A-03 — Child.waitTimeout after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  let _ = Child.waitTimeout(handle: h, timeoutMs: 100)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child waitTimeout failed",
        "Child.waitTimeout-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdout_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_stdout(999999)
}
"#,
    );
    // Wave A W109 / ADR-453 / A-03: Child stdout invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child stdout invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdout_invalid_handle_display_throws() {
    // Wave A W184 / ADR-528 / A-03 — Child.stdout invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Child.stdout(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stdout take failed",
        "Child.stdout invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdout_after_kill_display_throws() {
    // Wave A W184 / ADR-528 / A-03 — Child.stdout after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  let _ = Child.stdout(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stdout take failed",
        "Child.stdout-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stderr_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_stderr(999999)
}
"#,
    );
    // Wave A W110 / ADR-454 / A-03: Child stderr invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child stderr invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_stderr_invalid_handle_display_throws() {
    // Wave A W185 / ADR-529 / A-03 — Child.stderr invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Child.stderr(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stderr take failed",
        "Child.stderr invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stderr_after_kill_display_throws() {
    // Wave A W185 / ADR-529 / A-03 — Child.stderr after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  let _ = Child.stderr(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stderr take failed",
        "Child.stderr-after-kill Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdin_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_child_stdin(999999)
}
"#,
    );
    // Wave A W110 / ADR-454 / A-03: Child stdin invalid-handle Display product exact.
    assert_eq!(
        err, "[R003] Child invalid handle 999999",
        "Child stdin invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdin_invalid_handle_display_throws() {
    // Wave A W186 / ADR-530 / A-03 — Child.stdin invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Child.stdin(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stdin take failed",
        "Child.stdin invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_child_stdin_after_kill_display_throws() {
    // Wave A W186 / ADR-530 / A-03 — Child.stdin after-kill facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let b = ProcessBuilder.create(command: "/bin/sleep", args: ["30"])
  let h = ProcessBuilder.spawn(builder: b)
  Child.kill(handle: h)
  let _ = Child.stdin(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] Child stdin take failed",
        "Child.stdin-after-kill Display must exact: {err}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s6_01_process_builder_cwd_env_native_llvm() {
    let dir = tmp_dir("cwd_env_llvm");
    let marker = dir.join("marker.txt");
    let dir_s = dir.to_string_lossy().replace('\\', "\\\\");
    let src = facade_src(&format!(
        r#"
func main() {{
  try {{
    let b = ProcessBuilder.create(command: "/bin/sh", args: ["-c", "test \"$SYX_PB_KEY\" = \"v1\" && touch marker.txt"])
    ProcessBuilder.cwd(builder: b, path: "{dir_s}")
    ProcessBuilder.env(builder: b, key: "SYX_PB_KEY", value: "v1")
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("OK")
    print(code)
  }} catch (err) {{
    print("ERR")
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
        out, "OK\n0",
        "native llvm ProcessBuilder cwd+env+spawn must exact: {out}"
    );
    assert!(
        marker.exists(),
        "child must create marker in cwd {dir:?}: {out}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m2_s6_01_process_builder_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "ProcessBuilderCreate",
        "ProcessBuilderCwd",
        "ProcessBuilderEnv",
        "ProcessBuilderSpawn",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-587/ADR-602): {block}"
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
            (block.contains("ADR-602") || block.contains("ADR-587"))
                && block.contains("ADR-242"),
            "{name} notes must cite ADR-602/587 + ADR-242: {block}"
        );
    }
}

#[cfg(unix)]
#[test]
fn m2_s6_01_child_kill_native_llvm() {
    // Wave A W246 / ADR-590 — bare hosts + single-layer try.
    let src = facade_src(
        r#"
func main() {
  var h = -1
  let b = __syx_process_builder_create("/bin/sleep", ["30"])
  h = __syx_process_builder_spawn(b)
  __syx_child_kill(h)
  print("KILLED")
  try {
    let _ = __syx_child_wait(h)
    print("UNEXPECTED_WAIT")
  } catch (e2) {
    print("WAIT_FAIL")
  }
  try {
    __syx_child_kill(h)
    print("UNEXPECTED_KILL")
  } catch (e3) {
    print("KILL_FAIL")
  }
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
        out, "KILLED\nWAIT_FAIL\nKILL_FAIL",
        "native llvm ChildKill fail-after must exact: {out}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s6_01_child_wait_timeout_native_llvm() {
    let src = facade_src(
        r#"
func main() {
  let b = __syx_process_builder_create("/usr/bin/true", [])
  let h = __syx_process_builder_spawn(b)
  let code = __syx_child_wait_timeout(h, 5000)
  print("OK")
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
        out, "OK\n0",
        "native llvm ChildWaitTimeout success must exact: {out}"
    );
}

#[test]
fn m2_s6_01_child_kill_wait_timeout_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["ChildKill", "ChildWaitTimeout"] {
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
            block.contains("ADR-590") && block.contains("ADR-242"),
            "{name} notes must cite ADR-590 + ADR-242: {block}"
        );
    }
}
