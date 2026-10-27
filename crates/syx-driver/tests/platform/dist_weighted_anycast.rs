//! Dist weighted anycast MVP (weight/health over multi-PoP; not carrier BGP).

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_fingerprint, cdn_bgp_ga_ready, cdn_bgp_health,
    cdn_bgp_import_peer_mesh, cdn_bgp_mesh_pop_count, cdn_bgp_mesh_snapshot,
    cdn_bgp_multi_pop_ready, cdn_bgp_pop_weights, cdn_bgp_route, cdn_bgp_weighted_anycast_mvp_ready,
    cdn_bgp_weighted_anycast_ready, cdn_bgp_weights_differentiated, cdn_delivery_ack_clear,
    cdn_doctor_mode, dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P124_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_weighted_anycast_cdn_bgp_v7_weighted_anycast_fingerprint() {
    assert_eq!(cdn_bgp_fingerprint(), "dist-cdn-bgp-v8:sticky-pop-mvp");
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v8:sticky-pop-mvp"));
    assert!(src.contains("dist-cdn-bgp-v7:weighted-anycast-mvp"));
    assert!(src.contains("cdn_bgp_weighted_anycast_ready"));
    assert!(src.contains("cdn_bgp_weights_differentiated"));
    assert!(src.contains("cdn_bgp_pop_weights"));
    assert!(src.contains("cdn-bgp-weighted-anycast"));
    assert!(src.contains("not carrier BGP"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v8:sticky-pop-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v7:weighted-anycast-mvp"));
    assert!(lib.contains("heap_v96_phase124_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_weighted_anycast_equal_weight_multi_pop_not_weighted_ga() {
    let _guard = P124_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_c, _hc, stop_c) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);

    // Equal weights across ≥3 PoPs: multi-pop ok, weighted-anycast must NOT GA.
    cdn_bgp_configure_mesh(
        "us-east",
        &[
            ("203.0.113.0/24", 100, "bgp-us", url_a.as_str()),
            ("203.0.113.1/24", 100, "bgp-us-2", url_c.as_str()),
        ],
    );
    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 100, "bgp-eu", url_b.as_str())],
    );

    assert_eq!(cdn_bgp_mesh_pop_count(), 3);
    assert!(cdn_bgp_multi_pop_ready());
    assert!(!cdn_bgp_weights_differentiated());
    assert!(!cdn_bgp_weighted_anycast_ready());
    assert!(!cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-multi-pop");
    assert!(
        cdn_bgp_route("bgp-us", 1, 2, "equal-weight-reject").is_err(),
        "equal-weight multi-PoP must not route as weighted-anycast GA"
    );

    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}

#[test]
fn dist_weighted_anycast_cdn_bgp_weighted_anycast_failover_mvp() {
    let _guard = P124_DIST_LOCK.lock().unwrap();
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

    // Differentiated weights: highest-weight healthy PoP preferred (not carrier BGP).
    cdn_bgp_configure_mesh(
        "us-east",
        &[
            ("203.0.113.0/24", 400, "bgp-us", url_a.as_str()),
            ("203.0.113.1/24", 150, "bgp-us-2", url_c.as_str()),
        ],
    );
    assert!(!cdn_bgp_weighted_anycast_ready(), "imported peer mesh required");

    cdn_bgp_import_peer_mesh(
        "peer-eu",
        "eu-west",
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );

    assert_eq!(cdn_bgp_mesh_pop_count(), 3);
    assert!(cdn_bgp_weights_differentiated());
    assert!(cdn_bgp_multi_pop_ready());
    assert!(cdn_bgp_weighted_anycast_mvp_ready());
    assert!(cdn_bgp_weighted_anycast_ready());
    assert!(cdn_bgp_ga_ready());
    assert_eq!(cdn_doctor_mode(), "cdn-bgp-sticky-pop");

    let weights = cdn_bgp_pop_weights();
    assert!(weights.iter().any(|(p, w)| p == "bgp-us" && *w == 400));
    assert!(weights.iter().any(|(p, w)| p == "bgp-eu" && *w == 250));
    assert!(weights.iter().any(|(p, w)| p == "bgp-us-2" && *w == 150));

    let snap = cdn_bgp_mesh_snapshot("us-east", "bgp-us-2");
    assert_eq!(snap.fingerprint, "dist-cdn-bgp-v8:sticky-pop-mvp");
    assert!(snap.multi_pop_ready);
    assert!(snap.weighted_anycast_ready);
    assert_eq!(snap.mesh_pop_count, 3);
    assert!(!snap.pop_weights.is_empty());
    // Weight dominates preferred-pop hint: highest weight (bgp-us=400) first.
    assert_eq!(snap.cross_region_failover_order.first().map(String::as_str), Some("bgp-us"));

    let (active, ack) =
        cdn_bgp_route("bgp-us-2", 1, 2, "p124-weight-primary").expect("highest-weight route");
    assert_eq!(active, "bgp-us", "weight 400 must beat preferred lower-weight pop");
    assert_eq!(ack, "p124-weight-primary");

    stop_a.stop();
    thread::sleep(Duration::from_millis(25));
    assert!(!cdn_bgp_health("bgp-us"), "primary weight pop must be unhealthy after stop");

    let (failover_pop, failover_ack) =
        cdn_bgp_route("bgp-us", 3, 4, "p124-weight-failover").expect("weighted failover");
    assert_eq!(
        failover_pop, "bgp-eu",
        "next-highest healthy weight (250) should win over 150"
    );
    assert_eq!(failover_ack, "p124-weight-failover");

    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
