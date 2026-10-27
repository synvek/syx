//! HostFn TlsServe loopback / ALPN / multi-conn nails (native + wasm).

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rustls::pki_types::{CertificateDer, ServerName};
use rustls::ClientConfig;
use syx_driver::{build_native_executable, run_wasm_source_printed, DriverOptions};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn tls_client_appdata_probe(
    port: u16,
    client_cfg: Arc<ClientConfig>,
    server_name: ServerName<'static>,
    deadline: Duration,
    handled: Arc<AtomicBool>,
    expected_alpn: &'static [u8],
) {
    let start = Instant::now();
    while start.elapsed() < deadline {
        if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) {
            let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
            let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
            let mut conn = match rustls::ClientConnection::new(
                Arc::clone(&client_cfg),
                server_name.clone(),
            ) {
                Ok(c) => c,
                Err(_) => {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
            };
            loop {
                if !conn.is_handshaking() {
                    break;
                }
                match conn.complete_io(&mut stream) {
                    Ok((0, 0)) => break,
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
            if !conn.is_handshaking() {
                let alpn_ok = conn.alpn_protocol() == Some(expected_alpn);
                let mut tls = rustls::Stream::new(&mut conn, &mut stream);
                if tls.write_all(b"ping\n").is_ok() {
                    let _ = tls.flush();
                    let mut buf = [0u8; 64];
                    if let Ok(n) = tls.read(&mut buf) {
                        if n > 0 {
                            let s = String::from_utf8_lossy(&buf[..n]);
                            if alpn_ok && s.contains("HANDLED") {
                                handled.store(true, Ordering::SeqCst);
                            }
                        }
                    }
                }
            }
            let _ = stream.shutdown(std::net::Shutdown::Both);
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn client_cfg_with_alpn(
    roots: rustls::RootCertStore,
    protocols: Vec<Vec<u8>>,
) -> Arc<ClientConfig> {
    let mut cfg = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    cfg.alpn_protocols = protocols;
    Arc::new(cfg)
}

/// Wave A W393/W436/W437/W438 — TlsServe llvm + app-data + multi-conn + ALPN syx/1.
#[cfg(unix)]
#[test]
fn hostfn_tls_serve_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    std::env::remove_var("SYX_TLS_SERVE_ALPN");

    let cert = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into(), "localhost".into()])
        .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    let cert_der = CertificateDer::from(cert.cert);

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);

    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tls_llvm_serve_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let cert_path = dir.join("server.crt");
    let key_path = dir.join("server.key");
    fs::write(&cert_path, &cert_pem).expect("write cert");
    fs::write(&key_path, &key_pem).expect("write key");

    let cert_path_s = cert_path.to_str().expect("utf8 cert").replace('\\', "\\\\");
    let key_path_s = key_path.to_str().expect("utf8 key").replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let r = __syx_tls_serve("127.0.0.1", {port}, "{cert_path_s}", "{key_path_s}", "", 0)
  if r < 0 {{
    print("FAIL")
    return 1
  }}
  print("HANDSHAKED")
  print("CLOSED")
  return 0
}}
"#
    );
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(cert_der)
        .expect("add self-signed root for loopback client");
    let client_cfg = client_cfg_with_alpn(roots, vec![b"syx/1".to_vec()]);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled1 = Arc::new(AtomicBool::new(false));
    let handled2 = Arc::new(AtomicBool::new(false));
    // Spawn server first, then sequential probes (≥2) against idle-timeout accept (ADR-786).
    let child = Command::new(&exe)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn native");
    thread::sleep(Duration::from_millis(80));
    tls_client_appdata_probe(
        port,
        Arc::clone(&client_cfg),
        server_name.clone(),
        Duration::from_secs(15),
        Arc::clone(&handled1),
        b"syx/1",
    );
    tls_client_appdata_probe(
        port,
        client_cfg,
        server_name,
        Duration::from_secs(15),
        Arc::clone(&handled2),
        b"syx/1",
    );
    let output = child.wait_with_output().expect("wait native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native llvm tls serve exit {code}: {stdout}");
    assert_eq!(
        lines,
        vec!["HANDSHAKED".to_string(), "CLOSED".to_string()],
        "TlsServe llvm must exact HANDSHAKED/CLOSED: {lines:?}"
    );
    assert!(
        handled1.load(Ordering::SeqCst) && handled2.load(Ordering::SeqCst),
        "TlsServe llvm ALPN syx/1 + HANDLED required for both clients (ADR-782)"
    );
}

