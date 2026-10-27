//! EC-M2-3-G HttpBodyStream session live-count leak first nail.
// was: ec_m2_3_http_body_stream_leak

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{http_body_stream_force_close, http_body_stream_live_count};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn net_facade_src(body: &str) -> String {
    let parts = [
        include_str!("../../../../packages/io/file_handle.sy"),
        include_str!("../../../../packages/io/types/http_response.sy"),
        include_str!("../../../../packages/net/http_types.sy"),
        include_str!("../../../../packages/net/http_body_stream.sy"),
        include_str!("../../../../packages/net/mod.sy"),
        include_str!("../../../../packages/net/url.sy"),
    ]
    .join("\n");
    format!("{parts}\n{body}")
}

fn start_tiny_body_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let body = "hi";
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
                    let mut buf = [0u8; 4096];
                    let _ = stream.read(&mut buf);
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
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

#[test]
fn ec_m2_3_http_body_stream_leak_open_close_returns_to_baseline() {
    // Wave A W51 / ADR-395 / A-03: HttpBodyStream open+close product exact (drop print(h)).
    let (port, stop, server) = start_tiny_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let baseline = http_body_stream_live_count();
    let out = run(&net_facade_src(&format!(
        r#"
func main() {{
  let opened = HttpBodyStream.open(url: "http://127.0.0.1:{port}/ok")
  print("OPENED")
  print(opened.status)
  let h = opened.handle
  HttpBodyStream.close(handle: h)
  print("CLOSED")
}}
"#
    )));
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();
    assert_eq!(
        out, "OPENED\n200\nCLOSED",
        "open+close must exact: {out}"
    );
    assert_eq!(
        http_body_stream_live_count(),
        baseline,
        "open+close must restore http_body_stream_live_count"
    );
}

#[test]
fn ec_m2_3_http_body_stream_leak_open_without_close_increments() {
    let (port, stop, server) = start_tiny_body_loopback();
    thread::sleep(Duration::from_millis(50));
    let baseline = http_body_stream_live_count();
    let out = run(&net_facade_src(&format!(
        r#"
func main() {{
  let opened = HttpBodyStream.open(url: "http://127.0.0.1:{port}/leak")
  print("LEAKED")
  print(opened.status)
  print(opened.handle)
}}
"#
    )));
    stop.store(true, Ordering::Relaxed);
    let _ = server.join();
    assert!(out.contains("LEAKED"), "{out}");
    let after = http_body_stream_live_count();
    assert_eq!(after, baseline + 1, "baseline={baseline} after={after}");
    // status then handle — take last parseable int that looks like a session id (>=1)
    let id: i64 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .last()
        .expect("handle id");
    assert!(http_body_stream_force_close(id), "force_close {id}");
    assert_eq!(http_body_stream_live_count(), baseline);
}
