//! CDN peer-announce priority ordering; higher priority first.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_conflict_epoch_mesh_ready, cdn_bgp_peer_announce_default_priority,
    cdn_bgp_peer_announce_epoch_memory_clear, cdn_bgp_peer_announce_epoch_persist_ready,
    cdn_bgp_peer_announce_fanout_with_epoch, cdn_bgp_peer_announce_peer_priority,
    cdn_bgp_peer_announce_priority_fingerprint, cdn_bgp_peer_announce_priority_ready,
    cdn_bgp_peer_announce_priority_wins, cdn_bgp_peer_announce_receive_ack_with_epoch,
    cdn_bgp_peer_announce_sort_peers_by_priority, cdn_bgp_route_sticky,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key, cdn_delivery_ack_clear,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P135_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_priority_cdn_bgp_v18_peer_announce_priority_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_priority_fingerprint(),
        "dist-cdn-bgp-v18:peer-announce-priority-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v18:peer-announce-priority-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_sort_peers_by_priority"));
    assert!(src.contains("cdn_bgp_peer_announce_peer_priority"));
    assert!(src.contains("#priority:"));
    assert!(src.contains("SYX_DIST_PEER_ANNOUNCE_PRIORITY"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v18:peer-announce-priority-mvp"));
    assert!(lib.contains("heap_v107_phase135_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_peer_announce_priority_peer_announce_priority_ordering_wins() {
    assert!(cdn_bgp_peer_announce_priority_wins(300, 150));
    assert!(!cdn_bgp_peer_announce_priority_wins(100, 200));
    let sorted = cdn_bgp_peer_announce_sort_peers_by_priority(&["peer-low", "peer-high"]);
    assert_eq!(sorted.len(), 2);
}

#[test]
fn dist_peer_announce_priority_peer_announce_priority_fanout_order() {
    let _guard = P135_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    cdn_bgp_peer_announce_epoch_memory_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p135-peer-announce-priority-{}",
        std::process::id()
    ));
    let persist_file = std::env::temp_dir().join(format!(
        "syx-p135-peer-announce-epoch-{}.tsv",
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
    std::env::set_var("SYX_DIST_PEER_ANNOUNCE_TTL_MS", "120000");
    std::env::set_var("SYX_DIST_PEER_ANNOUNCE_PRIORITY", "100");
    std::env::set_var("SYX_DIST_PEER_PEER_EU_ANNOUNCE_PRIORITY", "350");
    std::env::set_var("SYX_DIST_PEER_PEER_AP_ANNOUNCE_PRIORITY", "120");
    std::env::remove_var("SYX_DIST_STICKY_PERSIST_FILE");
    std::env::remove_var("SYX_DIST_STICKY_SNAPSHOT_DIR");

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
        &[("198.51.100.0/24", 250, "bgp-eu", url_b.as_str())],
    );
    cdn_bgp_import_peer_mesh(
        "peer-ap",
        "ap-south",
        &[("192.0.2.0/24", 200, "bgp-ap", url_c.as_str())],
    );

    assert!(cdn_bgp_sticky_pop_ready());
    assert!(cdn_bgp_peer_announce_epoch_persist_ready());
    assert!(cdn_bgp_peer_announce_priority_ready());
    assert_eq!(cdn_bgp_peer_announce_default_priority(), 100);
    assert_eq!(cdn_bgp_peer_announce_peer_priority("peer-eu"), 350);
    assert_eq!(cdn_bgp_peer_announce_peer_priority("peer-ap"), 120);
    assert!(cdn_bgp_peer_announce_priority_wins(
        cdn_bgp_peer_announce_peer_priority("peer-eu"),
        cdn_bgp_peer_announce_peer_priority("peer-ap"),
    ));

    let client = "tenant-us:client-p135";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p135-announce").expect("bind");
    assert_eq!(pop, "bgp-us");

    let peers = ["peer-eu", "peer-ap"];
    let announced =
        cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1).expect("announce");
    assert_eq!(announced.len(), 2);
    assert_eq!(announced[0].0, "peer-eu", "higher priority peer first");
    assert_eq!(announced[1].0, "peer-ap");
    assert!(announced[0].1.contains("#priority:350"));
    assert!(announced[1].1.contains("#priority:120"));

    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-eu", "tenant-us", &announced[0].1)
        .expect("ack peer-eu");
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-ap", "tenant-us", &announced[1].1)
        .expect("ack peer-ap");
    assert!(
        cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "mesh-ready with priority-ordered fanout"
    );

    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_DIR");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_TTL_MS");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_PRIORITY");
    std::env::remove_var("SYX_DIST_PEER_PEER_EU_ANNOUNCE_PRIORITY");
    std::env::remove_var("SYX_DIST_PEER_PEER_AP_ANNOUNCE_PRIORITY");
    let _ = fs::remove_dir_all(&announce_dir);
    let _ = fs::remove_file(&persist_file);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    cdn_bgp_peer_announce_epoch_memory_clear();
}