/// Wave A W394/W436/W437/W438 — TlsServe wasm + app-data + multi-conn + ALPN syx/1.
#[test]
fn hostfn_tls_serve_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    std::env::remove_var("SYX_TLS_SERVE_ALPN");

    let cert = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into(), "localhost".into()])
        .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    let cert_der = CertificateDer::from(cert.cert);

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);

    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tls_wasm_serve_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let cert_path = dir.join("server.crt");
    let key_path = dir.join("server.key");
    fs::write(&cert_path, &cert_pem).expect("write cert");
    fs::write(&key_path, &key_pem).expect("write key");

    let cert_path_s = cert_path.to_str().expect("utf8 cert").replace('\\', "\\\\");
    let key_path_s = key_path.to_str().expect("utf8 key").replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let r = __syx_tls_serve("127.0.0.1", {port}, "{cert_path_s}", "{key_path_s}", "", 0)
  if r < 0 {{
    print("FAIL")
    return 1
  }}
  print("HANDSHAKED")
  print("CLOSED")
  return 0
}}
"#
    );
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(cert_der)
        .expect("add self-signed root for loopback client");
    let client_cfg = client_cfg_with_alpn(roots, vec![b"syx/1".to_vec()]);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled1 = Arc::new(AtomicBool::new(false));
    let handled2 = Arc::new(AtomicBool::new(false));
    let cfg_seq = Arc::clone(&client_cfg);
    let name_seq = server_name.clone();
    let h1 = Arc::clone(&handled1);
    let h2 = Arc::clone(&handled2);

    // Sequential probes (≥2) against idle-timeout unbounded accept (ADR-786).
    let join = thread::spawn(move || {
        tls_client_appdata_probe(
            port,
            Arc::clone(&cfg_seq),
            name_seq.clone(),
            Duration::from_secs(120),
            h1,
            b"syx/1",
        );
        tls_client_appdata_probe(port, cfg_seq, name_seq, Duration::from_secs(120), h2, b"syx/1");
    });

    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm tls serve failed: {e:?}"));
    let _ = join.join();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["HANDSHAKED".to_string(), "CLOSED".to_string()],
        "TlsServe wasm must exact HANDSHAKED/CLOSED: {lines:?}"
    );
    assert!(
        handled1.load(Ordering::SeqCst) && handled2.load(Ordering::SeqCst),
        "TlsServe wasm ALPN syx/1 + HANDLED required for both clients (ADR-782)"
    );
}

