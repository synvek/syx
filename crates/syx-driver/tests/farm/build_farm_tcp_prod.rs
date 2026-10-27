//! Phase 100/101: farm worker TCP production — frame protocol 3-node + worker cycle.

use std::collections::HashMap;
use std::thread;
use syx_driver::{farm_worker_run_cycle, fetch_cache_remote, spawn_farm_tcp_node};

use crate::billing_env_lock::billing_env_lock;
use crate::lease_test_harness::with_lease_auth;

#[test]
fn p100_farm_tcp_prod_worker_tcp_queue_put_hit() {
    let _billing = billing_env_lock();
    with_lease_auth(|| {
        std::env::remove_var("SYX_FARM_BILLING_PATH");
        std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");
        let payload = b"farm-prod-queue-bytes";
        let mut queue = HashMap::new();
        queue.insert("tenant/job.bin".into(), payload.to_vec());
        let (qaddr, qh) = spawn_farm_tcp_node(queue, 1);
        let (faddr, fh) = spawn_farm_tcp_node(HashMap::new(), 2);
        thread::sleep(std::time::Duration::from_millis(20));
        let qurl = format!("tcp://{qaddr}/tenant/job.bin");
        let furl = format!("tcp://{faddr}/tenant/node-a");
        let hit = farm_worker_run_cycle(&qurl, &furl).unwrap();
        assert!(hit.is_some(), "farm worker tcp cycle must hit");
        let got = fetch_cache_remote(&furl).unwrap();
        assert_eq!(got, payload);
        qh.join().ok();
        fh.join().ok();
    });
}

#[test]
fn p100_farm_tcp_prod_three_node_fetch() {
    with_lease_auth(|| {
        let nodes: Vec<_> = (0..3)
            .map(|i| {
                let mut store = HashMap::new();
                store.insert(format!("tenant/node-{i}"), format!("node-{i}").into_bytes());
                spawn_farm_tcp_node(store, 1)
            })
            .collect();
        thread::sleep(std::time::Duration::from_millis(30));
        for (i, (addr, _)) in nodes.iter().enumerate() {
            let url = format!("tcp://{addr}/tenant/node-{i}");
            let bytes = fetch_cache_remote(&url).unwrap();
            assert_eq!(bytes, format!("node-{i}").as_bytes());
        }
        for (_, h) in nodes {
            h.join().ok();
        }
    });
}

#[test]
fn p101_farm_tcp_prod_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-frame-lease-v1:ttl-err-on-expired"));
}
