//! Dist hostkey regression — loopback/synthetic must not pass production hostkey gate.

use std::net::SocketAddr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_delivery_ack_clear, cdn_doctor_mode, cdn_memory_dev_enabled, cdn_pop_clear,
    cdn_pop_configure, cdn_pop_has_endpoints, cdn_pop_route, cdn_route, cdn_route_counts_as_ga,
    cdn_routes_memory_clear, dist_actor_mp_clear, dist_host_key_addr_is_documentation,
    dist_host_key_addr_is_loopback, dist_host_key_register_runtime_bind,
    dist_region_multihost_clear, dist_region_multihost_configure_keys,
    dist_region_multihost_has_runtime_bound_host_key,
    dist_region_multihost_has_runtime_bound_host_key_v4, dist_region_multihost_route,
    dist_region_multihost_tcp_health, spawn_dist_tcp_mailbox_on,
};

static P114_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_hostkey_reject_cdn_pop_loopback_dual_port_not_ga() {
    let _guard = P114_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    cdn_routes_memory_clear();
    cdn_delivery_ack_clear();
    dist_region_multihost_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _handle_a, stop_a) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 16);
    let (addr_b, _handle_b, stop_b) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 16);
    thread::sleep(Duration::from_millis(25));
    let key_a = addr_a.ip().to_string();
    let key_b = addr_b.ip().to_string();
    cdn_pop_configure(&[
        ("pop-us", key_a.as_str(), format!("tcp://{addr_a}").as_str()),
        ("pop-eu", key_b.as_str(), format!("tcp://{addr_b}").as_str()),
    ]);

    assert!(!cdn_pop_has_endpoints(), "127.0.0.1 dual PoP must not be GA");
    assert_ne!(cdn_doctor_mode(), "cdn-pop-anycast");
    assert!(!cdn_route_counts_as_ga());
    assert!(
        cdn_pop_route("pop-us", 1, 2, "p114-loopback-dual")
            .expect_err("loopback dual port must not route as PoP GA")
            .contains("non-loopback")
    );
    assert!(
        !cdn_route("pop-us", 3, 4, "p114-loopback-cdn"),
        "loopback PoP must not succeed via cdn_route GA path"
    );

    stop_a.stop();
    stop_b.stop();
    cdn_pop_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_hostkey_reject_v4_loopback_bind_not_ga() {
    let _guard = P114_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("127.0.0.1:0", 8);
    thread::sleep(Duration::from_millis(20));
    let connect_url = format!("tcp://{bind_addr}");
    let host_key = bind_addr.ip().to_string();
    assert!(dist_host_key_addr_is_loopback(&host_key));
    dist_region_multihost_configure_keys(&[(
        "us-east",
        "edge-loop",
        host_key.as_str(),
        connect_url.as_str(),
    )]);
    assert!(
        !dist_region_multihost_has_runtime_bound_host_key(),
        "loopback runtime bind must not satisfy hostkey GA"
    );
    assert!(!dist_region_multihost_has_runtime_bound_host_key_v4());
    stop.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_hostkey_reject_v4_synthetic_nonloopback_not_ga() {
    let _guard = P114_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_host_key_register_runtime_bind(
        &"10.0.0.5:0"
            .parse::<SocketAddr>()
            .expect("synthetic non-loopback bind"),
    );
    dist_region_multihost_configure_keys(&[(
        "us-east",
        "edge-nl",
        "10.0.0.5",
        "tcp://127.0.0.1:9443",
    )]);
    assert!(!dist_host_key_addr_is_loopback("10.0.0.5"));
    assert!(!dist_host_key_addr_is_documentation("10.0.0.5"));
    assert!(
        !dist_region_multihost_has_runtime_bound_host_key(),
        "synthetic 10.x without live listener must not satisfy hostkey GA"
    );
    dist_region_multihost_clear();
}

#[test]
fn dist_hostkey_reject_v4_nonloopback_bind_route() {
    let _guard = P114_DIST_LOCK.lock().unwrap();
    dist_region_multihost_clear();
    dist_actor_mp_clear();
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let connect_url = format!("tcp://127.0.0.1:{}", bind_addr.port());
    let host_key = bind_addr.ip().to_string();
    assert!(!dist_host_key_addr_is_loopback(&host_key));
    dist_region_multihost_configure_keys(&[(
        "us-east",
        "edge-rt",
        host_key.as_str(),
        connect_url.as_str(),
    )]);
    assert!(dist_region_multihost_has_runtime_bound_host_key());
    assert!(dist_region_multihost_tcp_health("us-east", "edge-rt"));
    let (region, host, ack) =
        dist_region_multihost_route("us-east", "edge-rt", 1, 2, "p114-noloop-bind")
            .expect("route via non-loopback runtime bind");
    assert_eq!(region, "us-east");
    assert_eq!(host, "edge-rt");
    assert_eq!(ack, "p114-noloop-bind");
    stop.stop();
    dist_region_multihost_clear();
}

#[test]
fn dist_hostkey_reject_memory_cdn_not_ga() {
    let _guard = P114_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    dist_region_multihost_clear();
    cdn_routes_memory_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    assert!(!cdn_memory_dev_enabled());
    assert_eq!(cdn_doctor_mode(), "no-cdn-transport");
    assert!(!cdn_route_counts_as_ga());
    assert!(
        !cdn_route("us-east", 1, 2, "silent-memory"),
        "memory CDN must not succeed without PoP/TCP peers or SYX_DIST_CDN_MEMORY=1"
    );
}
