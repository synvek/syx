//! EC-M2-3-H TcpListener session live-count leak first nail.
//! TcpListener.close Display product nails.
//! TcpListen + TcpListenerClose HostFn llvm=real.
// was: ec_m2_3_tcp_listener_leak

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};
use syx_vm::{tcp_listener_force_close, tcp_listener_live_count};

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
        "syx_w247_tcp_listen_llvm_{}",
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
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let tcp = include_str!("../../../../packages/net/tcp.sy");
    format!("{tcp}\n{body}")
}

#[test]
fn ec_m2_3_tcp_listener_leak_bind_close_returns_to_baseline() {
    // Wave A W51 / ADR-395 / A-03: TcpListener bind+close product exact (drop print(h)).
    let baseline = tcp_listener_live_count();
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let h = TcpListener.bind(host: "127.0.0.1", port: 0)
    print("OPENED")
    TcpListener.close(handle: h)
    print("CLOSED")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "OPENED\nCLOSED", "bind+close must exact: {out}");
    assert_eq!(
        tcp_listener_live_count(),
        baseline,
        "bind+close must restore tcp_listener_live_count"
    );
}

#[test]
fn ec_m2_3_tcp_listener_leak_bind_without_close_increments() {
    let baseline = tcp_listener_live_count();
    let out = run(&facade_src(
        r#"
func main() {
  let h = TcpListener.bind(host: "127.0.0.1", port: 0)
  print(h)
}
"#,
    ));
    let id: i64 = out.trim().parse().expect("handle id");
    let after = tcp_listener_live_count();
    assert!(
        after > baseline,
        "bind without close must increment live_count"
    );
    assert!(tcp_listener_force_close(id), "force_close {id}");
    assert_eq!(tcp_listener_live_count(), baseline);
}

#[test]
fn m2_error_tcp_listener_close_invalid_handle_display_throws() {
    // Wave A W175 / ADR-519 / A-03 — TcpListener.close invalid-handle facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  TcpListener.close(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] TcpListener close failed",
        "TcpListener.close invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_tcp_listener_close_after_close_display_throws() {
    // Wave A W175 / ADR-519 / A-03 — TcpListener close-after-close facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let h = TcpListener.bind(host: "127.0.0.1", port: 0)
  TcpListener.close(handle: h)
  TcpListener.close(handle: h)
}
"#,
    ));
    assert_eq!(
        err, "[R007] TcpListener close failed",
        "TcpListener close-after-close Display must exact: {err}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s4_tcp_listen_close_native_llvm() {
    // Wave A W247 / ADR-591 — bare hosts under SYX_NATIVE_STRICT.
    let src = facade_src(
        r#"
func main() {
  let h = __syx_tcp_listen("127.0.0.1", 0)
  print("OPENED")
  __syx_tcp_listener_close(h)
  print("CLOSED")
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
        out, "OPENED\nCLOSED",
        "native llvm TcpListen+Close must exact: {out}"
    );
}

#[test]
fn m2_s4_tcp_listen_close_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["TcpListen", "TcpListenerClose"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-591): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-718): {block}"
        );
        assert!(
            block.contains("ADR-718") && block.contains("ADR-591") && block.contains("ADR-255"),
            "{name} notes must cite ADR-718 + ADR-591 + ADR-255: {block}"
        );
    }
}
