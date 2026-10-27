//! Dist sticky PoP binding file persist/restore.

use std::fs;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use syx_async::{
    cdn_bgp_clear, cdn_bgp_configure_mesh, cdn_bgp_import_peer_mesh,
    cdn_bgp_route_sticky, cdn_bgp_sticky_binding, cdn_bgp_sticky_persist_fingerprint,
    cdn_bgp_sticky_persist_ready, cdn_bgp_sticky_persist_restore_from_file,
    cdn_bgp_sticky_pop_clear, cdn_bgp_sticky_pop_ready, cdn_delivery_ack_clear,
    dist_tcp_url_for_bind_addr, spawn_dist_tcp_mailbox_on,
};

static P127_DIST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn dist_sticky_persist_cdn_bgp_v10_sticky_persist_fingerprint() {
    assert_eq!(
        cdn_bgp_sticky_persist_fingerprint(),
        "dist-cdn-bgp-v10:sticky-persist-mvp"
    );
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist-cdn-bgp-v10:sticky-persist-mvp"));
    assert!(src.contains("cdn_bgp_sticky_persist_save"));
    assert!(src.contains("cdn_bgp_sticky_persist_restore"));
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("dist-cdn-bgp-v10:sticky-persist-mvp"));
    assert!(lib.contains("heap_v99_phase127_track_d_farm_dist_fingerprint"));
}

#[test]
fn dist_sticky_persist_sticky_persist_restore_after_restart() {
    let _guard = P127_DIST_LOCK.lock().unwrap();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();

    let persist_file = std::env::temp_dir().join(format!(
        "syx-p127-sticky-persist-{}.tsv",
        std::process::id()
    ));
    let _ = fs::remove_file(&persist_file);
    std::env::set_var(
        "SYX_DIST_STICKY_PERSIST_FILE",
        persist_file.to_string_lossy().as_ref(),
    );

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
    assert!(cdn_bgp_sticky_persist_ready());

    let client = "tenant-p127-sticky-persist";
    let (pop, _) =
        cdn_bgp_route_sticky(client, "bgp-us-2", 1, 2, "p127-persist").expect("initial bind");
    assert_eq!(pop, "bgp-us");
    assert!(persist_file.exists());

    // Simulate process restart: in-memory bindings cleared but persist file kept.
    let saved_persist = fs::read_to_string(&persist_file).expect("persist snapshot");
    cdn_bgp_sticky_pop_clear();
    assert!(cdn_bgp_sticky_binding(client).is_none());
    fs::write(&persist_file, saved_persist).expect("restore persist snapshot");
    cdn_bgp_sticky_persist_restore_from_file();
    assert_eq!(cdn_bgp_sticky_binding(client).as_deref(), Some("bgp-us"));

    std::env::remove_var("SYX_DIST_STICKY_PERSIST_FILE");
    let _ = fs::remove_file(&persist_file);
    stop_a.stop();
    stop_b.stop();
    stop_c.stop();
    cdn_bgp_clear();
    cdn_delivery_ack_clear();
}
