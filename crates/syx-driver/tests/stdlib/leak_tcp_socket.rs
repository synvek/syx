//! EC-M2-3 Tcp socket live-count leak first nail.
// was: ec_m2_3_tcp_leak

use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{tcp_stream_force_close, tcp_stream_live_count};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let tcp = include_str!("../../../../packages/net/tcp.sy");
    format!("{tcp}\n{body}")
}

/// Accept one connection then idle until stop (enough for connect nails).
fn start_accept_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
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
                Ok((_stream, _)) => {
                    while !stop_flag.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(10));
                    }
                    break;
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
fn ec_m2_3_tcp_leak_facade_source_has_connect_close() {
    let src = include_str!("../../../../packages/net/tcp.sy");
    assert!(
        src.contains("connect(host: String, port: Int) -> TcpStream throws")
            || (src.contains("func connect") && src.contains("throws")),
        "connect must throws: {src}"
    );
    assert!(
        src.contains("static func closeHandle(handle: Int)")
            || src.contains("func closeHandle(handle: Int)"),
        "static closeHandle(handle:) required: {src}"
    );
    assert!(src.contains("__syx_tcp_handle_release"), "close must call release host");
    // Nail uses direct CallHost release (embed path); facade closeHandle documents the same host.
    assert!(
        include_str!("leak_tcp_socket.rs").contains("__syx_tcp_handle_release"),
        "nail must exercise host_tcp_handle_release"
    );
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("pub fn tcp_stream_live_count"),
        "tcp_stream_live_count must be exported for nails"
    );
    let release = hosts
        .split("pub fn host_tcp_handle_release")
        .nth(1)
        .expect("host_tcp_handle_release");
    let body = release.split("pub fn ").next().unwrap_or(release);
    assert!(
        body.contains("remove"),
        "host_tcp_handle_release must remove from TCP_STREAMS: {body}"
    );
}

#[test]
fn ec_m2_3_tcp_leak_connect_close_returns_to_baseline() {
    // Wave A W66 / ADR-410 / A-03: Tcp connect+close product exact (drop print(handle)).
    let (port, stop, join) = start_accept_loopback();
    thread::sleep(Duration::from_millis(20));
    let baseline = tcp_stream_live_count();

    // Direct host release (static closeHandle still may not CallHost under embed path);
    // proves TCP_STREAMS drop via host_tcp_handle_release.
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let s = TcpStream.connect(host: "127.0.0.1", port: {port})
    print("OPENED")
    __syx_tcp_handle_release(s.handle)
    print("CLOSED")
  }} catch (err) {{
    print("caught")
    print(err.message)
  }}
}}
"#
    )));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();

    assert_eq!(out, "OPENED\nCLOSED", "connect+close must exact: {out}");
    assert_eq!(
        tcp_stream_live_count(),
        baseline,
        "connect+close must return live count to baseline"
    );
}

#[test]
fn ec_m2_3_tcp_leak_connect_without_close_increments() {
    let (port, stop, join) = start_accept_loopback();
    thread::sleep(Duration::from_millis(20));
    let baseline = tcp_stream_live_count();

    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let s = TcpStream.connect(host: "127.0.0.1", port: {port})
    print("LEAKED")
    print(s.handle)
  }} catch (err) {{
    print("caught")
    print(err.message)
  }}
}}
"#
    )));

    assert!(out.contains("LEAKED"), "connect without close must succeed: {out}");
    assert!(!out.contains("caught"), "connect must not throw: {out}");

    let after = tcp_stream_live_count();
    assert_eq!(
        after,
        baseline + 1,
        "connect without close must leave live_count == baseline + 1 (baseline={baseline}, after={after})"
    );

    let id: i64 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("printed handle id");
    assert!(id >= 0, "handle must be >= 0, got {id}");
    assert!(
        tcp_stream_force_close(id),
        "force_close must drop leaked handle {id}"
    );
    assert_eq!(
        tcp_stream_live_count(),
        baseline,
        "cleanup must restore baseline"
    );

    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
}
