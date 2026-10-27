//! HostFn Q-S1 Tcp matrix nails (vm embed + native/wasm exact lines).

use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run(src: &str) -> Result<Vec<String>, String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .map(|r| r.printed)
        .map_err(|e| format!("{e:?}"))
}

fn facade_src(body: &str) -> String {
    let tcp = include_str!("../../../../packages/net/tcp.sy");
    format!("{tcp}\n{body}")
}

/// Accept one connection then idle until stop (enough for connect nails).
fn start_accept_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
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
                Ok((_stream, _)) => {
                    while !stop_flag.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(10));
                    }
                    break;
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

/// Accept connections and write fixed payload (TcpRead wasm evidence).
fn start_send_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let handle = thread::spawn(move || {
        let mut held = Vec::new();
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.write_all(b"hi");
                    let _ = stream.flush();
                    held.push(stream);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
        drop(held);
    });
    (port, stop, handle)
}

#[test]
fn hostfn_tcp_matrix_toml_lists_tcp_batch() {
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    // W3 Tcp fingerprint may be superseded as primary; Tcp rows remain.
    assert!(
        matrix.contains("host = \"TcpConnect\""),
        "matrix missing TcpConnect"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
    assert!(
        matrix.contains("vm = \"covered\"") && matrix.contains("mir = \"covered\""),
        "Tcp batch must claim vm+mir covered"
    );
    assert!(
        matrix.contains("llvm = \"covered\""),
        "W11 TcpConnect llvm cell / fingerprint must be present"
    );
    assert!(
        matrix.contains("TcpConnect") && matrix.contains("wasm = \"covered\""),
        "W23 TcpConnect wasm cell must be present"
    );
}

#[test]
fn hostfn_tcp_connect_refused_exact() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let s = TcpStream.connect(host: "127.0.0.1", port: 9)
    print("UNEXPECTED")
    print(s.handle)
  } catch (err) {
    print("refused")
    print(err.code)
  }
}
"#,
    ))
    .expect("refused must catch");
    assert_eq!(out, vec!["refused".to_string(), "1".to_string()]);
}

/// Wave A W11 / ADR-355 / A-05 — TcpConnect llvm cell (native CallHost try/catch).
/// Wave A W131 / ADR-475 / A-03: IOError.code ObjectGet native llvm exact.
#[cfg(unix)]
#[test]
fn hostfn_tcp_connect_refuse_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let s = TcpStream.connect(host: "127.0.0.1", port: 9)
    print("UNEXPECTED")
    print(s.handle)
  } catch (err) {
    print("refused")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_llvm_refuse_{}",
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
        vec!["refused".to_string(), "1".to_string()],
        "TcpConnect llvm refuse must exact-catch with code: {lines:?}"
    );
}

/// Wave A W23 / ADR-367 / A-05 — TcpConnect wasm cell (AOT CallHost try/catch).
/// Wave A W129 / ADR-473 / A-03: IOError.code ObjectGet wasm exact.
#[test]
fn hostfn_tcp_connect_refuse_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let s = TcpStream.connect(host: "127.0.0.1", port: 9)
    print("UNEXPECTED")
    print(s.handle)
  } catch (err) {
    print("refused")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_refuse_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm refuse connect failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["refused".to_string(), "1".to_string()],
        "TcpConnect wasm refuse must exact-catch with code: {lines:?}"
    );
}


#[test]
fn hostfn_tcp_connect_loopback_release_exact() {
    let (port, stop, handle) = start_accept_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = facade_src(&format!(
        r#"
func main() {{
  try {{
    let s = TcpStream.connect(host: "127.0.0.1", port: {port})
    print("opened")
    __syx_tcp_handle_release(s.handle)
    print("closed")
  }} catch (err) {{
    print("caught")
    print(err.message)
  }}
}}
"#
    ));
    let out = run(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let out = out.expect("loopback connect");
    assert_eq!(out, vec!["opened".to_string(), "closed".to_string()]);
}

/// Wave A W274 / ADR-618 — TcpRead wasm loopback exact `hi` + release.
#[test]
fn hostfn_tcp_read_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, join) = start_send_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"func main() {{
    let h = __syx_tcp_connect("127.0.0.1", {port})
    let s = __syx_tcp_read(h, 8)
    print(s)
    print(__syx_tcp_handle_release(h))
    return 0
}}"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_read_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tcp read failed: {e:?}"));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["hi".to_string(), "true".to_string()],
        "TcpRead wasm must exact hi+true: {lines:?}"
    );
}

