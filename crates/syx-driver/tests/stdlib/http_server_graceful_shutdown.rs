//! 03b server graceful shutdown nail.
//! HttpServe + HttpShutdown HostFn llvm=real (native AOT).
//! HttpServe + HttpShutdown HostFn wasm=real (Http 残余收官).
// was: m2_s4_03b_server_graceful_shutdown

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{
    build_native_executable, run_file_printed, run_wasm_source_printed, DriverOptions,
};

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind ephemeral")
        .local_addr()
        .expect("addr")
        .port()
}

fn wait_listening(port: u16, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }
    false
}

fn http_get(port: u16, path: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).expect("write request");
    let mut buf = Vec::new();
    let _ = stream.read_to_end(&mut buf);
    let text = String::from_utf8_lossy(&buf).into_owned();
    let status = text
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status, text)
}

/// Split raw HTTP response; return payload after headers (trim trailing CR/LF).
fn response_body(raw: &str) -> &str {
    let payload = raw
        .split("\r\n\r\n")
        .nth(1)
        .or_else(|| raw.split("\n\n").nth(1))
        .unwrap_or("");
    payload.trim_end_matches(['\r', '\n'])
}

fn product_block(toml: &str, id: &str) -> String {
    let marker = format!("id = \"{id}\"");
    let start = toml.find(&marker).expect("capability id");
    let rest = &toml[start..];
    let end = rest.find("\n[[capability]]").unwrap_or(rest.len());
    rest[..end].to_string()
}

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn serve_source(port: u16) -> String {
    format!(
        r#"import Net

func handle(req: Object) -> Object {{
    let path = req.path
    if path == "/health" {{
        return Http.response(200, "ok")
    }}
    if path == "/shutdown" {{
        Http.shutdown()
        return Http.response(200, "bye")
    }}
    return Http.response(404, "not found")
}}

func main() {{
    print("listening")
    Http.serve("127.0.0.1", {port}, "handle")
    print("stopped")
}}
"#
    )
}

#[test]
fn m2_s4_03b_graceful_shutdown_loopback() {
    // Wave A W67 / ADR-411 / A-03: Http.shutdown product exact (listening/stopped).
    // Must NOT use SYX_HTTP_SERVE_MAX_REQUESTS — that is ADR-177 count-exit, not graceful.
    assert!(
        std::env::var("SYX_HTTP_SERVE_MAX_REQUESTS").is_err(),
        "nail must run without SYX_HTTP_SERVE_MAX_REQUESTS"
    );

    let port = free_port();
    let src = serve_source(port);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_nail_graceful");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("graceful_{port}.sy"));
    std::fs::write(&path, src).expect("write temp");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let handle = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "server did not listen on {port}"
    );

    let (status, body) = http_get(port, "/health");
    assert_eq!(status, 200, "health status: {body}");
    // Wave A W98 / ADR-442 / A-03: graceful health/shutdown HTTP body product exact.
    assert_eq!(response_body(&body), "ok", "health body must exact: {body}");

    let (sh_status, sh_body) = http_get(port, "/shutdown");
    assert_eq!(sh_status, 200, "shutdown status: {sh_body}");
    assert_eq!(
        response_body(&sh_body),
        "bye",
        "shutdown body must exact: {sh_body}"
    );

    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve thread must join after Http.shutdown")
        .expect("run_file_printed");
    let _ = handle.join();
    let (code, printed) = run_result;
    let out = printed.join("\n");
    assert_eq!(code, 0, "serve must exit cleanly: {out}");
    assert_eq!(out, "listening\nstopped", "graceful shutdown stdout must exact: {out}");

    // Post-shutdown: connect should fail (socket closed).
    let refused = TcpStream::connect(("127.0.0.1", port)).is_err();
    assert!(refused, "post-shutdown connect must fail on port {port}");

    let _ = std::fs::remove_file(&path);
}

#[test]
fn m2_s4_03b_docs_and_capability() {
    let net = include_str!("../../../../packages/net/mod.sy");
    assert!(
        net.contains("func shutdown") && net.contains("__syx_http_serve_shutdown"),
        "Http.shutdown facade required"
    );

    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        http.contains("ACTIVE_HTTP_SHUTDOWN")
            && (http.contains("watch::channel") || http.contains("shutdown_tx")),
        "host must register watch cancel for Http.shutdown (W1176 hyper path)"
    );
    assert!(
        http.contains("≠ ADR-238") || http.contains("≠ graceful"),
        "MAX_REQUESTS must remain distinct from graceful"
    );

    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("Http.shutdown") && guide.contains("ADR-238"),
        "UsageGuide must document Http.shutdown / ADR-238"
    );
    let net_docs = include_str!("../../../../docs/11_NetworkStdlib.md");
    assert!(
        net_docs.contains("Http.shutdown") && net_docs.contains("M2-S4-03b"),
        "NetworkStdlib must claim graceful shutdown"
    );
    assert!(
        net_docs.contains("指望")
            && net_docs.contains("__syx_task_cancel")
            && net_docs.contains("Http.shutdown"),
        "NetworkStdlib must correct task_cancel misconception"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.server");
    assert!(
        block.contains("status = \"partial\""),
        "net.http.server must remain partial: {block}"
    );
    assert!(
        block.contains("m2_s4_03b_server_graceful_shutdown"),
        "tests must list nail: {block}"
    );
    assert!(
        block.contains("ADR-238") || block.contains("M2-S4-03b"),
        "notes must cite ADR-238: {block}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-238"),
        "inventory must note signed-ADR-238"
    );
}

