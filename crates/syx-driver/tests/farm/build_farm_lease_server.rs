//! Phase 106 Track D: farm TCP **node** enforces lease (raw frames bypass client fail-closed).

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use syx_driver::{
    farm_lease_acquire, farm_lease_auth_configure, farm_lease_reset, farm_lease_test_lock,
    farm_tcp_encode_get, farm_tcp_encode_put, farm_tcp_parse_response, farm_tcp_roundtrip,
    spawn_farm_lease_authority, spawn_farm_tcp_node, FarmTcpResponse,
};

fn with_lease_auth<F: FnOnce()>(f: F) {
    let _g = farm_lease_test_lock();
    let (auth_addr, _auth_handle) = spawn_farm_lease_authority(64);
    farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    farm_lease_reset();
    f();
    farm_lease_reset();
}

#[test]
fn p106_farm_lease_server_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-lease-server-v1:node-enforces-ttl"));
    assert!(include_str!("../../src/farm_lease_auth.rs")
        .contains("farm-lease-server-v1:node-enforces-ttl"));
    assert!(include_str!("../../src/lib.rs").contains("heap_v78_phase106_track_d_farm_fingerprint"));
}

#[test]
fn p106_farm_tcp_raw_get_no_grant_err() {
    with_lease_auth(|| {
        let mut store = HashMap::new();
        store.insert("k".into(), b"v".to_vec());
        let (addr, handle) = spawn_farm_tcp_node(store, 1);
        thread::sleep(Duration::from_millis(15));
        let host = addr.to_string();
        let payload = farm_tcp_roundtrip(&host, &farm_tcp_encode_get("k")).expect("tcp connect");
        match farm_tcp_parse_response(&payload).unwrap() {
            FarmTcpResponse::Err(msg) => {
                assert!(
                    msg.contains("lease") || msg.contains("granted"),
                    "expected lease ERR, got {msg}"
                );
            }
            other => panic!("raw GET without grant must ERR, got {other:?}"),
        }
        handle.join().ok();
    });
}

#[test]
fn p106_farm_tcp_raw_put_expired_lease_err() {
    with_lease_auth(|| {
        farm_lease_acquire(0).unwrap();
        let (addr, handle) = spawn_farm_tcp_node(HashMap::new(), 2);
        thread::sleep(Duration::from_millis(15));
        let host = addr.to_string();
        for req in [
            farm_tcp_encode_get("tenant/x"),
            farm_tcp_encode_put("tenant/x", b"bytes"),
        ] {
            let payload = farm_tcp_roundtrip(&host, &req).expect("tcp connect");
            match farm_tcp_parse_response(&payload).unwrap() {
                FarmTcpResponse::Err(msg) => {
                    assert!(
                        msg.contains("lease") || msg.contains("expired"),
                        "expected lease ERR, got {msg}"
                    );
                }
                other => panic!("expired lease raw frame must ERR, got {other:?}"),
            }
        }
        handle.join().ok();
    });
}

#[test]
fn p106_farm_tcp_raw_get_hit_when_lease_active() {
    with_lease_auth(|| {
        farm_lease_acquire(120).unwrap();
        let mut store = HashMap::new();
        store.insert("tenant/hit".into(), b"raw-bytes".to_vec());
        let (addr, handle) = spawn_farm_tcp_node(store, 1);
        thread::sleep(Duration::from_millis(15));
        let host = addr.to_string();
        let payload =
            farm_tcp_roundtrip(&host, &farm_tcp_encode_get("tenant/hit")).expect("tcp connect");
        match farm_tcp_parse_response(&payload).unwrap() {
            FarmTcpResponse::Ok(bytes) => assert_eq!(bytes, b"raw-bytes"),
            other => panic!("expected OK hit, got {other:?}"),
        }
        handle.join().ok();
    });
}

#[test]
fn p106_spawn_farm_tcp_node_zero_conns_joins_fast() {
    with_lease_auth(|| {
        let (addr, handle) = spawn_farm_tcp_node(HashMap::new(), 0);
        let _ = addr;
        let started = std::time::Instant::now();
        handle.join().expect("max_conns=0 must exit immediately");
        assert!(
            started.elapsed() < Duration::from_secs(1),
            "max_conns=0 must not block on accept"
        );
    });
}
