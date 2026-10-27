//! Http.serve upgradeTerminate first slice.
//! Serve-computed Sec-WebSocket-Accept + in-process masked text-frame echo.
//! close-path after text echo (client close → server unmasked close).
//! binary echo between text and close (opcode 2 → 0x82).
//! ping→pong between binary and close (opcode 9 → 0x8A).
//! fragmented text (FIN=0 text + continuation → single echo).
//! fragmented binary (FIN=0 binary + continuation → single 0x82).
//! multi-continuation N=3 (mid FIN=0 cont + final FIN=1 cont).
//! illegal close-status deepen first cluster — 1006 / 1015 / 999
//! → close(1002); ≠ close-code full set GA / ≠ WSS product GA.
//! illegal close-status deepen second cluster — 0 / 500 → close(1002)
//! ≠ close-code full set GA / ≠ WSS product GA.
//! ≠ WSS product GA / ≠ replace upgradeUpstream HMR tunnel / ≠ HostFn+1.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{mpsc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{run_file_printed, DriverOptions};

/// RFC6455 §1.3 example Accept for key `dGhlIHNhbXBsZSBub25jZQ==`.
const RFC_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
const RFC_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";

fn terminate_test_lock() -> MutexGuard<'static, ()> {
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

fn encode_client_data_frame(opcode_fin: u8, payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
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

/// Unmasked client frame (MASK=0) — W1200 negative nail only.
fn encode_client_unmasked_data_frame(opcode_fin: u8, payload: &[u8]) -> Vec<u8> {
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

/// FIN=0 + opcode text (first fragment).
fn encode_client_text_fragment_start(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x01, payload, mask)
}

/// FIN=0 + opcode binary (first fragment).
fn encode_client_binary_fragment_start(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x02, payload, mask)
}

/// FIN=0 + opcode continuation (middle fragment).
fn encode_client_continuation_mid(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x00, payload, mask)
}

/// FIN=1 + opcode continuation.
fn encode_client_continuation_end(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x80, payload, mask)
}

/// Split payload into three non-empty parts for N=3 fragment nails.
fn split_three(payload: &[u8]) -> (&[u8], &[u8], &[u8]) {
    assert!(payload.len() >= 3, "N=3 fragment needs len>=3");
    let a = payload.len() / 3;
    let b = (payload.len() * 2) / 3;
    let a = a.max(1);
    let b = b.max(a + 1).min(payload.len() - 1);
    (&payload[..a], &payload[a..b], &payload[b..])
}

/// Split payload into four non-empty parts for N=4 fragment nails.
fn split_four(payload: &[u8]) -> (&[u8], &[u8], &[u8], &[u8]) {
    assert!(payload.len() >= 4, "N=4 fragment needs len>=4");
    let a = (payload.len() / 4).max(1);
    let b = ((payload.len() * 2) / 4).max(a + 1);
    let c = ((payload.len() * 3) / 4).max(b + 1).min(payload.len() - 1);
    let b = b.min(c - 1).max(a + 1);
    (&payload[..a], &payload[a..b], &payload[b..c], &payload[c..])
}

/// Split payload into five non-empty parts for N=5 unbounded-N first-slice nails.
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

fn encode_client_ping_frame(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x89, payload, mask)
}

/// FIN=0 + ping (0x09) — W1201 fragmented-control negative nail only.
fn encode_client_fin0_ping_frame(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x09, payload, mask)
}

/// FIN=0 + close (0x08) — W1208 fragmented-control negative nail only.
fn encode_client_fin0_close_frame(mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x08, &[], mask)
}

/// FIN=1 + continuation (0x80) with no prior fragment — W1208 unexpected-cont nail only.
fn encode_client_unexpected_continuation_frame(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x80, payload, mask)
}

/// FIN=1 + pong (0x8A) unsolicited — W1208 unsolicited-pong nail only.
fn encode_client_pong_frame(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x8A, payload, mask)
}

/// FIN=1 ping with forced len7=126 (RFC6455 §5.5 illegal) — W1204 negative nail only.
/// Server must reject before reading extended length; trailing bytes are optional bait.
fn encode_client_oversized_ping_len7_126(mask: [u8; 4]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(2 + 2 + 4 + 1);
    frame.push(0x89); // FIN | ping
    frame.push(0x80 | 126); // MASK | len7=126
    frame.extend_from_slice(&1u16.to_be_bytes()); // ext length bait (unread on fail-closed)
    frame.extend_from_slice(&mask);
    frame.push(b'x' ^ mask[0]);
    frame
}

fn decode_server_data_frame(stream: &mut TcpStream, expect_opcode: u8) -> Vec<u8> {
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

fn decode_server_text_frame(stream: &mut TcpStream) -> Vec<u8> {
    decode_server_data_frame(stream, 0x1)
}

fn decode_server_binary_frame(stream: &mut TcpStream) -> Vec<u8> {
    decode_server_data_frame(stream, 0x2)
}

fn decode_server_pong_frame(stream: &mut TcpStream) -> Vec<u8> {
    decode_server_data_frame(stream, 0xA)
}

fn encode_client_close_frame(mask: [u8; 4]) -> Vec<u8> {
    let mut frame = vec![0x88, 0x80];
    frame.extend_from_slice(&mask);
    frame
}

/// Client close with 2-byte status + UTF-8 reason (masked).
fn encode_client_close_frame_with_reason(status: u16, reason: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(2 + reason.len());
    payload.extend_from_slice(&status.to_be_bytes());
    payload.extend_from_slice(reason);
    encode_client_data_frame(0x88, &payload, mask)
}

/// Client close with arbitrary application payload (masked). Used for len=1 negative nail.
fn encode_client_close_frame_with_payload(payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    encode_client_data_frame(0x88, payload, mask)
}

fn read_server_close_frame(stream: &mut TcpStream) {
    let _ = read_server_close_frame_payload(stream);
}

fn read_server_close_frame_payload(stream: &mut TcpStream) -> Vec<u8> {
    let mut hdr = [0u8; 2];
    stream.read_exact(&mut hdr).expect("close hdr");
    assert_eq!(hdr[0] & 0x0F, 0x8, "opcode close");
    assert_eq!(hdr[1] & 0x80, 0, "server close must be unmasked");
    let mut len = (hdr[1] & 0x7F) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        stream.read_exact(&mut ext).expect("close ext len");
        len = u16::from_be_bytes(ext) as usize;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream.read_exact(&mut payload).expect("close payload");
    }
    payload
}

