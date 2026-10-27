//! 05 TLS client real path (positive HTTPS + capability).
// was: m2_s4_05_client_tls

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rcgen::{
    BasicConstraints, CertificateParams, DnType, IsCa, KeyPair, KeyUsagePurpose, SanType,
};
use std::fs;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
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

fn run_ok(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded(src, &DriverOptions::default()).expect("run");
    r.printed.join("\n")
}

fn run_err(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W95 / ADR-439: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn write_tmp_main(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w282_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    main
}

fn run_wasm_ok(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let main = write_tmp_main(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm ok failed: {e:?}"));
    let _ = fs::remove_dir_all(main.parent().expect("parent"));
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_wasm_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let main = write_tmp_main(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let err = match run_wasm_source_printed(&main, "main", &opts) {
        Ok((0, lines)) => format!("UNEXPECTED_OK:{}", lines.join("\n")),
        Ok((_, lines)) if !lines.is_empty() => lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    };
    let _ = fs::remove_dir_all(main.parent().expect("parent"));
    err
}

fn run_native_aot_ok(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w342_tls_native_ok_{}_{}",
        label,
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
    assert!(
        output.status.success(),
        "native AOT must exit 0: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let _ = fs::remove_dir_all(&dir);
    stdout
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_native_aot_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w342_tls_native_err_{}_{}",
        label,
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
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _ = fs::remove_dir_all(&dir);
    let combined: String = [stdout.as_ref(), stderr.as_ref()]
        .join("\n")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    if combined.starts_with("UNEXPECTED_OK") {
        return combined;
    }
    if !combined.is_empty() {
        return combined;
    }
    if !output.status.success() {
        return format!("[native exit {}]", output.status);
    }
    format!("UNEXPECTED_OK:{stdout}")
}

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

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: nails use --test-threads=1; restore in Drop.
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }

    fn remove(key: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        unsafe { std::env::remove_var(key) };
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.prev {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

/// CA + leaf for 127.0.0.1; returns (ca_pem_path, port, stop, join).
fn start_ca_signed_https() -> (PathBuf, u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let ca_key = KeyPair::generate().expect("ca key");
    let mut ca_params = CertificateParams::default();
    ca_params
        .distinguished_name
        .push(DnType::CommonName, "syx-m2-s4-05-ca");
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    let ca_cert = ca_params.self_signed(&ca_key).expect("ca cert");

    let server_key = KeyPair::generate().expect("server key");
    let mut server_params = CertificateParams::new(vec!["localhost".into()]).expect("server params");
    server_params
        .subject_alt_names
        .push(SanType::IpAddress(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
    let server_cert = server_params
        .signed_by(&server_key, &ca_cert, &ca_key)
        .expect("server cert");

    let ca_path = std::env::temp_dir().join(format!(
        "syx_m2_s4_05_ca_{}.pem",
        std::process::id()
    ));
    std::fs::write(&ca_path, ca_cert.pem()).expect("write ca pem");

    let key_der = PrivatePkcs8KeyDer::from(server_key.serialize_der());
    let cert_der = CertificateDer::from(server_cert.der().to_vec());
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], PrivateKeyDer::Pkcs8(key_der))
        .expect("tls server config");
    let config = Arc::new(config);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind tls loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking tls listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut conn = match rustls::ServerConnection::new(Arc::clone(&config)) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };
                    let mut tls = rustls::Stream::new(&mut conn, &mut stream);
                    let mut buf = [0u8; 1024];
                    let _ = tls.read(&mut buf);
                    let body = b"ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                        body.len()
                    );
                    let _ = tls.write_all(resp.as_bytes());
                    let _ = tls.flush();
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });
    (ca_path, port, stop, handle)
}

/// Self-signed leaf only (must fail without EXTRA_ROOT).
fn start_self_signed_https() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])
        .expect("rcgen self-signed");
    let key_der = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());
    let cert_der = CertificateDer::from(cert.cert);
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], PrivateKeyDer::Pkcs8(key_der))
        .expect("tls server config");
    let config = Arc::new(config);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind tls loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking tls listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut conn = match rustls::ServerConnection::new(Arc::clone(&config)) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };
                    let mut tls = rustls::Stream::new(&mut conn, &mut stream);
                    let mut buf = [0u8; 1024];
                    let _ = tls.read(&mut buf);
                    let body = b"ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                        body.len()
                    );
                    let _ = tls.write_all(resp.as_bytes());
                    let _ = tls.flush();
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
fn m2_s4_05_https_loopback_ok_with_extra_root() {
    let (ca_path, port, stop, join) = start_ca_signed_https();
    let _env = EnvGuard::set(
        "SYX_HTTP_EXTRA_ROOT_PEM",
        ca_path.to_str().expect("utf8 path"),
    );
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    let resp = Http.get(url: "https://127.0.0.1:{port}/health")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}}
"#
    ));
    let out = run_ok(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = std::fs::remove_file(&ca_path);
    // Wave A W87 / ADR-431 / A-03: HTTPS EXTRA_ROOT loopback product exact.
    assert_eq!(out, "200\nok\ntrue", "HTTPS EXTRA_ROOT loopback must exact: {out}");
}

