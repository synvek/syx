//! 01 HTTP Request/Response/Headers/Body/URL named types.
//! net.http.types native/wasm product nails (Http.response + Url.parse).
//! HttpResponseOk/Header/Headers/Json HostFn llvm=real.
//! Http types×7 + HttpGet HostFn wasm=real.
// was: m2_s4_01_http_types

use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

fn run_ok(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded(src, &DriverOptions::default()).expect("run");
    r.printed.join("\n")
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w216_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_native_mir_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
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

/// Embed does not auto-load `import Net`; prepend named types + Http/Url facades (W90 pattern).
fn net_facade_src(body: &str) -> String {
    let parts = [
        include_str!("../../../../packages/io/types/http_response.sy"),
        include_str!("../../../../packages/net/http_types.sy"),
        include_str!("../../../../packages/net/mod.sy"),
        include_str!("../../../../packages/net/url.sy"),
    ]
    .join("\n");
    format!("{parts}\n{body}")
}

const MAKE_RESPONSE_BODY: &str = r#"
func main() {
    let resp = Http.response(status: 200, body: "ok")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}
"#;

const URL_PARSE_BODY: &str = r#"
func main() {
    let u = Url.parse(text: "https://example.com/a?q=1")
    print(u.host)
    print(u.path)
    print(u.query)
}
"#;

fn start_health_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
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

fn http_method_return_is_named(src: &str, method: &str) -> bool {
    let needle = format!("func {method}(");
    let Some(rest) = src.split(&needle).nth(1) else {
        return false;
    };
    let Some(sig) = rest.split('{').next() else {
        return false;
    };
    sig.contains("-> HTTPResponse") && !sig.contains("-> Object")
}

#[test]
fn m2_s4_01_facade_returns_http_response_not_object() {
    let net = include_str!("../../../../packages/net/mod.sy");
    for m in ["get", "post", "fetch", "request", "response"] {
        assert!(
            http_method_return_is_named(net, m),
            "Http.{m} must return HTTPResponse (not Object):\n{net}"
        );
    }
}

#[test]
fn m2_s4_01_url_parse_returns_url_not_object() {
    let url = include_str!("../../../../packages/net/url.sy");
    assert!(
        url.contains("func parse(text: String) -> Url"),
        "Url.parse must return Url: {url}"
    );
    assert!(
        !url.contains("-> Object"),
        "Url.parse must not return Object: {url}"
    );
}

#[test]
fn m2_s4_01_named_type_files_exist() {
    let http_resp = include_str!("../../../../packages/io/types/http_response.sy");
    assert!(
        http_resp.contains("class HTTPResponse"),
        "HTTPResponse type required: {http_resp}"
    );
    let types = include_str!("../../../../packages/net/http_types.sy");
    assert!(
        types.contains("HttpRequest")
            && types.contains("HttpHeaders")
            && types.contains("HttpBody"),
        "HttpRequest/Headers/Body required: {types}"
    );
    let url = include_str!("../../../../packages/net/url.sy");
    assert!(url.contains("class Url"), "Url class required: {url}");
}

#[test]
fn m2_s4_01_make_response_accessors() {
    // Wave A W64 / ADR-408 / A-03: Http.response accessors product exact (no loopback).
    let out = run_ok(&net_facade_src(MAKE_RESPONSE_BODY));
    assert_eq!(out, "200\nok\ntrue", "Http.response accessors must exact: {out}");
}

#[test]
fn m2_s4_01_loopback_http_get_accessors() {
    // Wave A W66 / ADR-410 / A-03: loopback Http.get accessors product exact.
    let (port, stop, handle) = start_health_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    let resp = Http.get(url: "http://127.0.0.1:{port}/health")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}}
"#
    ));
    let out = run_ok(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    assert_eq!(out, "200\nok\ntrue", "loopback Http.get accessors must exact: {out}");
}

#[test]
fn m2_s4_01_url_parse_fields() {
    // Wave A W65 / ADR-409 / A-03: Url.parse host/path/query product exact (no loopback).
    let out = run_ok(&net_facade_src(URL_PARSE_BODY));
    assert_eq!(out, "example.com\n/a\nq=1", "Url.parse fields must exact: {out}");
}

#[test]
fn m2_s4_01_make_response_accessors_native_mir() {
    let out = lines_joined(&run_native_mir_src(
        "resp_mir",
        &net_facade_src(MAKE_RESPONSE_BODY),
    ));
    assert_eq!(out, "200\nok\ntrue", "Http.response mir must exact: {out}");
}

#[test]
fn m2_s4_01_make_response_accessors_wasm() {
    let out = lines_joined(&run_wasm_src(
        "resp_wasm",
        &net_facade_src(MAKE_RESPONSE_BODY),
    ));
    assert_eq!(out, "200\nok\ntrue", "Http.response wasm must exact: {out}");
}

#[test]
fn m2_s4_01_url_parse_fields_native_mir() {
    let out = lines_joined(&run_native_mir_src(
        "url_mir",
        &net_facade_src(URL_PARSE_BODY),
    ));
    assert_eq!(out, "example.com\n/a\nq=1", "Url.parse mir must exact: {out}");
}

#[test]
fn m2_s4_01_url_parse_fields_wasm() {
    let out = lines_joined(&run_wasm_src(
        "url_wasm",
        &net_facade_src(URL_PARSE_BODY),
    ));
    assert_eq!(out, "example.com\n/a\nq=1", "Url.parse wasm must exact: {out}");
}

#[test]
fn m2_s4_01_http_types_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.types");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "net.http.types native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "net.http.types mir must stay real: {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "net.http.types status must stay implemented: {block}"
    );
    assert!(
        block.contains("ADR-560"),
        "notes must cite ADR-560: {block}"
    );
    assert!(
        block.contains("m2_s4_01_make_response_accessors_native_mir"),
        "tests must list response native mir nail: {block}"
    );
}