/// Wave A W443 / ADR-787 — env configurable ALPN (server list includes http/1.1; client picks it).
#[cfg(unix)]
#[test]
fn hostfn_tls_serve_native_llvm_alpn_config() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    std::env::set_var("SYX_TLS_SERVE_ALPN", "syx/1,http/1.1");

    let cert = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into(), "localhost".into()])
        .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    let cert_der = CertificateDer::from(cert.cert);

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);

    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tls_llvm_alpn_cfg_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let cert_path = dir.join("server.crt");
    let key_path = dir.join("server.key");
    fs::write(&cert_path, &cert_pem).expect("write cert");
    fs::write(&key_path, &key_pem).expect("write key");

    let cert_path_s = cert_path.to_str().expect("utf8 cert").replace('\\', "\\\\");
    let key_path_s = key_path.to_str().expect("utf8 key").replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let r = __syx_tls_serve("127.0.0.1", {port}, "{cert_path_s}", "{key_path_s}", "", 0)
  if r < 0 {{
    print("FAIL")
    return 1
  }}
  print("HANDSHAKED")
  print("CLOSED")
  return 0
}}
"#
    );
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(cert_der)
        .expect("add self-signed root for loopback client");
    // Client prefers http/1.1 only — must negotiate from server env list (≠ hardcoded syx/1-only).
    let client_cfg = client_cfg_with_alpn(roots, vec![b"http/1.1".to_vec()]);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled = Arc::new(AtomicBool::new(false));
    let child = Command::new(&exe)
        .env("SYX_TLS_SERVE_ALPN", "syx/1,http/1.1")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn native");
    thread::sleep(Duration::from_millis(80));
    tls_client_appdata_probe(
        port,
        client_cfg,
        server_name,
        Duration::from_secs(15),
        Arc::clone(&handled),
        b"http/1.1",
    );
    let output = child.wait_with_output().expect("wait native");
    std::env::remove_var("SYX_TLS_SERVE_ALPN");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native llvm alpn-config exit {code}: {stdout}");
    assert!(
        handled.load(Ordering::SeqCst),
        "TlsServe llvm must negotiate http/1.1 from SYX_TLS_SERVE_ALPN (ADR-787)"
    );
}

/// Wave A W444 / ADR-788 — overlapping dual-client concurrent HANDLED nail.
#[cfg(unix)]
#[test]
fn hostfn_tls_serve_native_llvm_concurrent() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    std::env::remove_var("SYX_TLS_SERVE_ALPN");

    let cert = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into(), "localhost".into()])
        .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    let cert_der = CertificateDer::from(cert.cert);

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);

    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tls_llvm_concurrent_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let cert_path = dir.join("server.crt");
    let key_path = dir.join("server.key");
    fs::write(&cert_path, &cert_pem).expect("write cert");
    fs::write(&key_path, &key_pem).expect("write key");

    let cert_path_s = cert_path.to_str().expect("utf8 cert").replace('\\', "\\\\");
    let key_path_s = key_path.to_str().expect("utf8 key").replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let r = __syx_tls_serve("127.0.0.1", {port}, "{cert_path_s}", "{key_path_s}", "", 0)
  if r < 0 {{
    print("FAIL")
    return 1
  }}
  print("HANDSHAKED")
  print("CLOSED")
  return 0
}}
"#
    );
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(cert_der)
        .expect("add self-signed root for loopback client");
    let client_cfg = client_cfg_with_alpn(roots, vec![b"syx/1".to_vec()]);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled1 = Arc::new(AtomicBool::new(false));
    let handled2 = Arc::new(AtomicBool::new(false));
    let child = Command::new(&exe)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn native");
    thread::sleep(Duration::from_millis(80));
    // Overlapping dual probes (ADR-788 concurrent) — both connect nearly at once.
    let cfg1 = Arc::clone(&client_cfg);
    let cfg2 = client_cfg;
    let name1 = server_name.clone();
    let name2 = server_name;
    let h1 = Arc::clone(&handled1);
    let h2 = Arc::clone(&handled2);
    let j1 = thread::spawn(move || {
        tls_client_appdata_probe(port, cfg1, name1, Duration::from_secs(15), h1, b"syx/1");
    });
    let j2 = thread::spawn(move || {
        tls_client_appdata_probe(port, cfg2, name2, Duration::from_secs(15), h2, b"syx/1");
    });
    let _ = j1.join();
    let _ = j2.join();
    let output = child.wait_with_output().expect("wait native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native llvm concurrent exit {code}: {stdout}");
    assert_eq!(
        lines,
        vec!["HANDSHAKED".to_string(), "CLOSED".to_string()],
        "TlsServe llvm concurrent must exact HANDSHAKED/CLOSED: {lines:?}"
    );
    assert!(
        handled1.load(Ordering::SeqCst) && handled2.load(Ordering::SeqCst),
        "TlsServe llvm overlapping dual-client HANDLED required (ADR-788)"
    );
}

