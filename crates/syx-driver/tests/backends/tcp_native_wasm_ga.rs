//! ADR-219 / M2 W75: EC-M2-4-H Tcp native/WASM GA fourth deepening (TcpRead llvm=real).
//! Wave A W274 / ADR-618: Tcp×4 HostFn wasm=real 收官（非 Task 余债）.
// was: ec_m2_4_tcp_native_wasm_ga4

use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

const ADR188_TCP_HOSTS: &[&str] = &[
    "TcpConnect",
    "TcpRead",
    "TcpWrite",
    "TcpHandleRelease",
];

const W274_WASM_REAL: &[&str] = &[
    "TcpConnect",
    "TcpRead",
    "TcpWrite",
    "TcpHandleRelease",
];

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_m2_w75_tcp_ga4_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

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
fn ec_m2_4_tcp_native_wasm_ga4_tcp_read_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "TcpRead");
    assert!(
        block.contains("vm = \"real\""),
        "TcpRead must keep vm=real: {block}"
    );
    assert!(
        block.contains("llvm = \"real\""),
        "TcpRead must declare llvm=real (ADR-219): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "TcpRead must declare wasm=real (ADR-618): {block}"
    );
    assert!(
        !block.contains("wasm = \"host-assisted\""),
        "TcpRead must not remain wasm=host-assisted: {block}"
    );
    assert!(
        block.contains("ADR-618") && (block.contains("ADR-219") || block.contains("llvm")),
        "TcpRead notes must cite ADR-618: {block}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_all_adr188_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ADR188_TCP_HOSTS {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (EC-M2-4-H): {block}"
        );
        assert!(
            !block.contains("llvm = \"partial\""),
            "{name} must not remain llvm=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-618): {block}"
        );
    }
}

#[test]
fn ec_m2_4_tcp_hosts_wasm_real_w274() {
    // Wave A W274 / ADR-618 — Tcp×4 HostFn wasm 收官 (non-Task host-assisted remainder).
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in W274_WASM_REAL {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-618): {block}"
        );
        assert!(
            block.contains("ADR-618"),
            "{name} notes must cite ADR-618: {block}"
        );
    }
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.tcp");
    assert!(
        block.contains("native = \"real\""),
        "net.tcp must keep native=real (ADR-635): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.tcp must declare wasm=real (ADR-623): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "net.tcp must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-635") && block.contains("W291"),
        "net.tcp notes must cite ADR-635+W291: {block}"
    );
    assert!(
        block.contains("ec_m2_4_tcp_native_wasm_ga4"),
        "net.tcp tests must include ec_m2_4_tcp_native_wasm_ga4: {block}"
    );
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
    let (code, native_out) = run_native_strict(&src);
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
    assert_eq!(native_lines[1], "true", "release must return true: {native_out}");
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
    let (code, native_out) = run_native_strict(src);
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
    let (code, native_out) = run_native_strict(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();

    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native TcpHandleRelease must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "true", "release must return true: {native_line}");
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["TcpStreamTls", "TcpSockbuf", "TcpHandleRefcount"] {
        assert!(
            !toml.contains(&format!("name = \"{name}\"")),
            "{name} must remain uncovered (fail-closed default); found override"
        );
    }

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_tcp_read")
        .expect("syx_rt_tcp_read must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::TcpRead") && body.contains("rt_call_host_native"),
        "syx_rt_tcp_read must route HostFnId::TcpRead (W453 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_tcp_read must leave rt_direct_exec after W453: {body}"
    );
    let idx_w = stubs
        .find("fn syx_rt_tcp_write")
        .expect("syx_rt_tcp_write must exist");
    let after_w = &stubs[idx_w..];
    let end_w = after_w.find("\n#[no_mangle]").unwrap_or(after_w.len().min(200));
    let body_w = &after_w[..end_w.min(200)];
    assert!(
        body_w.contains("HostFnId::TcpWrite") && body_w.contains("rt_call_host_native"),
        "syx_rt_tcp_write must route HostFnId::TcpWrite (W454 wire): {body_w}"
    );
    assert!(
        !body_w.contains("rt_direct_exec"),
        "syx_rt_tcp_write must leave rt_direct_exec after W454: {body_w}"
    );

    let idx_r = stubs
        .find("fn syx_rt_tcp_handle_release")
        .expect("syx_rt_tcp_handle_release must exist");
    let after_r = &stubs[idx_r..];
    let end_r = after_r.find("\n#[no_mangle]").unwrap_or(after_r.len().min(200));
    let body_r = &after_r[..end_r.min(200)];
    assert!(
        body_r.contains("HostFnId::TcpHandleRelease") && body_r.contains("rt_call_host_native"),
        "syx_rt_tcp_handle_release must route HostFnId::TcpHandleRelease (W455 wire): {body_r}"
    );
    assert!(
        !body_r.contains("rt_direct_exec"),
        "syx_rt_tcp_handle_release must leave rt_direct_exec after W455: {body_r}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_tcp_native_wasm_ga4_tier"),
        "inventory must contain ec_m2_4_tcp_native_wasm_ga4_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-219") || inv.contains("signed-ADR-219"),
        "inventory must note ADR-219: {inv}"
    );
}

#[test]
fn ec_m2_4_tcp_native_wasm_ga4_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_tcp_native_wasm_ga4"
    );
}
