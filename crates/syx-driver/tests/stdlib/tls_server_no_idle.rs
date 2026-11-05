//! TLS no-idle first-slice
//! SYX_TLS_SERVE_NO_IDLE + optional MAX_HANDLED; default idle unchanged; ≠ language noIdle / ≠ daemon GA.

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
use syx_driver::{build_native_executable, DriverOptions};

fn utf8_prefix_slice(s: &str, start: usize, want: usize) -> &str {
    let mut end = (start + want).min(s.len());
    while end > start && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[start..end]
}

fn tls_client_appdata_probe(
    port: u16,
    client_cfg: Arc<ClientConfig>,
    server_name: ServerName<'static>,
    deadline: Duration,
    handled: Arc<AtomicBool>,
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
                let mut tls = rustls::Stream::new(&mut conn, &mut stream);
                if tls.write_all(b"ping\n").is_ok() {
                    let _ = tls.flush();
                    let mut buf = [0u8; 64];
                    if let Ok(n) = tls.read(&mut buf) {
                        if n > 0 {
                            let s = String::from_utf8_lossy(&buf[..n]);
                            if s.contains("HANDLED") {
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

/// W457 nail: NO_IDLE + MAX_HANDLED=2 — after first HANDLED, wait >400ms, still accept 2nd.
#[cfg(unix)]
#[test]
fn tls_server_no_idle_native_cross_idle() {
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
        "syx_tls_no_idle_{}",
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
    let mut cfg = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    cfg.alpn_protocols = vec![b"syx/1".to_vec()];
    let client_cfg = Arc::new(cfg);
    let server_name = ServerName::try_from("localhost").expect("server name");
    let handled1 = Arc::new(AtomicBool::new(false));
    let handled2 = Arc::new(AtomicBool::new(false));

    let child = Command::new(&exe)
        .env("SYX_TLS_SERVE_NO_IDLE", "1")
        .env("SYX_TLS_SERVE_MAX_HANDLED", "2")
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
    );
    assert!(
        handled1.load(Ordering::SeqCst),
        "first connection must HANDLED under NO_IDLE"
    );
    // Cross the default idle window (400ms) — server must still accept.
    thread::sleep(Duration::from_millis(550));
    tls_client_appdata_probe(
        port,
        client_cfg,
        server_name,
        Duration::from_secs(15),
        Arc::clone(&handled2),
    );
    let output = child.wait_with_output().expect("wait native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "native no-idle tls serve exit {code}: {stdout}");
    assert!(
        handled2.load(Ordering::SeqCst),
        "second connection after >400ms idle gap must HANDLED under NO_IDLE+MAX_HANDLED=2"
    );
    assert!(
        stdout.contains("HANDSHAKED") && stdout.contains("CLOSED"),
        "server must return after MAX_HANDLED=2: {stdout}"
    );
}
