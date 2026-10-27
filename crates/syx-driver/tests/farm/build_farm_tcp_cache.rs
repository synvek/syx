//! Phase 100: farm TCP tenant cache via frame GET/PUT.

use std::collections::HashMap;
use std::thread;
use syx_driver::{fetch_cache_remote, push_cache_remote, spawn_farm_tcp_node};

use crate::lease_test_harness::with_lease_auth;

#[test]
fn p100_farm_tcp_cache_fetch_hit() {
    with_lease_auth(|| {
        let mut store = HashMap::new();
        store.insert("tenant/node-a".into(), b"tenant-cache-hit".to_vec());
        let (addr, handle) = spawn_farm_tcp_node(store, 1);
        thread::sleep(std::time::Duration::from_millis(20));
        let url = format!("tcp://{addr}/tenant/node-a");
        let bytes = fetch_cache_remote(&url).unwrap();
        assert_eq!(bytes, b"tenant-cache-hit");
        handle.join().ok();
    });
}

#[test]
fn p100_farm_tcp_cache_push_then_get_hit() {
    with_lease_auth(|| {
        let (addr, handle) = spawn_farm_tcp_node(HashMap::new(), 2);
        thread::sleep(std::time::Duration::from_millis(20));
        let url = format!("tcp://{addr}/tenant/push");
        push_cache_remote(&url, b"pushed").unwrap();
        let bytes = fetch_cache_remote(&url).unwrap();
        assert_eq!(bytes, b"pushed");
        handle.join().ok();
    });
}

#[test]
fn p100_farm_tcp_cache_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-tcp-frames-v1:get-put-hit"));
}
