//! CDN peer-announce hold/dampen; suppress re-announce within hold window.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_conflict_epoch_mesh_ready, cdn_bgp_peer_announce_epoch_memory_clear,
    cdn_bgp_peer_announce_epoch_persist_ready, cdn_bgp_peer_announce_fanout_with_epoch,
    cdn_bgp_peer_announce_filter_held_peers, cdn_bgp_peer_announce_hold_active,
    cdn_bgp_peer_announce_hold_fingerprint, cdn_bgp_peer_announce_hold_memory_clear,
    cdn_bgp_peer_announce_hold_ms, cdn_bgp_peer_announce_hold_ready,
    cdn_bgp_peer_announce_parse_hold_wire, cdn_bgp_peer_announce_priority_ready,
    cdn_bgp_peer_announce_receive_ack_with_epoch, cdn_bgp_route_sticky, cdn_bgp_sticky_pop_ready,
    cdn_bgp_sticky_shard_key, cdn_delivery_ack_clear, dist_tcp_url_for_bind_addr,
    spawn_dist_tcp_mailbox_on,
};

static P136_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_hold_cdn_bgp_v19_peer_announce_hold_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_hold_fingerprint(),
        "dist-cdn-bgp-v19:peer-announce-hold-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_hold_active"));
    assert!(src.contains("cdn_bgp_peer_announce_filter_held_peers"));
    assert!(src.contains("#hold:"));
    assert!(src.contains("SYX_DIST_PEER_ANNOUNCE_HOLD_MS"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
    assert!(lib.contains("heap_v108_phase136_track_d_farm_dist_os_fingerprint"));
}

#[test]
fn dist_peer_announce_hold_peer_announce_hold_dampen_window() {
    let _guard = P136_DIST_LOCK.lock().unwrap();
    cdn_bgp_peer_announce_hold_memory_clear();
    std::env::set_var("SYX_DIST_PEER_ANNOUNCE_HOLD_MS", "60000");
    assert_eq!(cdn_bgp_peer_announce_hold_ms(), 60_000);
    assert!(!cdn_bgp_peer_announce_hold_active("peer-a", "shard-x"));
    syx_async::cdn_bgp_peer_announce_hold_mark("peer-a", "shard-x");
    assert!(cdn_bgp_peer_announce_hold_active("peer-a", "shard-x"));
    let filtered = cdn_bgp_peer_announce_filter_held_peers(&["peer-a", "peer-b"], "shard-x");
    assert_eq!(filtered, vec!["peer-b".to_string()]);
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_HOLD_MS");
    cdn_bgp_peer_announce_hold_memory_clear();
}

#[test]
fn dist_peer_announce_hold_peer_announce_hold_fanout_dampen() {
    let _guard = P136_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
    cdn_bgp_peer_announce_epoch_memory_clear();
    cdn_bgp_peer_announce_hold_memory_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p136-peer-announce-hold-{}",
        std::process::id()
    ));
    let persist_file = std::env::temp_dir().join(format!(
        "syx-p136-peer-announce-epoch-{}.tsv",
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
    std::env::set_var("SYX_DIST_PEER_ANNOUNCE_HOLD_MS", "60000");
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
    assert!(cdn_bgp_peer_announce_hold_ready());

    let client = "tenant-us:client-p136";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p136-announce").expect("bind");
    assert_eq!(pop, "bgp-us");

    let peers = ["peer-eu", "peer-ap"];
    let announced =
        cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 1).expect("announce");
    assert_eq!(announced.len(), 2);
    assert!(announced[0].1.contains("#hold:60000"));
    assert_eq!(cdn_bgp_peer_announce_parse_hold_wire(&announced[0].1), Some(60_000));
    assert!(cdn_bgp_peer_announce_hold_active("peer-eu", "tenant-us"));
    assert!(cdn_bgp_peer_announce_hold_active("peer-ap", "tenant-us"));

    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-eu", "tenant-us", &announced[0].1)
        .expect("ack peer-eu");
    cdn_bgp_peer_announce_receive_ack_with_epoch("peer-ap", "tenant-us", &announced[1].1)
        .expect("ack peer-ap");
    assert!(
        cdn_bgp_peer_announce_conflict_epoch_mesh_ready(&peers, "tenant-us"),
        "mesh-ready after first announce"
    );

    let dampened = cdn_bgp_peer_announce_fanout_with_epoch(&peers, "tenant-us", 2);
    assert!(
        dampened
            .as_ref()
            .err()
            .map(|e| e.contains("hold") || e.contains("dampen"))
            .unwrap_or(false),
        "expected hold dampen on re-announce, got {dampened:?}"
    );

    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_DIR");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_EPOCH_PERSIST_FILE");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_TTL_MS");
    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_HOLD_MS");
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
    cdn_bgp_peer_announce_hold_memory_clear();
}
