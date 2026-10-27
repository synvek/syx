//! Dist BGP ops-visibility (peer ids + health map; not carrier BGP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure, cdn_bgp_fingerprint, cdn_bgp_ga_ready,
    cdn_bgp_import_peer_view, cdn_bgp_list_peer_ids, cdn_bgp_ops_snapshot,
    cdn_bgp_ops_visibility_ready, cdn_bgp_peer_health_map, cdn_bgp_route,
    cdn_delivery_ack_clear, cdn_doctor_mode, dist_tcp_url_for_bind_addr,
    spawn_dist_tcp_mailbox_on,
};

static P120_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_bgp_ops_cdn_bgp_v3_ops_visibility_fingerprint() {
    // Phase 124 fingerprint supersedes — see dist_weighted_anycast.
    assert_eq!(
        cdn_bgp_fingerprint(),
        "dist-cdn-bgp-v7:weighted-anycast-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v3:ops-visibility"));
    assert!(src.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
    assert!(src.contains("cdn_bgp_list_peer_ids"));
    assert!(src.contains("cdn_bgp_peer_health_map"));
    assert!(src.contains("cdn_bgp_ops_snapshot"));
    let lib = include_str!("../../../syx-async/src/lib.rs");
    assert!(lib.contains("cdn_bgp_ops_snapshot"));
    assert!(lib.contains("cdn_bgp_list_peer_ids"));
}

#[test]
fn dist_bgp_ops_cdn_bgp_ops_visibility_mvp() {
    let _guard = P120_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    std::env::remove_var("SYX_DIST_CDN_MEMORY");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);

    cdn_bgp_configure(&[("203.0.113.0/24", 100, "bgp-us", url_a.as_str())]);
    assert!(!cdn_bgp_ops_visibility_ready(), "imported peer required");

    cdn_bgp_import_peer_view(
        "peer-eu",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );
    assert!(cdn_bgp_ops_visibility_ready());
    assert!(!cdn_bgp_ga_ready(), "ops visibility alone must not pass mesh GA");
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-route-convergence");

    let peer_ids = cdn_bgp_list_peer_ids();
    assert_eq!(peer_ids, vec!["peer-eu".to_string()]);

    let health = cdn_bgp_peer_health_map();
    assert!(health.iter().any(|(pop, ok)| pop == "bgp-us" && *ok));
    assert!(health.iter().any(|(pop, ok)| pop == "bgp-eu" && *ok));

    let snap = cdn_bgp_ops_snapshot();
    assert_eq!(snap.fingerprint, "dist-cdn-bgp-v5:multi-region-mesh-mvp");
    assert!(snap.ops_visibility_ready);
    assert_eq!(snap.imported_peer_count, 1);
    assert!(snap.merged_endpoint_count >= 2);
    assert_eq!(snap.peer_ids, peer_ids);
    assert_eq!(snap.pop_health.len(), health.len());

    assert!(
        cdn_bgp_route("bgp-us", 1, 2, "p120-bgp-ops-visibility").is_err(),
        "route requires multi-region mesh GA"
    );

    stop_a.stop();
    stop_b.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