/// Wave A W445 / ADR-789 — language ALPN 5th arg (no env); client negotiates http/1.1.
#[cfg(unix)]
#[test]
fn hostfn_tls_serve_native_llvm_alpn_lang() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    // Must not rely on env — language arg alone drives ALPN.
    std::env::remove_var("SYX_TLS_SERVE_ALPN");

    let cert = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into(), "localhost".into()])
        .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();
    let cert_der = CertificateDer::from(cert.cert);

    let probe = TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("addr").port();
    drop(probe);

    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_tls_llvm_alpn_lang_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let cert_path = dir.join("server.crt");
    let key_path = dir.join("server.key");
    fs::write(&cert_path, &cert_pem).expect("write cert");
    fs::write(&key_path, &key_pem).expect("write key");

    let cert_path_s = cert_path.to_str().expect("utf8 cert").replace('\\', "\\\\");
    let key_path_s = key_path.to_str().expect("utf8 key").replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let r = __syx_tls_serve("127.0.0.1", {port}, "{cert_path_s}", "{key_path_s}", "syx/1,http/1.1", 0)
  if r < 0 {{
    print("FAIL")
    return 1
  }}
  print("HANDSHAKED")
  print("CLOSED")
  return 0
}}
"#
    );
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(cert_der)
        .expect("add self-signed root for loopback client");
    let client_cfg = client_cfg_with_alpn(roots, vec![b"http/1.1".to_vec()]);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled = Arc::new(AtomicBool::new(false));
    let child = Command::new(&exe)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn native");
    thread::sleep(Duration::from_millis(80));
    tls_client_appdata_probe(
        port,
        client_cfg,
        server_name,
        Duration::from_secs(15),
        Arc::clone(&handled),
        b"http/1.1",
    );
    let output = child.wait_with_output().expect("wait native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native llvm alpn-lang exit {code}: {stdout}");
    assert!(
        handled.load(Ordering::SeqCst),
        "TlsServe llvm must negotiate http/1.1 from language alpn arg (ADR-789)"
    );
}

#[test]
fn hostfn_tls_serve_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "TlsServe");
    assert!(
        block.contains("llvm = \"real\""),
        "TlsServe must declare llvm=real (ADR-737): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "TlsServe must declare wasm=real (ADR-738): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "TlsServe must be status=implemented (ADR-785 / W441): {block}"
    );
    assert!(
        block.contains("ADR-738") && block.contains("ADR-737"),
        "TlsServe notes must cite ADR-738 + ADR-737: {block}"
    );
    assert!(
        block.contains("ADR-782")
            && (block.contains("ALPN") || block.contains("syx/1")),
        "TlsServe notes must cite ADR-782 ALPN first-slice: {block}"
    );
    assert!(
        block.contains("ADR-785"),
        "TlsServe notes must cite ADR-785 HostFn status raise: {block}"
    );
    assert!(
        block.contains("ADR-786")
            && (block.contains("unbounded") || block.contains("idle")),
        "TlsServe notes must cite ADR-786 unbounded first-slice: {block}"
    );
    assert!(
        block.contains("ADR-787")
            && block.contains("SYX_TLS_SERVE_ALPN"),
        "TlsServe notes must cite ADR-787 configurable ALPN: {block}"
    );
    assert!(
        block.contains("ADR-788")
            && (block.contains("concurrent") || block.contains("thread-per-conn")),
        "TlsServe notes must cite ADR-788 concurrent: {block}"
    );
    assert!(
        block.contains("ADR-789")
            && (block.contains("language ALPN")
                || block.contains("5th arg")
                || block.contains("serveOnceAlpn")),
        "TlsServe notes must cite ADR-789 language ALPN: {block}"
    );
}
