//! HostFn Q-S1 File matrix nails (vm embed + native/wasm exact lines).

use std::fs;
use std::path::PathBuf;
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
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!("{fh}\n{body}")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_hostfn_file_{name}_{}", std::process::id()))
}

#[test]
fn hostfn_file_matrix_toml_lists_file_batch() {
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    // W1 File fingerprint superseded as primary by W2 Http fingerprint; File rows remain.
    assert!(
        matrix.contains("host = \"FileOpen\"")
            && matrix.contains("host = \"FileClose\"")
            && matrix.contains("host = \"FileRead\"")
            && matrix.contains("host = \"FileWrite\""),
        "File batch rows must remain in matrix"
    );
    assert!(
        matrix.contains("vm = \"covered\"") && matrix.contains("mir = \"covered\""),
        "File batch must claim vm+mir covered"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
    assert!(
        matrix.contains("FileWrite") && matrix.contains("wasm = \"covered\""),
        "W21 FileWrite wasm cell must be present"
    );
}

#[test]
fn hostfn_file_open_write_read_close_success() {
    let path = tmp_path("success");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let _ = fs::remove_file(&path);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let h = FileHandle.open(path: "{path_s}", mode: "w")
  let n = FileHandle.write(handle: h, bytes: [65u8, 66u8, 67u8])
  FileHandle.close(handle: h)
  let h2 = FileHandle.open(path: "{path_s}", mode: "r")
  let bytes = FileHandle.read(handle: h2, maxBytes: 16)
  FileHandle.close(handle: h2)
  print(n)
  print(bytes.count)
  print(bytes[0])
}}
"#
    )))
    .expect("success path");
    let _ = fs::remove_file(&path);
    assert_eq!(out, vec!["3".to_string(), "3".to_string(), "65".to_string()]);
}

#[test]
fn hostfn_file_open_missing_io_err() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let h = FileHandle.open(path: "/tmp/syx_qs1_no_such_file_wave_a_w1.bin", mode: "r")
    print("UNEXPECTED_OK")
    print(h)
  } catch (err) {
    print("caught")
    print(err.code)
  }
}
"#,
    ))
    .expect("missing open must catch");
    assert_eq!(out[0], "caught");
    assert!(
        out.len() >= 2 && !out[1].is_empty(),
        "IOError code expected: {out:?}"
    );
}

/// Wave A W9 / ADR-353 / A-05 — FileOpen llvm cell (native CallHost try/catch).
/// Wave A W127 / ADR-471 / A-03: IOError.code ObjectGet native llvm exact.
#[cfg(unix)]
#[test]
fn hostfn_file_open_missing_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let h = FileHandle.open(path: "/tmp/syx_qs1_no_such_file_wave_a_w1.bin", mode: "r")
    print("UNEXPECTED_OK")
    print(h)
  } catch (err) {
    print("caught")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_llvm_open_{}",
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
        vec!["caught".to_string(), "1".to_string()],
        "FileOpen llvm missing-path must exact-catch with code: {lines:?}"
    );
}

/// Wave A W18 / ADR-362 / A-05 — FileOpen wasm cell (AOT CallHost try/catch).
/// Wave A W128 / ADR-472 / A-03: IOError.code ObjectGet wasm exact.
#[test]
fn hostfn_file_open_missing_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let h = FileHandle.open(path: "/tmp/syx_qs1_no_such_file_wave_a_w1.bin", mode: "r")
    print("UNEXPECTED_OK")
    print(h)
  } catch (err) {
    print("caught")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_wasm_open_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm missing-path run failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["caught".to_string(), "1".to_string()],
        "FileOpen wasm missing-path must exact-catch with code: {lines:?}"
    );
}

#[test]
fn hostfn_file_close_bad_handle_param_err() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    FileHandle.close(handle: 999999001)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
    print(err.code)
  }
}
"#,
    ))
    .expect("bad close must catch");
    assert_eq!(out[0], "caught");
}

