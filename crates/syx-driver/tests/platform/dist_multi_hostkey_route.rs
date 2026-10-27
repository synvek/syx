//! Dist multi-hostkey-route PoP (≥2 distinct remote hostkeys).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_delivery_ack_clear, cdn_delivery_ack_count, cdn_doctor_mode, cdn_pop_clear,
    cdn_pop_configure, cdn_pop_fingerprint, cdn_pop_has_endpoints, cdn_pop_health,
    cdn_pop_multi_hostkey_route_ready, cdn_pop_route, dist_connect_url_is_loopback,
    dist_host_key_remote_for_bind, dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P119_DIST_LOCK: Mutex<()> = Mutex::new(());

fn spawn_remote_pop(max_conns: usize) -> (String, String, String, syx_async::ArcStop) {
    let (bind_addr, _handle, stop) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", max_conns);
    thread::sleep(Duration::from_millis(25));
    let host_key = dist_host_key_remote_for_bind(&bind_addr);
    let connect_url = dist_tcp_url_for_bind_addr(&bind_addr);
    let region = format!("pop-{}", bind_addr.port());
    assert!(!connect_url.contains("127.0.0.1"));
    assert!(!dist_connect_url_is_loopback(&connect_url));
    (region, host_key, connect_url, stop)
}

#[test]
fn dist_multi_hostkey_cdn_pop_v6_fingerprint() {
    assert_eq!(cdn_pop_fingerprint(), "dist-cdn-pop-v6:multi-hostkey-route");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(src.contains("cdn_pop_multi_hostkey_route_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-pop-v6:multi-hostkey-route"));
}

#[test]
fn dist_multi_hostkey_cdn_pop_multi_hostkey_route_e2e() {
    let _guard = P119_DIST_LOCK.lock().unwrap();
    cdn_pop_clear();
    cdn_delivery_ack_clear();

    let (region_a, key_a, url_a, stop_a) = spawn_remote_pop(8);
    let (region_b, key_b, url_b, stop_b) = spawn_remote_pop(8);
    assert_ne!(key_a, key_b, "multi-hostkey PoPs must have distinct hostkeys");

    cdn_pop_configure(&[
        (region_a.as_str(), key_a.as_str(), url_a.as_str()),
        (region_b.as_str(), key_b.as_str(), url_b.as_str()),
    ]);
    assert!(cdn_pop_has_endpoints());
    assert_eq!(cdn_doctor_mode(), "cdn-pop-multi-hostkey-route");
    assert!(cdn_pop_health(&region_a));
    assert!(cdn_pop_health(&region_b));
    assert!(
        !cdn_pop_multi_hostkey_route_ready(),
        "route proof must not be set before successful delivery"
    );

    let ack_before = cdn_delivery_ack_count();
    let (active, ack) = cdn_pop_route(&region_a, 1, 2, "p119-multi-hostkey-primary")
        .expect("multi-hostkey PoP route");
    assert_eq!(active, region_a);
    assert_eq!(ack, "p119-multi-hostkey-primary");
    assert!(cdn_delivery_ack_count() > ack_before);
    assert!(cdn_pop_multi_hostkey_route_ready());

    stop_a.stop();
    stop_b.stop();
    cdn_pop_clear();
    cdn_delivery_ack_clear();
}
