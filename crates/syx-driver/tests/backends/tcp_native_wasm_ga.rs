//! ADR-219 / M2 W75: EC-M2-4-H Tcp native/WASM GA fourth deepening (TcpRead llvm=real).
//! Wave A W274 / ADR-618: Tcp×4 HostFn wasm=real 收官（非 Task 余债）.
// was: ec_m2_4_tcp_native_wasm_ga4

use std::io::Write;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{compile_and_run_embedded, DriverOptions};

use crate::native_strict::run_native_strict_or_mirror_vm;

/// Accept connections until stop; write fixed payload on each (VM + native both connect).
fn start_send_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let handle = thread::spawn(move || {
        let mut held = Vec::new();
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.write_all(b"hi");
                    let _ = stream.flush();
                    // Keep stream open until stop so client can finish read/release.
                    held.push(stream);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
        drop(held);
    });
    (port, stop, handle)
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_native_tcp_read_exec() {
    let (port, stop, join) = start_send_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"func main() {{
    let h = __syx_tcp_connect("127.0.0.1", {port})
    let s = __syx_tcp_read(h, 8)
    print(s)
    print(__syx_tcp_handle_release(h))
    return 0
}}"#
    );
    let vm_out = compile_and_run_embedded(&src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict_or_mirror_vm(&src, &vm_out, "tcp_ga");
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();

    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_lines: Vec<&str> = vm_out.lines().map(|l| l.trim()).collect();
    let native_lines: Vec<&str> = native_out.lines().map(|l| l.trim()).collect();
    assert!(
        vm_lines.len() >= 2 && native_lines.len() >= 2,
        "expected payload + true; vm={vm_out:?} native={native_out:?}"
    );
    assert_eq!(
        native_lines[0], vm_lines[0],
        "payload mismatch; vm={vm_lines:?} native={native_lines:?}"
    );
    assert_eq!(native_lines[0], "hi", "read must return hi: {native_out}");
    assert_eq!(
        native_lines[1], "true",
        "release must return true: {native_out}"
    );
    assert_eq!(native_lines[1], vm_lines[1], "true line must match VM");
    assert!(
        !native_out.contains("tcp-ok"),
        "M0-B02: no status strings: {native_out}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga_native_tcp_connect_exec() {
    // Wave A W150 / ADR-494 / A-03 — offline refuse (no listener)
    let src = r#"func main() {
    print(__syx_tcp_connect("127.0.0.1", 9))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "tcp_ga");
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native TcpConnect must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "-1", "refuse must be -1: {native_line}");
    assert!(
        !native_out.contains("tcp-ok"),
        "M0-B02: no status strings: {native_out}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga_native_tcp_handle_release_exec() {
    // Wave A W150 / ADR-494 / A-03
    let (port, stop, join) = start_send_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"func main() {{
    let h = __syx_tcp_connect("127.0.0.1", {port})
    print(__syx_tcp_handle_release(h))
    return 0
}}"#
    );
    let vm_out = compile_and_run_embedded(&src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict_or_mirror_vm(&src, &vm_out, "tcp_ga");
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();

    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native TcpHandleRelease must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(
        native_line, "true",
        "release must return true: {native_line}"
    );
}
