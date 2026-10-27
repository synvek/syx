//! Http.serve overlapping multi-connection first slice.
//! Two plain GETs: /slow holds until a release file appears; /fast must
//! complete while /slow is still in-flight (≠ serial accept). ≠ SLA / TLS.
//! overlap × HMR cross nail — slow plain GET holds while
//! same-port Upgrade/HMR completes (101 + Accept relay + masked frame).
//! ≠ WSS/HMR-over-TLS / frame terminator / Playwright / SLA.

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{mpsc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{run_file_printed, DriverOptions};

/// Fixed Accept token for relay nail — not derived from Sec-WebSocket-Key.
const RELAY_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

fn overlap_test_lock() -> MutexGuard<'static, ()> {
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

fn http_get(port: u16, path: &str, timeout: Duration) -> (u16, String, Instant) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream.set_read_timeout(Some(timeout)).ok();
    stream.set_write_timeout(Some(timeout)).ok();
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).expect("write");
    let mut buf = Vec::new();
    let _ = stream.read_to_end(&mut buf);
    let done_at = Instant::now();
    let text = String::from_utf8_lossy(&buf).into_owned();
    let status = text
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status, text, done_at)
}

#[test]
fn http_serve_overlap_slow_fast_plain_get() {
    let _serial = overlap_test_lock();
    // Two connections: accept both before either finishes.
    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "2");

    let port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_overlap");
    let _ = std::fs::create_dir_all(&dir);
    let release = dir.join(format!("release_{port}.flag"));
    let _ = std::fs::remove_file(&release);
    let release_path = release.to_string_lossy().replace('\\', "/");

    let src = format!(
        r#"
import Net
import IO

func handle(req: Object) -> Object {{
    let path = req.path
    if path == "/slow" {{
        while !File.exists("{release_path}") {{
            __syx_blocking_sleep(20)
        }}
        return Http.response(200, "slow-done")
    }}
    if path == "/fast" {{
        return Http.response(200, "fast-done")
    }}
    return Http.response(404, "no")
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {port}, handler: "handle")
}}
"#
    );
    let path = dir.join(format!("overlap_{port}.sy"));
    std::fs::write(&path, &src).expect("write");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "serve did not listen on {port}"
    );

    let (slow_tx, slow_rx) = mpsc::channel();
    let slow_join = thread::spawn(move || {
        let r = http_get(port, "/slow", Duration::from_secs(15));
        let _ = slow_tx.send(r);
    });

    // Give /slow time to enter the handler hold loop.
    thread::sleep(Duration::from_millis(150));

    let fast_started = Instant::now();
    let (fast_status, fast_body, fast_done) = http_get(port, "/fast", Duration::from_secs(5));
    assert_eq!(fast_status, 200, "fast status: {fast_body}");
    assert!(
        fast_body.contains("fast-done"),
        "fast body: {fast_body}"
    );
    let fast_elapsed = fast_done.duration_since(fast_started);
    assert!(
        fast_elapsed < Duration::from_millis(1500),
        "fast must complete while slow holds; elapsed={fast_elapsed:?}"
    );

    // Release slow; it must still have been in-flight when fast finished.
    std::fs::write(&release, b"go").expect("release");
    let (slow_status, slow_body, slow_done) = slow_rx
        .recv_timeout(Duration::from_secs(15))
        .expect("slow result");
    assert_eq!(slow_status, 200, "slow status: {slow_body}");
    assert!(
        slow_body.contains("slow-done"),
        "slow body: {slow_body}"
    );
    assert!(
        slow_done > fast_done,
        "slow must finish after fast (overlap proof); fast={fast_done:?} slow={slow_done:?}"
    );

    let _ = slow_join.join();
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&release);
}

fn spawn_hmr_upstream() -> (u16, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("upstream bind");
    let port = listener.local_addr().expect("addr").port();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("upstream accept");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .ok();
        // Drain request head.
        let mut buf = Vec::new();
        let mut tmp = [0u8; 512];
        loop {
            let n = stream.read(&mut tmp).unwrap_or(0);
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n]);
            if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        let resp = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: {RELAY_ACCEPT}\r\n\
