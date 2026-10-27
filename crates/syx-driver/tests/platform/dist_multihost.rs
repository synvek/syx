//! Dist multi-host keyed region routing + memory CDN dev-only gate.

use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_doctor_mode, cdn_memory_dev_enabled, cdn_route, cdn_route_counts_as_ga, cdn_route_ga,
    cdn_routes_memory_clear, cdn_routes_memory_count, dist_actor_mp_clear,
    dist_region_multihost_clear, dist_region_multihost_configure,
    dist_region_multihost_route,
    dist_region_multihost_tcp_health, spawn_dist_tcp_mailbox,
};

static P110_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_multihost_fingerprint() {
    // Live fingerprint advanced in Phase 114; P110 string remains in heap_v82.
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("cdn-memory-dev-only") || src.contains("SYX_DIST_CDN_MEMORY"));
    assert!(src.contains("dist_region_multihost_route"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-region-multihost-v1:keyed-failover"));
}

#[test]
fn dist_multihost_memory_cdn_not_ga_without_tcp() {
    let _guard = P110_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    cdn_routes_memory_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    assert!(!cdn_memory_dev_enabled());
    assert_eq!(cdn_doctor_mode(), "no-cdn-transport");
    assert!(!cdn_route_counts_as_ga());
    assert!(
        !cdn_route("us-east", 1, 2, "silent-memory"),
        "memory CDN must not succeed without TCP regions or SYX_DIST_CDN_MEMORY=1"
    );
    assert_eq!(cdn_routes_memory_count(), 0);
    let err = cdn_route_ga("us-east", 1, 2, "not-ga").expect_err("memory CDN not GA");
    assert!(
        err.contains("GA") || err.contains("multihost") || err.contains("anycast"),
        "got {err}"
    );
}

#[test]
fn dist_multihost_memory_cdn_doctor_dev_only() {
    let _guard = P110_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    cdn_routes_memory_clear();
    std::env::set_var("SYX_DIST_CDN_MEMORY", "1");
    assert!(cdn_memory_dev_enabled());
    assert_eq!(cdn_doctor_mode(), "cdn-memory-dev-only");
    assert!(!cdn_route_counts_as_ga());
    assert!(cdn_route("us-east", 1, 2, "dev-memory"));
    assert!(cdn_routes_memory_count() > 0);
    std::env::remove_var("SYX_DIST_CDN_MEMORY");
    cdn_routes_memory_clear();
}

#[test]
fn dist_multihost_kill_host_key_failover() {
    let _guard = P110_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    let (addr_a, handle_a, stop_a) = spawn_dist_tcp_mailbox(16);
    let (addr_b, handle_b, stop_b) = spawn_dist_tcp_mailbox(16);
    thread::sleep(Duration::from_millis(25));
    let url_a = format!("tcp://{addr_a}");
    let url_b = format!("tcp://{addr_b}");
    dist_region_multihost_configure(&[
        ("us-east", "host-a", url_a.as_str()),
        ("us-east", "host-b", url_b.as_str()),
    ]);

    assert!(dist_region_multihost_tcp_health("us-east", "host-a"));
    assert!(dist_region_multihost_tcp_health("us-east", "host-b"));

    let (region, host, ack) =
        dist_region_multihost_route("us-east", "host-a", 1, 2, "p110-primary")
            .expect("primary host key route");
    assert_eq!(region, "us-east");
    assert_eq!(host, "host-a");
    assert_eq!(ack, "p110-primary");

    stop_a.stop();
    let _ = handle_a.join();
    thread::sleep(Duration::from_millis(40));
    assert!(
        !dist_region_multihost_tcp_health("us-east", "host-a"),
        "killed host key must fail health probe"
    );
    assert!(dist_region_multihost_tcp_health("us-east", "host-b"));

    let (region2, host2, ack2) =
        dist_region_multihost_route("us-east", "host-a", 1, 2, "p110-failover")
            .expect("keyed failover after host-a kill");
    assert_eq!(region2, "us-east");
    assert_eq!(host2, "host-b", "must failover to healthy host key");
    assert_eq!(ack2, "p110-failover");

    stop_b.stop();
    let _ = handle_b.join();
    dist_region_multihost_clear();
}

#[test]
fn dist_multihost_cross_process_ack() {
    if let Ok(url) = std::env::var("SYX_DIST_MULTIHOST_CHILD_URL_P110") {
        use syx_async::region_route_tcp;
        let ack = region_route_tcp(&url, "us-east", 9, 10, "p110-child-ack")
            .expect("child peer send");
        assert_eq!(ack, "p110-child-ack");
        println!("SYX_DIST_MULTIHOST_ACK={ack}");
        return;
    }

    let _guard = P110_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    let (addr, handle, stop) = spawn_dist_tcp_mailbox(8);
    thread::sleep(Duration::from_millis(20));
    let url = format!("tcp://{addr}");
    dist_region_multihost_configure(&[("us-east", "host-a", url.as_str())]);
    let exe = std::env::current_exe().expect("current_exe");
    let output = Command::new(&exe)
        .env("SYX_DIST_MULTIHOST_CHILD_URL_P110", &url)
        .args([
            "dist_multihost::dist_multihost_cross_process_ack",
            "--exact",
            "--nocapture",
        ])
        .output()
        .expect("spawn child process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "child failed status={:?}\nstdout={stdout}\nstderr={stderr}",
        output.status
    );
    assert!(
        stdout.contains("SYX_DIST_MULTIHOST_ACK=p110-child-ack")
            || stderr.contains("SYX_DIST_MULTIHOST_ACK=p110-child-ack"),
        "expected child ACK\nstdout={stdout}\nstderr={stderr}"
    );
    stop.stop();
    let _ = handle.join();
    dist_region_multihost_clear();
}

#[test]
fn dist_multihost_source_gate() {
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_failover_order"));
    assert!(src.contains("SYX_DIST_CDN_MEMORY"));
    assert!(src.contains("cdn_route_counts_as_ga"));
    let cdn_fn = src
        .split("pub fn cdn_route(")
        .nth(1)
        .expect("cdn_route(")
        .split("pub fn cdn_route_ga")
        .next()
        .expect("cdn_route body");
    assert!(
        cdn_fn.contains("cdn_memory_dev_enabled"),
        "cdn_route must gate memory VecDeque on explicit dev opt-in"
    );
}
