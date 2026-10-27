//! HostFn Q-S1 Process matrix nails (vm embed + native/wasm exact lines).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

fn run(src: &str) -> Result<Vec<String>, String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .map(|r| r.printed)
        .map_err(|e| format!("{e:?}"))
}

fn facade_src(body: &str) -> String {
    let proc = include_str!("../../../../packages/process/mod.sy");
    let pb = include_str!("../../../../packages/process/process_builder.sy");
    format!("{proc}\n{pb}\n{body}")
}

#[test]
fn hostfn_process_matrix_toml_lists_process_batch() {
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    assert!(
        matrix.contains("host = \"ProcessBuilderSpawn\"")
            && matrix.contains("host = \"ProcessBuilderCreate\"")
            && matrix.contains("host = \"ChildWait\""),
        "matrix missing ProcessBuilder/ChildWait rows"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
    assert!(
        matrix.contains("ProcessBuilderCreate")
            && matrix.contains("llvm = \"covered\""),
        "W16 ProcessBuilderCreate llvm cell must be present"
    );
    assert!(
        matrix.contains("ProcessBuilderSpawn") && matrix.contains("wasm = \"covered\""),
        "W24 ProcessBuilderSpawn wasm cell must be present"
    );
    assert!(
        matrix.contains("ChildWait") && matrix.contains("wasm = \"covered\""),
        "W25 ChildWait wasm cell must be present"
    );
    assert!(
        matrix.contains("ProcessBuilderCreate") && matrix.contains("wasm = \"covered\""),
        "W26 ProcessBuilderCreate wasm cell must be present"
    );
    assert!(
        matrix.contains("vm = \"covered\"") && matrix.contains("mir = \"covered\""),
        "Process batch must claim vm+mir covered"
    );
}

#[test]
fn hostfn_process_spawn_missing_cmd_fail_exact() {
    // Wave A W189 / ADR-533: spawn CallHost→facade; caught ProcessError.code == 4.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/nonexistent/syx_qs1_missing_cmd", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    print("UNEXPECTED")
    print(h)
  } catch (err) {
    print("failed")
    print(err.code)
  }
}
"#,
    ))
    .expect("missing cmd must catch");
    assert_eq!(out, vec!["failed".to_string(), "4".to_string()]);
}

/// Wave A W12 / ADR-356 / A-05 — ProcessBuilderSpawn llvm cell (native CallHost try/catch).
/// Wave A W131 / ADR-475 / A-03: IOError.code ObjectGet native llvm exact.
#[cfg(unix)]
#[test]
fn hostfn_process_spawn_missing_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/nonexistent/syx_qs1_missing_cmd", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    print("UNEXPECTED")
    print(h)
  } catch (err) {
    print("failed")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_llvm_spawn_missing_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["failed".to_string(), "4".to_string()],
        "ProcessBuilderSpawn llvm missing-cmd must exact-catch with code: {lines:?}"
    );
}

/// Wave A W24 / ADR-368 / A-05 — ProcessBuilderSpawn wasm cell (AOT CallHost try/catch).
/// Wave A W129 / ADR-473 / A-03: IOError.code ObjectGet wasm exact.
#[cfg(unix)]
#[test]
fn hostfn_process_spawn_missing_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/nonexistent/syx_qs1_missing_cmd", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    print("UNEXPECTED")
    print(h)
  } catch (err) {
    print("failed")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_wasm_spawn_missing_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm spawn missing failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["failed".to_string(), "4".to_string()],
        "ProcessBuilderSpawn wasm missing-cmd must exact-catch with code: {lines:?}"
    );
}

#[test]
fn hostfn_process_spawn_true_wait_exact() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("spawned")
    print(code)
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    ))
    .expect("true spawn+wait");
    assert_eq!(out, vec!["spawned".to_string(), "0".to_string()]);
}

/// Wave A W13 / ADR-357 / A-05 — ChildWait llvm cell (native CallHost success path).
/// Exact spawn+wait lines — err.code ObjectGet on llvm remains deepen (W14+).
#[cfg(unix)]
#[test]
fn hostfn_process_wait_success_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("spawned")
    print(code)
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_llvm_wait_success_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["spawned".to_string(), "0".to_string()],
        "ChildWait llvm spawn-wait must exact-success: {lines:?}"
    );
}

/// Wave A W25 / ADR-369 / A-05 — ChildWait wasm cell (AOT CallHost success path).
/// Exact spawn+wait lines — err.code ObjectGet remains deepen (W26+).
#[cfg(unix)]
#[test]
fn hostfn_process_wait_success_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print("spawned")
    print(code)
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_wasm_wait_success_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm wait success failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["spawned".to_string(), "0".to_string()],
        "ChildWait wasm spawn-wait must exact-success: {lines:?}"
    );
}

/// Wave A W16 / ADR-360 / A-05 — ProcessBuilderCreate llvm cell (native CallHost success path).
/// Exact create marker + wait exit — err.code ObjectGet on llvm remains deepen (W17+).
#[cfg(unix)]
#[test]
fn hostfn_process_create_success_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    print("created")
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print(code)
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_llvm_create_success_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["created".to_string(), "0".to_string()],
        "ProcessBuilderCreate llvm create-success must exact: {lines:?}"
    );
}

/// Wave A W26 / ADR-370 / A-05 — ProcessBuilderCreate wasm cell (AOT CallHost success path).
/// Exact create marker + wait exit — err.code ObjectGet remains deepen (W27+).
#[cfg(unix)]
#[test]
fn hostfn_process_create_success_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let b = ProcessBuilder.create(command: "/usr/bin/true", args: [])
    print("created")
    let h = ProcessBuilder.spawn(builder: b)
    let code = Child.wait(handle: h)
    print(code)
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_process_wasm_create_success_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm create success failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["created".to_string(), "0".to_string()],
        "ProcessBuilderCreate wasm create-success must exact: {lines:?}"
    );
}

#[test]
fn hostfn_process_plan_a05_mentions_process_batch() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    let a05 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-05 |"))
        .expect("A-05 row");
    assert!(
        a05.contains("| [~] |") || a05.contains("| [x] |"),
        "A-05 must stay in progress or done: {a05}"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}