#[test]
fn m2_s4_01_docs_and_capability() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-236")
            && usage.contains("HTTPResponse")
            && usage.contains("M2-S4-01"),
        "UsageGuide must document named HTTP types / ADR-236"
    );
    assert!(
        usage.contains("M2-S4-05") || usage.contains("EC-M2-4-Z") || usage.contains("streaming"),
        "UsageGuide must fence S4-05 / streaming / EC-M2-4-Z"
    );

    let net = include_str!("../../../../docs/11_NetworkStdlib.md");
    assert!(
        net.contains("ADR-236") && net.contains("HTTPResponse") && net.contains("Url.parse"),
        "NetworkStdlib must document named types / ADR-236"
    );
    assert!(
        net.contains("M2-S4-05") || net.contains("streaming") || net.contains("EC-M2-4-Z"),
        "NetworkStdlib must fence S4-05 / streaming / EC-M2-4-Z"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.types");
    assert!(
        block.contains("status = \"implemented\""),
        "net.http.types must be implemented: {block}"
    );
    assert!(
        block.contains("ADR-560") || block.contains("ADR-236") || block.contains("M2-S4-01"),
        "notes must cite ADR-560 or ADR-236: {block}"
    );
    assert!(
        block.contains("m2_s4_01_http_types"),
        "tests must list nail: {block}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-236") || inv.contains("ADR-236"),
        "inventory must note ADR-236"
    );

    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("HostFnId::UrlParse => \"Url\""),
        "registry UrlParse must return Url"
    );
    assert!(
        reg.contains("HostFnId::HttpMakeResponse => \"HTTPResponse\""),
        "registry HttpMakeResponse must return HTTPResponse"
    );
}

#[test]
fn m2_s4_01_fence_no_s4_05_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S4-01")
            && roadmap.contains("ADR-236")
            && roadmap.contains("M2-S4-05"),
        "roadmap must list M2-S4-01 / ADR-236 and lock next W93 = M2-S4-05"
    );
    assert!(
        roadmap.contains("**M2 已退出**")
            || roadmap.contains("ADR-259")
            || roadmap.contains("| **exited** |"),
        "W115 may claim M2 exited"
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

/// Wave A W249 / ADR-593: Response Ok/Header/Headers/Json under SYX_NATIVE_STRICT.
#[cfg(unix)]
#[test]
fn m2_s4_01_response_accessors_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let r = __syx_http_make_response(200, "{\"n\":42}")
  print(__syx_http_response_ok(r))
  print(__syx_http_response_header(r, "Content-Type"))
  let hs = __syx_http_response_headers(r)
  print(hs)
  let j = __syx_http_response_json(r)
  print(j.n)
  let opts = __syx_json_decode("{\"status\":200,\"headers\":{\"Content-Type\":\"application/json\"}}")
  let body = __syx_json_decode("{\"n\":42}")
  let r2 = __syx_http_make_response(opts, body)
  print(__syx_http_response_header(r2, "Content-Type"))
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_w249_http_response_llvm_{}",
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
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        output.status.code().unwrap_or(1),
        0,
        "native llvm exit: {stdout}"
    );
    assert_eq!(
        out, "true\nnil\nObject({})\n42\napplication/json",
        "native llvm Response accessors must exact: {out}"
    );
}

#[test]
fn m2_s4_01_response_accessors_header_json_wasm() {
    // Wave A W270 / ADR-614 — Header/Headers/Json HostFn wasm=real (mirror W249 llvm bare hosts).
    let src = r#"
func main() {
  let r = __syx_http_make_response(200, "{\"n\":42}")
  print(__syx_http_response_ok(r))
  print(__syx_http_response_header(r, "Content-Type"))
  let hs = __syx_http_response_headers(r)
  print(hs)
  let j = __syx_http_response_json(r)
  print(j.n)
  let opts = __syx_json_decode("{\"status\":200,\"headers\":{\"Content-Type\":\"application/json\"}}")
  let body = __syx_json_decode("{\"n\":42}")
  let r2 = __syx_http_make_response(opts, body)
  print(__syx_http_response_header(r2, "Content-Type"))
  return 0
}
"#;
    let out = lines_joined(&run_wasm_src("resp_hdr_json_wasm", src));
    // wasm Debug for empty Object prints `{...}` (native llvm prints `Object({})`).
    assert_eq!(
        out, "true\nnil\n{...}\n42\napplication/json",
        "wasm Response accessors Header/Headers/Json must exact: {out}"
    );
}

#[test]
fn m2_s4_01_http_response_accessor_hosts_llvm_real() {
    // Wave A W249 / ADR-593 llvm + Wave A W270 / ADR-614 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "HttpResponseOk",
        "HttpResponseHeader",
        "HttpResponseHeaders",
        "HttpResponseJson",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-593): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-614): {block}"
        );
        assert!(
            block.contains("ADR-614") && block.contains("ADR-593"),
            "{name} notes must cite ADR-614 + ADR-593: {block}"
        );
    }
}

#[test]
fn m2_s4_01_http_types_hosts_wasm_real_w270() {
    // Wave A W270 / ADR-614 — Http types×7 + HttpGet wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "HttpMakeResponse",
        "HttpResponseStatus",
        "HttpResponseText",
        "HttpResponseOk",
        "HttpResponseHeader",
        "HttpResponseHeaders",
        "HttpResponseJson",
        "HttpGet",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-614): {block}"
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
            block.contains("ADR-614"),
            "{name} notes must cite ADR-614: {block}"
        );
    }
}
