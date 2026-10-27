//! Http.serve in-process HTTPS first slice.
//! Diagnostic PEM env → rustls terminate → hyper HTTP/1; client trusts via
//! SYX_HTTP_EXTRA_ROOT_PEM.
//! Upgrade-over-TLS / HMR-over-HTTPS first slice — same-port
//! Vite-like Upgrade after rustls terminate; ≠ WSS product GA / frame terminator
//! Playwright / HostFn+1.
//! upgradeTerminate-over-TLS product first cluster — Accept+text
//! RSV→close(1002) / lifecycle / HMR Accept still relayed; ≠ WSS product GA.
//! upgradeTerminate-over-TLS close(1002) product first cluster
//! 0xB / FIN=0 ping / unexpected continuation / unmasked; ≠ WSS product GA.
//! upgradeTerminate-over-TLS close(1002) product second cluster
//! oversized ping / close len=1 / status 1005 / text non-UTF-8; ≠ WSS product GA.
//! upgradeTerminate-over-TLS close(1002) product third cluster
//! close-reason non-UTF-8 / unsolicited pong / status 1004 / frag text non-UTF-8
//! ≠ WSS product GA.
//! upgradeTerminate-over-TLS close(1002) product fourth cluster
//! FIN=0 close / FIN=0 close-during-fragment / reserved 0x3 → TLS close(1002)
//! matrix 16/16 vs plain; ≠ WSS product GA.
//! illegal close-status deepen — TLS close status 1006 → close(1002)
//! ≠ close-code full set GA / ≠ WSS product GA.
//! illegal close-status deepen second cluster — TLS 1015 / 999 →
//! close(1002); ≠ close-code full set GA / ≠ WSS product GA.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use rustls::pki_types::{CertificateDer, ServerName, pem::PemObject};
use rustls::{ClientConfig, RootCertStore};
use syx_driver::{run_file_printed, DriverOptions};

/// Fixed Accept token for relay nail — not derived from Sec-WebSocket-Key.
const RELAY_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
/// RFC6455 §1.3 example Accept for key `dGhlIHNhbXBsZSBub25jZQ==` (same bytes as RELAY_ACCEPT).
const RFC_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
const RFC_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
/// Alternate client Key whose serve-computed Accept ≠ RELAY_ACCEPT (HMR honesty nail).
const ALT_KEY: &str = "x3JJHMbDL1EzLkh9GBhXDw==";
const ALT_COMPUTED_ACCEPT: &str = "HSmrc0sMlYUkAGmm5OPpG2HaGWk=";

type TlsClient = rustls::StreamOwned<rustls::ClientConnection, TcpStream>;

fn tls_test_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        unsafe { std::env::set_var(key, value) };
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

#[test]
fn http_serve_https_loopback_get_with_extra_root() {
    let _serial = tls_test_lock();

    let cert = rcgen::generate_simple_self_signed(vec![
        "127.0.0.1".into(),
        "localhost".into(),
    ])
    .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();

    let port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https");
    let _ = std::fs::create_dir_all(&dir);
    let cert_path = dir.join(format!("serve_{port}.crt"));
    let key_path = dir.join(format!("serve_{port}.key"));
    std::fs::write(&cert_path, &cert_pem).expect("write cert");
    std::fs::write(&key_path, &key_pem).expect("write key");

    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");
    let _cert_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_CERT",
        cert_path.to_str().expect("utf8 cert"),
    );
    let _key_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_KEY",
        key_path.to_str().expect("utf8 key"),
    );
    // Client trust: same self-signed leaf as EXTRA_ROOT (≠ skip-verify).
    let _root_env = EnvGuard::set(
        "SYX_HTTP_EXTRA_ROOT_PEM",
        cert_path.to_str().expect("utf8 root"),
    );

    let src = format!(
        r#"
import Net
import IO

func handle(req: Object) -> Object {{
    if req.path == "/health" {{
        return Http.response(200, "https-ok")
    }}
    return Http.response(404, "no")
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {port}, handler: "handle")
}}
"#
    );
    let serve_path = dir.join(format!("https_serve_{port}.sy"));
    std::fs::write(&serve_path, &src).expect("write serve");

    let (tx, rx) = mpsc::channel();
    let path_clone = serve_path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "HTTPS serve did not listen on {port}"
    );

    // Client via Syx Http.get (EXTRA_ROOT already set in this process).
    let client_src = format!(
        r#"
import Net

func main() {{
    let resp = Http.get(url: "https://127.0.0.1:{port}/health")
    print(resp.status())
    print(resp.text())
    print(resp.ok())
}}
"#
    );
    let client_path = dir.join(format!("https_client_{port}.sy"));
    std::fs::write(&client_path, &client_src).expect("write client");
    let client_result =
        run_file_printed(&client_path, &DriverOptions::default()).expect("client run");
    let (code, printed) = client_result;
    assert_eq!(code, 0, "client exit: {}", printed.join("\n"));
    let out = printed.join("\n");
    assert!(
        out.contains("200") && out.contains("https-ok") && out.contains("true"),
        "HTTPS loopback must 200/https-ok/true: {out}"
    );

    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (serve_code, serve_printed) = run_result;
    assert_eq!(serve_code, 0, "serve exit: {}", serve_printed.join("\n"));

    let _ = std::fs::remove_file(&serve_path);
    let _ = std::fs::remove_file(&client_path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

fn read_http_request_headers(stream: &mut TcpStream) {
    let mut buf = [0u8; 4096];
    let mut got = Vec::new();
    loop {
        let n = stream.read(&mut buf).unwrap_or(0);
        if n == 0 {
            break;
        }
        got.extend_from_slice(&buf[..n]);
        if got.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
}

fn spawn_upgrade_upstream(response: &'static str, echo_payload: bool) -> (u16, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("upstream bind");
    let port = listener.local_addr().expect("addr").port();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("upstream accept");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .ok();
        read_http_request_headers(&mut stream);
        stream
            .write_all(response.as_bytes())
            .expect("write upstream resp");
        stream.flush().ok();
        if echo_payload {
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap_or(0);
            if n > 0 {
                let _ = stream.write_all(&buf[..n]);
                let _ = stream.flush();
            }
        }
        let _ = stream.shutdown(Shutdown::Both);
    });
    (port, handle)
}

fn encode_client_text_frame(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + 4 + payload.len() + 8);
    frame.push(0x81); // FIN + text
    let len = payload.len();
    if len < 126 {
        frame.push(0x80 | (len as u8));
    } else if len <= 0xFFFF {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(0x80 | 127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }
    frame.extend_from_slice(&mask);
    for (i, b) in payload.iter().enumerate() {
        frame.push(b ^ mask[i % 4]);
    }
    frame
}

fn tls_client_stream(
    port: u16,
    cert_pem_path: &std::path::Path,
) -> rustls::StreamOwned<rustls::ClientConnection, TcpStream> {
    let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_file_iter(cert_pem_path)
        .expect("pem iter")
        .collect::<Result<Vec<_>, _>>()
        .expect("pem certs");
    let mut roots = RootCertStore::empty();
    for c in certs {
        roots.add(c).expect("add root");
    }
    let client_cfg = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    );
    let tcp = TcpStream::connect(("127.0.0.1", port)).expect("tcp connect");
    tcp.set_read_timeout(Some(Duration::from_secs(5))).ok();
    tcp.set_write_timeout(Some(Duration::from_secs(5))).ok();
    let name = ServerName::try_from("127.0.0.1")
        .expect("server name")
        .to_owned();
    let client = rustls::ClientConnection::new(client_cfg, name).expect("client conn");
    let mut tls = rustls::StreamOwned::new(client, tcp);
    while tls.conn.is_handshaking() {
        tls.conn
            .complete_io(&mut tls.sock)
            .expect("tls handshake");
    }
    tls
}

