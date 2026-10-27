//! CDN peer-announce fanout to ≥2 peers; mesh-ready only when all acks.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_fanout, cdn_bgp_peer_announce_fanout_fingerprint,
    cdn_bgp_peer_announce_fanout_mesh_ready, cdn_bgp_peer_announce_fanout_ready,
    cdn_bgp_peer_announce_sticky_path_for, cdn_bgp_peer_announce_sticky_receive_and_ack,
    cdn_bgp_route_sticky, cdn_bgp_sticky_binding, cdn_bgp_sticky_binding_expires_at_ms,
    cdn_bgp_sticky_pop_clear, cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key,
    cdn_delivery_ack_clear, dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P131_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_fanout_cdn_bgp_v14_peer_announce_fanout_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_fanout_fingerprint(),
        "dist-cdn-bgp-v14:peer-announce-fanout-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v14:peer-announce-fanout-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_fanout"));
    assert!(src.contains("cdn_bgp_peer_announce_fanout_mesh_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v14:peer-announce-fanout-mvp"));
    assert!(lib.contains("heap_v103_phase131_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_peer_announce_fanout_peer_announce_fanout_partial_ack_not_mesh_ready_ttl_inherit() {
    let _guard = P131_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p131-peer-announce-fanout-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&announce_dir);
    fs::create_dir_all(&announce_dir).expect("announce dir");
    std::env::set_var(
        "SYX_DIST_PEER_ANNOUNCE_DIR",
        announce_dir.to_string_lossy().as_ref(),
    );
    std::env::remove_var("SYX_DIST_STICKY_PERSIST_FILE");
    std::env::remove_var("SYX_DIST_STICKY_SNAPSHOT_DIR");

    let (addr_a, _ha, stop_a) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_b, _hb, stop_b) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_c, _hc, stop_c) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    let (addr_d, _hd, stop_d) = spawn_dist_tcp_mailbox_on("0.0.0.0:0", 16);
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);
    let url_d = dist_tcp_url_for_bind_addr(&addr_d);

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
    cdn_bgp_import_peer_mesh(
        "peer-ap",
        "ap-south",
        &[("192.0.2.0/24", 200, "bgp-ap", url_d.as_str())],
    );

    assert!(cdn_bgp_sticky_pop_ready());
    assert!(cdn_bgp_peer_announce_fanout_ready());

    let client = "tenant-us:client-p131";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p131-announce").expect("bind");
    assert_eq!(pop, "bgp-us");
    let expires = cdn_bgp_sticky_binding_expires_at_ms(client).expect("ttl");

    let peers = ["peer-eu", "peer-ap"];
    let announced =
        cdn_bgp_peer_announce_fanout(&peers, "tenant-us").expect("fanout announce");
    assert_eq!(announced.len(), 2);
    for (peer_id, wire) in &announced {
        assert!(wire.contains(client) && wire.contains("bgp-us"));
        let path = cdn_bgp_peer_announce_sticky_path_for(peer_id, "tenant-us").expect("path");
        assert!(fs::metadata(&path).is_ok(), "missing announce file {path}");
    }

    // No acks yet → not mesh-ready.
    assert!(
        !cdn_bgp_peer_announce_fanout_mesh_ready(&peers, "tenant-us"),
        "unacked fanout must not be mesh-ready"
    );

    // Partial ack (only peer-eu) → still not mesh-ready.
    let wire_eu = &announced[0].1;
    let restored_eu =
        cdn_bgp_peer_announce_sticky_receive_and_ack("peer-eu", "tenant-us", wire_eu)
            .expect("ack peer-eu");
    assert!(restored_eu >= 1);
    assert!(
        !cdn_bgp_peer_announce_fanout_mesh_ready(&peers, "tenant-us"),
        "partial fanout ack must not be mesh-ready"
    );

    // Ack remaining peer → mesh-ready; TTL inherited.
    cdn_bgp_sticky_pop_clear();
    assert!(cdn_bgp_sticky_binding(client).is_none());
    let wire_ap = &announced[1].1;
    let restored_ap =
        cdn_bgp_peer_announce_sticky_receive_and_ack("peer-ap", "tenant-us", wire_ap)
            .expect("ack peer-ap");
    assert!(restored_ap >= 1);
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert_eq!(
        cdn_bgp_sticky_binding_expires_at_ms(client),
        Some(expires),
        "TTL must be inherited from announce"
    );
    assert!(
        cdn_bgp_peer_announce_fanout_mesh_ready(&peers, "tenant-us"),
        "all peer acks must be mesh-ready"
    );

    let (routed, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p131-reroute").expect("affinity");
    assert_eq!(routed, "bgp-us");

    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_DIR");
    let _ = fs::remove_dir_all(&announce_dir);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    stop_d.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
