//! Dist BGP route-convergence / failover hard gate (not carrier BGP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure, cdn_bgp_fingerprint, cdn_bgp_ga_ready,
    cdn_bgp_import_peer_view, cdn_bgp_multi_region_mesh_ready, cdn_bgp_route_convergence_ready,
    cdn_delivery_ack_clear, dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P121_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_bgp_convergence_cdn_bgp_v4_route_convergence_fingerprint() {
    // Phase 124 fingerprint supersedes — see dist_weighted_anycast.
    assert_eq!(
        cdn_bgp_fingerprint(),
        "dist-cdn-bgp-v7:weighted-anycast-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(src.contains("cdn_bgp_route_convergence_ready"));
    assert!(src.contains("cdn_bgp_multi_region_mesh_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(lib.contains("heap_v94_phase122_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_bgp_convergence_cdn_bgp_route_convergence_without_mesh_ga() {
    let _guard = P121_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);

    cdn_bgp_configure(&[("203.0.113.0/24", 250, "bgp-us", url_a.as_str())]);
    cdn_bgp_import_peer_view(
        "peer-eu",
        &[("198.51.100.0/24", 100, "bgp-eu", url_b.as_str())],
    );
    assert!(cdn_bgp_route_convergence_ready());
    assert!(
        !cdn_bgp_multi_region_mesh_ready(),
        "legacy peer view without mesh regions must not pass v5 GA"
    );
    assert!(!cdn_bgp_ga_ready());

    stop_a.stop();
    stop_b.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
