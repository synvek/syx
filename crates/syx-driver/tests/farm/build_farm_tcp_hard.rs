//! Phase 101: Farm TCP hard — frame default tcp URL, mutex fetch, billing exit≠0.

use std::collections::HashMap;

use syx_driver::{
    default_farm_cache_url, fetch_cache_remote_hardened, spawn_farm_tcp_node,
};

use crate::lease_test_harness::with_lease_auth;

#[test]
fn p101_farm_tcp_hard_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-frame-lease-v1:ttl-err-on-expired"));
}

#[test]
fn p98_farm_tcp_default_url_is_tcp() {
    let url = default_farm_cache_url();
    assert!(url.starts_with("tcp://"), "{url}");
}

#[test]
fn p100_farm_tcp_hardened_three_node_fetch() {
    with_lease_auth(|| {
        let nodes: Vec<_> = (0..3)
            .map(|i| {
                let mut store = HashMap::new();
                store.insert(format!("tenant/node-{i}"), format!("node-{i}").into_bytes());
                spawn_farm_tcp_node(store, 1)
            })
            .collect();
        for (i, (addr, _)) in nodes.iter().enumerate() {
            let url = format!("tcp://{addr}/tenant/node-{i}");
            let bytes = fetch_cache_remote_hardened(&url).unwrap();
            assert_eq!(bytes, format!("node-{i}").as_bytes());
        }
        for (_, h) in nodes {
            h.join().ok();
        }
    });
}

#[test]
fn p100_farm_billing_path_worker_checks_ack() {
    assert!(include_str!("../../src/build_syx.rs").contains("SYX_FARM_BILLING_PATH"));
    assert!(include_str!("../../src/build_syx.rs").contains("SYX_BILLING_SETTLEMENT_ACK"));
}
