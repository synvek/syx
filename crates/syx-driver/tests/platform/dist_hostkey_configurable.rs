//! Dist configurable non-loopback host keys (decoupled from bind) + TCP CDN peer MVP.

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_delivery_ack_clear, cdn_delivery_ack_count, cdn_doctor_mode, cdn_memory_dev_enabled,
    cdn_route, cdn_route_counts_as_ga, cdn_route_ga, cdn_routes_memory_clear, cdn_tcp_peer_clear,
    cdn_tcp_peer_configure, cdn_tcp_peer_fingerprint, cdn_tcp_peer_has_endpoints,
    cdn_tcp_peer_inbox_count, cdn_tcp_peer_route, dist_actor_mp_clear, dist_actor_mp_inbox_clear,
    dist_host_key_addr_is_loopback, dist_region_multihost_clear,
    dist_region_multihost_configure_keys, dist_region_multihost_has_nonloopback_host_key,
    dist_region_multihost_route, dist_region_multihost_tcp_health, spawn_dist_tcp_mailbox_on,
};

static P112_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_hostkey_fingerprint() {
    // Phase 114 supersedes live fingerprints; P112 strings remain in heap_v84.
    assert_eq!(cdn_tcp_peer_fingerprint(), "dist-cdn-tcp-v2:peer-delivery");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_configure_keys"));
    assert!(src.contains("SYX_DIST_CDN_MEMORY"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-region-hostkey-v2:config-nonloopback"));
    assert!(lib.contains("dist-cdn-tcp-v1:peer-mvp"));
}

#[test]
fn dist_hostkey_nonloopback_host_key_decoupled_from_bind() {
    let _guard = P112_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    // Bind on all interfaces; host key addr is non-loopback logical identity.
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let connect_url = format!("tcp://127.0.0.1:{}", bind_addr.port());
    assert!(
        !dist_host_key_addr_is_loopback("192.0.2.10"),
        "RFC5737 doc addr must count as non-loopback host key"
    );
    assert!(dist_host_key_addr_is_loopback("127.0.0.1"));
    dist_region_multihost_configure_keys(&[
        ("us-east", "edge-a", "192.0.2.10", connect_url.as_str()),
        ("us-east", "edge-b", "127.0.0.1", connect_url.as_str()),
    ]);
    assert!(dist_region_multihost_has_nonloopback_host_key());
    assert!(dist_region_multihost_tcp_health("us-east", "edge-a"));
    let (region, host, ack) =
        dist_region_multihost_route("us-east", "edge-a", 1, 2, "p112-nonloopback-key")
            .expect("route via decoupled non-loopback host key");
    assert_eq!(region, "us-east");
    assert_eq!(host, "edge-a");
    assert_eq!(ack, "p112-nonloopback-key");
    stop.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_hostkey_kill_nonloopback_host_key_failover() {
    let _guard = P112_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    let (addr_a, handle_a, stop_a) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 16);
    let (addr_b, handle_b, stop_b) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = format!("tcp://{addr_a}");
    let url_b = format!("tcp://{addr_b}");
    dist_region_multihost_configure_keys(&[
        ("us-east", "primary", "192.0.2.11", url_a.as_str()),
        ("us-east", "backup", "192.0.2.12", url_b.as_str()),
    ]);
    assert!(dist_region_multihost_has_nonloopback_host_key());

    let (_, host, ack) =
        dist_region_multihost_route("us-east", "primary", 1, 2, "p112-primary")
            .expect("primary host key");
    assert_eq!(host, "primary");
    assert_eq!(ack, "p112-primary");

    stop_a.stop();
    let _ = handle_a.join();
    thread::sleep(Duration::from_millis(40));
    assert!(!dist_region_multihost_tcp_health("us-east", "primary"));

    let (_, host2, ack2) =
        dist_region_multihost_route("us-east", "primary", 1, 2, "p112-failover")
            .expect("failover to backup host key");
    assert_eq!(host2, "backup");
    assert_eq!(ack2, "p112-failover");

    stop_b.stop();
    let _ = handle_b.join();
    dist_region_multihost_clear();
}

#[test]
fn dist_hostkey_tcp_cdn_peer_mvp() {
    let _guard = P112_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    cdn_tcp_peer_clear();
    cdn_routes_memory_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    dist_actor_mp_inbox_clear();
    cdn_delivery_ack_clear();
    let (addr, _handle, stop) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 8);
    thread::sleep(Duration::from_millis(20));
    let url = format!("tcp://{addr}");
    cdn_tcp_peer_configure(&[("us-east", url.as_str())]);
    assert!(cdn_tcp_peer_has_endpoints());
    assert_eq!(cdn_doctor_mode(), "tcp-cdn-peer");
    assert!(cdn_route_counts_as_ga());

    let ack_before = cdn_delivery_ack_count();
    let (region, ack) = cdn_tcp_peer_route("us-east", 3, 4, "p112-tcp-cdn")
        .expect("TCP CDN peer route");
    assert_eq!(region, "us-east");
    assert_eq!(ack, "p112-tcp-cdn");
    assert_eq!(cdn_tcp_peer_inbox_count(), 0);
    assert!(
        cdn_delivery_ack_count() > ack_before,
        "CDN success proof = ACK registry, not tcp_mailbox_inbox"
    );

    assert!(cdn_route("us-east", 5, 6, "p112-cdn-route"));
    let ga = cdn_route_ga("us-east", 7, 8, "p112-cdn-ga").expect("TCP CDN GA");
    assert!(ga.0.contains("us-east"));
    assert_eq!(ga.1, "p112-cdn-ga");

    stop.stop();
    cdn_tcp_peer_clear();
}

#[test]
fn dist_hostkey_memory_cdn_not_ga() {
    let _guard = P112_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    cdn_tcp_peer_clear();
    cdn_routes_memory_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    assert!(!cdn_memory_dev_enabled());
    assert_eq!(cdn_doctor_mode(), "no-cdn-transport");
    assert!(!cdn_route_counts_as_ga());
    assert!(
        !cdn_route("us-east", 1, 2, "silent-memory"),
        "memory CDN must not succeed without TCP peers or SYX_DIST_CDN_MEMORY=1"
    );
    let err = cdn_route_ga("us-east", 1, 2, "not-ga").expect_err("memory CDN not GA");
    assert!(
        err.contains("GA") || err.contains("peer") || err.contains("multihost"),
        "got {err}"
    );
}

#[test]
fn dist_hostkey_source_gate() {
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_failover_order"));
    assert!(src.contains("dist_host_key_addr_is_loopback"));
    assert!(src.contains("dist_region_multihost_has_nonloopback_host_key"));
    assert!(src.contains("cdn_tcp_peer_route"));
    assert!(src.contains("spawn_dist_tcp_mailbox_on"));
}
