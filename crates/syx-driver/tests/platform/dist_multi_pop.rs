//! Dist multi-PoP mesh (≥3 PoPs, cross-region anycast; not carrier BGP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_cross_region_failover_ready,
    cdn_bgp_fingerprint, cdn_bgp_ga_ready, cdn_bgp_health, cdn_bgp_import_peer_mesh,
    cdn_bgp_mesh_pop_count, cdn_bgp_mesh_pop_ids, cdn_bgp_mesh_region_count,
    cdn_bgp_mesh_snapshot, cdn_bgp_multi_pop_mvp_ready,
    cdn_bgp_multi_pop_ready, cdn_bgp_multi_region_mesh_ready, cdn_bgp_route,
    cdn_delivery_ack_clear, cdn_doctor_mode, dist_tcp_url_for_bind_addr,
    spawn_dist_tcp_mailbox_on,
};

static P123_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_multi_pop_cdn_bgp_v6_multi_pop_fingerprint() {
    // Phase 124 supersedes runtime fingerprint to v7; v6 remains as inherited literal.
    assert_eq!(cdn_bgp_fingerprint(), "dist-cdn-bgp-v7:weighted-anycast-mvp");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v6:multi-pop-mvp"));
    assert!(src.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(src.contains("cdn_bgp_multi_pop_ready"));
    assert!(src.contains("cdn_bgp_mesh_pop_count"));
    assert!(src.contains("cdn-bgp-multi-pop"));
    assert!(src.contains("not carrier BGP"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v6:multi-pop-mvp"));
    assert!(lib.contains("heap_v95_phase123_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_multi_pop_two_pops_not_multi_pop_ga() {
    let _guard = P123_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);

    cdn_bgp_configure_mesh("us-east", &[("203.0.113.0/24", 250, "bgp-us", url_a.as_str())]);
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 100, "bgp-eu", url_b.as_str())],
    );

    assert_eq!(cdn_bgp_mesh_pop_count(), 2);
    assert!(cdn_bgp_multi_region_mesh_ready());
    assert!(!cdn_bgp_multi_pop_ready());
    assert!(!cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-region-mesh");

    stop_a.stop();
    stop_b.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_multi_pop_cdn_bgp_multi_pop_failover_mvp() {
    let _guard = P123_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

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
            ("203.0.113.0/24", 300, "bgp-us", url_a.as_str()),
            ("203.0.113.1/24", 200, "bgp-us-2", url_c.as_str()),
        ],
    );
    assert!(!cdn_bgp_multi_pop_ready(), "imported peer mesh required");

    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );

    assert_eq!(cdn_bgp_mesh_region_count(), 2);
    assert_eq!(cdn_bgp_mesh_pop_count(), 3);
    let pops = cdn_bgp_mesh_pop_ids();
    assert!(pops.contains(&"bgp-us".to_string()));
    assert!(pops.contains(&"bgp-us-2".to_string()));
    assert!(pops.contains(&"bgp-eu".to_string()));
    assert!(cdn_bgp_cross_region_failover_ready());
    assert!(cdn_bgp_multi_region_mesh_ready());
    assert!(cdn_bgp_multi_pop_mvp_ready());
    assert!(cdn_bgp_multi_pop_ready());
    assert!(cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-weighted-anycast");

    let snap = cdn_bgp_mesh_snapshot("us-east", "bgp-us");
    assert_eq!(snap.fingerprint, "dist-cdn-bgp-v7:weighted-anycast-mvp");
    assert!(snap.multi_region_mesh_ready);
    assert!(snap.multi_pop_ready);
    assert!(snap.weighted_anycast_ready);
    assert_eq!(snap.mesh_pop_count, 3);
    assert_eq!(snap.mesh_regions.len(), 2);
    assert!(snap.cross_region_failover_order.contains(&"bgp-eu".to_string()));

    let (active, ack) =
        cdn_bgp_route("bgp-us", 1, 2, "p123-pop-primary").expect("primary healthy route");
    assert_eq!(active, "bgp-us");
    assert_eq!(ack, "p123-pop-primary");

    stop_a.stop();
    thread::sleep(Duration::from_millis(25));
    assert!(!cdn_bgp_health("bgp-us"), "primary pop must be unhealthy after stop");

    let (failover_pop, failover_ack) =
        cdn_bgp_route("bgp-us", 3, 4, "p123-pop-failover").expect("multi-pop failover");
    assert!(
        failover_pop == "bgp-us-2" || failover_pop == "bgp-eu",
        "expected intra- or cross-region failover, got {failover_pop}"
    );
    assert_eq!(failover_ack, "p123-pop-failover");

    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
