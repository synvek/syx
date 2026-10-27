//! CDN peer-announce ack after affinity apply; unacked not mesh-ready.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_peer_announce_ack_fingerprint, cdn_bgp_peer_announce_ack_mesh_ready,
    cdn_bgp_peer_announce_ack_path_for, cdn_bgp_peer_announce_ack_ready,
    cdn_bgp_peer_announce_sticky, cdn_bgp_peer_announce_sticky_path_for,
    cdn_bgp_peer_announce_sticky_receive_and_ack,
    cdn_bgp_peer_announce_sticky_receive_from_peer, cdn_bgp_route_sticky,
    cdn_bgp_sticky_binding, cdn_bgp_sticky_binding_expires_at_ms, cdn_bgp_sticky_pop_clear,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key, cdn_delivery_ack_clear,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P130_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_peer_announce_ack_cdn_bgp_v13_peer_announce_ack_fingerprint() {
    assert_eq!(
        cdn_bgp_peer_announce_ack_fingerprint(),
        "dist-cdn-bgp-v13:peer-announce-ack-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v13:peer-announce-ack-mvp"));
    assert!(src.contains("cdn_bgp_peer_announce_write_ack"));
    assert!(src.contains("cdn_bgp_peer_announce_ack_mesh_ready"));
    assert!(src.contains("cdn_bgp_peer_announce_sticky_receive_and_ack"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v13:peer-announce-ack-mvp"));
    assert!(lib.contains("heap_v102_phase130_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_peer_announce_ack_peer_announce_ack_mesh_ready_ttl_inherit() {
    let _guard = P130_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let announce_dir = std::env::temp_dir().join(format!(
        "syx-p130-peer-announce-ack-{}",
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
    thread::sleep(Duration::from_millis(25));
    let url_a = dist_tcp_url_for_bind_addr(&addr_a);
    let url_b = dist_tcp_url_for_bind_addr(&addr_b);
    let url_c = dist_tcp_url_for_bind_addr(&addr_c);

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

    assert!(cdn_bgp_sticky_pop_ready());
    assert!(cdn_bgp_peer_announce_ack_ready());

    let client = "tenant-us:client-p130";
    assert_eq!(cdn_bgp_sticky_shard_key(client), "tenant-us");
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p130-announce").expect("bind");
    assert_eq!(pop, "bgp-us");
    let expires = cdn_bgp_sticky_binding_expires_at_ms(client).expect("ttl");

    let wire = cdn_bgp_peer_announce_sticky("peer-eu", "tenant-us").expect("announce");
    assert!(wire.contains(client) && wire.contains("bgp-us"));
    let path = cdn_bgp_peer_announce_sticky_path_for("peer-eu", "tenant-us").expect("path");
    assert!(fs::metadata(&path).is_ok(), "missing announce file {path}");

    // Unacked announce is not mesh-ready.
    assert!(
        !cdn_bgp_peer_announce_ack_mesh_ready("peer-eu", "tenant-us"),
        "unacked announce must not be mesh-ready"
    );
    assert!(cdn_bgp_peer_announce_ack_path_for("peer-eu", "tenant-us")
        .map(|p| fs::metadata(&p).is_err())
        .unwrap_or(true));

    // Simulate receiver: clear memory, restore + ack from announce wire.
    cdn_bgp_sticky_pop_clear();
    assert!(cdn_bgp_sticky_binding(client).is_none());

    let restored =
        cdn_bgp_peer_announce_sticky_receive_and_ack("peer-eu", "tenant-us", &wire)
            .expect("receive+ack");
    assert!(restored >= 1);
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert_eq!(
        cdn_bgp_sticky_binding_expires_at_ms(client),
        Some(expires),
        "TTL must be inherited from announce"
    );
    assert!(
        cdn_bgp_peer_announce_ack_mesh_ready("peer-eu", "tenant-us"),
        "acked announce must be mesh-ready"
    );
    let ack_path = cdn_bgp_peer_announce_ack_path_for("peer-eu", "tenant-us").expect("ack path");
    assert!(fs::metadata(&ack_path).is_ok(), "missing ack file {ack_path}");

    // Affinity routing after restore: sticky binding wins.
    let (routed, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p130-reroute").expect("affinity");
    assert_eq!(routed, "bgp-us");

    // File-path receive also writes ack (idempotent).
    cdn_bgp_sticky_pop_clear();
    let from_peer =
        cdn_bgp_peer_announce_sticky_receive_from_peer("peer-eu", "tenant-us").expect("from peer");
    assert!(from_peer >= 1);
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));
    assert!(cdn_bgp_peer_announce_ack_mesh_ready("peer-eu", "tenant-us"));

    std::env::remove_var("SYX_DIST_PEER_ANNOUNCE_DIR");
    let _ = fs::remove_dir_all(&announce_dir);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
