//! Dist BGP peer-exchange + sibling-process PoP.

use std::fs;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_export_peer_view, cdn_bgp_fingerprint,
    cdn_bgp_ga_ready, cdn_bgp_health, cdn_bgp_import_peer_mesh, cdn_bgp_peer_exchange_done,
    cdn_bgp_peer_view_count, cdn_bgp_route, cdn_delivery_ack_clear, cdn_delivery_ack_count,
    cdn_doctor_mode, cdn_pop_clear, cdn_pop_configure, cdn_pop_fingerprint,
    cdn_pop_has_endpoints, cdn_pop_health, cdn_pop_register_serve_pid, cdn_pop_route,
    cdn_pop_sibling_process_ready, cdn_route_counts_as_ga, cdn_route_ga, cdn_routes_memory_clear,
    dist_connect_url_is_loopback, dist_host_key_remote_for_bind,
    dist_pop_note_spawned_child, dist_pop_os_child_try_serve, dist_pop_pids_are_siblings,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P118_DIST_LOCK: Mutex<()> = Mutex::new(());

const ENV_DIST_POP_SUPERVISOR: &str = "SYX_DIST_POP_SUPERVISOR";
const ENV_DIST_POP_PEER: &str = "SYX_DIST_POP_PEER";
const ENV_DIST_POP_RESULT: &str = "SYX_DIST_POP_SUPERVISOR_RESULT";

fn spawn_remote_pop(max_conns: usize) -> (String, String, syx_async::ArcStop) {
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", max_conns);
    thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_remote_for_bind(&bind_addr);
    let connect_url = dist_tcp_url_for_bind_addr(&bind_addr);
    assert!(!connect_url.contains("127.0.0.1"));
    assert!(!dist_connect_url_is_loopback(&connect_url));
    (host_key, connect_url, stop)
}

fn run_pop_peer_side(peer_label: &str, ready_path: &std::path::Path) {
    let deadline = Instant::now() + Duration::from_secs(30);
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_remote_for_bind(&bind_addr);
    let connect_url = dist_tcp_url_for_bind_addr(&bind_addr);
    let pid = std::process::id();
    let ppid = std::env::var("SYX_DIST_POP_PPID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let _ = fs::write(
        ready_path,
        format!("{peer_label}\n{host_key}\n{connect_url}\n{pid}\n{ppid}\n"),
    );
    while std::env::var("SYX_DIST_POP_PEER_STOP").is_err() {
        thread::sleep(Duration::from_millis(50));
        if Instant::now() >= deadline {
            break;
        }
    }
    stop.stop();
}