/// Wave A W6 / ADR-350 / A-05 — FileClose llvm first cell (native CallHost try/catch).
/// Exact catch marker matches embed nail's asserted first line. IOError.code ObjectGet on
/// llvm remains deepen (native data-class fields still nil) — not claimed here.
#[cfg(unix)]
#[test]
fn hostfn_file_close_bad_handle_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    FileHandle.close(handle: 999999001)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_llvm_close_{}",
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
        vec!["caught".to_string()],
        "FileClose llvm bad-handle must exact-catch: {lines:?}"
    );
}

/// Wave A W19 / ADR-363 / A-05 — FileClose wasm cell (AOT CallHost try/catch).
/// Exact catch marker only — err.code ObjectGet remains deepen (W20+).
#[test]
fn hostfn_file_close_bad_handle_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    FileHandle.close(handle: 999999001)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_wasm_close_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm bad-handle close failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["caught".to_string()],
        "FileClose wasm bad-handle must exact-catch: {lines:?}"
    );
}

#[test]
fn hostfn_file_read_bad_handle_io_err() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.read(handle: 999999002, maxBytes: 8)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
    print(err.code)
  }
}
"#,
    ))
    .expect("bad read must catch");
    assert_eq!(out[0], "caught");
}

/// Wave A W7 / ADR-351 / A-05 — FileRead llvm cell (native CallHost try/catch).
/// Exact catch marker only — IOError.code ObjectGet on llvm remains deepen (W10+).
#[cfg(unix)]
#[test]
fn hostfn_file_read_bad_handle_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.read(handle: 999999002, maxBytes: 8)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_llvm_read_{}",
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
        vec!["caught".to_string()],
        "FileRead llvm bad-handle must exact-catch: {lines:?}"
    );
}

/// Wave A W20 / ADR-364 / A-05 — FileRead wasm cell (AOT CallHost try/catch).
/// Exact catch marker only — err.code ObjectGet remains deepen (W22+).
#[test]
fn hostfn_file_read_bad_handle_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.read(handle: 999999002, maxBytes: 8)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_wasm_read_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm bad-handle read failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["caught".to_string()],
        "FileRead wasm bad-handle must exact-catch: {lines:?}"
    );
}

/// Wave A W21 / ADR-365 / A-05 — FileWrite wasm cell (AOT CallHost try/catch).
/// Exact catch marker only — err.code ObjectGet remains deepen (W22+).
#[test]
fn hostfn_file_write_bad_handle_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.write(handle: 999999003, bytes: [1u8])
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_wasm_write_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm bad-handle write failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["caught".to_string()],
        "FileWrite wasm bad-handle must exact-catch: {lines:?}"
    );
}

#[test]
fn hostfn_file_write_bad_handle_io_err() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.write(handle: 999999003, bytes: [1u8])
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
    print(err.code)
  }
}
"#,
    ))
    .expect("bad write must catch");
    assert_eq!(out[0], "caught");
}

/// Wave A W8 / ADR-352 / A-05 — FileWrite llvm cell (native CallHost try/catch).
/// Exact catch marker only — IOError.code ObjectGet on llvm remains deepen (W10+).
#[cfg(unix)]
#[test]
fn hostfn_file_write_bad_handle_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let _ = FileHandle.write(handle: 999999003, bytes: [1u8])
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_file_llvm_write_{}",
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
        vec!["caught".to_string()],
        "FileWrite llvm bad-handle must exact-catch: {lines:?}"
    );
}

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let marker = format!("name = \"{name}\"");
    let start = toml
        .find(&marker)
        .unwrap_or_else(|| panic!("host {name} missing"));
    let rest = &toml[start..];
    let end = rest.find("\n[[host]]").unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn hostfn_file_hosts_wasm_real_w268() {
    // Wave A W268 / ADR-612 — FileOpen/Close/Read/Write wasm=real (behavior nails: qs1_*_wasm)
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileOpen", "FileClose", "FileRead", "FileWrite"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("ADR-612"),
            "{name} notes must cite ADR-612: {block}"
        );
    }
}

