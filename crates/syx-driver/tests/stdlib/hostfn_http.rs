//! HostFn Q-S1 Http matrix nails (vm embed + native/wasm exact lines).

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

fn run(src: &str) -> Result<Vec<String>, String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .map(|r| r.printed)
        .map_err(|e| format!("{e:?}"))
}

/// Ephemeral HTTP/1.1 `/` → 200 + body `ok` on 127.0.0.1.
fn start_ok_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut buf = [0u8; 2048];
                    let _ = stream.read(&mut buf);
                    let body = b"ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                        body.len()
                    );
                    let _ = stream.write_all(resp.as_bytes());
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });
    (port, stop, handle)
}

#[test]
fn hostfn_http_matrix_toml_lists_http_batch() {
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    // W2 Http fingerprint may be superseded as primary; Http rows remain.
    assert!(
        matrix.contains("host = \"HttpGet\""),
        "matrix missing HttpGet"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
    assert!(
        matrix.contains("vm = \"covered\"") && matrix.contains("mir = \"covered\""),
        "Http batch must claim vm+mir covered"
    );
    assert!(
        matrix.contains("llvm = \"covered\""),
        "W10 HttpGet llvm cell / fingerprint must be present"
    );
    assert!(
        matrix.contains("HttpGet") && matrix.contains("wasm = \"covered\""),
        "W22 HttpGet wasm cell must be present"
    );
}

#[test]
fn hostfn_http_get_connect_refused_io_err() {
    let out = run(
        r#"
func main() {
  try {
    let r = __syx_http_get("http://127.0.0.1:1/x")
    print("UNEXPECTED")
    print(r.status)
  } catch (err) {
    print("refused")
    print(err.code)
  }
}
"#,
    )
    .expect("refused must catch");
    assert_eq!(out, vec!["refused".to_string(), "R003".to_string()]);
}

/// Wave A W10 / ADR-354 / A-05 — HttpGet llvm cell (native CallHost try/catch).
/// Wave A W127 / ADR-471 / A-03: IOError.code ObjectGet native llvm exact.
#[cfg(unix)]
#[test]
fn hostfn_http_get_refuse_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  try {
    let r = __syx_http_get("http://127.0.0.1:1/x")
    print("UNEXPECTED")
    print(r.status)
  } catch (err) {
    print("refused")
    print(err.code)
  }
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_llvm_refuse_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
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
        vec!["refused".to_string(), "R003".to_string()],
        "HttpGet llvm refuse must exact-catch with code: {lines:?}"
    );
}

/// Wave A W22 / ADR-366 / A-05 — HttpGet wasm cell (AOT CallHost try/catch).
/// Wave A W128 / ADR-472 / A-03: IOError.code ObjectGet wasm exact.
#[test]
fn hostfn_http_get_refuse_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  try {
    let r = __syx_http_get("http://127.0.0.1:1/x")
    print("UNEXPECTED")
    print(r.status)
  } catch (err) {
    print("refused")
    print(err.code)
  }
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_wasm_refuse_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm refuse get failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["refused".to_string(), "R003".to_string()],
        "HttpGet wasm refuse must exact-catch with code: {lines:?}"
    );
}

#[test]
fn hostfn_http_get_loopback_success() {
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let r = __syx_http_get("http://127.0.0.1:{port}/")
  print(r.status)
  print(r.body)
}}
"#
    );
    let out = run(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let out = out.expect("loopback get");
    assert_eq!(out, vec!["200".to_string(), "ok".to_string()]);
}

/// Wave A W17 / ADR-361 / A-05 — HttpGet llvm loopback-success deepen (native CallHost).
/// Exact status+body — err.code ObjectGet on llvm remains deepen (W18+).
#[cfg(unix)]
#[test]
fn hostfn_http_get_loopback_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let r = __syx_http_get("http://127.0.0.1:{port}/")
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_llvm_loopback_{}",
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
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpGet llvm loopback must exact-success: {lines:?}"
    );
}

