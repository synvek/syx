//! 04 HTTP body stream nail.
//! HttpBodyStream HostFn llvm=real (native AOT nails).
//! HttpBodyStream HostFn wasm=real (Http 残余收官).
// was: m2_s4_04_http_body_stream

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
        "syx_w242_http_body_stream_llvm_{}",
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

fn run_ok(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded(src, &DriverOptions::default()).expect("run");
    r.printed.join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W117 / ADR-461: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

/// Embed does not auto-load `import Net`; prepend facades (W92 pattern).
/// Wave A W174: include file_handle.sy for IOError (HttpBodyStream.close throws).
fn net_facade_src(body: &str) -> String {
    let parts = [
        include_str!("../../../../packages/io/file_handle.sy"),
        include_str!("../../../../packages/io/types/http_response.sy"),
        include_str!("../../../../packages/net/http_types.sy"),
        include_str!("../../../../packages/net/http_body_stream.sy"),
        include_str!("../../../../packages/net/mod.sy"),
        include_str!("../../../../packages/net/url.sy"),
    ]
    .join("\n");
    format!("{parts}\n{body}")
}

const BODY_LEN: usize = 4096;

fn start_big_body_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>, String) {
    let body: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .cycle()
        .take(BODY_LEN)
        .collect();
    let body_clone = body.clone();
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
                    let mut buf = [0u8; 4096];
                    let _ = stream.read(&mut buf);
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body_clone.len(),
                        body_clone
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
    (port, stop, handle, body)
}

#[test]
fn m2_s4_04_chunked_read_multi_kb() {
    let (port, stop, server, body) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));

    let src = net_facade_src(&format!(
        r#"
func main() {{
    let opened = HttpBodyStream.open(url: "http://127.0.0.1:{port}/big")
    print(opened.status)
    let handle = opened.handle
    var nonempty = 0
    var chunk = HttpBodyStream.readChunk(handle: handle, maxBytes: 1024)
    print("BEGIN")
    while chunk != "" {{
        nonempty = nonempty + 1
        print(chunk)
        chunk = HttpBodyStream.readChunk(handle: handle, maxBytes: 1024)
    }}
    print("END")
    HttpBodyStream.close(handle: handle)
    print(nonempty)
}}
"#
    ));

    let out = run_ok(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();

    // Wave A W90 / ADR-434 / A-03: HTTP body stream chunked read product exact.
    let chunks = (0..4)
        .map(|i| &body[i * 1024..(i + 1) * 1024])
        .collect::<Vec<_>>()
        .join("\n");
    let expected = format!("200\nBEGIN\n{chunks}\nEND\n4");
    assert_eq!(out, expected, "chunked multi-kb body stream must exact");
}