/// W1181 / ADR-1527: HMR-over-HTTPS same-port first slice.
/// Client TLS → serve Upgrade → Vite-like upstream; 101 + Accept relay + masked frame.
/// ≠ WSS product GA / ≠ serve-computed Accept / ≠ Playwright.
#[test]
fn http_serve_https_hmr_path_same_port_roundtrip() {
    let _serial = tls_test_lock();

    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\
\r\n",
        true,
    );

    let cert = rcgen::generate_simple_self_signed(vec![
        "127.0.0.1".into(),
        "localhost".into(),
    ])
    .expect("rcgen self-signed");
    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();

    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_hmr");
    let _ = std::fs::create_dir_all(&dir);
    let cert_path = dir.join(format!("hmr_{serve_port}.crt"));
    let key_path = dir.join(format!("hmr_{serve_port}.key"));
    std::fs::write(&cert_path, &cert_pem).expect("write cert");
    std::fs::write(&key_path, &key_pem).expect("write key");

    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");
    let _cert_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_CERT",
        cert_path.to_str().expect("utf8 cert"),
    );
    let _key_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_KEY",
        key_path.to_str().expect("utf8 key"),
    );

    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    o["upgradeUpstream"] = "http://127.0.0.1:{up_port}/@vite/client"
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let serve_path = dir.join(format!("https_hmr_serve_{serve_port}.sy"));
    std::fs::write(&serve_path, &src).expect("write serve");

    let (tx, rx) = mpsc::channel();
    let path_clone = serve_path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(serve_port, Duration::from_secs(30)),
        "HTTPS HMR serve did not listen on {serve_port}"
    );

    let mut client = tls_client_stream(serve_port, &cert_path);
    let req = format!(
        "GET /@vite/client HTTP/1.1\r\n\
Host: 127.0.0.1:{serve_port}\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n"
    );
    client.write_all(req.as_bytes()).expect("write upgrade");
    client.flush().ok();

    let mut head = Vec::new();
    let mut tmp = [0u8; 512];
    loop {
        let n = client.read(&mut tmp).expect("read response head");
        assert!(n > 0, "eof before response head");
        head.extend_from_slice(&tmp[..n]);
        if head.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let head_text = String::from_utf8_lossy(&head).into_owned();
    assert!(
        head_text.contains("101"),
        "expected 101 Switching Protocols over TLS, got: {head_text}"
    );
    assert!(
        head_text.contains(RELAY_ACCEPT),
        "Vite-like Accept must be relayed over HTTPS (≠ serve compute): {head_text}"
    );

    let payload = b"hmr-https-vite-ping";
    let mask = [0x11, 0x22, 0x33, 0x44];
    let frame = encode_client_text_frame(payload, mask);
    client.write_all(&frame).expect("write hmr frame");
    client.flush().ok();
    let mut echo = vec![0u8; frame.len()];
    client.read_exact(&mut echo).expect("read hmr frame echo");
    assert_eq!(
        &echo[..],
        &frame[..],
        "HMR-over-HTTPS must echo masked frame intact"
    );

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let _ = up_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));

    let _ = std::fs::remove_file(&serve_path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

// --- W1211 / ADR-1557: upgradeTerminate-over-TLS product first cluster ---

fn encode_ws_data_frame(opcode_fin: u8, payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + 4 + payload.len() + 8);
    frame.push(opcode_fin);
    let len = payload.len();
    if len < 126 {
        frame.push(0x80 | (len as u8));
    } else if len <= 0xFFFF {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(0x80 | 127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }
    frame.extend_from_slice(&mask);
    for (i, b) in payload.iter().enumerate() {
        frame.push(b ^ mask[i % 4]);
    }
    frame
}

fn decode_ws_server_frame(stream: &mut TlsClient, expect_opcode: u8) -> Vec<u8> {
    let mut hdr = [0u8; 2];
    stream.read_exact(&mut hdr).expect("frame hdr");
    assert_eq!(hdr[0] & 0x0F, expect_opcode, "opcode");
    assert_eq!(hdr[1] & 0x80, 0, "server frame must be unmasked");
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        stream.read_exact(&mut ext).expect("ext len");
        len = u16::from_be_bytes(ext) as usize;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream.read_exact(&mut payload).expect("payload");
    }
    payload
}

fn read_ws_server_close_payload(stream: &mut TlsClient) -> Vec<u8> {
    decode_ws_server_frame(stream, 0x8)
}

fn assert_tls_protocol_close_1002(stream: &mut TlsClient) {
    stream
        .sock
        .set_read_timeout(Some(Duration::from_millis(800)))
        .ok();
    let payload = read_ws_server_close_payload(stream);
    assert!(
        payload.len() >= 2,
        "close(1002) must carry status; got len={}",
        payload.len()
    );
    let status = u16::from_be_bytes([payload[0], payload[1]]);
    assert_eq!(status, 1002u16, "expected close status 1002, got {status}");
}

fn write_pem_pair(dir: &std::path::Path, tag: &str, port: u16) -> (PathBuf, PathBuf) {
    let cert = rcgen::generate_simple_self_signed(vec![
        "127.0.0.1".into(),
        "localhost".into(),
    ])
    .expect("rcgen self-signed");
    let cert_path = dir.join(format!("{tag}_{port}.crt"));
    let key_path = dir.join(format!("{tag}_{port}.key"));
    std::fs::write(&cert_path, cert.cert.pem()).expect("write cert");
    std::fs::write(&key_path, cert.key_pair.serialize_pem()).expect("write key");
    (cert_path, key_path)
}

fn start_https_terminate_serve(
    serve_port: u16,
    cert_path: &std::path::Path,
    key_path: &std::path::Path,
) -> (
    PathBuf,
    mpsc::Receiver<Result<(i32, Vec<String>), syx_diagnostics::SyxDiagnostic>>,
    thread::JoinHandle<()>,
    EnvGuard,
    EnvGuard,
    EnvGuard,
) {
    let max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");
    let cert_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_CERT",
        cert_path.to_str().expect("utf8 cert"),
    );
    let key_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_KEY",
        key_path.to_str().expect("utf8 key"),
    );
    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{\"upgradeTerminate\":true}}")
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("https_terminate_{serve_port}.sy"));
    std::fs::write(&path, &src).expect("write serve");
    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });
    assert!(
        wait_listening(serve_port, Duration::from_secs(30)),
        "HTTPS terminate serve did not listen on {serve_port}"
    );
    (path, rx, serve_join, max_env, cert_env, key_env)
}