/// Wave A W29 / ADR-373 / A-05 — HttpGet wasm loopback-success deepen (AOT CallHost).
/// Exact status+body — err.code ObjectGet remains deepen (W30+).
#[cfg(unix)]
#[test]
fn hostfn_http_get_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let r = __syx_http_get("http://127.0.0.1:{port}/")
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_wasm_loopback_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let result = run_wasm_source_printed(&main, "main", &opts);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let (_, lines) = result.unwrap_or_else(|e| panic!("wasm loopback get failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpGet wasm loopback must exact-success: {lines:?}"
    );
}


fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

#[test]
fn hostfn_http_get_host_llvm_real() {
    // Wave A W247 / ADR-591 — inventory llvm=real meta (behavior: qs1_http_get_loopback_native_llvm).
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "HttpGet");
    assert!(
        block.contains("llvm = \"real\""),
        "HttpGet must declare llvm=real (ADR-591): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "HttpGet must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "HttpGet must declare wasm=real (ADR-614): {block}"
    );
    assert!(
        block.contains("ADR-614") && block.contains("ADR-591"),
        "HttpGet notes must cite ADR-614 + ADR-591: {block}"
    );
}

/// Wave A W248 / ADR-592 / A-05 — HttpPost llvm loopback-success (native CallHost).
#[cfg(unix)]
#[test]
fn hostfn_http_post_loopback_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let r = __syx_http_post("http://127.0.0.1:{port}/", "hi")
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_post_llvm_loopback_{}",
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
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpPost llvm loopback must exact-success: {lines:?}"
    );
}

/// Wave A W271 / ADR-615 — HttpPost wasm loopback-success.
#[cfg(unix)]
#[test]
fn hostfn_http_post_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let r = __syx_http_post("http://127.0.0.1:{port}/", "hi")
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_post_wasm_loopback_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let result = run_wasm_source_printed(&main, "main", &opts);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let (_, lines) = result.unwrap_or_else(|e| panic!("wasm loopback post failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpPost wasm loopback must exact-success: {lines:?}"
    );
}

#[test]
fn hostfn_http_post_host_llvm_real() {
    // Wave A W248 / ADR-592 llvm + Wave A W271 / ADR-615 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "HttpPost");
    assert!(
        block.contains("llvm = \"real\""),
        "HttpPost must declare llvm=real (ADR-592): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "HttpPost must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "HttpPost must declare wasm=real (ADR-615): {block}"
    );
    assert!(
        block.contains("ADR-615") && block.contains("ADR-592"),
        "HttpPost notes must cite ADR-615 + ADR-592: {block}"
    );
}

/// Wave A W249 / ADR-593 / A-05 — HttpRequest llvm loopback-success (native CallHost).
#[cfg(unix)]
#[test]
fn hostfn_http_request_loopback_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let r = __syx_http_request("http://127.0.0.1:{port}/", opts)
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_request_llvm_loopback_{}",
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
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpRequest llvm loopback must exact-success: {lines:?}"
    );
}

/// Wave A W271 / ADR-615 — HttpRequest wasm loopback-success.
#[cfg(unix)]
#[test]
fn hostfn_http_request_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, handle) = start_ok_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let r = __syx_http_request("http://127.0.0.1:{port}/", opts)
  print(r.status)
  print(r.body)
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_http_request_wasm_loopback_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let result = run_wasm_source_printed(&main, "main", &opts);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let (_, lines) = result.unwrap_or_else(|e| panic!("wasm loopback request failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["200".to_string(), "ok".to_string()],
        "HttpRequest wasm loopback must exact-success: {lines:?}"
    );
}

#[test]
fn hostfn_http_request_host_llvm_real() {
    // Wave A W249 / ADR-593 llvm + Wave A W271 / ADR-615 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "HttpRequest");
    assert!(
        block.contains("llvm = \"real\""),
        "HttpRequest must declare llvm=real (ADR-593): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "HttpRequest must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "HttpRequest must declare wasm=real (ADR-615): {block}"
    );
    assert!(
        block.contains("ADR-615") && block.contains("ADR-593"),
        "HttpRequest notes must cite ADR-615 + ADR-593: {block}"
    );
}