fn run_pop_supervisor(result_path: &std::path::Path) {
    let supervisor_pid = std::process::id();
    let ready_a = std::env::temp_dir().join(format!("syx-pop-peer-a-{supervisor_pid}"));
    let ready_b = std::env::temp_dir().join(format!("syx-pop-peer-b-{supervisor_pid}"));
    let _ = fs::remove_file(&ready_a);
    let _ = fs::remove_file(&ready_b);

    let exe = std::env::current_exe().expect("current_exe");
    let spawn_peer = |label: &str, ready: &std::path::Path| {
        Command::new(&exe)
            .env(ENV_DIST_POP_PEER, label)
            .env("SYX_DIST_POP_PPID", supervisor_pid.to_string())
            .env("SYX_DIST_POP_PEER_READY", ready)
            .args([
                "dist_bgp_peer_exchange::dist_bgp_peer_pop_peer_entry",
                "--exact",
                "--nocapture",
                "--test-threads=1",
            ])
            .spawn()
            .expect("spawn sibling peer")
    };

    let mut peer_a = spawn_peer("pop-us", &ready_a);
    let mut peer_b = spawn_peer("pop-eu", &ready_b);

    let mut pop_a = None;
    let mut pop_b = None;
    for _ in 0..120 {
        if pop_a.is_none() {
            if let Ok(body) = fs::read_to_string(&ready_a) {
                let mut lines = body.lines();
                let _label = lines.next();
                let host_key = lines.next().unwrap_or("").to_string();
                let url = lines.next().unwrap_or("").to_string();
                let pid: u32 = lines.next().unwrap_or("0").parse().unwrap_or(0);
                let ppid: u32 = lines.next().unwrap_or("0").parse().unwrap_or(0);
                if !host_key.is_empty() && !url.is_empty() && pid != 0 {
                    pop_a = Some(("pop-us", host_key, url, pid, ppid));
                }
            }
        }
        if pop_b.is_none() {
            if let Ok(body) = fs::read_to_string(&ready_b) {
                let mut lines = body.lines();
                let _label = lines.next();
                let host_key = lines.next().unwrap_or("").to_string();
                let url = lines.next().unwrap_or("").to_string();
                let pid: u32 = lines.next().unwrap_or("0").parse().unwrap_or(0);
                let ppid: u32 = lines.next().unwrap_or("0").parse().unwrap_or(0);
                if !host_key.is_empty() && !url.is_empty() && pid != 0 {
                    pop_b = Some(("pop-eu", host_key, url, pid, ppid));
                }
            }
        }
        if pop_a.is_some() && pop_b.is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    let (region_a, key_a, url_a, pid_a, ppid_a) = pop_a.expect("peer-a ready");
    let (region_b, key_b, url_b, pid_b, ppid_b) = pop_b.expect("peer-b ready");
    assert!(
        dist_pop_pids_are_siblings(pid_a, ppid_a, pid_b, ppid_b),
        "PoP peers must be siblings (same ppid), got {pid_a}/{ppid_a} vs {pid_b}/{ppid_b}"
    );
    assert_ne!(key_a, key_b, "sibling PoPs must have distinct hostkeys");

    cdn_pop_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");
    cdn_pop_configure(&[
        (region_a, key_a.as_str(), url_a.as_str()),
        (region_b, key_b.as_str(), url_b.as_str()),
    ]);
    cdn_pop_register_serve_pid(region_a, pid_a, ppid_a);
    cdn_pop_register_serve_pid(region_b, pid_b, ppid_b);
    assert!(cdn_pop_sibling_process_ready());
    assert!(cdn_pop_has_endpoints());
    assert_eq!(cdn_doctor_mode(), "cdn-pop-multi-hostkey-route");
    assert!(cdn_route_counts_as_ga());
    assert!(cdn_pop_health(region_a));
    assert!(cdn_pop_health(region_b));

    let ack_before = cdn_delivery_ack_count();
    let (active, ack) = cdn_pop_route(region_a, 1, 2, "p118-sibling-primary")
        .expect("sibling-process PoP route");
    assert_eq!(active, region_a);
    assert_eq!(ack, "p118-sibling-primary");
    assert!(cdn_delivery_ack_count() > ack_before);

    let ga = cdn_route_ga(region_a, 3, 4, "p118-sibling-ga").expect("CDN PoP sibling GA");
    assert!(ga.0.contains("pop"));
    assert_eq!(ga.1, "p118-sibling-ga");

    let _ = fs::write(result_path, "ok\n");
    std::env::set_var("SYX_DIST_POP_PEER_STOP", "1");
    let _ = peer_a.wait();
    let _ = peer_b.wait();
    std::env::remove_var("SYX_DIST_POP_PEER_STOP");
    let _ = fs::remove_file(&ready_a);
    let _ = fs::remove_file(&ready_b);
    cdn_pop_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_bgp_peer_pop_peer_entry() {
    if let Ok(label) = std::env::var(ENV_DIST_POP_PEER) {
        let ready = std::env::var("SYX_DIST_POP_PEER_READY")
            .map(std::path::PathBuf::from)
            .expect("peer ready path");
        run_pop_peer_side(&label, &ready);
        return;
    }
    if std::env::var(ENV_DIST_POP_SUPERVISOR).is_ok() {
        let result = std::env::var(ENV_DIST_POP_RESULT)
            .map(std::path::PathBuf::from)
            .expect("supervisor result path");
        run_pop_supervisor(&result);
        return;
    }
    if dist_pop_os_child_try_serve() {
        return;
    }
}

#[test]
fn dist_bgp_peer_cdn_bgp_pop_fingerprints() {
    // Phase 122 fingerprint supersedes v4 — see dist_multi_region_mesh.
    assert_eq!(
        cdn_bgp_fingerprint(),
        "dist-cdn-bgp-v5:multi-region-mesh-mvp"
    );
    assert_eq!(cdn_pop_fingerprint(), "dist-cdn-pop-v6:multi-hostkey-route");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v3:ops-visibility"));
    assert!(src.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(src.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(src.contains("cdn_bgp_import_peer_view"));
    assert!(src.contains("cdn_bgp_ops_snapshot"));
    assert!(src.contains("cdn_pop_multi_hostkey_route_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v3:ops-visibility"));
    assert!(lib.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(lib.contains("dist-cdn-pop-v6:multi-hostkey-route"));
}

#[test]
fn dist_bgp_peer_cdn_bgp_peer_exchange_mvp() {
    let _guard = P118_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_pop_clear();
    cdn_routes_memory_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);

    cdn_bgp_configure_mesh("us-east", &[("203.0.113.0/24", 100, "bgp-us", url_a.as_str())]);
    assert!(!cdn_bgp_ga_ready(), "local-only must not pass mesh GA");
    assert_eq!(cdn_bgp_peer_view_count(), 0);

    let exported = cdn_bgp_export_peer_view();
    assert_eq!(exported.len(), 1);
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );
    assert!(cdn_bgp_peer_exchange_done());
    assert!(cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-region-mesh");
    assert!(cdn_bgp_health("bgp-us"));
    assert!(cdn_bgp_health("bgp-eu"));

    let (active, ack) =
        cdn_bgp_route("bgp-us", 1, 2, "p118-bgp-peer-exchange").expect("bgp peer-exchange route");
    assert_eq!(active, "bgp-eu");
    assert_eq!(ack, "p118-bgp-peer-exchange");

    stop_a.stop();
    stop_b.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_bgp_peer_cdn_pop_sibling_serve_pids_unit() {
    let _guard = P118_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    let (key_a, url_a, stop_a) = spawn_remote_pop(8);
    let (key_b, url_b, stop_b) = spawn_remote_pop(8);
    cdn_pop_configure(&[
        ("pop-us", key_a.as_str(), url_a.as_str()),
        ("pop-eu", key_b.as_str(), url_b.as_str()),
    ]);
    cdn_pop_register_serve_pid("pop-us", 11_001, 9_999);
    cdn_pop_register_serve_pid("pop-eu", 11_002, 9_999);
    assert!(dist_pop_pids_are_siblings(11_001, 9_999, 11_002, 9_999));
    assert!(cdn_pop_sibling_process_ready());
    stop_a.stop();
    stop_b.stop();
    cdn_pop_clear();
}

#[test]
fn dist_bgp_peer_cdn_pop_parent_child_spawn_not_ga() {
    let _guard = P118_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    cdn_delivery_ack_clear();

    let ready_path = std::env::temp_dir().join(format!(
        "syx-dist-pop-p118-child-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&ready_path);
    let exe = std::env::current_exe().expect("current_exe");
    let mut child = Command::new(&exe)
        .env("SYX_DIST_POP_CHILD_BIND", "0.0.0.0:0")
        .env("SYX_DIST_POP_CHILD_READY_FILE", &ready_path)
        .env("SYX_DIST_POP_CHILD_MAX_CONNS", "8")
        .env("SYX_DIST_POP_PPID", std::process::id().to_string())
        .args([
            "dist_bgp_peer_exchange::dist_bgp_peer_pop_peer_entry",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .spawn()
        .expect("spawn child PoP");
    dist_pop_note_spawned_child(child.id());

    let mut child_host_key = String::new();
    let mut child_url = String::new();
    for _ in 0..80 {
        if let Ok(body) = fs::read_to_string(&ready_path) {
            let mut lines = body.lines();
            child_host_key = lines.next().unwrap_or("").to_string();
            child_url = lines.next().unwrap_or("").to_string();
            if !child_host_key.is_empty() && !child_url.is_empty() {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert!(!child_host_key.is_empty() && !child_url.is_empty());

    let (key_a, url_a, stop_a) = spawn_remote_pop(8);
    cdn_pop_configure(&[
        ("pop-us", key_a.as_str(), url_a.as_str()),
        ("pop-eu", child_host_key.as_str(), child_url.as_str()),
    ]);
    cdn_pop_register_serve_pid("pop-us", std::process::id(), 1);
    cdn_pop_register_serve_pid("pop-eu", child.id(), std::process::id());
    assert!(
        !cdn_pop_sibling_process_ready(),
        "parent/child current_exe spawn must not pass pop-v5 sibling GA"
    );
    let err = cdn_pop_route("pop-us", 1, 2, "parent-child")
        .expect_err("parent/child PoP must not route under pop-v5");
    assert!(
        err.contains("sibling") || err.contains("serve pid"),
        "unexpected err: {err}"
    );

    std::env::set_var("SYX_DIST_POP_CHILD_STOP", "1");
    let _ = child.wait();
    std::env::remove_var("SYX_DIST_POP_CHILD_STOP");
    stop_a.stop();
    let _ = fs::remove_file(&ready_path);
    cdn_pop_clear();
}

#[test]
fn dist_bgp_peer_cdn_pop_sibling_supervisor_e2e() {
    if std::env::var(ENV_DIST_POP_PEER).is_ok() || std::env::var(ENV_DIST_POP_SUPERVISOR).is_ok() {
        return;
    }

    let result_path = std::env::temp_dir().join(format!(
        "syx-dist-pop-p118-supervisor-{}",
        std::process::id()
    ));
    let _ = fs::remove_file(&result_path);
    let exe = std::env::current_exe().expect("current_exe");
    let status = Command::new(&exe)
        .env(ENV_DIST_POP_SUPERVISOR, "1")
        .env(ENV_DIST_POP_RESULT, &result_path)
        .args([
            "dist_bgp_peer_exchange::dist_bgp_peer_pop_peer_entry",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .status()
        .expect("supervisor status");
    assert!(status.success(), "supervisor sibling PoP E2E must succeed");
    let body = fs::read_to_string(&result_path).expect("supervisor result");
    assert!(body.contains("ok"), "supervisor result: {body}");
    let _ = fs::remove_file(&result_path);
}

#[test]
fn dist_bgp_peer_global_cdn_sla_purged_completely() {
    let lock = include_str!("../../../syx-pkg/src/lockfile.rs");
    assert!(!lock.contains("global_cdn_sla_tier"));
    let enforce = include_str!("../../../syx-pkg/src/lock_enforce.rs");
    assert!(!enforce.contains("enforce_global_cdn_sla"));
    assert!(!enforce.contains("global_cdn_sla"));
    let resolver = include_str!("../../../syx-pkg/src/resolver.rs");
    assert!(!resolver.contains("global_cdn_sla"));
    let pkg = include_str!("../../../syx-pkg/src/lib.rs");
    assert!(!pkg.contains("enforce_global_cdn_sla"));
    let billing = include_str!("../../src/farm_billing_report.rs");
    assert!(!billing.contains("global_cdn_sla"));
}