#[test]
fn m2_s4_03b_fence_no_streaming_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S4-03b")
            && roadmap.contains("ADR-238")
            && roadmap.contains("M2-S4-04"),
        "roadmap must list M2-S4-03b / ADR-238 and lock next W95 = M2-S4-04"
    );
    assert!(
        roadmap.contains("**M2 已退出**")
            || roadmap.contains("ADR-259")
            || roadmap.contains("| **exited** |"),
        "W115 may claim M2 exited"
    );
    let tls = product_block(
        include_str!("../../../../packages/internal/capabilities.toml"),
        "net.tls.server",
    );
    assert!(
        tls.contains("status = \"implemented\"") && tls.contains("ADR-779"),
        "net.tls.server product status raise is W435/ADR-779 (M2 graceful shutdown deferred server)"
    );
}

#[cfg(unix)]
#[test]
fn m2_s4_03b_graceful_shutdown_native_llvm() {
    // Wave A W248 / ADR-592: HttpServe + HttpShutdown under SYX_NATIVE_STRICT.
    assert!(
        std::env::var("SYX_HTTP_SERVE_MAX_REQUESTS").is_err(),
        "nail must run without SYX_HTTP_SERVE_MAX_REQUESTS"
    );
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");

    let port = free_port();
    let src = serve_source(port);
    let dir = std::env::temp_dir().join(format!(
        "syx_w248_graceful_llvm_{}_{}",
        port,
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

    let (tx, rx) = mpsc::channel();
    let exe_clone = exe.clone();
    let handle = thread::spawn(move || {
        let output = Command::new(&exe_clone).output().expect("run native");
        let _ = tx.send((
            output.status.code().unwrap_or(1),
            String::from_utf8_lossy(&output.stdout).into_owned(),
        ));
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "native llvm server did not listen on {port}"
    );

    let (status, body) = http_get(port, "/health");
    assert_eq!(status, 200, "health status: {body}");
    assert_eq!(response_body(&body), "ok", "health body must exact: {body}");

    let (sh_status, sh_body) = http_get(port, "/shutdown");
    assert_eq!(sh_status, 200, "shutdown status: {sh_body}");
    assert_eq!(
        response_body(&sh_body),
        "bye",
        "shutdown body must exact: {sh_body}"
    );

    let (code, stdout) = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve thread must join after Http.shutdown");
    let _ = handle.join();
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm serve must exit cleanly: {out}");
    assert_eq!(
        out, "listening\nstopped",
        "native llvm graceful shutdown stdout must exact: {out}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn m2_s4_03b_graceful_shutdown_wasm() {
    // Wave A W271 / ADR-615: HttpServe + HttpShutdown wasm=real.
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    assert!(
        std::env::var("SYX_HTTP_SERVE_MAX_REQUESTS").is_err(),
        "nail must run without SYX_HTTP_SERVE_MAX_REQUESTS"
    );

    let port = free_port();
    let src = format!(
        r#"
func handle(req: Object) -> Object {{
    let path = req.path
    if path == "/health" {{
        return __syx_http_make_response(200, "ok")
    }}
    if path == "/shutdown" {{
        __syx_http_serve_shutdown()
        return __syx_http_make_response(200, "bye")
    }}
    return __syx_http_make_response(404, "not found")
}}

func main() {{
    print("listening")
    __syx_http_serve("127.0.0.1", {port}, "handle")
    print("stopped")
}}
"#
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_w271_graceful_wasm_{}_{}",
        port,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");

    let (tx, rx) = mpsc::channel();
    let main_clone = main.clone();
    let handle = thread::spawn(move || {
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        let result = run_wasm_source_printed(&main_clone, "main", &opts);
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(60)),
        "wasm server did not listen on {port}"
    );

    let (status, body) = http_get(port, "/health");
    assert_eq!(status, 200, "health status: {body}");
    assert_eq!(response_body(&body), "ok", "health body must exact: {body}");

    let (sh_status, sh_body) = http_get(port, "/shutdown");
    assert_eq!(sh_status, 200, "shutdown status: {sh_body}");
    assert_eq!(
        response_body(&sh_body),
        "bye",
        "shutdown body must exact: {sh_body}"
    );

    let run_result = rx
        .recv_timeout(Duration::from_secs(60))
        .expect("serve thread must join after Http.shutdown")
        .unwrap_or_else(|e| panic!("wasm serve failed: {e:?}"));
    let _ = handle.join();
    let (code, printed) = run_result;
    let out = printed
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "wasm serve must exit cleanly: {out}");
    assert_eq!(
        out, "listening\nstopped",
        "wasm graceful shutdown stdout must exact: {out}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn m2_s4_03b_http_serve_shutdown_host_llvm_real() {
    // Wave A W248 / ADR-592 llvm + Wave A W271 / ADR-615 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["HttpServe", "HttpShutdown"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-592): {block}"
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
            block.contains("ADR-615") && block.contains("ADR-592"),
            "{name} notes must cite ADR-615 + ADR-592: {block}"
        );
    }
}