#[test]
fn m2_error_http_body_stream_read_chunk_invalid_handle_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_http_body_stream_read_chunk(999999, 1)
}
"#,
    );
    // Wave A W117 / ADR-461 / A-03: HttpBodyStream readChunk invalid-handle Display product exact.
    // maxBytes must be >= 1 (maxBytes==0 skips handle check).
    assert_eq!(
        err, "[R003] HttpBodyStream invalid handle 999999",
        "HttpBodyStream readChunk invalid-handle must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_close_invalid_handle_display_throws() {
    // Wave A W174 / ADR-518 / A-03 — HttpBodyStream.close invalid-handle facade Display.
    let err = run_err(&net_facade_src(
        r#"
func main() {
  HttpBodyStream.close(handle: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] HttpBodyStream close failed",
        "close invalid-handle Display must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_close_after_close_display_throws() {
    // Wave A W174 / ADR-518 / A-03 — HttpBodyStream close-after-close facade Display.
    let (port, stop, server, _) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let err = run_err(&net_facade_src(&format!(
        r#"
func main() {{
  let opened = HttpBodyStream.open(url: "http://127.0.0.1:{port}/x")
  let h = opened.handle
  HttpBodyStream.close(handle: h)
  HttpBodyStream.close(handle: h)
}}
"#
    )));
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();
    assert_eq!(
        err, "[R007] HttpBodyStream close failed",
        "close-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_open_opts_not_object_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_http_body_stream_open_opts("http://127.0.0.1", 0)
}
"#,
    );
    // Wave A W125 / ADR-469 / A-03: HttpBodyStream openWithOptions options-not-object Display product exact.
    assert_eq!(
        err, "[R003] HTTP request options must be an object",
        "HttpBodyStream openWithOptions options-not-object must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_open_opts_invalid_method_throws() {
    let err = run_err(
        r#"
func main() {
  let opts = __syx_json_decode("{\"method\":\"BAD METHOD\"}")
  let _ = __syx_http_body_stream_open_opts("http://127.0.0.1", opts)
}
"#,
    );
    // Wave A W125 / ADR-469 / A-03: HttpBodyStream openWithOptions invalid-method Display product exact.
    // Note: "ZZZ" is a valid extension method in reqwest; use token-invalid "BAD METHOD".
    assert_eq!(
        err, "[R003] Invalid HTTP method 'BAD METHOD'",
        "HttpBodyStream openWithOptions invalid-method must exact: {err}"
    );
}

#[test]
fn m2_s4_04_docs_and_capability() {
    let facade = include_str!("../../../../packages/net/http_body_stream.sy");
    assert!(
        facade.contains("func open")
            && facade.contains("readChunk")
            && facade.contains("__syx_http_body_stream_open")
            && (facade.contains("func close(handle: Int) throws")
                || facade.contains("close(handle: Int) throws")),
        "HttpBodyStream facade required (close throws after W174)"
    );

    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        http.contains("HTTP_BODY_STREAM_SESSIONS")
            && http.contains("std::io::Read::read")
            && http.contains("host_http_body_stream_open"),
        "host must keep Response session and Read chunks"
    );
    // Stream open path must not call response.bytes() — only execute_http_request may.
    let stream_open = http
        .split("host_http_body_stream_open")
        .nth(1)
        .and_then(|s| s.split("host_http_body_stream_close").next())
        .unwrap_or("");
    assert!(
        !stream_open.contains(".bytes()"),
        "body stream path must not call response.bytes()"
    );

    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("HttpBodyStream") && guide.contains("ADR-239"),
        "UsageGuide must document HttpBodyStream / ADR-239"
    );
    let net_docs = include_str!("../../../../docs/11_NetworkStdlib.md");
    assert!(
        net_docs.contains("HttpBodyStream")
            && net_docs.contains("M2-S4-04")
            && net_docs.contains("JSONStream"),
        "NetworkStdlib must claim body stream and fence JSONStream"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.stream");
    assert!(
        block.contains("status = \"implemented\""),
        "net.http.stream must be implemented: {block}"
    );
    assert!(
        block.contains("native = \"real\""),
        "net.http.stream must declare native=real (ADR-601): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.http.stream must declare wasm=real (ADR-622): {block}"
    );
    assert!(
        (block.contains("ADR-601") || block.contains("ADR-239")) && block.contains("W257"),
        "net.http.stream notes must cite ADR-601+239: {block}"
    );
    assert!(
        block.contains("ADR-622") && block.contains("W278"),
        "net.http.stream notes must cite ADR-622+W278: {block}"
    );
    assert!(
        block.contains("m2_s4_04_http_body_stream"),
        "tests must list nail: {block}"
    );

    let http_cap = product_block(caps, "net.http");
    assert!(
        http_cap.contains("partial") || http_cap.contains("status = \"partial\""),
        "net.http must stay partial"
    );
    let server = product_block(caps, "net.http.server");
    assert!(
        server.contains("status = \"partial\""),
        "net.http.server must stay partial"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-239"),
        "inventory must note signed-ADR-239"
    );
}

#[test]
fn m2_s4_04_fence_no_server_stream_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S4-04")
            && roadmap.contains("ADR-239")
            && roadmap.contains("M2-S3-03"),
        "roadmap must list M2-S4-04 / ADR-239 and lock next W96 = M2-S3-03"
    );
    assert!(
        roadmap.contains("**M2 已退出**")
            || roadmap.contains("ADR-259")
            || roadmap.contains("| **exited** |"),
        "W115 may claim M2 exited"
    );
    assert!(
        roadmap.contains("S4 叶子收口") || roadmap.contains("S4 收口"),
        "roadmap must note S4 leaf closeout"
    );
}

