//! Dist sticky PoP MVP (client-key affinity to healthy PoP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_fingerprint, cdn_bgp_ga_ready, cdn_bgp_health,
    cdn_bgp_import_peer_mesh, cdn_bgp_mesh_pop_count, cdn_bgp_mesh_snapshot,
    cdn_bgp_multi_pop_ready, cdn_bgp_route, cdn_bgp_route_sticky, cdn_bgp_sticky_binding,
    cdn_bgp_sticky_pop_binding_count, cdn_bgp_sticky_pop_mvp_ready, cdn_bgp_sticky_pop_ready,
    cdn_bgp_weighted_anycast_ready, cdn_delivery_ack_clear, cdn_doctor_mode,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P125_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_sticky_pop_cdn_bgp_v8_sticky_pop_fingerprint() {
    assert_eq!(cdn_bgp_fingerprint(), "dist-cdn-bgp-v8:sticky-pop-mvp");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v8:sticky-pop-mvp"));
    assert!(src.contains("dist-cdn-bgp-v7:weighted-anycast-mvp"));
    assert!(src.contains("cdn_bgp_sticky_pop_ready"));
    assert!(src.contains("cdn_bgp_route_sticky"));
    assert!(src.contains("CDN_BGP_STICKY_BINDINGS"));
    assert!(src.contains("cdn-bgp-sticky-pop"));
    assert!(src.contains("not carrier BGP"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v8:sticky-pop-mvp"));
    assert!(lib.contains("heap_v97_phase125_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_sticky_pop_sticky_client_key_reuses_healthy_pop() {
    let _guard = P125_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_c, _hc, stop_c) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);

    cdn_bgp_configure_mesh(
        "us-east",
        &[
            ("203.0.113.0/24", 400, "bgp-us", url_a.as_str()),
            ("203.0.113.1/24", 150, "bgp-us-2", url_c.as_str()),
        ],
    );
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );

    assert_eq!(cdn_bgp_mesh_pop_count(), 3);
    assert!(cdn_bgp_multi_pop_ready());
    assert!(cdn_bgp_weighted_anycast_ready());
    assert!(cdn_bgp_sticky_pop_ready());
    assert!(cdn_bgp_sticky_pop_mvp_ready());
    assert!(cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-sticky-pop");

    let snap = cdn_bgp_mesh_snapshot("us-east", "bgp-us-2");
    assert_eq!(snap.fingerprint, "dist-cdn-bgp-v8:sticky-pop-mvp");
    assert!(snap.sticky_pop_ready);
    assert!(snap.weighted_anycast_ready);

    let client = "tenant-p125-sticky-a";
    let (first_pop, first_ack) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p125-sticky-first").expect("first sticky route");
    assert_eq!(first_pop, "bgp-us", "initial sticky bind picks highest-weight healthy PoP");
    assert_eq!(first_ack, "p125-sticky-first");
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert_eq!(cdn_bgp_sticky_pop_binding_count(), 1);

    // Preferred hint would pick bgp-us-2 via weighted route, but sticky client must stay on bgp-us.
    let (second_pop, second_ack) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 3, 4, "p125-sticky-second").expect("second sticky route");
    assert_eq!(second_pop, "bgp-us", "same client key must stick to healthy PoP");
    assert_eq!(second_ack, "p125-sticky-second");

    let (weighted_pop, _) =
        cdn_bgp_route("bgp-us-2", 5, 6, "p125-weight-only").expect("weighted route still works");
    assert_eq!(weighted_pop, "bgp-us");

    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_sticky_pop_sticky_rebinds_when_bound_pop_unhealthy() {
    let _guard = P125_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_c, _hc, stop_c) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);

    cdn_bgp_configure_mesh(
        "us-east",
        &[
            ("203.0.113.0/24", 400, "bgp-us", url_a.as_str()),
            ("203.0.113.1/24", 150, "bgp-us-2", url_c.as_str()),
        ],
    );
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );

    let client = "tenant-p125-sticky-failover";
    let (bound, _) =
        cdn_bgp_route_sticky(client, "bgp-us", 1, 2, "p125-bind").expect("initial bind");
    assert_eq!(bound, "bgp-us");

    stop_a.stop();
    thread::sleep(Duration::from_millis(25));
    assert!(!cdn_bgp_health("bgp-us"));

    let (failover_pop, failover_ack) =
        cdn_bgp_route_sticky(client, "bgp-us", 3, 4, "p125-rebind").expect("sticky failover");
    assert_eq!(failover_pop, "bgp-eu", "sticky client rebinds to next healthy PoP");
    assert_eq!(failover_ack, "p125-rebind");
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-eu"));

    let (still_eu, _) =
        cdn_bgp_route_sticky(client, "bgp-us", 5, 6, "p125-still-sticky").expect("remain on rebound pop");
    assert_eq!(still_eu, "bgp-eu");

    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_sticky_pop_sticky_requires_client_key() {
    let _guard = P125_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_c, _hc, stop_c) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);

    cdn_bgp_configure_mesh(
        "us-east",
        &[("203.0.113.0/24", 400, "bgp-us", url_a.as_str())],
    );
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[
            ("198.51.100.0/24", 250, "bgp-eu", url_b.as_str()),
            ("198.51.100.1/24", 150, "bgp-eu-2", url_c.as_str()),
        ],
    );

    assert!(cdn_bgp_sticky_pop_ready());
    assert!(
        cdn_bgp_route_sticky("", "bgp-us", 1, 2, "empty-key").is_err(),
        "empty client_key must not route sticky GA"
    );

    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
