//! CDN peer-announce wire TTL; expire stale peers.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_conflict_epoch_mesh_ready, cdn_bgp_peer_announce_epoch_memory_clear,
    cdn_bgp_peer_announce_epoch_persist_ready, cdn_bgp_peer_announce_fanout_with_epoch,
    cdn_bgp_peer_announce_peer_wire_expired, cdn_bgp_peer_announce_purge_expired_peers,
    cdn_bgp_peer_announce_receive_ack_with_epoch, cdn_bgp_peer_announce_ttl_fingerprint,
    cdn_bgp_peer_announce_ttl_ms, cdn_bgp_peer_announce_ttl_ready, cdn_bgp_route_sticky,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key, cdn_delivery_ack_clear,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P134_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_ttl_cdn_bgp_v17_peer_announce_ttl_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_ttl_fingerprint(),
        "dist-cdn-bgp-v17:peer-announce-ttl-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v17:peer-announce-ttl-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_ttl_ms"));
    assert!(src.contains("cdn_bgp_peer_announce_purge_expired_peers"));
    assert!(src.contains("SYX_DIST_PEER_ANNOUNCE_TTL_MS"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v17:peer-announce-ttl-mvp"));
    assert!(lib.contains("heap_v106_phase134_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_peer_announce_ttl_peer_announce_ttl_expires_stale_peers() {
    let _guard = P134_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    cdn_bgp_peer_announce_epoch_memory_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p134-peer-announce-ttl-{}",
        std::process::id()
    ));
    let persist_file = std::env::temp_dir().join(format!(
        "syx-p134-peer-announce-epoch-{}.tsv",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&announce_dir);
    let _ = fs::remove_file(&persist_file);
    fs::create_dir_all(&announce_dir).expect("announce dir");
    std::env::set_var(
        "SYX_DIST_PEER_ANNOUNCE_DIR",
        announce_dir.to_string_lossy().as_ref(),
    );
    std::env::set_var(
        "SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE",
        persist_file.to_string_lossy().as_ref(),
    );
    std::env::set_var("SYX_DIST_PEER_ANNOUNCE_TTL_MS", "200");
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
    assert!(cdn_bgp_peer_announce_epoch_persist_ready());
    assert!(cdn_bgp_peer_announce_ttl_ready());
    assert_eq!(cdn_bgp_peer_announce_ttl_ms(), 200);

    let client = "tenant-us:client-p134";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p134-announce").expect("bind");
    assert_eq!(pop, "bgp-us");

    let peers = ["peer-eu", "peer-ap"];
    let announced =
        cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1).expect("announce");
    assert!(announced[0].1.contains("#expires:"));

    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-eu", "tenant-us", &announced[0].1)
        .expect("ack peer-eu");
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-ap", "tenant-us", &announced[1].1)
        .expect("ack peer-ap");
    assert!(
        cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "mesh-ready before TTL expiry"
    );

    thread::sleep(Duration::from_millis(250));

    assert!(
        cdn_bgp_peer_announce_peer_wire_expired("peer-eu", "tenant-us"),
        "announce wire must expire"
    );
    assert!(
        !cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "mesh-ready must fail after TTL expiry"
    );

    let purged = cdn_bgp_peer_announce_purge_expired_peers(&peers, "tenant-us");
    assert!(purged >= 2, "purge expired announce/ack files, got {purged}");

    let stale_ack = cdn_bgp_peer_announce_receive_ack_with_epoch(
        "peer-eu",
        "tenant-us",
        &announced[0].1,
    )
    .expect_err("expired wire ack must reject");
    assert!(
        stale_ack.contains("expired"),
        "unexpected stale ack err: {stale_ack}"
    );

    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_DIR");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_TTL_MS");
    let _ = fs::remove_dir_all(&announce_dir);
    let _ = fs::remove_file(&persist_file);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    stop_d.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    cdn_bgp_peer_announce_epoch_memory_clear();
}