/// Wave A W282 / ADR-626 — EXTRA_ROOT HTTPS product exact on wasm AOT.
#[cfg(unix)]
#[test]
fn m2_s4_05_https_loopback_ok_with_extra_root_wasm() {
    let (ca_path, port, stop, join) = start_ca_signed_https();
    let _env = EnvGuard::set(
        "SYX_HTTP_EXTRA_ROOT_PEM",
        ca_path.to_str().expect("utf8 path"),
    );
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    let resp = Http.get(url: "https://127.0.0.1:{port}/health")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}}
"#
    ));
    let out = run_wasm_ok("https_extra_root", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = std::fs::remove_file(&ca_path);
    assert_eq!(
        out, "200\nok\ntrue",
        "wasm HTTPS EXTRA_ROOT loopback must exact: {out}"
    );
}

/// Wave A W342 / ADR-686 — EXTRA_ROOT HTTPS product exact on native AOT.
#[cfg(unix)]
#[test]
fn m2_s4_05_https_loopback_ok_with_extra_root_native_aot() {
    let (ca_path, port, stop, join) = start_ca_signed_https();
    let _env = EnvGuard::set(
        "SYX_HTTP_EXTRA_ROOT_PEM",
        ca_path.to_str().expect("utf8 path"),
    );
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    let resp = Http.get(url: "https://127.0.0.1:{port}/health")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}}
"#
    ));
    let out = run_native_aot_ok("https_extra_root", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    let _ = std::fs::remove_file(&ca_path);
    assert_eq!(
        out, "200\nok\ntrue",
        "native AOT HTTPS EXTRA_ROOT loopback must exact: {out}"
    );
}

#[test]
fn m2_s4_05_self_signed_still_fail_closed_without_extra_root() {
    let _clear = EnvGuard::remove("SYX_HTTP_EXTRA_ROOT_PEM");
    let (port, stop, join) = start_self_signed_https();
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    print(Http.get(url: "https://127.0.0.1:{port}/"))
}}
"#
    ));
    let err = run_err(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "self-signed must fail without EXTRA_ROOT: {err}"
    );
    // Wave A W95 / ADR-439 / A-03: self-signed without EXTRA_ROOT fail-closed product exact.
    assert_eq!(
        err, "[R003] HTTP request failed",
        "self-signed without EXTRA_ROOT must exact: {err}"
    );
}

/// Wave A W282 / ADR-626 — self-signed fail-closed product exact on wasm AOT.
#[cfg(unix)]
#[test]
fn m2_s4_05_self_signed_fail_closed_wasm() {
    let _clear = EnvGuard::remove("SYX_HTTP_EXTRA_ROOT_PEM");
    let (port, stop, join) = start_self_signed_https();
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
    print(Http.get(url: "https://127.0.0.1:{port}/"))
}}
"#
    ));
    let err = run_wasm_err("self_signed_fail", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "wasm self-signed must fail without EXTRA_ROOT: {err}"
    );
    assert_eq!(
        err, "[R003] HTTP request failed",
        "wasm self-signed without EXTRA_ROOT must exact: {err}"
    );
}

