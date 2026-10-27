//! CDN peer-announce conflict-epoch; higher epoch wins; stale rejects.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_conflict_epoch_fingerprint,
    cdn_bgp_peer_announce_conflict_epoch_mesh_ready, cdn_bgp_peer_announce_conflict_epoch_ready,
    cdn_bgp_peer_announce_fanout_with_epoch, cdn_bgp_peer_announce_receive_ack_with_epoch,
    cdn_bgp_peer_announce_sticky_path_for, cdn_bgp_peer_announce_winning_epoch,
    cdn_bgp_route_sticky, cdn_bgp_sticky_binding, cdn_bgp_sticky_binding_expires_at_ms,
    cdn_bgp_sticky_pop_clear, cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key,
    cdn_delivery_ack_clear, dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P132_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_conflict_cdn_bgp_v15_peer_announce_conflict_epoch_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_conflict_epoch_fingerprint(),
        "dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_fanout_with_epoch"));
    assert!(src.contains("cdn_bgp_peer_announce_conflict_epoch_mesh_ready"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"));
    assert!(lib.contains("heap_v104_phase132_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_peer_announce_conflict_peer_announce_conflict_epoch_higher_wins_stale_reject_ttl() {
    let _guard = P132_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p132-peer-announce-epoch-{}",
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
    assert!(cdn_bgp_peer_announce_conflict_epoch_ready());

    let client = "tenant-us:client-p132";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p132-announce").expect("bind");
    assert_eq!(pop, "bgp-us");
    let expires = cdn_bgp_sticky_binding_expires_at_ms(client).expect("ttl");

    let peers = ["peer-eu", "peer-ap"];

    // Epoch 1 announce.
    let announced_e1 =
        cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1).expect("epoch1");
    assert_eq!(announced_e1.len(), 2);
    assert_eq!(cdn_bgp_peer_announce_winning_epoch("tenant-us"), 1);
    for (peer_id, wire) in &announced_e1 {
        assert!(wire.starts_with("#epoch:1\n"));
        assert!(wire.contains(client) && wire.contains("bgp-us"));
        let path = cdn_bgp_peer_announce_sticky_path_for(peer_id, "tenant-us").expect("path");
        assert!(fs::metadata(&path).is_ok(), "missing announce file {path}");
    }

    // Stale concurrent announce (epoch 1 again is ok / same; epoch 0 rejected via API; epoch smaller rejected).
    let stale = cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1);
    assert!(stale.is_ok(), "same epoch re-announce allowed");

    // Higher epoch wins and clears prior acks/announces.
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-eu", "tenant-us", &announced_e1[0].1)
        .expect("ack e1 peer-eu");
    let announced_e2 =
        cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 2).expect("epoch2 wins");
    assert_eq!(cdn_bgp_peer_announce_winning_epoch("tenant-us"), 2);
    assert!(
        !cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "new winning epoch must reset mesh-ready"
    );

    // Stale epoch 1 after winning 2 must reject.
    let stale_low = cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1)
        .expect_err("stale epoch must reject");
    assert!(
        stale_low.contains("stale"),
        "unexpected stale err: {stale_low}"
    );
    let stale_ack = cdn_bgp_peer_announce_receive_ack_with_epoch(
        "peer-eu",
        "tenant-us",
        &announced_e1[0].1,
    )
    .expect_err("stale ack must reject");
    assert!(
        stale_ack.contains("stale"),
        "unexpected stale ack err: {stale_ack}"
    );

    // Winning epoch fanout all-ack → mesh-ready; TTL inherit.
    assert!(
        !cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "unacked winning epoch not mesh-ready"
    );
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-eu", "tenant-us", &announced_e2[0].1)
        .expect("ack e2 peer-eu");
    assert!(
        !cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "partial winning-epoch ack not mesh-ready"
    );

    cdn_bgp_sticky_pop_clear();
    assert!(cdn_bgp_sticky_binding(client).is_none());
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-ap", "tenant-us", &announced_e2[1].1)
        .expect("ack e2 peer-ap");
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert_eq!(
        cdn_bgp_sticky_binding_expires_at_ms(client),
        Some(expires),
        "TTL must be inherited from announce"
    );
    assert!(
        cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "winning epoch all-ack must be mesh-ready"
    );

    let (routed, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p132-reroute").expect("affinity");
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