/// Wave A W274 / ADR-618 — TcpWrite wasm loopback exact nbytes + release.
#[test]
fn hostfn_tcp_write_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, join) = start_accept_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"func main() {{
    let h = __syx_tcp_connect("127.0.0.1", {port})
    print(__syx_tcp_write(h, "hi"))
    print(__syx_tcp_handle_release(h))
    return 0
}}"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_write_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tcp write failed: {e:?}"));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["2".to_string(), "true".to_string()],
        "TcpWrite wasm must exact 2+true: {lines:?}"
    );
}

/// Wave A W274 / ADR-618 — TcpHandleRelease wasm loopback exact `true`.
#[test]
fn hostfn_tcp_handle_release_loopback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, join) = start_accept_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"func main() {{
    let h = __syx_tcp_connect("127.0.0.1", {port})
    print(__syx_tcp_handle_release(h))
    return 0
}}"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_release_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tcp release failed: {e:?}"));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["true".to_string()],
        "TcpHandleRelease wasm must exact true: {lines:?}"
    );
}

/// Wave A W383 / ADR-727 — TcpListenerAccept llvm first-slice loopback exact ACCEPTED/CLOSED.
#[cfg(unix)]
#[test]
fn hostfn_tcp_listener_accept_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);
    let src = format!(
        r#"
func main() {{
  let h = __syx_tcp_listen("127.0.0.1", {port})
  let s = __syx_tcp_listener_accept(h)
  print("ACCEPTED")
  __syx_tcp_handle_release(s)
  __syx_tcp_listener_close(h)
  print("CLOSED")
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_llvm_accept_{}",
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
    let join = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)) {
                thread::sleep(Duration::from_millis(200));
                drop(stream);
                return;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    let output = Command::new(&exe).output().expect("run native");
    let _ = join.join();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native llvm accept exit {code}: {stdout}");
    assert_eq!(
        lines,
        vec!["ACCEPTED".to_string(), "CLOSED".to_string()],
        "TcpListenerAccept llvm must exact ACCEPTED/CLOSED: {lines:?}"
    );
}

#[test]
fn hostfn_tcp_listener_accept_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "TcpListenerAccept");
    assert!(
        block.contains("llvm = \"real\""),
        "TcpListenerAccept must declare llvm=real (ADR-727): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "TcpListenerAccept must declare wasm=real (ADR-728): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "TcpListenerAccept must keep status=partial: {block}"
    );
    assert!(
        block.contains("ADR-728") && block.contains("ADR-727"),
        "TcpListenerAccept notes must cite ADR-728 + ADR-727: {block}"
    );
}

/// Wave A W384 / ADR-728 — TcpListenerAccept wasm first-slice loopback exact ACCEPTED/CLOSED.
#[test]
fn hostfn_tcp_listener_accept_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);
    let src = format!(
        r#"
func main() {{
  let h = __syx_tcp_listen("127.0.0.1", {port})
  let s = __syx_tcp_listener_accept(h)
  print("ACCEPTED")
  __syx_tcp_handle_release(s)
  __syx_tcp_listener_close(h)
  print("CLOSED")
  return 0
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_accept_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let join = thread::spawn(move || {
        // Wasm AOT compile can exceed a short window before listen binds.
        let deadline = Instant::now() + Duration::from_secs(120);
        while Instant::now() < deadline {
            if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)) {
                thread::sleep(Duration::from_millis(500));
                drop(stream);
                return;
            }
            thread::sleep(Duration::from_millis(20));
        }
    });
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tcp accept failed: {e:?}"));
    let _ = join.join();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["ACCEPTED".to_string(), "CLOSED".to_string()],
        "TcpListenerAccept wasm must exact ACCEPTED/CLOSED: {lines:?}"
    );
}

/// Wave A W374 / ADR-718 — TcpListen+TcpListenerClose wasm bind+close exact OPENED/CLOSED.
#[test]
fn hostfn_tcp_listen_close_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let h = __syx_tcp_listen("127.0.0.1", 0)
  print("OPENED")
  __syx_tcp_listener_close(h)
  print("CLOSED")
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tcp_wasm_listen_close_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tcp listen/close failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["OPENED".to_string(), "CLOSED".to_string()],
        "TcpListen+Close wasm must exact OPENED/CLOSED: {lines:?}"
    );
}

/// Wave A W274 / ADR-618 — Tcp×4 HostFn wasm=real meta flip.
#[test]
fn hostfn_tcp_host_wasm_real_w274() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for (name, llvm_adr) in [
        ("TcpConnect", "ADR-207"),
        ("TcpRead", "ADR-219"),
        ("TcpWrite", "ADR-217"),
        ("TcpHandleRelease", "ADR-212"),
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-618): {block}"
        );
        assert!(
            block.contains("ADR-618") && block.contains(llvm_adr),
            "{name} notes must cite ADR-618 + {llvm_adr}: {block}"
        );
    }
}

#[test]
fn hostfn_tcp_plan_a05_mentions_tcp_batch() {
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
