//! Http.serve Upgrade → upgradeUpstream byte proxy.
//! First slice: raw byte echo after 101. Third slice: masked RFC6455 text frame
//! round-trip (tunnel does not parse frames; ≠ terminator / TLS / hyper).
//! Fourth slice: non-101 → status.max(400); 101 relays Sec-WebSocket-Accept (≠ compute).
//! Fifth slice: missing upgradeUpstream → 502; dial fail → 502.
//! Sixth slice: empty upgradeUpstream → 502; handler Err → 502; charter closeout.
//! Todo 8 third slice : same-port HMR path nail — client→serve→Vite-like
//! upstream (101 + Accept relay + masked text-frame); ≠ browser Playwright GA.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{mpsc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{run_file_printed, DriverOptions};

/// Fixed Accept token for relay nail — not derived from Sec-WebSocket-Key.
const RELAY_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

/// Serialize sessions: `SYX_HTTP_SERVE_MAX_REQUESTS` is process-global.
fn upgrade_test_lock() -> MutexGuard<'static, ()> {
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

fn spawn_echo_upgrade_upstream() -> (u16, thread::JoinHandle<()>) {
    spawn_upgrade_upstream(
        "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
\r\n",
        true,
    )
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

/// Client-masked RFC6455 text frame (FIN+text). Aligns with
/// `syx_vm::ws_rfc6455` encode shape; kept local — no VM export.
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

struct UpgradeSession {
    client: TcpStream,
    /// Full HTTP response head (through `\r\n\r\n`).
    head_text: String,
    path: PathBuf,
    rx: mpsc::Receiver<Result<(i32, Vec<String>), syx_diagnostics::SyxDiagnostic>>,
    serve_join: thread::JoinHandle<()>,
    up_join: thread::JoinHandle<()>,
    _max_env: EnvGuard,
    _serial: MutexGuard<'static, ()>,
}

fn start_upgrade_session() -> UpgradeSession {
    let (up_port, up_join) = spawn_echo_upgrade_upstream();
    start_upgrade_session_with(up_port, up_join, true)
}

fn start_upgrade_session_with(
    up_port: u16,
    up_join: thread::JoinHandle<()>,
    expect_101: bool,
) -> UpgradeSession {
    let serve_port = free_port();
    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    o["upgradeUpstream"] = "http://127.0.0.1:{up_port}/ws"
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    start_upgrade_session_from_src(src, up_join, expect_101, serve_port)
}

/// Custom handler source; `serve_port` must match the port embedded in `src`.
fn start_upgrade_session_from_src(
    src: String,
    up_join: thread::JoinHandle<()>,
    expect_101: bool,
    serve_port: u16,
) -> UpgradeSession {
    let serial = upgrade_test_lock();
    let max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_upgrade");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("upgrade_{serve_port}.sy"));
    std::fs::write(&path, src).expect("write");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(serve_port, Duration::from_secs(30)),
        "serve did not listen on {serve_port}"
    );

    let mut client = TcpStream::connect(("127.0.0.1", serve_port)).expect("client connect");
    client
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
    let req = format!(
        "GET /ws HTTP/1.1\r\n\
Host: 127.0.0.1:{serve_port}\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n"
    );
    client.write_all(req.as_bytes()).expect("write upgrade");

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
    if expect_101 {
        assert!(
            head_text.contains("101"),
            "expected 101 Switching Protocols, got: {head_text}"
        );
        assert!(
            !head_text.contains("501"),
            "must not reject Upgrade with 501: {head_text}"
        );
    }

    UpgradeSession {
        client,
        head_text,
        path,
        rx,
        serve_join,
        up_join,
        _max_env: max_env,
        _serial: serial,
    }
}

