//! Phase 104 Track D: cross-node farm lease authority (TCP grant/renew).

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use syx_driver::{
    farm_lease_acquire, farm_lease_auth_configure, farm_lease_expired, farm_lease_reset,
    fetch_cache_remote, push_cache_remote, spawn_farm_lease_authority, spawn_farm_tcp_node,
};

use crate::lease_test_harness::without_lease_auth;

fn with_lease_auth<F: FnOnce()>(f: F) {
    let _g = syx_driver::farm_lease_test_lock();
    let (auth_addr, _auth_handle) = spawn_farm_lease_authority(32);
    farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    farm_lease_reset();
    f();
    farm_lease_reset();
}

#[test]
fn p104_farm_lease_auth_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-lease-auth-v1:cross-node-ttl"));
    assert!(include_str!("../../src/farm_lease_auth.rs")
        .contains("farm-lease-auth-v1:cross-node-ttl"));
    assert!(
        !include_str!("../../src/build_syx.rs").contains("FARM_LEASE_DEADLINE_MS"),
        "process-local Mutex lease must not be GA"
    );
}

#[test]
fn p105_farm_lease_failclosed_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-lease-failclosed-v1:no-auth-is-err"));
    assert!(include_str!("../../src/farm_lease_auth.rs")
        .contains("farm-lease-failclosed-v1:no-auth-is-err"));
}

#[test]
fn p105_farm_tcp_no_auth_get_put_err() {
    without_lease_auth(|| {
        // Client fail-closed runs before TCP connect — no farm node needed.
        let url = "tcp://127.0.0.1:9/tenant/key";
        let get_err = fetch_cache_remote(url).unwrap_err();
        assert!(
            get_err.contains("lease") || get_err.contains("authority"),
            "GET without auth must fail-closed: {get_err}"
        );
        let put_err = push_cache_remote(url, b"x").unwrap_err();
        assert!(
            put_err.contains("lease") || put_err.contains("authority"),
            "PUT without auth must fail-closed: {put_err}"
        );
    });
}

#[test]
fn p105_farm_lease_query_fail_is_expired() {
    without_lease_auth(|| {
        assert!(
            farm_lease_expired(),
            "missing authority must read as blocked for TCP farm"
        );
    });
}

#[test]
fn p104_farm_dual_node_expired_get_put_err() {
    with_lease_auth(|| {
        farm_lease_acquire(60).unwrap();
        assert!(!farm_lease_expired());

        let mut store_a = HashMap::new();
        store_a.insert("tenant/a".into(), b"node-a".to_vec());
        let mut store_b = HashMap::new();
        store_b.insert("tenant/b".into(), b"node-b".to_vec());

        // One accept each for the successful GETs; expired ops fail client-side (no connect).
        // Server-side TTL enforcement: p106_farm_tcp_raw_* gates.
        let (addr_a, handle_a) = spawn_farm_tcp_node(store_a, 1);
        let (addr_b, handle_b) = spawn_farm_tcp_node(store_b, 1);
        thread::sleep(Duration::from_millis(15));

        let url_a = format!("tcp://{addr_a}/tenant/a");
        let url_b = format!("tcp://{addr_b}/tenant/b");
        assert_eq!(fetch_cache_remote(&url_a).unwrap(), b"node-a");
        assert_eq!(fetch_cache_remote(&url_b).unwrap(), b"node-b");
        handle_a.join().ok();
        handle_b.join().ok();

        farm_lease_acquire(0).unwrap();
        assert!(farm_lease_expired());

        let err_a = fetch_cache_remote(&url_a).unwrap_err();
        assert!(
            err_a.contains("lease") || err_a.contains("ERR"),
            "node A GET must ERR when authority expired: {err_a}"
        );
        let err_b = fetch_cache_remote(&url_b).unwrap_err();
        assert!(
            err_b.contains("lease") || err_b.contains("ERR"),
            "node B GET must ERR when authority expired: {err_b}"
        );

        let put_a = push_cache_remote(&url_a, b"new-a").unwrap_err();
        assert!(
            put_a.contains("lease") || put_a.contains("ERR"),
            "node A PUT must ERR: {put_a}"
        );
        let put_b = push_cache_remote(&url_b, b"new-b").unwrap_err();
        assert!(
            put_b.contains("lease") || put_b.contains("ERR"),
            "node B PUT must ERR: {put_b}"
        );
    });
}


#[test]
fn p105_farm_tcp_multi_accept_smoke() {
    with_lease_auth(|| {
        farm_lease_acquire(60).unwrap();
        let mut store = HashMap::new();
        store.insert("k".into(), b"v".to_vec());
        let (addr, handle) = spawn_farm_tcp_node(store, 3);
        thread::sleep(Duration::from_millis(20));
        let url = format!("tcp://{addr}/k");
        assert_eq!(fetch_cache_remote(&url).unwrap(), b"v");
        assert_eq!(fetch_cache_remote(&url).unwrap(), b"v");
        assert_eq!(fetch_cache_remote(&url).unwrap(), b"v");
        handle.join().ok();
    });
}
