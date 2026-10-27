//! Dist sticky PoP multi-file/shard-key snapshots + TTL inherit.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh, cdn_bgp_route_sticky,
    cdn_bgp_sticky_binding, cdn_bgp_sticky_binding_expires_at_ms,
    cdn_bgp_sticky_multi_snapshot_dir, cdn_bgp_sticky_multi_snapshot_fingerprint,
    cdn_bgp_sticky_multi_snapshot_path_for_key, cdn_bgp_sticky_multi_snapshot_ready,
    cdn_bgp_sticky_multi_snapshot_restore_by_key, cdn_bgp_sticky_pop_clear,
    cdn_bgp_sticky_pop_ready, cdn_bgp_sticky_shard_key, cdn_delivery_ack_clear,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P128_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_sticky_multi_cdn_bgp_v11_sticky_multi_snapshot_fingerprint() {
    assert_eq!(
        cdn_bgp_sticky_multi_snapshot_fingerprint(),
        "dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"));
    assert!(src.contains("cdn_bgp_sticky_multi_snapshot_save_all"));
    assert!(src.contains("cdn_bgp_sticky_multi_snapshot_restore_by_key"));
    assert!(src.contains("SYX_DIST_STICKY_SNAPSHOT_DIR"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"));
    assert!(lib.contains("heap_v100_phase128_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_sticky_multi_sticky_multi_snapshot_restore_by_key_after_restart() {
    let _guard = P128_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let snap_dir = std::env::temp_dir().join(format!(
        "syx-p128-sticky-multi-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&snap_dir);
    fs::create_dir_all(&snap_dir).expect("snapshot dir");
    std::env::set_var(
        "SYX_DIST_STICKY_SNAPSHOT_DIR",
        snap_dir.to_string_lossy().as_ref(),
    );
    std::env::remove_var("SYX_DIST_STICKY_PERSIST_FILE");

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
    assert!(cdn_bgp_sticky_multi_snapshot_ready());
    assert!(cdn_bgp_sticky_multi_snapshot_dir().is_some());

    let client_a = "tenant-us:client-a";
    let client_b = "tenant-eu:client-b";
    assert_eq!(cdn_bgp_sticky_shard_key(client_a), "tenant-us");
    assert_eq!(cdn_bgp_sticky_shard_key(client_b), "tenant-eu");

    let (pop_a, _) =
        cdn_bgp_route_sticky(client_a, "bgp-us-2", 1, 2, "p128-a").expect("bind a");
    assert_eq!(pop_a, "bgp-us");
    let expires_a = cdn_bgp_sticky_binding_expires_at_ms(client_a).expect("ttl a");

    let (pop_b, _) =
        cdn_bgp_route_sticky(client_b, "bgp-eu", 1, 2, "p128-b").expect("bind b");
    assert!(!pop_b.is_empty());

    let path_us = cdn_bgp_sticky_multi_snapshot_path_for_key("tenant-us").expect("path us");
    let path_eu = cdn_bgp_sticky_multi_snapshot_path_for_key("tenant-eu").expect("path eu");
    assert!(fs::metadata(&path_us).is_ok(), "missing shard file {path_us}");
    assert!(fs::metadata(&path_eu).is_ok(), "missing shard file {path_eu}");

    let saved_us = fs::read_to_string(&path_us).expect("read us shard");
    let saved_eu = fs::read_to_string(&path_eu).expect("read eu shard");

    // Simulate process restart: clear memory, restore one shard by key.
    cdn_bgp_sticky_pop_clear();
    assert!(cdn_bgp_sticky_binding(client_a).is_none());
    assert!(cdn_bgp_sticky_binding(client_b).is_none());
    fs::write(&path_us, &saved_us).expect("rewrite us");
    fs::write(&path_eu, &saved_eu).expect("rewrite eu");

    cdn_bgp_sticky_multi_snapshot_restore_by_key("tenant-us");
    assert_eq!(cdn_bgp_sticky_binding(client_a).as_deref(), Some("bgp-us"));
    // TTL inherit: expires_at_ms restored from snapshot.
    assert_eq!(
        cdn_bgp_sticky_binding_expires_at_ms(client_a),
        Some(expires_a)
    );
    // Other shard not yet restored.
    assert!(cdn_bgp_sticky_binding(client_b).is_none());

    cdn_bgp_sticky_multi_snapshot_restore_by_key("tenant-eu");
    assert!(cdn_bgp_sticky_binding(client_b).is_some());

    std::env::remove_var("SYX_DIST_STICKY_SNAPSHOT_DIR");
    let _ = fs::remove_dir_all(&snap_dir);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