fn finish_upgrade_session(session: UpgradeSession) {
    let UpgradeSession {
        client,
        path,
        rx,
        serve_join,
        up_join,
        ..
    } = session;
    // Non-101 responses may still have a body; closing frees tiny_http / serve loop.
    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let _ = up_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

fn status_line(head: &str) -> &str {
    head.lines().next().unwrap_or("")
}

#[test]
fn http_serve_upgrade_proxy_roundtrip() {
    let mut session = start_upgrade_session();
    let payload = b"ping-upgrade";
    session.client.write_all(payload).expect("write payload");
    let mut echo = vec![0u8; payload.len()];
    session.client.read_exact(&mut echo).expect("read echo");
    assert_eq!(&echo[..], payload, "echo mismatch");
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_masked_text_frame_roundtrip() {
    let mut session = start_upgrade_session();
    let payload = b"hmr-ping";
    let mask = [0x12, 0x34, 0x56, 0x78];
    let frame = encode_client_text_frame(payload, mask);
    assert_eq!(frame[0], 0x81, "FIN+text opcode");
    assert_eq!(frame[1], 0x80 | (payload.len() as u8), "masked length");
    session.client.write_all(&frame).expect("write frame");
    let mut echo = vec![0u8; frame.len()];
    session.client.read_exact(&mut echo).expect("read frame echo");
    assert_eq!(&echo[..], &frame[..], "masked text frame must pass tunnel intact");
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_non_101_maps_status_max_400() {
    // Upstream 500 → client 500 (`status.max(400)` leaves >=400 unchanged).
    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 500 Internal Server Error\r\n\
Content-Type: text/plain\r\n\
\r\n\
fail",
        false,
    );
    let session = start_upgrade_session_with(up_port, up_join, false);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("500"),
        "expected client status 500, got: {line} full={}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade on non-101: {}",
        session.head_text
    );
    finish_upgrade_session(session);

    // Upstream 200 → client 400 (`status.max(400)`).
    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 200 OK\r\n\
Content-Type: text/plain\r\n\
\r\n\
ok",
        false,
    );
    let session = start_upgrade_session_with(up_port, up_join, false);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("400"),
        "expected client status 400 for upstream 200, got: {line} full={}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade on non-101: {}",
        session.head_text
    );
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_relays_sec_websocket_accept() {
    // Static response string so Accept token is identical on the wire.
    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\
\r\n",
        true,
    );
    let mut session = start_upgrade_session_with(up_port, up_join, true);
    let head_lower = session.head_text.to_ascii_lowercase();
    assert!(
        head_lower.contains("sec-websocket-accept"),
        "Accept header must be relayed: {}",
        session.head_text
    );
    assert!(
        session.head_text.contains(RELAY_ACCEPT),
        "Accept token must match upstream (relay ≠ compute): {}",
        session.head_text
    );
    // Still a working tunnel after header relay.
    let payload = b"accept-relay";
    session.client.write_all(payload).expect("write payload");
    let mut echo = vec![0u8; payload.len()];
    session.client.read_exact(&mut echo).expect("read echo");
    assert_eq!(&echo[..], payload, "echo mismatch after Accept relay");
    finish_upgrade_session(session);
}