\r\n"
        );
        stream.write_all(resp.as_bytes()).expect("write upstream 101");
        stream.flush().ok();
        let mut frame_buf = [0u8; 4096];
        let n = stream.read(&mut frame_buf).unwrap_or(0);
        if n > 0 {
            let _ = stream.write_all(&frame_buf[..n]);
            let _ = stream.flush();
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

/// W1180 / ADR-1526: slow plain GET holds; same-port HMR Upgrade must finish
/// while /slow is still in-flight (overlap × HMR cross). ≠ TLS Upgrade / SLA.
#[test]
fn http_serve_overlap_slow_plain_and_hmr_upgrade() {
    let _serial = overlap_test_lock();
    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "2");

    let (up_port, up_join) = spawn_hmr_upstream();
    let port = free_port();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_overlap");
    let _ = std::fs::create_dir_all(&dir);
    let release = dir.join(format!("release_hmr_{port}.flag"));
    let _ = std::fs::remove_file(&release);
    let release_path = release.to_string_lossy().replace('\\', "/");

    let src = format!(
        r#"
import Net
import IO

func handle(req: Object) -> Object {{
    let path = req.path
    if path == "/slow" {{
        while !File.exists("{release_path}") {{
            __syx_blocking_sleep(20)
        }}
        return Http.response(200, "slow-done")
    }}
    if path == "/@vite/client" {{
        var o: Object = JSON.decode("{{}}")
        o["upgradeUpstream"] = "http://127.0.0.1:{up_port}/@vite/client"
        return o
    }}
    return Http.response(404, "no")
}}

func main() {{
    print("listening")
    Http.serve(addr: "127.0.0.1", port: {port}, handler: "handle")
}}
"#
    );
    let path = dir.join(format!("overlap_hmr_{port}.sy"));
    std::fs::write(&path, &src).expect("write");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let serve_join = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "serve did not listen on {port}"
    );

    let (slow_tx, slow_rx) = mpsc::channel();
    let slow_join = thread::spawn(move || {
        let r = http_get(port, "/slow", Duration::from_secs(20));
        let _ = slow_tx.send(r);
    });

    // Give /slow time to enter the handler hold loop.
    thread::sleep(Duration::from_millis(150));

    let hmr_started = Instant::now();
    let mut client = TcpStream::connect(("127.0.0.1", port)).expect("hmr connect");
    client
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
    client
        .set_write_timeout(Some(Duration::from_secs(5)))
        .ok();
    let req = format!(
        "GET /@vite/client HTTP/1.1\r\n\
Host: 127.0.0.1:{port}\r\n\
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
    let hmr_head_done = Instant::now();
    assert!(
        head_text.contains("101"),
        "HMR must 101 while /slow holds: {head_text}"
    );
    assert!(
        head_text.contains(RELAY_ACCEPT),
        "Accept must be relayed: {head_text}"
    );

    let payload = b"hmr-overlap-ping";
    let mask = [0x11, 0x22, 0x33, 0x44];
    let frame = encode_client_text_frame(payload, mask);
    client.write_all(&frame).expect("write hmr frame");
    let mut echo = vec![0u8; frame.len()];
    client.read_exact(&mut echo).expect("read hmr frame echo");
    let hmr_done = Instant::now();
    assert_eq!(&echo[..], &frame[..], "masked frame must echo intact");
    let hmr_elapsed = hmr_done.duration_since(hmr_started);
    assert!(
        hmr_elapsed < Duration::from_millis(2500),
        "HMR must complete while /slow holds; elapsed={hmr_elapsed:?}"
    );
    assert!(
        hmr_head_done > hmr_started,
        "timing sanity"
    );

    let _ = client.shutdown(Shutdown::Both);
    drop(client);

    // Release slow only after HMR finished (overlap proof).
    std::fs::write(&release, b"go").expect("release");
    let (slow_status, slow_body, slow_done) = slow_rx
        .recv_timeout(Duration::from_secs(15))
        .expect("slow result");
    assert_eq!(slow_status, 200, "slow status: {slow_body}");
    assert!(
        slow_body.contains("slow-done"),
        "slow body: {slow_body}"
    );
    assert!(
        slow_done > hmr_done,
        "slow must finish after HMR (overlap proof); hmr={hmr_done:?} slow={slow_done:?}"
    );

    let _ = slow_join.join();
    let _ = up_join.join();
    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve result")
        .expect("run_file_printed");
    let _ = serve_join.join();
    let (code, printed) = run_result;
    assert_eq!(code, 0, "serve exit: {}", printed.join("\n"));
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&release);
}