/// W1209 / ADR-1555: server must reply unmasked close(status=1002) on protocol error.
fn assert_server_protocol_close_1002(stream: &mut TcpStream) {
    stream
        .set_read_timeout(Some(Duration::from_millis(800)))
        .ok();
    let payload = read_server_close_frame_payload(stream);
    assert!(
        payload.len() >= 2,
        "close(1002) must carry status bytes; got len={}",
        payload.len()
    );
    let status = u16::from_be_bytes([payload[0], payload[1]]);
    assert_eq!(status, 1002u16, "expected protocol close status 1002, got {status}");
}


/// W1188–W1190 lifecycle: N=3 frag-text → N=3 frag-binary → ping/pong → close.
fn echo_fragmented_text_fragmented_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    echo_n3_optional_mid_pings_then_close(client, text, None, binary, None, ping);
}

/// W1189 helper: N=3 text mid-ping → N=3 binary (no mid-ping) → lifecycle ping → close.
fn echo_n3_text_optional_mid_ping_n3_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    mid_frag_ping: Option<&[u8]>,
    binary: &[u8],
    ping: &[u8],
) {
    echo_n3_optional_mid_pings_then_close(client, text, mid_frag_ping, binary, None, ping);
}

/// N=3 text/binary with optional ping between each frag1 and mid cont → lifecycle ping → close.
fn echo_n3_optional_mid_pings_then_close(
    client: &mut TcpStream,
    text: &[u8],
    text_mid_ping: Option<&[u8]>,
    binary: &[u8],
    binary_mid_ping: Option<&[u8]>,
    ping: &[u8],
) {
    let (t1, t2, t3) = split_three(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    if let Some(mid_ping) = text_mid_ping {
        let frame = encode_client_ping_frame(mid_ping, [0xfe, 0xed, 0xfa, 0xce]);
        client.write_all(&frame).expect("write text mid-frag ping");
        client.flush().ok();
        let pong = decode_server_pong_frame(client);
        assert_eq!(
            &pong[..],
            mid_ping,
            "server must pong text mid-fragment ping without aborting reassembly"
        );
    }
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text mid cont");
    client
        .write_all(&encode_client_continuation_end(t3, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=3 fragmented text");

    let (b1, b2, b3) = split_three(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    if let Some(mid_ping) = binary_mid_ping {
        let frame = encode_client_ping_frame(mid_ping, [0xbe, 0xef, 0xca, 0xfe]);
        client.write_all(&frame).expect("write binary mid-frag ping");
        client.flush().ok();
        let pong = decode_server_pong_frame(client);
        assert_eq!(
            &pong[..],
            mid_ping,
            "server must pong binary mid-fragment ping without aborting reassembly"
        );
    }
    client
        .write_all(&encode_client_continuation_mid(b2, [0x11, 0x22, 0x33, 0x44]))
        .expect("write binary mid cont");
    client
        .write_all(&encode_client_continuation_end(b3, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=3 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

fn start_terminate_serve(serve_port: u16) -> (
    PathBuf,
    mpsc::Receiver<Result<(i32, Vec<String>), syx_diagnostics::SyxDiagnostic>>,
    thread::JoinHandle<()>,
    EnvGuard,
) {
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
    let max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_upgrade_terminate");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("terminate_{serve_port}.sy"));
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
    (path, rx, serve_join, max_env)
}

fn handshake_terminate_client(serve_port: u16) -> (TcpStream, String) {
    let mut client = TcpStream::connect(("127.0.0.1", serve_port)).expect("client connect");
    client
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
    let req = format!(
        "GET /ws HTTP/1.1\r\n\
Host: 127.0.0.1:{serve_port}\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: {RFC_KEY}\r\n\
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
    (client, head_text)
}

/// W1182 / ADR-1528: upgradeTerminate → serve-computed Accept + text echo.
#[test]
fn http_serve_upgrade_terminate_accept_and_text_echo() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(
        head_text.contains("101"),
        "expected 101 Switching Protocols, got: {head_text}"
    );
    assert!(
        head_text.contains(RFC_ACCEPT),
        "serve must compute RFC Accept from client Key (≠ relay fixed token): {head_text}"
    );

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"terminate-echo-ping",
        b"\x00\x01\x02binary",
        b"ping-payload",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1183 / ADR-1529: Accept + text echo + client/server close exchange.
#[test]
fn http_serve_upgrade_terminate_accept_text_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"terminate-close-ping",
        b"close-path-bin",
        b"close-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1184 / ADR-1530: Accept + text + binary echo + close exchange.
#[test]
fn http_serve_upgrade_terminate_accept_text_binary_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"w1184-text",
        &[0xff, 0x00, 0x7f, b'b', b'i', b'n'],
        b"w1184-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1185 / ADR-1531: Accept + text + binary + ping/pong + close exchange.
#[test]
fn http_serve_upgrade_terminate_accept_text_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"w1185-text",
        b"w1185-bin",
        &[0x70, 0x69, 0x6e, 0x67],
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1186 / ADR-1532: Accept + fragmented text + binary + ping/pong + close.
#[test]
fn http_serve_upgrade_terminate_accept_fragmented_text_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"w1186-fragmented-hello",
        b"w1186-bin",
        b"w1186-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1187 / ADR-1533: Accept + frag-text + frag-binary + ping/pong + close.
#[test]
fn http_serve_upgrade_terminate_accept_fragmented_text_fragmented_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"w1187-frag-text",
        &[0xff, 0x00, 0x7f, b'b', b'i', b'n', b'7'],
        b"w1187-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1188 / ADR-1534: Accept + N=3 multi-continuation text/binary + ping/pong + close.
#[test]
fn http_serve_upgrade_terminate_accept_multi_continuation_n3_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_fragmented_text_fragmented_binary_ping_then_close(
        &mut client,
        b"w1188-multi-cont-text",
        b"w1188-multi-bin",
        b"w1188-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1189 / ADR-1535: Accept + N=3 text with control-during-fragment ping + N=3 binary + close.
#[test]
fn http_serve_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n3_text_optional_mid_ping_n3_binary_ping_then_close(
        &mut client,
        b"w1189-ctrl-frag-text",
        Some(b"mid-frag-ping"),
        b"w1189-ctrl-bin",
        b"w1189-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1190 / ADR-1536: Accept + N=3 text + N=3 binary with control-during-fragment ping + close.
#[test]
fn http_serve_upgrade_terminate_accept_control_during_fragment_n3_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n3_optional_mid_pings_then_close(
        &mut client,
        b"w1190-text",
        None,
        b"w1190-ctrl-bin",
        Some(b"bin-mid-ping"),
        b"w1190-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1191: N=4 frag-text → N=3 frag-binary → ping/pong → close.
fn echo_n4_text_n3_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont3");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3) = split_three(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary mid cont");
    client
        .write_all(&encode_client_continuation_end(b3, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=3 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1191 / ADR-1537: Accept + N=4 multi-continuation text + N=3 binary + ping/pong + close.
#[test]
fn http_serve_upgrade_terminate_accept_multi_continuation_n4_text_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n3_binary_ping_then_close(
        &mut client,
        b"w1191-n4-text-abcd",
        b"w1191-bin",
        b"w1191-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1192: N=4 frag-text → N=4 frag-binary → ping/pong → close.
fn echo_n4_text_n4_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont3");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont3");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1192 / ADR-1538: Accept + N=4 text + N=4 binary multi-continuation + ping/pong + close.
#[test]
fn http_serve_upgrade_terminate_accept_multi_continuation_n4_text_and_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close(
        &mut client,
        b"w1192-n4-text-abcd",
        b"w1192-n4-bin-efgh",
        b"w1192-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1193: N=4 text with ping between cont1 and cont2 → N=4 binary → ping/pong → close.
fn echo_n4_text_mid_final_ping_n4_binary_then_close(
    client: &mut TcpStream,
    text: &[u8],
    mid_final_ping: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    let frame = encode_client_ping_frame(mid_final_ping, [0xfe, 0xed, 0xfa, 0xce]);
    client.write_all(&frame).expect("write mid↔final ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(
        &pong[..],
        mid_final_ping,
        "server must pong mid↔final ping without aborting reassembly"
    );
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 text after mid↔final ping");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont3");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1193 / ADR-1539: Accept + N=4 text mid↔final control ping + N=4 binary + close.
#[test]
fn http_serve_upgrade_terminate_accept_mid_final_control_n4_text_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_mid_final_ping_n4_binary_then_close(
        &mut client,
        b"w1193-n4-text-abcd",
        b"mid-final-ping",
        b"w1193-n4-bin-efgh",
        b"w1193-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1194: N=4 text → N=4 binary with ping between cont1 and cont2 → ping/pong → close.
fn echo_n4_text_n4_binary_mid_final_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    mid_final_ping: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x55, 0x66, 0x77, 0x88]))
        .expect("write binary cont1");
    let frame = encode_client_ping_frame(mid_final_ping, [0xbe, 0xef, 0xca, 0xfe]);
    client.write_all(&frame).expect("write binary mid↔final ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(
        &pong[..],
        mid_final_ping,
        "server must pong binary mid↔final ping without aborting reassembly"
    );
    client
        .write_all(&encode_client_continuation_mid(b3, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(
        &echo[..],
        binary,
        "server must reassemble N=4 binary after mid↔final ping"
    );

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1194 / ADR-1540: Accept + N=4 text + N=4 binary mid↔final control ping + close.
#[test]
fn http_serve_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_mid_final_ping_then_close(
        &mut client,
        b"w1194-n4-text-abcd",
        b"w1194-n4-bin-efgh",
        b"bin-mid-final-ping",
        b"w1194-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1195: N=5 frag-text → N=4 frag-binary → ping/pong → close.
fn echo_n5_text_n4_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4, t5) = split_five(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_mid(t4, [0x55, 0x66, 0x77, 0x88]))
        .expect("write text cont3");
    client
        .write_all(&encode_client_continuation_end(t5, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=5 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1195 / ADR-1541: Accept + N=5 text unbounded-N first slice + N=4 binary + ping/close.
#[test]
fn http_serve_upgrade_terminate_accept_multi_continuation_n5_text_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n5_text_n4_binary_ping_then_close(
        &mut client,
        b"w1195-n5-text-abcde",
        b"w1195-n4-bin-efgh",
        b"w1195-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1196: N=5 frag-text → N=5 frag-binary → ping/pong → close.
fn echo_n5_text_n5_binary_ping_then_close(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4, t5) = split_five(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_mid(t4, [0x55, 0x66, 0x77, 0x88]))
        .expect("write text cont3");
    client
        .write_all(&encode_client_continuation_end(t5, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=5 fragmented text");

    let (b1, b2, b3, b4, b5) = split_five(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_mid(b4, [0xfe, 0xdc, 0xba, 0x98]))
        .expect("write binary cont3");
    client
        .write_all(&encode_client_continuation_end(b5, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=5 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close");
    client.flush().ok();
    read_server_close_frame(client);
}

/// W1196 / ADR-1542: Accept + N=5 text + N=5 binary unbounded-N + ping/close.
#[test]
fn http_serve_upgrade_terminate_accept_multi_continuation_n5_text_and_binary_ping_and_close() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n5_text_n5_binary_ping_then_close(
        &mut client,
        b"w1196-n5-text-abcde",
        b"w1196-n5-bin-fghij",
        b"w1196-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1197: N=4 text → N=4 binary → ping → close(status, reason) → assert echo.
fn echo_n4_text_n4_binary_ping_then_close_status_reason(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
    status: u16,
    reason: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame_with_reason(status, reason, [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close with reason");
    client.flush().ok();
    let close_payload = read_server_close_frame_payload(client);
    let mut expect = Vec::with_capacity(2 + reason.len());
    expect.extend_from_slice(&status.to_be_bytes());
    expect.extend_from_slice(reason);
    assert_eq!(
        close_payload, expect,
        "server must echo close status+reason as unmasked close payload"
    );
}

/// W1197 / ADR-1543: Accept + lifecycle + close status+reason echo.
#[test]
fn http_serve_upgrade_terminate_accept_text_close_status_reason_echo() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_status_reason(
        &mut client,
        b"w1197-n4-text-abcd",
        b"w1197-n4-bin-efgh",
        b"w1197-ping",
        1000,
        b"bye",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1199: N=4 text → N=4 binary → ping → close(len=1) → assert no close echo.
fn echo_n4_text_n4_binary_ping_then_close_len1_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_close_frame_with_payload(&[0x00], [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close len=1");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1205: N=4 text → N=4 binary → ping → close(1005) → assert no close echo.
fn echo_n4_text_n4_binary_ping_then_close_status_1005_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    // 1005 is reserved / MUST NOT appear on the wire (RFC6455 §7.4.1).
    let close = encode_client_close_frame_with_reason(1005, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close status 1005");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1206: N=4 text → N=4 binary → ping → close(1000, non-UTF-8 reason) → assert no close echo.
fn echo_n4_text_n4_binary_ping_then_close_reason_non_utf8_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    // Legal status 1000 + illegal UTF-8 reason (0xFF) — RFC6455 §5.5.1.
    let close = encode_client_close_frame_with_reason(1000, &[0xFF], [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close non-UTF-8 reason");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1206 / ADR-1552: Accept + lifecycle + close reason non-UTF-8 fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_close_reason_non_utf8_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_reason_non_utf8_fail_closed(
        &mut client,
        b"w1206-n4-text-abcd",
        b"w1206-n4-bin-efgh",
        b"w1206-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1207: N=4 text → N=4 binary → ping → close(1004) → assert no close echo.
fn echo_n4_text_n4_binary_ping_then_close_status_1004_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    // 1004 is reserved (RFC6455 §7.4.1) — must not appear on the wire.
    let close = encode_client_close_frame_with_reason(1004, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write close status 1004");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1207 / ADR-1553: Accept + FIN=1 text non-UTF-8 fail-closed (no text echo).
#[test]
fn http_serve_upgrade_terminate_accept_text_non_utf8_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN=1 text with illegal UTF-8 payload (0xFF) — RFC6455 §5.6.
    let bad = encode_client_data_frame(0x81, &[0xFF], [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&bad).expect("write non-UTF-8 text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1207 / ADR-1553: Accept + fragmented text non-UTF-8 fail-closed (no text echo).
#[test]
fn http_serve_upgrade_terminate_accept_fragmented_text_non_utf8_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN=0 text [0xC2] + FIN=1 cont [0xFF] → reassembled non-UTF-8.
    client
        .write_all(&encode_client_text_fragment_start(&[0xC2], [0x11, 0x22, 0x33, 0x44]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_end(&[0xFF], [0x55, 0x66, 0x77, 0x88]))
        .expect("write text end cont");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1207 / ADR-1553: Accept + lifecycle + illegal close status 1004 fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1004_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_status_1004_fail_closed(
        &mut client,
        b"w1207-n4-text-abcd",
        b"w1207-n4-bin-efgh",
        b"w1207-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1208: N=4 text → N=4 binary → ping → FIN=0 close → assert no close echo.
fn echo_n4_text_n4_binary_ping_then_fin0_close_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write ping");
    client.flush().ok();
    let pong = decode_server_pong_frame(client);
    assert_eq!(&pong[..], ping, "server must echo ping payload as unmasked pong");

    let close = encode_client_fin0_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write FIN=0 close");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1208 / ADR-1554: Accept + lifecycle + FIN=0 close fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_fin0_close_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_fin0_close_fail_closed(
        &mut client,
        b"w1208-n4-text-abcd",
        b"w1208-n4-bin-efgh",
        b"w1208-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1208 / ADR-1554: Accept + unexpected continuation (opcode 0x0) fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_unexpected_continuation_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let bad = encode_client_unexpected_continuation_frame(b"orphan", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&bad).expect("write unexpected continuation");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1208 / ADR-1554: Accept + unsolicited client pong fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_unsolicited_pong_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let bad = encode_client_pong_frame(b"unsolicited", [0x11, 0x22, 0x33, 0x44]);
    client.write_all(&bad).expect("write unsolicited pong");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1208 / ADR-1554: Accept + FIN=0 close mid-fragment fail-closed (control FIN centralization).
#[test]
fn http_serve_upgrade_terminate_accept_fin0_close_during_fragment_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // Start N=4 text; after frag1 send FIN=0 close — must abort with no text echo.
    client
        .write_all(&encode_client_text_fragment_start(
            b"w120",
            [0xde, 0xad, 0xbe, 0xef],
        ))
        .expect("write text frag1");
    let close = encode_client_fin0_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write FIN=0 close mid-fragment");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1205 / ADR-1551: Accept + lifecycle + illegal close status 1005 fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1005_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_status_1005_fail_closed(
        &mut client,
        b"w1205-n4-text-abcd",
        b"w1205-n4-bin-efgh",
        b"w1205-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1199 / ADR-1545: Accept + lifecycle + close body len=1 fail-closed.
#[test]
fn http_serve_upgrade_terminate_accept_close_len1_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_len1_fail_closed(
        &mut client,
        b"w1199-n4-text-abcd",
        b"w1199-n4-bin-efgh",
        b"w1199-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1204 / ADR-1550: Accept + oversized control ping (len7=126) → fail-closed (no pong).
#[test]
fn http_serve_upgrade_terminate_accept_oversized_ping_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|ping with len7=126 — must not be accepted / must not pong.
    let oversized = encode_client_oversized_ping_len7_126([0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&oversized).expect("write oversized ping");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1203 / ADR-1549: Accept + data-reserved opcode 0x3 → fail-closed (no echo).
#[test]
fn http_serve_upgrade_terminate_accept_reserved_opcode_0x3_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|data-reserved 0x3 (0x83) — must not be accepted.
    let reserved = encode_client_data_frame(0x83, b"w1203-reserved-0x3", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&reserved).expect("write reserved 0x3");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1202 / ADR-1548 / W1209: Accept + reserved control opcode 0xB → close(1002).
#[test]
fn http_serve_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|reserved-control 0xB (0x8B) — must not be accepted.
    let reserved = encode_client_data_frame(0x8B, b"w1202-reserved-0xb", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&reserved).expect("write reserved 0xB");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1201 / W1209: N=4 text → N=4 binary → FIN=0 ping → close(1002).
fn echo_n4_text_n4_binary_then_fin0_ping_fail_closed(
    client: &mut TcpStream,
    text: &[u8],
    binary: &[u8],
    ping: &[u8],
) {
    let (t1, t2, t3, t4) = split_four(text);
    client
        .write_all(&encode_client_text_fragment_start(t1, [0xde, 0xad, 0xbe, 0xef]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_mid(t2, [0xca, 0xfe, 0xba, 0xbe]))
        .expect("write text cont1");
    client
        .write_all(&encode_client_continuation_mid(t3, [0x11, 0x22, 0x33, 0x44]))
        .expect("write text cont2");
    client
        .write_all(&encode_client_continuation_end(t4, [0x01, 0x02, 0x03, 0x04]))
        .expect("write text end cont");
    client.flush().ok();
    let echo = decode_server_text_frame(client);
    assert_eq!(&echo[..], text, "server must reassemble N=4 fragmented text");

    let (b1, b2, b3, b4) = split_four(binary);
    client
        .write_all(&encode_client_binary_fragment_start(b1, [0xaa, 0xbb, 0xcc, 0xdd]))
        .expect("write binary frag1");
    client
        .write_all(&encode_client_continuation_mid(b2, [0x12, 0x34, 0x56, 0x78]))
        .expect("write binary cont1");
    client
        .write_all(&encode_client_continuation_mid(b3, [0x9a, 0xbc, 0xde, 0xf0]))
        .expect("write binary cont2");
    client
        .write_all(&encode_client_continuation_end(b4, [0x99, 0xaa, 0xbb, 0xcc]))
        .expect("write binary end cont");
    client.flush().ok();
    let echo = decode_server_binary_frame(client);
    assert_eq!(&echo[..], binary, "server must reassemble N=4 fragmented binary");

    let frame = encode_client_fin0_ping_frame(ping, [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write FIN=0 ping");
    client.flush().ok();
    assert_server_protocol_close_1002(client);
}

/// W1201 / ADR-1547 / W1209: Accept + lifecycle + FIN=0 ping → close(1002).
#[test]
fn http_serve_upgrade_terminate_accept_fin0_ping_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_then_fin0_ping_fail_closed(
        &mut client,
        b"w1201-n4-text-abcd",
        b"w1201-n4-bin-efgh",
        b"w1201-fin0-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1200 / ADR-1546: Accept + unmasked text → fail-closed (no text echo).
#[test]
fn http_serve_upgrade_terminate_accept_unmasked_text_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|text (0x81) without MASK bit — client MUST mask.
    let unmasked = encode_client_unmasked_data_frame(0x81, b"w1200-unmasked-bad");
    client.write_all(&unmasked).expect("write unmasked text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1198 / ADR-1544 / W1209: Accept + RSV1 text → close(1002).
#[test]
fn http_serve_upgrade_terminate_accept_rsv_text_fail_closed() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN|RSV1|text (0xC1) — nonzero RSV → close(1002).
    let rsv_text = encode_client_data_frame(0xC1, b"w1198-rsv-bad", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&rsv_text).expect("write RSV1 text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1209 / ADR-1555: Accept + RSV1 text → close(1002) product reply (slice 40).
#[test]
fn http_serve_upgrade_terminate_accept_rsv_text_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let rsv_text = encode_client_data_frame(0xC1, b"w1209-rsv-1002", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&rsv_text).expect("write RSV1 text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1209 / ADR-1555: Accept + reserved opcode 0xB → close(1002) product reply (slice 41).
#[test]
fn http_serve_upgrade_terminate_accept_reserved_opcode_0xb_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let reserved = encode_client_data_frame(0x8B, b"w1209-reserved-0xb", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&reserved).expect("write reserved 0xB");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1209 / ADR-1555: Accept + FIN=0 ping → close(1002) product reply (slice 42).
#[test]
fn http_serve_upgrade_terminate_accept_fin0_ping_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let frame = encode_client_fin0_ping_frame(b"w1209-fin0-ping", [0x01, 0x02, 0x03, 0x04]);
    client.write_all(&frame).expect("write FIN=0 ping");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1209 / ADR-1555: Accept + unexpected continuation → close(1002) product reply (slice 43).
#[test]
fn http_serve_upgrade_terminate_accept_unexpected_continuation_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let bad = encode_client_unexpected_continuation_frame(b"orphan", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&bad).expect("write unexpected continuation");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1210 / ADR-1556: Accept + unmasked text → close(1002) product reply (slice 44).
#[test]
fn http_serve_upgrade_terminate_accept_unmasked_text_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let unmasked = encode_client_unmasked_data_frame(0x81, b"w1210-unmasked-1002");
    client.write_all(&unmasked).expect("write unmasked text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1210 / ADR-1556: Accept + oversized ping → close(1002) product reply (slice 45).
#[test]
fn http_serve_upgrade_terminate_accept_oversized_ping_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let oversized = encode_client_oversized_ping_len7_126([0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&oversized).expect("write oversized ping");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1210 / ADR-1556: Accept + close body len=1 → close(1002) product reply (slice 46).
#[test]
fn http_serve_upgrade_terminate_accept_close_len1_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_len1_fail_closed(
        &mut client,
        b"w1210-n4-text-abcd",
        b"w1210-n4-bin-efgh",
        b"w1210-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1210 / ADR-1556: Accept + illegal close status 1005 → close(1002) product reply (slice 47).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1005_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_status_1005_fail_closed(
        &mut client,
        b"w1210-n4-text-abcd",
        b"w1210-n4-bin-efgh",
        b"w1210-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1212 / ADR-1558: Accept + FIN=1 text non-UTF-8 → close(1002) product reply (slice 52).
#[test]
fn http_serve_upgrade_terminate_accept_text_non_utf8_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let bad = encode_client_data_frame(0x81, &[0xFF], [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&bad).expect("write non-UTF-8 text");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1212 / ADR-1558: Accept + close reason non-UTF-8 → close(1002) product reply (slice 53).
#[test]
fn http_serve_upgrade_terminate_accept_close_reason_non_utf8_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_reason_non_utf8_fail_closed(
        &mut client,
        b"w1212-n4-text-abcd",
        b"w1212-n4-bin-efgh",
        b"w1212-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1212 / ADR-1558: Accept + unsolicited client pong → close(1002) product reply (slice 54).
#[test]
fn http_serve_upgrade_terminate_accept_unsolicited_pong_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let bad = encode_client_pong_frame(b"w1212-unsolicited", [0x11, 0x22, 0x33, 0x44]);
    client.write_all(&bad).expect("write unsolicited pong");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1212 / ADR-1558: Accept + illegal close status 1004 → close(1002) product reply (slice 55).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1004_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_close_status_1004_fail_closed(
        &mut client,
        b"w1212-n4-text-abcd",
        b"w1212-n4-bin-efgh",
        b"w1212-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1213 / ADR-1559: Accept + fragmented text non-UTF-8 → close(1002) product reply (slice 56).
#[test]
fn http_serve_upgrade_terminate_accept_fragmented_text_non_utf8_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    // FIN=0 text [0xC2] + FIN=1 cont [0xFF] → reassembled non-UTF-8.
    client
        .write_all(&encode_client_text_fragment_start(&[0xC2], [0x11, 0x22, 0x33, 0x44]))
        .expect("write text frag1");
    client
        .write_all(&encode_client_continuation_end(&[0xFF], [0x55, 0x66, 0x77, 0x88]))
        .expect("write text end cont");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1213 / ADR-1559: Accept + lifecycle + FIN=0 close → close(1002) product reply (slice 57).
#[test]
fn http_serve_upgrade_terminate_accept_fin0_close_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    echo_n4_text_n4_binary_ping_then_fin0_close_fail_closed(
        &mut client,
        b"w1213-n4-text-abcd",
        b"w1213-n4-bin-efgh",
        b"w1213-ping",
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1213 / ADR-1559: Accept + FIN=0 close mid-fragment → close(1002) product reply (slice 58).
#[test]
fn http_serve_upgrade_terminate_accept_fin0_close_during_fragment_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    client
        .write_all(&encode_client_text_fragment_start(
            b"w121",
            [0xde, 0xad, 0xbe, 0xef],
        ))
        .expect("write text frag1");
    let close = encode_client_fin0_close_frame([0x99, 0xaa, 0xbb, 0xcc]);
    client.write_all(&close).expect("write FIN=0 close mid-fragment");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1213 / ADR-1559: Accept + reserved data opcode 0x3 → close(1002) product reply (slice 59).
#[test]
fn http_serve_upgrade_terminate_accept_reserved_opcode_0x3_close_1002() {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let reserved = encode_client_data_frame(0x83, b"w1213-reserved-0x3", [0xde, 0xad, 0xbe, 0xef]);
    client.write_all(&reserved).expect("write reserved 0x3");
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

// --- W1218 / ADR-1564: illegal close-status deepen first cluster ---

fn run_terminate_illegal_close_status_close_1002(status: u16, tag: &str) {
    let _serial = terminate_test_lock();
    let serve_port = free_port();
    let (path, rx, serve_join, _max_env) = start_terminate_serve(serve_port);
    let (mut client, head_text) = handshake_terminate_client(serve_port);
    assert!(head_text.contains("101") && head_text.contains(RFC_ACCEPT), "{head_text}");

    let close = encode_client_close_frame_with_reason(status, b"", [0x99, 0xaa, 0xbb, 0xcc]);
    client
        .write_all(&close)
        .unwrap_or_else(|_| panic!("write close status {status} ({tag})"));
    client.flush().ok();
    assert_server_protocol_close_1002(&mut client);

    let _ = client.shutdown(Shutdown::Both);
    drop(client);
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
}

/// W1218 / ADR-1564: Accept + illegal close status 1006 → close(1002) (slice 75).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1006_close_1002() {
    run_terminate_illegal_close_status_close_1002(1006, "1006");
}

/// W1218 / ADR-1564: Accept + illegal close status 1015 → close(1002) (slice 76).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1015_close_1002() {
    run_terminate_illegal_close_status_close_1002(1015, "1015");
}

/// W1218 / ADR-1564: Accept + illegal close status 999 (0–999 axis) → close(1002) (slice 77).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_999_close_1002() {
    run_terminate_illegal_close_status_close_1002(999, "999");
}

/// W1219 / ADR-1565: Accept + illegal close status 0 (0–999 lower bound) → close(1002) (slice 81).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_0_close_1002() {
    run_terminate_illegal_close_status_close_1002(0, "0");
}

/// W1219 / ADR-1565: Accept + illegal close status 500 (0–999 midpoint) → close(1002) (slice 82).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_500_close_1002() {
    run_terminate_illegal_close_status_close_1002(500, "500");
}

// --- W1228 / ADR-1574: illegal close-status fail_closed soft-twin first cluster ---
// Soft-twins of W1218 *_close_1002; behavior = close(1002).

/// W1228 / ADR-1574: Accept + illegal close status 1006 fail_closed name twin (slice 115).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1006_fail_closed() {
    run_terminate_illegal_close_status_close_1002(1006, "fc1006");
}

/// W1228 / ADR-1574: Accept + illegal close status 1015 fail_closed name twin (slice 116).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1015_fail_closed() {
    run_terminate_illegal_close_status_close_1002(1015, "fc1015");
}

/// W1228 / ADR-1574: Accept + illegal close status 999 fail_closed name twin (slice 117).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_999_fail_closed() {
    run_terminate_illegal_close_status_close_1002(999, "fc999");
}

// --- W1229 / ADR-1575: illegal close-status fail_closed soft-twin second cluster (plain) ---
// Soft-twins of W1219 plain *_close_1002; behavior = close(1002).

/// W1229 / ADR-1575: Accept + illegal close status 0 fail_closed name twin (slice 121).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_0_fail_closed() {
    run_terminate_illegal_close_status_close_1002(0, "fc0");
}

/// W1229 / ADR-1575: Accept + illegal close status 500 fail_closed name twin (slice 122).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_500_fail_closed() {
    run_terminate_illegal_close_status_close_1002(500, "fc500");
}

// --- W1231 / ADR-1577: illegal close-status deepen third cluster ---
// Sample points in already-rejected 0..=999 (≠ close-code full GA).

/// W1231 / ADR-1577: Accept + illegal close status 1 (0–999 low) → close(1002) (slice 125).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1_close_1002() {
    run_terminate_illegal_close_status_close_1002(1, "1");
}

/// W1231 / ADR-1577: Accept + illegal close status 100 (0–999 mid) → close(1002) (slice 126).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_100_close_1002() {
    run_terminate_illegal_close_status_close_1002(100, "100");
}

/// W1231 / ADR-1577: Accept + illegal close status 998 (0–999 near upper) → close(1002) (slice 127).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_998_close_1002() {
    run_terminate_illegal_close_status_close_1002(998, "998");
}

// --- W1232 / ADR-1578: illegal close-status fail_closed soft-twin (align W1231) ---
// Soft-twins of W1231 *_close_1002; behavior = close(1002).

/// W1232 / ADR-1578: Accept + illegal close status 1 fail_closed name twin (slice 129).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_1_fail_closed() {
    run_terminate_illegal_close_status_close_1002(1, "fc1");
}

/// W1232 / ADR-1578: Accept + illegal close status 100 fail_closed name twin (slice 130).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_100_fail_closed() {
    run_terminate_illegal_close_status_close_1002(100, "fc100");
}

/// W1232 / ADR-1578: Accept + illegal close status 998 fail_closed name twin (slice 131).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_998_fail_closed() {
    run_terminate_illegal_close_status_close_1002(998, "fc998");
}

// --- W1233 / ADR-1579: illegal close-status near-bound deepen (plain 2/997) ---
// Sample points near W1231 1/998; ≠ close-code full GA.

/// W1233 / ADR-1579: Accept + illegal close status 2 (near 1) → close(1002) (slice 135).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_2_close_1002() {
    run_terminate_illegal_close_status_close_1002(2, "2");
}

/// W1233 / ADR-1579: Accept + illegal close status 997 (near 998) → close(1002) (slice 136).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_997_close_1002() {
    run_terminate_illegal_close_status_close_1002(997, "997");
}

// --- W1234 / ADR-1580: illegal close-status fail_closed soft-twin (plain, align W1233) ---
// Soft-twins of W1233 plain *_close_1002; behavior = close(1002).

/// W1234 / ADR-1580: Accept + illegal close status 2 fail_closed name twin (slice 139).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_2_fail_closed() {
    run_terminate_illegal_close_status_close_1002(2, "fc2");
}

/// W1234 / ADR-1580: Accept + illegal close status 997 fail_closed name twin (slice 140).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_997_fail_closed() {
    run_terminate_illegal_close_status_close_1002(997, "fc997");
}

// --- W1235 / ADR-1581: illegal close-status near-bound deepen (plain 3/996) ---
// Sample points near W1233 2/997; ≠ close-code full GA.

/// W1235 / ADR-1581: Accept + illegal close status 3 (near 2) → close(1002) (slice 143).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_3_close_1002() {
    run_terminate_illegal_close_status_close_1002(3, "3");
}

/// W1235 / ADR-1581: Accept + illegal close status 996 (near 997) → close(1002) (slice 144).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_996_close_1002() {
    run_terminate_illegal_close_status_close_1002(996, "996");
}

// --- W1236 / ADR-1582: illegal close-status fail_closed soft-twin (plain, align W1235) ---
// Soft-twins of W1235 plain *_close_1002; behavior = close(1002).

/// W1236 / ADR-1582: Accept + illegal close status 3 fail_closed name twin (slice 147).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_3_fail_closed() {
    run_terminate_illegal_close_status_close_1002(3, "fc3");
}

/// W1236 / ADR-1582: Accept + illegal close status 996 fail_closed name twin (slice 148).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_996_fail_closed() {
    run_terminate_illegal_close_status_close_1002(996, "fc996");
}

// --- W1237 / ADR-1583: illegal close-status near-bound deepen (plain 4/995) ---
// Sample points near W1235 3/996; ≠ close-code full GA.

/// W1237 / ADR-1583: Accept + illegal close status 4 (near 3) → close(1002) (slice 151).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_4_close_1002() {
    run_terminate_illegal_close_status_close_1002(4, "4");
}

/// W1237 / ADR-1583: Accept + illegal close status 995 (near 996) → close(1002) (slice 152).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_995_close_1002() {
    run_terminate_illegal_close_status_close_1002(995, "995");
}

// --- W1238 / ADR-1584: illegal close-status fail_closed soft-twin (plain, align W1237) ---
// Soft-twins of W1237 plain *_close_1002; behavior = close(1002).

/// W1238 / ADR-1584: Accept + illegal close status 4 fail_closed name twin (slice 155).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_4_fail_closed() {
    run_terminate_illegal_close_status_close_1002(4, "fc4");
}

/// W1238 / ADR-1584: Accept + illegal close status 995 fail_closed name twin (slice 156).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_995_fail_closed() {
    run_terminate_illegal_close_status_close_1002(995, "fc995");
}

// --- W1239 / ADR-1585: illegal close-status near-bound deepen (plain 5/994) ---
// Sample points near W1237 4/995; ≠ close-code full GA.

/// W1239 / ADR-1585: Accept + illegal close status 5 (near 4) → close(1002) (slice 159).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_5_close_1002() {
    run_terminate_illegal_close_status_close_1002(5, "5");
}

/// W1239 / ADR-1585: Accept + illegal close status 994 (near 995) → close(1002) (slice 160).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_994_close_1002() {
    run_terminate_illegal_close_status_close_1002(994, "994");
}

// --- W1240 / ADR-1586: illegal close-status fail_closed soft-twin (plain, align W1239) ---
// Soft-twins of W1239 plain *_close_1002; behavior = close(1002).

/// W1240 / ADR-1586: Accept + illegal close status 5 fail_closed name twin (slice 163).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_5_fail_closed() {
    run_terminate_illegal_close_status_close_1002(5, "fc5");
}

/// W1240 / ADR-1586: Accept + illegal close status 994 fail_closed name twin (slice 164).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_994_fail_closed() {
    run_terminate_illegal_close_status_close_1002(994, "fc994");
}

// --- W1241 / ADR-1587: illegal close-status near-bound deepen (plain 6/993) ---
// Sample points near W1239 5/994; ≠ close-code full GA.

/// W1241 / ADR-1587: Accept + illegal close status 6 (near 5) → close(1002) (slice 167).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_6_close_1002() {
    run_terminate_illegal_close_status_close_1002(6, "6");
}

/// W1241 / ADR-1587: Accept + illegal close status 993 (near 994) → close(1002) (slice 168).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_993_close_1002() {
    run_terminate_illegal_close_status_close_1002(993, "993");
}

// --- W1242 / ADR-1588: illegal close-status fail_closed soft-twin (plain, align W1241) ---
// Soft-twins of W1241 plain *_close_1002; behavior = close(1002).

/// W1242 / ADR-1588: Accept + illegal close status 6 fail_closed name twin (slice 171).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_6_fail_closed() {
    run_terminate_illegal_close_status_close_1002(6, "fc6");
}

/// W1242 / ADR-1588: Accept + illegal close status 993 fail_closed name twin (slice 172).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_993_fail_closed() {
    run_terminate_illegal_close_status_close_1002(993, "fc993");
}

// --- W1243 / ADR-1589: illegal close-status near-bound deepen (plain 7/992) ---
// Sample points near W1241 6/993; ≠ close-code full GA.

/// W1243 / ADR-1589: Accept + illegal close status 7 (near 6) → close(1002) (slice 175).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_7_close_1002() {
    run_terminate_illegal_close_status_close_1002(7, "7");
}

/// W1243 / ADR-1589: Accept + illegal close status 992 (near 993) → close(1002) (slice 176).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_992_close_1002() {
    run_terminate_illegal_close_status_close_1002(992, "992");
}

// --- W1244 / ADR-1590: illegal close-status fail_closed soft-twin (plain, align W1243) ---
// Soft-twins of W1243 plain *_close_1002; behavior = close(1002).

/// W1244 / ADR-1590: Accept + illegal close status 7 fail_closed name twin (slice 179).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_7_fail_closed() {
    run_terminate_illegal_close_status_close_1002(7, "fc7");
}

/// W1244 / ADR-1590: Accept + illegal close status 992 fail_closed name twin (slice 180).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_992_fail_closed() {
    run_terminate_illegal_close_status_close_1002(992, "fc992");
}

// --- W1245 / ADR-1591: illegal close-status near-bound deepen (plain 8/991) ---
// Sample points near W1243 7/992; ≠ close-code full GA.

/// W1245 / ADR-1591: Accept + illegal close status 8 (near 7) → close(1002) (slice 183).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_8_close_1002() {
    run_terminate_illegal_close_status_close_1002(8, "8");
}

/// W1245 / ADR-1591: Accept + illegal close status 991 (near 992) → close(1002) (slice 184).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_991_close_1002() {
    run_terminate_illegal_close_status_close_1002(991, "991");
}

// --- W1246 / ADR-1592: illegal close-status fail_closed soft-twin (plain, align W1245) ---
// Soft-twins of W1245 plain *_close_1002; behavior = close(1002).

/// W1246 / ADR-1592: Accept + illegal close status 8 fail_closed name twin (slice 187).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_8_fail_closed() {
    run_terminate_illegal_close_status_close_1002(8, "fc8");
}

/// W1246 / ADR-1592: Accept + illegal close status 991 fail_closed name twin (slice 188).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_991_fail_closed() {
    run_terminate_illegal_close_status_close_1002(991, "fc991");
}

// --- W1247 / ADR-1593: illegal close-status near-bound deepen (plain 9/990) ---
// Sample points near W1245 8/991; ≠ close-code full GA.

/// W1247 / ADR-1593: Accept + illegal close status 9 (near 8) → close(1002) (slice 191).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_9_close_1002() {
    run_terminate_illegal_close_status_close_1002(9, "9");
}

/// W1247 / ADR-1593: Accept + illegal close status 990 (near 991) → close(1002) (slice 192).
#[test]
fn http_serve_upgrade_terminate_accept_close_status_990_close_1002() {
    run_terminate_illegal_close_status_close_1002(990, "990");
}

// --- W1248 / ADR-1594: illegal near-bound sampling axis soft-close (plain soft debt) ---
// Soft-twins of W1247 plain *_close_1002; behavior = close(1002). Axis deepen frozen.

/// W1248 / ADR-1594: Accept + illegal close status 9 fail_closed name twin.
#[test]
fn http_serve_upgrade_terminate_accept_close_status_9_fail_closed() {
    run_terminate_illegal_close_status_close_1002(9, "fc9");
}

/// W1248 / ADR-1594: Accept + illegal close status 990 fail_closed name twin.
#[test]
fn http_serve_upgrade_terminate_accept_close_status_990_fail_closed() {
    run_terminate_illegal_close_status_close_1002(990, "fc990");
}