/// Todo 8 / W1177: same-port HMR product path — client dials **serve** only;
/// handler returns `upgradeUpstream` to a Vite-like WS upstream (`/@vite/client`);
/// 101 + Accept relay + masked text-frame round-trip. ≠ browser Playwright GA /
/// ≠ frame terminator / ≠ serve-computed Accept.
#[test]
fn http_serve_upgrade_proxy_hmr_path_same_port_roundtrip() {
    let (up_port, up_join) = spawn_upgrade_upstream(
        "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\
\r\n",
        true,
    );
    let serve_port = free_port();
    // Vite-like HMR path on both serve request and upstream URL (sgate :8080→:9999 shape).
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
    let serial = upgrade_test_lock();
    let max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_upgrade");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("upgrade_hmr_{serve_port}.sy"));
    std::fs::write(&path, &src).expect("write");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(serve_port, Duration::from_secs(30)),
        "serve did not listen on {serve_port}"
    );

    // Client connects **only** to the serve port (same-origin HMR semantics).
    let mut client = TcpStream::connect(("127.0.0.1", serve_port)).expect("client connect");
    client
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
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
        "expected 101 Switching Protocols on serve port, got: {head_text}"
    );
    assert!(
        head_text.contains(RELAY_ACCEPT),
        "Vite-like Accept must be relayed (≠ serve compute): {head_text}"
    );

    // Masked text frame round-trip (HMR-shaped payload; tunnel opaque).
    let payload = b"hmr-vite-ping";
    let mask = [0xaa, 0xbb, 0xcc, 0xdd];
    let frame = encode_client_text_frame(payload, mask);
    client.write_all(&frame).expect("write hmr frame");
    let mut echo = vec![0u8; frame.len()];
    client.read_exact(&mut echo).expect("read hmr frame echo");
    assert_eq!(
        &echo[..],
        &frame[..],
        "same-port HMR path must echo masked frame intact"
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let _ = up_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    drop(max_env);
    drop(serial);
}

#[test]
fn http_serve_upgrade_proxy_missing_upgrade_upstream_returns_502() {
    let serve_port = free_port();
    let up_join = thread::spawn(|| {});
    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let session = start_upgrade_session_from_src(src, up_join, false, serve_port);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("502"),
        "expected 502 without upgradeUpstream, got: {line} full={}",
        session.head_text
    );
    assert!(
        session.head_text.contains("upgradeUpstream"),
        "body should mention upgradeUpstream: {}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade: {}",
        session.head_text
    );
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_dial_fail_returns_502() {
    // Ephemeral port with no listener → connect fails → 502 (not hang).
    let dead_port = free_port();
    let serve_port = free_port();
    let up_join = thread::spawn(|| {});
    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    o["upgradeUpstream"] = "http://127.0.0.1:{dead_port}/ws"
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let session = start_upgrade_session_from_src(src, up_join, false, serve_port);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("502"),
        "expected 502 on dial fail, got: {line} full={}",
        session.head_text
    );
    assert!(
        session
            .head_text
            .to_ascii_lowercase()
            .contains("bad gateway")
            || session.head_text.contains("upgrade dial"),
        "body should describe dial failure: {}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade on dial fail: {}",
        session.head_text
    );
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_empty_upgrade_upstream_returns_502() {
    let serve_port = free_port();
    let up_join = thread::spawn(|| {});
    let src = format!(
        r#"
import Net

func handle(req: Object) -> Object {{
    var o: Object = JSON.decode("{{}}")
    o["upgradeUpstream"] = ""
    return o
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "handle")
}}
"#
    );
    let session = start_upgrade_session_from_src(src, up_join, false, serve_port);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("502"),
        "expected 502 for empty upgradeUpstream, got: {line} full={}",
        session.head_text
    );
    assert!(
        session.head_text.contains("upgradeUpstream"),
        "empty string treated as missing upgradeUpstream: {}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade: {}",
        session.head_text
    );
    finish_upgrade_session(session);
}

#[test]
fn http_serve_upgrade_proxy_handler_err_returns_502() {
    // Unknown handler name → invoke_http_handler Err → 502 Bad Gateway.
    let serve_port = free_port();
    let up_join = thread::spawn(|| {});
    let src = format!(
        r#"
import Net

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {serve_port}, handler: "no_such_upgrade_handler")
}}
"#
    );
    let session = start_upgrade_session_from_src(src, up_join, false, serve_port);
    let line = status_line(&session.head_text);
    assert!(
        line.contains("502"),
        "expected 502 on handler Err, got: {line} full={}",
        session.head_text
    );
    assert!(
        session
            .head_text
            .to_ascii_lowercase()
            .contains("bad gateway"),
        "body should be Bad Gateway: {}",
        session.head_text
    );
    assert!(
        !session.head_text.contains("101"),
        "must not upgrade on handler Err: {}",
        session.head_text
    );
    finish_upgrade_session(session);
}