#[test]
fn m2_s4_04_http_body_stream_native_llvm() {
    let (port, stop, server, _) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let src = format!(
        r#"
func main() {{
  let opened = __syx_http_body_stream_open("http://127.0.0.1:{port}/big")
  let h = opened.handle
  var n = 0
  var chunk = __syx_http_body_stream_read_chunk(h, 1024)
  while chunk.count > 0 {{
    n = n + 1
    chunk = __syx_http_body_stream_read_chunk(h, 1024)
  }}
  __syx_http_body_stream_close(h)
  print(n)
}}
"#
    );
    let (code, stdout) = run_native_strict(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "4",
        "native llvm HttpBodyStream open+readChunk×4+close must exact: {out}"
    );
}

#[test]
fn m2_s4_04_http_body_stream_open_opts_native_llvm() {
    let (port, stop, server, _) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let opened = __syx_http_body_stream_open_opts("http://127.0.0.1:{port}/big", opts)
  __syx_http_body_stream_close(opened.handle)
  print("OK")
}}
"#
    );
    let (code, stdout) = run_native_strict(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm open_opts exit {code}: {stdout}");
    assert_eq!(
        out, "OK",
        "native llvm HttpBodyStream open_opts+close must exact: {out}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s4_04_http_body_stream_wasm() {
    // Wave A W271 / ADR-615 — BodyStream Open/ReadChunk/Close wasm=real
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, server, _) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let src = format!(
        r#"
func main() {{
  let opened = __syx_http_body_stream_open("http://127.0.0.1:{port}/big")
  let h = opened.handle
  var n = 0
  var chunk = __syx_http_body_stream_read_chunk(h, 1024)
  while chunk.count > 0 {{
    n = n + 1
    chunk = __syx_http_body_stream_read_chunk(h, 1024)
  }}
  __syx_http_body_stream_close(h)
  print(n)
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_w271_http_body_stream_wasm_{}",
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
    let _ = server.join();
    let (_, lines) = result.unwrap_or_else(|e| panic!("wasm body stream failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let out = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        out, "4",
        "wasm HttpBodyStream open+readChunk×4+close must exact: {out}"
    );
}

#[cfg(unix)]
#[test]
fn m2_s4_04_http_body_stream_open_opts_wasm() {
    // Wave A W271 / ADR-615 — BodyStream OpenOpts wasm=real
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let (port, stop, server, _) = start_big_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let opened = __syx_http_body_stream_open_opts("http://127.0.0.1:{port}/big", opts)
  __syx_http_body_stream_close(opened.handle)
  print("OK")
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_w271_http_body_stream_opts_wasm_{}",
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
    let _ = server.join();
    let (_, lines) = result.unwrap_or_else(|e| panic!("wasm body stream opts failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let out = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        out, "OK",
        "wasm HttpBodyStream open_opts+close must exact: {out}"
    );
}

#[test]
fn m2_s4_04_http_body_stream_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "HttpBodyStreamOpen",
        "HttpBodyStreamOpenOpts",
        "HttpBodyStreamReadChunk",
        "HttpBodyStreamClose",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-586/ADR-601): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-615): {block}"
        );
        assert!(
            block.contains("ADR-615"),
            "{name} notes must cite ADR-615: {block}"
        );
    }
}