/// Wave A W342 / ADR-686 — self-signed fail-closed product exact on native AOT.
#[cfg(unix)]
#[test]
fn m2_s4_05_self_signed_fail_closed_native_aot() {
    let _clear = EnvGuard::remove("SYX_HTTP_EXTRA_ROOT_PEM");
    let (port, stop, join) = start_self_signed_https();
    thread::sleep(Duration::from_millis(20));
    let src = net_facade_src(&format!(
        r#"
func main() {{
  try {{
    print(__syx_http_get("https://127.0.0.1:{port}/"))
    print("UNEXPECTED_OK")
  }} catch (err) {{
    print("[" + err.code + "] " + err.message)
  }}
}}
"#
    ));
    let err = run_native_aot_err("self_signed_fail", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "native AOT self-signed must fail without EXTRA_ROOT: {err}"
    );
    assert_eq!(
        err, "[R003] HTTP request failed",
        "native AOT self-signed without EXTRA_ROOT must exact: {err}"
    );
}

#[test]
fn m2_s4_05_docs_and_capability() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-237")
            && usage.contains("SYX_HTTP_EXTRA_ROOT_PEM")
            && usage.contains("M2-S4-05"),
        "UsageGuide must document TLS client real path / ADR-237"
    );
    assert!(
        usage.contains("skip-verify") || usage.contains("server TLS") || usage.contains("EC-M2-4-Z"),
        "UsageGuide must fence skip-verify / server TLS / EC-M2-4-Z"
    );

    let net = include_str!("../../../../docs/11_NetworkStdlib.md");
    assert!(
        net.contains("ADR-237") && net.contains("SYX_HTTP_EXTRA_ROOT_PEM"),
        "NetworkStdlib must document EXTRA_ROOT / ADR-237"
    );
    assert!(
        (net.contains("ADR-779") || net.contains("status=implemented") || net.contains("implemented"))
            && (net.contains("TLS server") || net.contains("server TLS")),
        "NetworkStdlib must document TLS server product status raise (ADR-779)"
    );
    assert!(
        net.contains("handler") || net.contains("ALPN") || net.contains("multi-conn"),
        "NetworkStdlib must fence TLS handler/ALPN/multi-conn deepen"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.tls.client");
    assert!(
        block.contains("status = \"implemented\""),
        "net.tls.client must be implemented: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.tls.client must declare wasm=real (ADR-626): {block}"
    );
    assert!(
        block.contains("native = \"real\""),
        "net.tls.client must declare native=real (ADR-687): {block}"
    );
    assert!(
        block.contains("ADR-626") && block.contains("W282"),
        "notes must cite ADR-626+W282: {block}"
    );
    assert!(
        block.contains("ADR-687") && block.contains("W343"),
        "notes must cite ADR-687+W343 native raise: {block}"
    );
    assert!(
        block.contains("ADR-237") || block.contains("M2-S4-05"),
        "notes must cite ADR-237: {block}"
    );
    assert!(
        block.contains("m2_s4_05_client_tls")
            && block.contains("m2_s4_05_https_loopback_ok_with_extra_root_wasm")
            && block.contains("m2_s4_05_https_loopback_ok_with_extra_root_native_aot")
            && block.contains("m2_s4_05_self_signed_fail_closed_native_aot"),
        "tests must list VM+wasm+native AOT nails: {block}"
    );

    let tls_server = product_block(caps, "net.tls.server");
    assert!(
        tls_server.contains("status = \"implemented\"") && tls_server.contains("ADR-779"),
        "net.tls.server product status raise is W435/ADR-779 (M2 W93 deferred server): {tls_server}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-237") || inv.contains("ADR-237"),
        "inventory must note ADR-237"
    );

    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        http.contains("SYX_HTTP_EXTRA_ROOT_PEM") && http.contains("add_root_certificate"),
        "http.rs must append roots via EXTRA_ROOT"
    );
    assert!(
        !http.contains("danger_accept_invalid_certs("),
        "http.rs must not call danger_accept_invalid_certs (skip-verify)"
    );
}

#[test]
fn m2_s4_05_fence_no_server_tls_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S4-05")
            && roadmap.contains("ADR-237")
            && roadmap.contains("M2-S4-03b"),
        "roadmap must list M2-S4-05 / ADR-237 and lock next W94 = M2-S4-03b"
    );
    assert!(
        roadmap.contains("**M2 已退出**")
            || roadmap.contains("ADR-259")
            || roadmap.contains("| **exited** |"),
        "W115 may claim M2 exited"
    );
}
