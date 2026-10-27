//! Dist multi-region peer mesh (≥2 regions; not carrier BGP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_cross_region_failover_ready,
    cdn_bgp_fingerprint, cdn_bgp_ga_ready, cdn_bgp_health, cdn_bgp_import_peer_mesh,
    cdn_bgp_mesh_region_count, cdn_bgp_mesh_regions, cdn_bgp_mesh_snapshot,
    cdn_bgp_multi_region_mesh_ready, cdn_bgp_route, cdn_delivery_ack_clear, cdn_doctor_mode,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P122_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_multi_region_cdn_bgp_v5_multi_region_mesh_fingerprint() {
    // Phase 124 supersedes runtime fingerprint to v7; v5 remains as inherited literal.
    assert_eq!(
        cdn_bgp_fingerprint(),
        "dist-cdn-bgp-v7:weighted-anycast-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(src.contains("cdn_bgp_configure_mesh"));
    assert!(src.contains("cdn_bgp_import_peer_mesh"));
    assert!(src.contains("cdn_bgp_multi_region_mesh_ready"));
    assert!(src.contains("cdn_bgp_cross_region_failover_order_ids"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(lib.contains("heap_v94_phase122_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_multi_region_cdn_bgp_multi_region_mesh_failover_mvp() {
    let _guard = P122_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);

    cdn_bgp_configure_mesh("us-east", &[("203.0.113.0/24", 250, "bgp-us", url_a.as_str())]);
    assert!(
        !cdn_bgp_multi_region_mesh_ready(),
        "imported peer mesh + ≥2 regions required"
    );
    assert!(!cdn_bgp_ga_ready());

    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 100, "bgp-eu", url_b.as_str())],
    );
    assert_eq!(cdn_bgp_mesh_region_count(), 2);
    assert!(cdn_bgp_mesh_regions().contains(&"us-east".to_string()));
    assert!(cdn_bgp_mesh_regions().contains(&"eu-west".to_string()));
    assert!(cdn_bgp_cross_region_failover_ready());
    assert!(cdn_bgp_multi_region_mesh_ready());
    // Phase 123+ GA requires ≥3 PoPs / weighted anycast — mesh alone is not GA.
    assert!(!cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-region-mesh");

    let snap = cdn_bgp_mesh_snapshot("us-east", "bgp-us");
    assert_eq!(snap.fingerprint, "dist-cdn-bgp-v7:weighted-anycast-mvp");
    assert!(snap.multi_region_mesh_ready);
    assert!(!snap.multi_pop_ready);
    assert!(!snap.weighted_anycast_ready);
    assert_eq!(snap.mesh_regions.len(), 2);
    assert_eq!(snap.cross_region_failover_order.first().map(String::as_str), Some("bgp-us"));
    assert!(snap.cross_region_failover_order.contains(&"bgp-eu".to_string()));

    assert!(
        cdn_bgp_route("bgp-us", 1, 2, "p122-mesh-primary").is_err(),
        "2-PoP mesh must not satisfy weighted-anycast route GA"
    );

    stop_a.stop();
    thread::sleep(Duration::from_millis(25));
    assert!(
        !cdn_bgp_health("bgp-us"),
        "primary pop must be unhealthy after stop"
    );

    stop_b.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
