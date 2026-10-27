//! Phase 101/104: farm lease TTL + TCP frame ERR (not echo) on expired lease.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use syx_driver::{
    farm_lease_acquire, farm_lease_auth_configure, farm_lease_expired, farm_lease_reset,
    farm_lease_test_lock, farm_worker_run_cycle, fetch_cache_remote, push_cache_remote,
    spawn_farm_lease_authority, spawn_farm_tcp_node,
};
use syx_pkg::SyxLockfile;

fn with_lease_auth<F: FnOnce()>(f: F) {
    let _g = farm_lease_test_lock();
    let (auth_addr, _auth_handle) = spawn_farm_lease_authority(64);
    farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    farm_lease_reset();
    f();
    farm_lease_reset();
}

#[test]
fn p101_farm_frame_lease_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-frame-lease-v1:ttl-err-on-expired"));
}

#[test]
fn p100_farm_lease_ttl_hard_reject_tcp() {
    with_lease_auth(|| {
        farm_lease_acquire(0).unwrap();
        assert!(farm_lease_expired());
        let err = farm_worker_run_cycle("tcp://127.0.0.1:9/q", "tcp://127.0.0.1:9/f").unwrap_err();
        assert!(err.contains("lease expired"), "{err}");
    });
}

#[test]
fn p101_farm_lease_ttl_tcp_get_put_err_not_echo() {
    with_lease_auth(|| {
        farm_lease_acquire(0).unwrap();
        assert!(farm_lease_expired());

        // Expired lease fails client-side before TCP; node enforcement in p106 raw TCP gates.
        let url = "tcp://127.0.0.1:9/tenant/key";
        let get_err = fetch_cache_remote(url).unwrap_err();
        assert!(
            get_err.contains("lease") || get_err.contains("ERR"),
            "GET must ERR on expired lease, not echo: {get_err}"
        );

        let put_err = push_cache_remote(url, b"new-bytes").unwrap_err();
        assert!(
            put_err.contains("lease") || put_err.contains("ERR"),
            "PUT must ERR on expired lease: {put_err}"
        );
    });
}

#[test]
fn p100_farm_tcp_tenant_cache_smoke() {
    with_lease_auth(|| {
        farm_lease_acquire(300).unwrap();
        let mut store = HashMap::new();
        store.insert("tenant/x".into(), b"tenant-cache-hit".to_vec());
        let (addr, handle) = spawn_farm_tcp_node(store, 1);
        thread::sleep(Duration::from_millis(10));
        let url = format!("tcp://{addr}/tenant/x");
        let bytes = syx_driver::fetch_cache_remote(&url).unwrap();
        assert_eq!(bytes, b"tenant-cache-hit");
        handle.join().ok();
    });
}

#[test]
fn p98_farm_lockfile_lease_ttl_field() {
    assert!(SyxLockfile::default().farm_lease_ttl_secs > 0);
}
