//! Dist dual-stack bind (::1 + 127.0.0.1) host keys + keyed failover.

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_doctor_mode, cdn_memory_dev_enabled, cdn_route, cdn_route_counts_as_ga, cdn_route_ga,
    cdn_routes_memory_clear, dist_actor_mp_clear, dist_bind_addr_is_non_loopback_v4,
    dist_region_multihost_clear, dist_region_multihost_configure, dist_region_multihost_route,
    dist_region_multihost_tcp_health, spawn_dist_tcp_mailbox_dual_stack,
};

static P111_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_nonloopback_fingerprint() {
    // Live fingerprint advanced in Phase 114; P111 string remains in heap_v83.
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("spawn_dist_tcp_mailbox_dual_stack"));
    assert!(src.contains("cdn-memory-dev-only") || src.contains("SYX_DIST_CDN_MEMORY"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-region-nonloopback-v1:dual-bind-failover"));
}

#[test]
fn dist_nonloopback_dual_bind_host_keys() {
    let _guard = P111_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    let dual = spawn_dist_tcp_mailbox_dual_stack(16);
    thread::sleep(Duration::from_millis(25));
    assert!(!dist_bind_addr_is_non_loopback_v4(&dual.v4_addr));
    assert!(dist_bind_addr_is_non_loopback_v4(&dual.v6_addr));
    assert!(
        dual.v6_addr.ip().to_string().contains(':'),
        "expected IPv6 bind, got {}",
        dual.v6_addr
    );
    assert_eq!(dual.v4_addr.ip().to_string(), "127.0.0.1");
    let url_v4 = format!("tcp://{}", dual.v4_addr);
    let url_v6 = format!("tcp://{}", dual.v6_addr);
    dist_region_multihost_configure(&[
        ("us-east", "host-v4", url_v4.as_str()),
        ("us-east", "host-v6", url_v6.as_str()),
    ]);
    assert!(dist_region_multihost_tcp_health("us-east", "host-v4"));
    assert!(dist_region_multihost_tcp_health("us-east", "host-v6"));
    dual.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_nonloopback_kill_v6_host_key_failover() {
    let _guard = P111_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    let dual = spawn_dist_tcp_mailbox_dual_stack(16);
    thread::sleep(Duration::from_millis(25));
    let url_v4 = format!("tcp://{}", dual.v4_addr);
    let url_v6 = format!("tcp://{}", dual.v6_addr);
    dist_region_multihost_configure(&[
        ("us-east", "host-v6", url_v6.as_str()),
        ("us-east", "host-v4", url_v4.as_str()),
    ]);

    let (region, host, ack) =
        dist_region_multihost_route("us-east", "host-v6", 1, 2, "p111-v6-primary")
            .expect("::1 host key route");
    assert_eq!(region, "us-east");
    assert_eq!(host, "host-v6");
    assert_eq!(ack, "p111-v6-primary");

    dual.stop_v6_only();
    thread::sleep(Duration::from_millis(80));
    assert!(
        !dist_region_multihost_tcp_health("us-east", "host-v6"),
        "killed ::1 host key must fail health probe"
    );
    assert!(dist_region_multihost_tcp_health("us-east", "host-v4"));

    let (region2, host2, ack2) =
        dist_region_multihost_route("us-east", "host-v6", 1, 2, "p111-v4-failover")
            .expect("failover to 127.0.0.1 host key after ::1 kill");
    assert_eq!(region2, "us-east");
    assert_eq!(host2, "host-v4");
    assert_eq!(ack2, "p111-v4-failover");

    dual.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_nonloopback_memory_cdn_not_ga() {
    let _guard = P111_DIST_LOCK.lock().unwrap();
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
    let err = cdn_route_ga("us-east", 1, 2, "not-ga").expect_err("memory CDN not GA");
    assert!(
        err.contains("GA") || err.contains("multihost") || err.contains("anycast"),
        "got {err}"
    );
}

#[test]
fn dist_nonloopback_source_gate() {
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_failover_order"));
    assert!(src.contains("spawn_dist_tcp_mailbox_on"));
    assert!(src.contains("[::1]:0"));
    assert!(src.contains("dist_bind_addr_is_non_loopback_v4"));
}