fn handshake_https_terminate(
    serve_port: u16,
    cert_path: &std::path::Path,
    key: &str,
) -> (TlsClient, String) {
    let mut client = tls_client_stream(serve_port, cert_path);
    let req = format!(
        "GET /ws HTTP/1.1\r\n\
Host: 127.0.0.1:{serve_port}\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: {key}\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n"
    );
    client.write_all(req.as_bytes()).expect("write upgrade");
    client.flush().ok();
    let mut head = Vec::new();
    let mut tmp = [0u8; 512];
    loop {
        let n = client.read(&mut tmp).expect("read response head");
        assert!(n > 0, "eof before response head");
        head.extend_from_slice(&tmp[..n]);
        if head.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let head_text = String::from_utf8_lossy(&head).into_owned();
    (client, head_text)
}

fn echo_fin1_text_binary_ping_then_close(client: &mut TlsClient, text: &[u8], binary: &[u8], ping: &[u8]) {
    client
        .write_all(&encode_ws_data_frame(0x81, text, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must echo FIN=1 text over TLS");

    client
        .write_all(&encode_ws_data_frame(0x82, binary, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must echo FIN=1 binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// W1211 / ADR-1557: HTTPS upgradeTerminate → serve-computed Accept + text echo (slice 48).
#[test]
fn http_serve_https_upgrade_terminate_accept_and_text_echo() {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, "term", serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(
        head_text.contains("101"),
        "expected 101 over TLS, got: {head_text}"
    );
    assert!(
        head_text.contains(RFC_ACCEPT),
        "serve must compute RFC Accept over TLS (≠ upstream relay): {head_text}"
    );

    client
        .write_all(&encode_ws_data_frame(
            0x81,
            b"w1211-https-text-echo",
            [0x11, 0x22, 0x33, 0x44],
        ))
        .expect("write text");
    client.flush().ok();
    let echo = decode_ws_server_frame(&mut client, 0x1);
    assert_eq!(
        &echo[..],
        b"w1211-https-text-echo",
        "HTTPS terminate must echo masked text"
    );

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1211 / ADR-1557: HTTPS upgradeTerminate RSV≠0 → close(1002) (slice 49).
#[test]
fn http_serve_https_upgrade_terminate_accept_rsv_text_close_1002() {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, "rsv", serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|RSV1|text (0xC1)
    client
        .write_all(&encode_ws_data_frame(
            0xC1,
            b"w1211-rsv-tls",
            [0xde, 0xad, 0xbe, 0xef],
        ))
        .expect("write RSV text");
    client.flush().ok();
    assert_tls_protocol_close_1002(&mut client);

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1211 / ADR-1557: HTTPS upgradeTerminate text→binary→ping→close (slice 50).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_binary_ping_and_close() {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, "life", serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fin1_text_binary_ping_then_close(
        &mut client,
        b"w1211-tls-text",
        b"w1211-tls-bin",
        b"w1211-tls-ping",
    );

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1211 / ADR-1557: HMR-over-HTTPS Accept still relayed ≠ serve-computed (slice 51).
/// Client Key would compute ALT_COMPUTED_ACCEPT; upstream still returns RELAY_ACCEPT.
#[test]
fn http_serve_https_hmr_path_accept_still_relayed_not_serve_computed() {
    let _serial = tls_test_lock();

    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\
\r\n",
        true,
    );

    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_hmr");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, "hmr_honesty", serve_port);

    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");
    let _cert_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_CERT",
        cert_path.to_str().expect("utf8 cert"),
    );
    let _key_env = EnvGuard::set(
        "SYX_HTTP_SERVE_TLS_KEY",
        key_path.to_str().expect("utf8 key"),
    );

    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    o["upgradeUpstream"] = "http://127.0.0.1:{up_port}/@vite/client"
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let serve_path = dir.join(format!("https_hmr_honesty_{serve_port}.sy"));
    std::fs::write(&serve_path, &src).expect("write serve");

    let (tx, rx) = mpsc::channel();
    let path_clone = serve_path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(serve_port, Duration::from_secs(30)),
        "HTTPS HMR honesty serve did not listen on {serve_port}"
    );

    let mut client = tls_client_stream(serve_port, &cert_path);
    let req = format!(
        "GET /@vite/client HTTP/1.1\r\n\
Host: 127.0.0.1:{serve_port}\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: {ALT_KEY}\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n"
    );
    client.write_all(req.as_bytes()).expect("write upgrade");
    client.flush().ok();

    let mut head = Vec::new();
    let mut tmp = [0u8; 512];
    loop {
        let n = client.read(&mut tmp).expect("read response head");
        assert!(n > 0, "eof before response head");
        head.extend_from_slice(&tmp[..n]);
        if head.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let head_text = String::from_utf8_lossy(&head).into_owned();
    assert!(
        head_text.contains("101"),
        "expected 101 over TLS HMR, got: {head_text}"
    );
    assert!(
        head_text.contains(RELAY_ACCEPT),
        "HMR-over-HTTPS must relay upstream Accept: {head_text}"
    );
    assert!(
        !head_text.contains(ALT_COMPUTED_ACCEPT),
        "HMR must NOT serve-compute Accept from client Key (got computed token): {head_text}"
    );

    let payload = b"hmr-honesty-ping";
    let frame = encode_client_text_frame(payload, [0x11, 0x22, 0x33, 0x44]);
    client.write_all(&frame).expect("write hmr frame");
    client.flush().ok();
    let mut echo = vec![0u8; frame.len()];
    client.read_exact(&mut echo).expect("read hmr frame echo");
    assert_eq!(&echo[..], &frame[..], "HMR frame must echo intact");

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let _ = up_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));

    let _ = std::fs::remove_file(&serve_path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

// --- W1214 / ADR-1560: upgradeTerminate-over-TLS close(1002) product first cluster ---

fn encode_ws_unmasked_data_frame(opcode_fin: u8, payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + payload.len() + 8);
    frame.push(opcode_fin);
    let len = payload.len();
    if len < 126 {
        frame.push(len as u8);
    } else if len <= 0xFFFF {
        frame.push(126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(127);
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }
    frame.extend_from_slice(payload);
    frame
}

fn run_https_terminate_protocol_close_nail(tag: &str, bad_frame: &[u8]) {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, tag, serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(
        head_text.contains("101") && head_text.contains(RFC_ACCEPT),
        "{head_text}"
    );

    client.write_all(bad_frame).expect("write protocol-bad frame");
    client.flush().ok();
    assert_tls_protocol_close_1002(&mut client);

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1214 / ADR-1560: HTTPS upgradeTerminate reserved opcode 0xB → close(1002) (slice 60).
#[test]
fn http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_close_1002() {
    let reserved = encode_ws_data_frame(0x8B, b"w1214-reserved-0xb", [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("op0xb", &reserved);
}

/// W1214 / ADR-1560: HTTPS upgradeTerminate FIN=0 ping → close(1002) (slice 61).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_ping_close_1002() {
    // FIN=0|ping (0x09)
    let fin0 = encode_ws_data_frame(0x09, b"w1214-fin0-ping", [0x01, 0x02, 0x03, 0x04]);
    run_https_terminate_protocol_close_nail("fin0ping", &fin0);
}

/// W1214 / ADR-1560: HTTPS upgradeTerminate unexpected continuation → close(1002) (slice 62).
#[test]
fn http_serve_https_upgrade_terminate_accept_unexpected_continuation_close_1002() {
    // FIN=1|continuation (0x80) with no prior fragment.
    let orphan = encode_ws_data_frame(0x80, b"w1214-orphan-cont", [0x11, 0x22, 0x33, 0x44]);
    run_https_terminate_protocol_close_nail("unexpcont", &orphan);
}

/// W1214 / ADR-1560: HTTPS upgradeTerminate unmasked client text → close(1002) (slice 63).
#[test]
fn http_serve_https_upgrade_terminate_accept_unmasked_text_close_1002() {
    let unmasked = encode_ws_unmasked_data_frame(0x81, b"w1214-unmasked-1002");
    run_https_terminate_protocol_close_nail("unmasked", &unmasked);
}

// --- W1215 / ADR-1561: upgradeTerminate-over-TLS close(1002) product second cluster ---

/// FIN=1 ping with forced len7=126 (RFC6455 §5.5 illegal) — align plain W1204/W1210.
fn encode_ws_oversized_ping_len7_126(mask: [u8; 4]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + 2 + 4 + 1);
    frame.push(0x89); // FIN | ping
    frame.push(0x80 | 126); // MASK | len7=126
    frame.extend_from_slice(&1u16.to_be_bytes());
    frame.extend_from_slice(&mask);
    frame.push(b'x' ^ mask[0]);
    frame
}

fn encode_ws_close_with_payload(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_ws_data_frame(0x88, payload, mask)
}

fn encode_ws_close_with_reason(status: u16, reason: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(2 + reason.len());
    payload.extend_from_slice(&status.to_be_bytes());
    payload.extend_from_slice(reason);
    encode_ws_data_frame(0x88, &payload, mask)
}

/// W1215 / ADR-1561: HTTPS upgradeTerminate oversized ping → close(1002) (slice 64).
#[test]
fn http_serve_https_upgrade_terminate_accept_oversized_ping_close_1002() {
    let oversized = encode_ws_oversized_ping_len7_126([0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("ovsping", &oversized);
}

/// W1215 / ADR-1561: HTTPS upgradeTerminate close body len=1 → close(1002) (slice 65).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_len1_close_1002() {
    let close = encode_ws_close_with_payload(&[0x00], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("closelen1", &close);
}

/// W1215 / ADR-1561: HTTPS upgradeTerminate illegal close status 1005 → close(1002) (slice 66).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1005_close_1002() {
    let close = encode_ws_close_with_reason(1005, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st1005", &close);
}

/// W1215 / ADR-1561: HTTPS upgradeTerminate text non-UTF-8 → close(1002) (slice 67).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_non_utf8_close_1002() {
    let bad = encode_ws_data_frame(0x81, &[0xFF], [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("textutf8", &bad);
}

// --- W1216 / ADR-1562: upgradeTerminate-over-TLS close(1002) product third cluster ---

fn run_https_terminate_protocol_close_nail_frames(tag: &str, bad_frames: &[&[u8]]) {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, tag, serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(
        head_text.contains("101") && head_text.contains(RFC_ACCEPT),
        "{head_text}"
    );

    for frame in bad_frames {
        client.write_all(frame).expect("write protocol-bad frame");
    }
    client.flush().ok();
    assert_tls_protocol_close_1002(&mut client);

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1216 / ADR-1562: HTTPS upgradeTerminate close reason non-UTF-8 → close(1002) (slice 68).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_close_1002() {
    let close = encode_ws_close_with_reason(1000, &[0xFF], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("reasonutf8", &close);
}

/// W1216 / ADR-1562: HTTPS upgradeTerminate unsolicited pong → close(1002) (slice 69).
#[test]
fn http_serve_https_upgrade_terminate_accept_unsolicited_pong_close_1002() {
    let pong = encode_ws_data_frame(0x8A, b"w1216-unsolicited", [0x11, 0x22, 0x33, 0x44]);
    run_https_terminate_protocol_close_nail("pong", &pong);
}

/// W1216 / ADR-1562: HTTPS upgradeTerminate illegal close status 1004 → close(1002) (slice 70).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1004_close_1002() {
    let close = encode_ws_close_with_reason(1004, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st1004", &close);
}

/// W1216 / ADR-1562: HTTPS upgradeTerminate fragmented text non-UTF-8 → close(1002) (slice 71).
#[test]
fn http_serve_https_upgrade_terminate_accept_fragmented_text_non_utf8_close_1002() {
    // FIN=0 text [0xC2] + FIN=1 cont [0xFF] → reassembled non-UTF-8.
    let frag1 = encode_ws_data_frame(0x01, &[0xC2], [0x11, 0x22, 0x33, 0x44]);
    let end = encode_ws_data_frame(0x80, &[0xFF], [0x55, 0x66, 0x77, 0x88]);
    run_https_terminate_protocol_close_nail_frames("fragutf8", &[&frag1, &end]);
}

// --- W1217 / ADR-1563: upgradeTerminate-over-TLS close(1002) product fourth cluster ---

/// W1217 / ADR-1563: HTTPS upgradeTerminate FIN=0 close → close(1002) (slice 72).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_close_close_1002() {
    // FIN=0 + close opcode (0x08), empty payload — aligns plain encode_client_fin0_close_frame.
    let close = encode_ws_data_frame(0x08, &[], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fin0close", &close);
}

/// W1217 / ADR-1563: HTTPS upgradeTerminate FIN=0 close mid-fragment → close(1002) (slice 73).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_close_during_fragment_close_1002() {
    let frag1 = encode_ws_data_frame(0x01, b"w121", [0xde, 0xad, 0xbe, 0xef]);
    let close = encode_ws_data_frame(0x08, &[], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail_frames("fin0midfrag", &[&frag1, &close]);
}

/// W1217 / ADR-1563: HTTPS upgradeTerminate reserved data opcode 0x3 → close(1002) (slice 74).
#[test]
fn http_serve_https_upgrade_terminate_accept_reserved_opcode_0x3_close_1002() {
    let reserved = encode_ws_data_frame(0x83, b"w1217-reserved-0x3", [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("res0x3", &reserved);
}

/// W1218 / ADR-1564: HTTPS upgradeTerminate illegal close status 1006 → close(1002) (slice 78).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1006_close_1002() {
    let close = encode_ws_close_with_reason(1006, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st1006", &close);
}

/// W1219 / ADR-1565: HTTPS upgradeTerminate illegal close status 1015 → close(1002) (slice 79).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1015_close_1002() {
    let close = encode_ws_close_with_reason(1015, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st1015", &close);
}

/// W1219 / ADR-1565: HTTPS upgradeTerminate illegal close status 999 → close(1002) (slice 80).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_999_close_1002() {
    let close = encode_ws_close_with_reason(999, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st999", &close);
}

// --- W1220 / ADR-1566: upgradeTerminate-over-TLS positive codec deepen first cluster ---

fn split_three(payload: &[u8]) -> (&[u8], &[u8], &[u8]) {
    assert!(payload.len() >= 3, "N=3 fragment needs len>=3");
    let a = payload.len() / 3;
    let b = (payload.len() * 2) / 3;
    let a = a.max(1);
    let b = b.max(a + 1).min(payload.len() - 1);
    (&payload[..a], &payload[a..b], &payload[b..])
}

fn split_four(payload: &[u8]) -> (&[u8], &[u8], &[u8], &[u8]) {
    assert!(payload.len() >= 4, "N=4 fragment needs len>=4");
    let a = (payload.len() / 4).max(1);
    let b = ((payload.len() * 2) / 4).max(a + 1);
    let c = ((payload.len() * 3) / 4).max(b + 1).min(payload.len() - 1);
    let b = b.min(c - 1).max(a + 1);
    (&payload[..a], &payload[a..b], &payload[b..c], &payload[c..])
}

/// N=3 text/binary → ping/pong → close (TLS twin of plain dual-frag / n3 multi-cont).
fn echo_n3_text_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3) = split_three(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, t3, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=3 fragmented text over TLS");

    let (b1, b2, b3) = split_three(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x11, 0x22, 0x33, 0x44]))
        .expect("write binary mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, b3, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=3 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// N=4 text/binary → ping → close(status+reason) echo (TLS twin of plain W1197).
fn echo_n4_text_binary_ping_then_close_status_reason(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
    status: u16,
    reason: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text over TLS");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let close = encode_ws_close_with_reason(status, reason, [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close with reason");
    client.flush().ok();
    let close_payload = read_ws_server_close_payload(client);
    let mut expect = Vec::with_capacity(2 + reason.len());
    expect.extend_from_slice(&status.to_be_bytes());
    expect.extend_from_slice(reason);
    assert_eq!(
        close_payload, expect,
        "server must echo close status+reason as unmasked close payload over TLS"
    );
}

/// N=4 text with mid↔final ping → N=4 binary → ping → close (TLS twin of plain W1193).
fn echo_n4_text_mid_final_ping_n4_binary_then_close(
    client: &mut TlsClient,
    text: &[u8],
    mid_final_ping: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_ws_data_frame(0x89, mid_final_ping, [0xfe, 0xed, 0xfa, 0xce]))
        .expect("write mid↔final ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(
        &pong[..],
        mid_final_ping,
        "server must pong mid↔final ping without aborting reassembly over TLS"
    );
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(
        &echo[..],
        text,
        "server must reassemble N=4 text after mid↔final ping over TLS"
    );

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

fn run_https_terminate_positive_lifecycle_nail(
    tag: &str,
    body: impl FnOnce(&mut TlsClient),
) {
    let _serial = tls_test_lock();
    let serve_port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_https_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let (cert_path, key_path) = write_pem_pair(&dir, tag, serve_port);
    let (path, rx, serve_join, _max, _c, _k) =
        start_https_terminate_serve(serve_port, &cert_path, &key_path);

    let (mut client, head_text) = handshake_https_terminate(serve_port, &cert_path, RFC_KEY);
    assert!(
        head_text.contains("101") && head_text.contains(RFC_ACCEPT),
        "expected 101 + RFC Accept over TLS, got: {head_text}"
    );

    body(&mut client);

    let _ = client.sock.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&cert_path);
    let _ = std::fs::remove_file(&key_path);
}

/// W1220 / ADR-1566: HTTPS upgradeTerminate close status+reason echo (slice 83).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_close_status_reason_echo() {
    run_https_terminate_positive_lifecycle_nail("closereason", |client| {
        echo_n4_text_binary_ping_then_close_status_reason(
            client,
            b"w1220-n4-text-abcd",
            b"w1220-n4-bin-efgh",
            b"w1220-ping",
            1000,
            b"bye",
        );
    });
}

/// W1220 / ADR-1566: HTTPS upgradeTerminate dual-frag text+binary + ping + close (slice 84).
#[test]
fn http_serve_https_upgrade_terminate_accept_fragmented_text_fragmented_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("dualfrag", |client| {
        echo_n3_text_binary_ping_then_close(
            client,
            b"w1220-frag-text",
            &[0xff, 0x00, 0x7f, b'b', b'i', b'n', b'7'],
            b"w1220-ping",
        );
    });
}

/// W1220 / ADR-1566: HTTPS upgradeTerminate multi-continuation N=3 + ping + close (slice 85).
#[test]
fn http_serve_https_upgrade_terminate_accept_multi_continuation_n3_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("n3multicont", |client| {
        echo_n3_text_binary_ping_then_close(
            client,
            b"w1220-multi-cont-text",
            b"w1220-multi-bin",
            b"w1220-ping",
        );
    });
}

/// W1220 / ADR-1566: HTTPS upgradeTerminate mid↔final control N=4 text + ping + close (slice 86).
#[test]
fn http_serve_https_upgrade_terminate_accept_mid_final_control_n4_text_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("midfinaln4", |client| {
        echo_n4_text_mid_final_ping_n4_binary_then_close(
            client,
            b"w1220-n4-text-abcd",
            b"mid-final-ping",
            b"w1220-n4-bin-efgh",
            b"w1220-ping",
        );
    });
}

// --- W1221 / ADR-1567: upgradeTerminate-over-TLS positive codec deepen second cluster ---

/// N=3 text/binary with optional mid-fragment ping → lifecycle ping → close (TLS twin of W1189/W1190).
fn echo_n3_optional_mid_pings_then_close(
    client: &mut TlsClient,
    text: &[u8],
    text_mid_ping: Option<&[u8]>,
    binary: &[u8],
    binary_mid_ping: Option<&[u8]>,
    ping: &[u8],
) {
    let (t1, t2, t3) = split_three(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    if let Some(mid_ping) = text_mid_ping {
        client
            .write_all(&encode_ws_data_frame(0x89, mid_ping, [0xfe, 0xed, 0xfa, 0xce]))
            .expect("write text mid-frag ping");
        client.flush().ok();
        let pong = decode_ws_server_frame(client, 0xA);
        assert_eq!(
            &pong[..],
            mid_ping,
            "server must pong text mid-fragment ping without aborting reassembly over TLS"
        );
    }
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, t3, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=3 fragmented text over TLS");

    let (b1, b2, b3) = split_three(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    if let Some(mid_ping) = binary_mid_ping {
        client
            .write_all(&encode_ws_data_frame(0x89, mid_ping, [0xbe, 0xef, 0xca, 0xfe]))
            .expect("write binary mid-frag ping");
        client.flush().ok();
        let pong = decode_ws_server_frame(client, 0xA);
        assert_eq!(
            &pong[..],
            mid_ping,
            "server must pong binary mid-fragment ping without aborting reassembly over TLS"
        );
    }
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x11, 0x22, 0x33, 0x44]))
        .expect("write binary mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, b3, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=3 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// N=4 text → N=3 binary → ping → close (TLS twin of plain W1191).
fn echo_n4_text_n3_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text over TLS");

    let (b1, b2, b3) = split_three(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, b3, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=3 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// N=4 text → N=4 binary → ping → close (TLS twin of plain W1192).
fn echo_n4_text_n4_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text over TLS");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// W1221 / ADR-1567: HTTPS upgradeTerminate control-during-fragment N=3 text (slice 87).
#[test]
fn http_serve_https_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("ctrln3text", |client| {
        echo_n3_optional_mid_pings_then_close(
            client,
            b"w1221-ctrl-frag-text",
            Some(b"mid-frag-ping"),
            b"w1221-ctrl-bin",
            None,
            b"w1221-ping",
        );
    });
}

/// W1221 / ADR-1567: HTTPS upgradeTerminate control-during-fragment N=3 binary (slice 88).
#[test]
fn http_serve_https_upgrade_terminate_accept_control_during_fragment_n3_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("ctrln3bin", |client| {
        echo_n3_optional_mid_pings_then_close(
            client,
            b"w1221-text",
            None,
            b"w1221-ctrl-bin",
            Some(b"bin-mid-ping"),
            b"w1221-ping",
        );
    });
}

/// W1221 / ADR-1567: HTTPS upgradeTerminate multi-continuation N=4 text (slice 89).
#[test]
fn http_serve_https_upgrade_terminate_accept_multi_continuation_n4_text_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("n4text", |client| {
        echo_n4_text_n3_binary_ping_then_close(
            client,
            b"w1221-n4-text-abcd",
            b"w1221-bin",
            b"w1221-ping",
        );
    });
}

/// W1221 / ADR-1567: HTTPS upgradeTerminate multi-continuation N=4 text+binary (slice 90).
#[test]
fn http_serve_https_upgrade_terminate_accept_multi_continuation_n4_text_and_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("n4textbin", |client| {
        echo_n4_text_n4_binary_ping_then_close(
            client,
            b"w1221-n4-text-abcd",
            b"w1221-n4-bin-efgh",
            b"w1221-ping",
        );
    });
}

// --- W1222 / ADR-1568: upgradeTerminate-over-TLS positive codec deepen third cluster ---

fn split_five(payload: &[u8]) -> (&[u8], &[u8], &[u8], &[u8], &[u8]) {
    assert!(payload.len() >= 5, "N=5 fragment needs len>=5");
    let a = (payload.len() / 5).max(1);
    let b = ((payload.len() * 2) / 5).max(a + 1);
    let c = ((payload.len() * 3) / 5).max(b + 1);
    let d = ((payload.len() * 4) / 5).max(c + 1).min(payload.len() - 1);
    let c = c.min(d - 1).max(b + 1);
    let b = b.min(c - 1).max(a + 1);
    (
        &payload[..a],
        &payload[a..b],
        &payload[b..c],
        &payload[c..d],
        &payload[d..],
    )
}

/// N=4 text → N=4 binary with mid↔final ping → lifecycle ping → close (TLS twin of plain W1194).
fn echo_n4_text_n4_binary_mid_final_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    mid_final_ping: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text over TLS");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont1");
    client
        .write_all(&encode_ws_data_frame(0x89, mid_final_ping, [0xbe, 0xef, 0xca, 0xfe]))
        .expect("write binary mid↔final ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(
        &pong[..],
        mid_final_ping,
        "server must pong binary mid↔final ping without aborting reassembly over TLS"
    );
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(
        &echo[..],
        binary,
        "server must reassemble N=4 binary after mid↔final ping over TLS"
    );

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// N=5 text → N=4 binary → ping → close (TLS twin of plain W1195).
fn echo_n5_text_n4_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4, t5) = split_five(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, t4, [0x55, 0x66, 0x77, 0x88]))
        .expect("write text cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, t5, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=5 fragmented text over TLS");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x80, b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// N=5 text → N=5 binary → ping → close (TLS twin of plain W1196).
fn echo_n5_text_n5_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4, t5) = split_five(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, t4, [0x55, 0x66, 0x77, 0x88]))
        .expect("write text cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, t5, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble N=5 fragmented text over TLS");

    let (b1, b2, b3, b4, b5) = split_five(binary);
    client
        .write_all(&encode_ws_data_frame(0x02, b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_ws_data_frame(0x00, b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_ws_data_frame(0x00, b4, [0xfe, 0xdc, 0xba, 0x98]))
        .expect("write binary cont3");
    client
        .write_all(&encode_ws_data_frame(0x80, b5, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must reassemble N=5 fragmented binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// Fragmented text (N=3) + FIN=1 binary → ping → close (TLS twin of plain W1186 stair).
fn echo_fragmented_text_fin1_binary_ping_then_close(
    client: &mut TlsClient,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3) = split_three(text);
    client
        .write_all(&encode_ws_data_frame(0x01, t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_ws_data_frame(0x00, t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text mid cont");
    client
        .write_all(&encode_ws_data_frame(0x80, t3, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x1);
    assert_eq!(&echo[..], text, "server must reassemble fragmented text over TLS");

    client
        .write_all(&encode_ws_data_frame(0x82, binary, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write FIN=1 binary");
    client.flush().ok();
    let echo = decode_ws_server_frame(client, 0x2);
    assert_eq!(&echo[..], binary, "server must echo FIN=1 binary over TLS");

    client
        .write_all(&encode_ws_data_frame(0x89, ping, [0x01, 0x02, 0x03, 0x04]))
        .expect("write ping");
    client.flush().ok();
    let pong = decode_ws_server_frame(client, 0xA);
    assert_eq!(&pong[..], ping, "server must pong over TLS");

    let mut close = vec![0x88, 0x80];
    close.extend_from_slice(&[0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    let _ = read_ws_server_close_payload(client);
}

/// W1222 / ADR-1568: HTTPS upgradeTerminate mid↔final control N=4 binary (slice 91).
#[test]
fn http_serve_https_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("midfinaln4bin", |client| {
        echo_n4_text_n4_binary_mid_final_ping_then_close(
            client,
            b"w1222-n4-text-abcd",
            b"w1222-n4-bin-efgh",
            b"bin-mid-final-ping",
            b"w1222-ping",
        );
    });
}

/// W1222 / ADR-1568: HTTPS upgradeTerminate multi-continuation N=5 text (slice 92).
#[test]
fn http_serve_https_upgrade_terminate_accept_multi_continuation_n5_text_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("n5text", |client| {
        echo_n5_text_n4_binary_ping_then_close(
            client,
            b"w1222-n5-text-abcde",
            b"w1222-n4-bin-efgh",
            b"w1222-ping",
        );
    });
}

/// W1222 / ADR-1568: HTTPS upgradeTerminate multi-continuation N=5 text+binary (slice 93).
#[test]
fn http_serve_https_upgrade_terminate_accept_multi_continuation_n5_text_and_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("n5textbin", |client| {
        echo_n5_text_n5_binary_ping_then_close(
            client,
            b"w1222-n5-text-abcde",
            b"w1222-n5-bin-fghij",
            b"w1222-ping",
        );
    });
}

/// W1222 / ADR-1568: HTTPS upgradeTerminate fragmented text + FIN=1 binary (slice 94).
#[test]
fn http_serve_https_upgrade_terminate_accept_fragmented_text_binary_ping_and_close() {
    run_https_terminate_positive_lifecycle_nail("fragtextbin", |client| {
        echo_fragmented_text_fin1_binary_ping_then_close(
            client,
            b"w1222-fragmented-hello",
            b"w1222-bin",
            b"w1222-ping",
        );
    });
}

// --- W1223 / ADR-1569: upgradeTerminate-over-TLS twin closeout + illegal 0/500 ---

/// W1223 / ADR-1569: HTTPS upgradeTerminate text+close twin (slice 95; aligns plain W1183).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_and_close() {
    run_https_terminate_positive_lifecycle_nail("textclose", |client| {
        // Plain W1183 uses N=3 dual-frag lifecycle helper with these payloads.
        echo_n3_text_binary_ping_then_close(
            client,
            b"terminate-close-ping",
            b"close-path-bin",
            b"close-ping",
        );
    });
}

/// W1223 / ADR-1569: HTTPS upgradeTerminate text+binary+close twin (slice 96; aligns plain W1184).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_binary_and_close() {
    run_https_terminate_positive_lifecycle_nail("textbinclose", |client| {
        echo_n3_text_binary_ping_then_close(
            client,
            b"w1223-text",
            &[0xff, 0x00, 0x7f, b'b', b'i', b'n'],
            b"w1223-ping",
        );
    });
}

/// W1223 / ADR-1569: HTTPS upgradeTerminate illegal close status 0 → close(1002) (slice 97).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_0_close_1002() {
    let close = encode_ws_close_with_reason(0, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st0", &close);
}

/// W1223 / ADR-1569: HTTPS upgradeTerminate illegal close status 500 → close(1002) (slice 98).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_500_close_1002() {
    let close = encode_ws_close_with_reason(500, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st500", &close);
}

// --- W1224 / ADR-1570: upgradeTerminate-over-TLS fail_closed name-table first cluster ---
// Soft-twins of plain *_fail_closed; behavior = close(1002) (same harness as W1214 *_close_1002).

/// W1224 / ADR-1570: HTTPS upgradeTerminate reserved opcode 0xB fail_closed name twin (slice 99).
#[test]
fn http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed() {
    let reserved = encode_ws_data_frame(0x8B, b"w1224-reserved-0xb", [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("fc0xb", &reserved);
}

/// W1224 / ADR-1570: HTTPS upgradeTerminate FIN=0 ping fail_closed name twin (slice 100).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_ping_fail_closed() {
    let fin0 = encode_ws_data_frame(0x09, b"w1224-fin0-ping", [0x01, 0x02, 0x03, 0x04]);
    run_https_terminate_protocol_close_nail("fcfin0ping", &fin0);
}

/// W1224 / ADR-1570: HTTPS upgradeTerminate unexpected continuation fail_closed name twin (slice 101).
#[test]
fn http_serve_https_upgrade_terminate_accept_unexpected_continuation_fail_closed() {
    let orphan = encode_ws_data_frame(0x80, b"w1224-orphan-cont", [0x11, 0x22, 0x33, 0x44]);
    run_https_terminate_protocol_close_nail("fcunexpcont", &orphan);
}

/// W1224 / ADR-1570: HTTPS upgradeTerminate unmasked text fail_closed name twin (slice 102).
#[test]
fn http_serve_https_upgrade_terminate_accept_unmasked_text_fail_closed() {
    let unmasked = encode_ws_unmasked_data_frame(0x81, b"w1224-unmasked-fc");
    run_https_terminate_protocol_close_nail("fcunmasked", &unmasked);
}

// --- W1225 / ADR-1571: upgradeTerminate-over-TLS fail_closed name-table second cluster ---
// Soft-twins of plain *_fail_closed; behavior = close(1002) (same harness as W1215 *_close_1002).

/// W1225 / ADR-1571: HTTPS upgradeTerminate oversized ping fail_closed name twin (slice 103).
#[test]
fn http_serve_https_upgrade_terminate_accept_oversized_ping_fail_closed() {
    let oversized = encode_ws_oversized_ping_len7_126([0xca, 0xfe, 0xba, 0xbe]);
    run_https_terminate_protocol_close_nail("fcovsping", &oversized);
}

/// W1225 / ADR-1571: HTTPS upgradeTerminate close body len=1 fail_closed name twin (slice 104).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_len1_fail_closed() {
    let close = encode_ws_close_with_payload(&[0x00], [0x11, 0x22, 0x33, 0x44]);
    run_https_terminate_protocol_close_nail("fccloselen1", &close);
}

/// W1225 / ADR-1571: HTTPS upgradeTerminate illegal close status 1005 fail_closed name twin (slice 105).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1005_fail_closed() {
    let close = encode_ws_close_with_reason(1005, b"", [0x55, 0x66, 0x77, 0x88]);
    run_https_terminate_protocol_close_nail("fcst1005", &close);
}

/// W1225 / ADR-1571: HTTPS upgradeTerminate text non-UTF-8 fail_closed name twin (slice 106).
#[test]
fn http_serve_https_upgrade_terminate_accept_text_non_utf8_fail_closed() {
    let bad = encode_ws_data_frame(0x81, &[0xFF], [0xab, 0xcd, 0xef, 0x01]);
    run_https_terminate_protocol_close_nail("fctextutf8", &bad);
}

// --- W1226 / ADR-1572: upgradeTerminate-over-TLS fail_closed name-table third cluster ---
// Soft-twins of plain *_fail_closed; behavior = close(1002) (same harness as W1216 *_close_1002).

/// W1226 / ADR-1572: HTTPS upgradeTerminate close reason non-UTF-8 fail_closed name twin (slice 107).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_fail_closed() {
    let close = encode_ws_close_with_reason(1000, &[0xFF], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcreasonutf8", &close);
}

/// W1226 / ADR-1572: HTTPS upgradeTerminate unsolicited pong fail_closed name twin (slice 108).
#[test]
fn http_serve_https_upgrade_terminate_accept_unsolicited_pong_fail_closed() {
    let pong = encode_ws_data_frame(0x8A, b"w1226-unsolicited", [0x11, 0x22, 0x33, 0x44]);
    run_https_terminate_protocol_close_nail("fcpong", &pong);
}

/// W1226 / ADR-1572: HTTPS upgradeTerminate illegal close status 1004 fail_closed name twin (slice 109).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1004_fail_closed() {
    let close = encode_ws_close_with_reason(1004, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst1004", &close);
}

/// W1226 / ADR-1572: HTTPS upgradeTerminate fragmented text non-UTF-8 fail_closed name twin (slice 110).
#[test]
fn http_serve_https_upgrade_terminate_accept_fragmented_text_non_utf8_fail_closed() {
    // FIN=0 text [0xC2] + FIN=1 cont [0xFF] → reassembled non-UTF-8.
    let frag1 = encode_ws_data_frame(0x01, &[0xC2], [0x11, 0x22, 0x33, 0x44]);
    let end = encode_ws_data_frame(0x80, &[0xFF], [0x55, 0x66, 0x77, 0x88]);
    run_https_terminate_protocol_close_nail_frames("fcfragutf8", &[&frag1, &end]);
}

// --- W1227 / ADR-1573: upgradeTerminate-over-TLS fail_closed name-table fourth/final cluster ---
// Soft-twins of plain *_fail_closed; behavior = close(1002) (same harness as W1217 / RSV *_close_1002).

/// W1227 / ADR-1573: HTTPS upgradeTerminate FIN=0 close fail_closed name twin (slice 111).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_close_fail_closed() {
    // FIN=0 + close opcode (0x08), empty payload — aligns plain encode_client_fin0_close_frame.
    let close = encode_ws_data_frame(0x08, &[], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcfin0close", &close);
}

/// W1227 / ADR-1573: HTTPS upgradeTerminate FIN=0 close mid-fragment fail_closed name twin (slice 112).
#[test]
fn http_serve_https_upgrade_terminate_accept_fin0_close_during_fragment_fail_closed() {
    let frag1 = encode_ws_data_frame(0x01, b"w122", [0xde, 0xad, 0xbe, 0xef]);
    let close = encode_ws_data_frame(0x08, &[], [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail_frames("fcfin0midfrag", &[&frag1, &close]);
}

/// W1227 / ADR-1573: HTTPS upgradeTerminate reserved opcode 0x3 fail_closed name twin (slice 113).
#[test]
fn http_serve_https_upgrade_terminate_accept_reserved_opcode_0x3_fail_closed() {
    let reserved = encode_ws_data_frame(0x83, b"w1227-reserved-0x3", [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("fc0x3", &reserved);
}

/// W1227 / ADR-1573: HTTPS upgradeTerminate RSV text fail_closed name twin (slice 114).
#[test]
fn http_serve_https_upgrade_terminate_accept_rsv_text_fail_closed() {
    // FIN|RSV1|text (0xC1)
    let rsv = encode_ws_data_frame(0xC1, b"w1227-rsv-fc", [0xde, 0xad, 0xbe, 0xef]);
    run_https_terminate_protocol_close_nail("fcrsv", &rsv);
}

// --- W1228 / ADR-1574: illegal close-status fail_closed soft-twin first cluster (TLS) ---
// Soft-twin of W1218 TLS *_close_1002; behavior = close(1002).

/// W1228 / ADR-1574: HTTPS upgradeTerminate illegal close status 1006 fail_closed name twin (slice 118).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1006_fail_closed() {
    let close = encode_ws_close_with_reason(1006, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst1006", &close);
}

// --- W1229 / ADR-1575: illegal close-status fail_closed soft-twin second cluster (TLS) ---
// Soft-twins of W1219 TLS *_close_1002; behavior = close(1002).

/// W1229 / ADR-1575: HTTPS upgradeTerminate illegal close status 1015 fail_closed name twin (slice 119).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1015_fail_closed() {
    let close = encode_ws_close_with_reason(1015, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst1015", &close);
}

/// W1229 / ADR-1575: HTTPS upgradeTerminate illegal close status 999 fail_closed name twin (slice 120).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_999_fail_closed() {
    let close = encode_ws_close_with_reason(999, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst999", &close);
}

// --- W1230 / ADR-1576: illegal close-status fail_closed soft-twin closeout (TLS 0/500) ---
// Soft-twins of W1223 TLS *_close_1002; behavior = close(1002); → illegal fail_closed 10/10.

/// W1230 / ADR-1576: HTTPS upgradeTerminate illegal close status 0 fail_closed name twin (slice 123).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_0_fail_closed() {
    let close = encode_ws_close_with_reason(0, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst0", &close);
}

/// W1230 / ADR-1576: HTTPS upgradeTerminate illegal close status 500 fail_closed name twin (slice 124).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_500_fail_closed() {
    let close = encode_ws_close_with_reason(500, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst500", &close);
}

// --- W1231 / ADR-1577: illegal close-status deepen third cluster (TLS sample) ---
// Aligns plain close status 1 → close(1002); ≠ close-code full GA.

/// W1231 / ADR-1577: HTTPS upgradeTerminate illegal close status 1 → close(1002) (slice 128).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1_close_1002() {
    let close = encode_ws_close_with_reason(1, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st1", &close);
}

// --- W1232 / ADR-1578: illegal close-status fail_closed soft-twin (TLS, align W1231) ---
// Soft-twin of W1231 TLS *_close_1002; behavior = close(1002).

/// W1232 / ADR-1578: HTTPS upgradeTerminate illegal close status 1 fail_closed name twin (slice 132).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_1_fail_closed() {
    let close = encode_ws_close_with_reason(1, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst1", &close);
}

// --- W1233 / ADR-1579: TLS 100/998 close(1002) twin closeout ---
// Aligns plain W1231 *_close_1002; ≠ close-code full GA.

/// W1233 / ADR-1579: HTTPS upgradeTerminate illegal close status 100 → close(1002) (slice 133).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_100_close_1002() {
    let close = encode_ws_close_with_reason(100, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st100", &close);
}

/// W1233 / ADR-1579: HTTPS upgradeTerminate illegal close status 998 → close(1002) (slice 134).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_998_close_1002() {
    let close = encode_ws_close_with_reason(998, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st998", &close);
}

// --- W1234 / ADR-1580: illegal close-status fail_closed soft-twin (TLS, align W1233) ---
// Soft-twins of W1233 TLS *_close_1002; behavior = close(1002).

/// W1234 / ADR-1580: HTTPS upgradeTerminate illegal close status 100 fail_closed name twin (slice 137).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_100_fail_closed() {
    let close = encode_ws_close_with_reason(100, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst100", &close);
}

/// W1234 / ADR-1580: HTTPS upgradeTerminate illegal close status 998 fail_closed name twin (slice 138).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_998_fail_closed() {
    let close = encode_ws_close_with_reason(998, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst998", &close);
}

// --- W1235 / ADR-1581: TLS 2/997 close(1002) twin closeout ---
// Aligns plain W1233 *_close_1002; ≠ close-code full GA.

/// W1235 / ADR-1581: HTTPS upgradeTerminate illegal close status 2 → close(1002) (slice 141).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_2_close_1002() {
    let close = encode_ws_close_with_reason(2, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st2", &close);
}

/// W1235 / ADR-1581: HTTPS upgradeTerminate illegal close status 997 → close(1002) (slice 142).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_997_close_1002() {
    let close = encode_ws_close_with_reason(997, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st997", &close);
}

// --- W1236 / ADR-1582: illegal close-status fail_closed soft-twin (TLS, align W1235) ---
// Soft-twins of W1235 TLS *_close_1002; behavior = close(1002).

/// W1236 / ADR-1582: HTTPS upgradeTerminate illegal close status 2 fail_closed name twin (slice 145).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_2_fail_closed() {
    let close = encode_ws_close_with_reason(2, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst2", &close);
}

/// W1236 / ADR-1582: HTTPS upgradeTerminate illegal close status 997 fail_closed name twin (slice 146).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_997_fail_closed() {
    let close = encode_ws_close_with_reason(997, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst997", &close);
}

// --- W1237 / ADR-1583: TLS 3/996 close(1002) twin closeout ---
// Aligns plain W1235 *_close_1002; ≠ close-code full GA.

/// W1237 / ADR-1583: HTTPS upgradeTerminate illegal close status 3 → close(1002) (slice 149).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_3_close_1002() {
    let close = encode_ws_close_with_reason(3, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st3", &close);
}

/// W1237 / ADR-1583: HTTPS upgradeTerminate illegal close status 996 → close(1002) (slice 150).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_996_close_1002() {
    let close = encode_ws_close_with_reason(996, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st996", &close);
}

// --- W1238 / ADR-1584: illegal close-status fail_closed soft-twin (TLS, align W1237) ---
// Soft-twins of W1237 TLS *_close_1002; behavior = close(1002).

/// W1238 / ADR-1584: HTTPS upgradeTerminate illegal close status 3 fail_closed name twin (slice 153).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_3_fail_closed() {
    let close = encode_ws_close_with_reason(3, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst3", &close);
}

/// W1238 / ADR-1584: HTTPS upgradeTerminate illegal close status 996 fail_closed name twin (slice 154).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_996_fail_closed() {
    let close = encode_ws_close_with_reason(996, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst996", &close);
}

// --- W1239 / ADR-1585: TLS 4/995 close(1002) twin closeout ---
// Aligns plain W1237 *_close_1002; ≠ close-code full GA.

/// W1239 / ADR-1585: HTTPS upgradeTerminate illegal close status 4 → close(1002) (slice 157).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_4_close_1002() {
    let close = encode_ws_close_with_reason(4, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st4", &close);
}

/// W1239 / ADR-1585: HTTPS upgradeTerminate illegal close status 995 → close(1002) (slice 158).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_995_close_1002() {
    let close = encode_ws_close_with_reason(995, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st995", &close);
}

// --- W1240 / ADR-1586: illegal close-status fail_closed soft-twin (TLS, align W1239) ---
// Soft-twins of W1239 TLS *_close_1002; behavior = close(1002).

/// W1240 / ADR-1586: HTTPS upgradeTerminate illegal close status 4 fail_closed name twin (slice 161).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_4_fail_closed() {
    let close = encode_ws_close_with_reason(4, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst4", &close);
}

/// W1240 / ADR-1586: HTTPS upgradeTerminate illegal close status 995 fail_closed name twin (slice 162).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_995_fail_closed() {
    let close = encode_ws_close_with_reason(995, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst995", &close);
}

// --- W1241 / ADR-1587: TLS 5/994 close(1002) twin closeout ---
// Aligns plain W1239 *_close_1002; ≠ close-code full GA.

/// W1241 / ADR-1587: HTTPS upgradeTerminate illegal close status 5 → close(1002) (slice 165).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_5_close_1002() {
    let close = encode_ws_close_with_reason(5, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st5", &close);
}

/// W1241 / ADR-1587: HTTPS upgradeTerminate illegal close status 994 → close(1002) (slice 166).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_994_close_1002() {
    let close = encode_ws_close_with_reason(994, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st994", &close);
}

// --- W1242 / ADR-1588: illegal close-status fail_closed soft-twin (TLS, align W1241) ---
// Soft-twins of W1241 TLS *_close_1002; behavior = close(1002).

/// W1242 / ADR-1588: HTTPS upgradeTerminate illegal close status 5 fail_closed name twin (slice 169).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_5_fail_closed() {
    let close = encode_ws_close_with_reason(5, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst5", &close);
}

/// W1242 / ADR-1588: HTTPS upgradeTerminate illegal close status 994 fail_closed name twin (slice 170).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_994_fail_closed() {
    let close = encode_ws_close_with_reason(994, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst994", &close);
}

// --- W1243 / ADR-1589: TLS 6/993 close(1002) twin closeout ---
// Aligns plain W1241 *_close_1002; ≠ close-code full GA.

/// W1243 / ADR-1589: HTTPS upgradeTerminate illegal close status 6 → close(1002) (slice 173).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_6_close_1002() {
    let close = encode_ws_close_with_reason(6, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st6", &close);
}

/// W1243 / ADR-1589: HTTPS upgradeTerminate illegal close status 993 → close(1002) (slice 174).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_993_close_1002() {
    let close = encode_ws_close_with_reason(993, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st993", &close);
}

// --- W1244 / ADR-1590: illegal close-status fail_closed soft-twin (TLS, align W1243) ---
// Soft-twins of W1243 TLS *_close_1002; behavior = close(1002).

/// W1244 / ADR-1590: HTTPS upgradeTerminate illegal close status 6 fail_closed name twin (slice 177).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_6_fail_closed() {
    let close = encode_ws_close_with_reason(6, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst6", &close);
}

/// W1244 / ADR-1590: HTTPS upgradeTerminate illegal close status 993 fail_closed name twin (slice 178).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_993_fail_closed() {
    let close = encode_ws_close_with_reason(993, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst993", &close);
}

// --- W1245 / ADR-1591: TLS 7/992 close(1002) twin closeout ---
// Aligns plain W1243 *_close_1002; ≠ close-code full GA.

/// W1245 / ADR-1591: HTTPS upgradeTerminate illegal close status 7 → close(1002) (slice 181).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_7_close_1002() {
    let close = encode_ws_close_with_reason(7, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st7", &close);
}

/// W1245 / ADR-1591: HTTPS upgradeTerminate illegal close status 992 → close(1002) (slice 182).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_992_close_1002() {
    let close = encode_ws_close_with_reason(992, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st992", &close);
}

// --- W1246 / ADR-1592: illegal close-status fail_closed soft-twin (TLS, align W1245) ---
// Soft-twins of W1245 TLS *_close_1002; behavior = close(1002).

/// W1246 / ADR-1592: HTTPS upgradeTerminate illegal close status 7 fail_closed name twin (slice 185).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_7_fail_closed() {
    let close = encode_ws_close_with_reason(7, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst7", &close);
}

/// W1246 / ADR-1592: HTTPS upgradeTerminate illegal close status 992 fail_closed name twin (slice 186).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_992_fail_closed() {
    let close = encode_ws_close_with_reason(992, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst992", &close);
}

// --- W1247 / ADR-1593: TLS 8/991 close(1002) twin closeout ---
// Aligns plain W1245 *_close_1002; ≠ close-code full GA.

/// W1247 / ADR-1593: HTTPS upgradeTerminate illegal close status 8 → close(1002) (slice 189).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_8_close_1002() {
    let close = encode_ws_close_with_reason(8, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st8", &close);
}

/// W1247 / ADR-1593: HTTPS upgradeTerminate illegal close status 991 → close(1002) (slice 190).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_991_close_1002() {
    let close = encode_ws_close_with_reason(991, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st991", &close);
}

// --- W1248 / ADR-1594: illegal near-bound sampling axis soft-close (TLS) ---
// Soft-twin W1247 debt + edge-align TLS twin of plain 9/990; no further N/999-N deepen.

/// W1248 / ADR-1594: HTTPS upgradeTerminate illegal close status 8 fail_closed name twin.
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_8_fail_closed() {
    let close = encode_ws_close_with_reason(8, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst8", &close);
}

/// W1248 / ADR-1594: HTTPS upgradeTerminate illegal close status 991 fail_closed name twin.
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_991_fail_closed() {
    let close = encode_ws_close_with_reason(991, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("fcst991", &close);
}

/// W1248 / ADR-1594: HTTPS upgradeTerminate illegal close status 9 → close(1002) (TLS edge align).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_9_close_1002() {
    let close = encode_ws_close_with_reason(9, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st9", &close);
}

/// W1248 / ADR-1594: HTTPS upgradeTerminate illegal close status 990 → close(1002) (TLS edge align).
#[test]
fn http_serve_https_upgrade_terminate_accept_close_status_990_close_1002() {
    let close = encode_ws_close_with_reason(990, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    run_https_terminate_protocol_close_nail("st990", &close);
}
