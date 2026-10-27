//! Dist sticky PoP TTL + expiry rebind.

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh, cdn_bgp_mesh_snapshot,
    cdn_bgp_route_sticky, cdn_bgp_sticky_binding, cdn_bgp_sticky_binding_expires_at_ms,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_ttl_fingerprint, cdn_bgp_sticky_ttl_ms,
    cdn_bgp_sticky_ttl_ready, cdn_delivery_ack_clear, dist_tcp_url_for_bind_addr,
    spawn_dist_tcp_mailbox_on,
};

static P126_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_sticky_ttl_cdn_bgp_v9_sticky_ttl_fingerprint() {
    assert_eq!(
        cdn_bgp_sticky_ttl_fingerprint(),
        "dist-cdn-bgp-v9:sticky-ttl-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v9:sticky-ttl-mvp"));
    assert!(src.contains("cdn_bgp_sticky_ttl_ms"));
    assert!(src.contains("expires_at_ms"));
    assert!(src.contains("sticky_ttl_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v9:sticky-ttl-mvp"));
    assert!(lib.contains("heap_v98_phase126_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_sticky_ttl_sticky_ttl_exports_in_mesh_snapshot() {
    let _guard = P126_DIST_LOCK.lock().unwrap();
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

    assert!(cdn_bgp_sticky_pop_ready());
    assert!(cdn_bgp_sticky_ttl_ready());
    assert_eq!(cdn_bgp_sticky_ttl_ms(), 60_000);

    let client = "tenant-p126-sticky-ttl";
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p126-ttl-bind").expect("initial bind");
    assert_eq!(pop, "bgp-us");
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert!(cdn_bgp_sticky_binding_expires_at_ms(client).is_some());

    let snap = cdn_bgp_mesh_snapshot("us-east", "bgp-us-2");
    assert!(snap.sticky_ttl_ready);
    assert_eq!(snap.sticky_ttl_ms, 60_000);
    assert_eq!(snap.sticky_binding_count, 1);

    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_sticky_ttl_sticky_rebinds_after_ttl_expiry() {
    let _guard = P126_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    std::env::set_var("SYX_DIST_STICKY_TTL_MS", "1");

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

    let client = "tenant-p126-sticky-ttl-expire";
    let (first, _) =
        cdn_bgp_route_sticky(client, "bgp-us", 1, 2, "p126-first").expect("first bind");
    assert_eq!(first, "bgp-us");

    thread::sleep(Duration::from_millis(5));
    assert!(cdn_bgp_sticky_binding(client).is_none(), "TTL must expire binding");

    let (rebound, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 3, 4, "p126-rebind").expect("rebind after ttl");
    assert_eq!(rebound, "bgp-us", "expired client must rebind via weighted route");
    assert!(cdn_bgp_sticky_binding_expires_at_ms(client).is_some());

    std::env::remove_var("SYX_DIST_STICKY_TTL_MS");
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
