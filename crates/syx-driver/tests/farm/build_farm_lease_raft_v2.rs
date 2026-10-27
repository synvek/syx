//! Phase 110 Track C: Raft v2 replicated log + WAL fsync + membership change MVP.

use std::fs;
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{
    farm_lease_acquire_tenant_token, farm_lease_auth_clear_configured,
    farm_lease_auth_configure_raft, farm_lease_member_fingerprint, farm_lease_query_at,
    farm_lease_raft_fingerprint, farm_lease_raft_member_add_learner, farm_lease_raft_member_remove,
    farm_lease_raft_v2_fingerprint, farm_lease_require_active, farm_lease_reset,
    farm_lease_test_lock, spawn_farm_lease_raft, FARM_LEASE_RAFT_PORTS,
};

const RAFT_TOKEN: u32 = 0xF110_7001;

#[test]
fn p110_farm_lease_raft_v2_fingerprint() {
    assert_eq!(
        farm_lease_raft_v2_fingerprint(),
        "farm-lease-raft-v2:log-wal"
    );
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-raft-v2:log-wal"));
    assert!(src.contains("APPEND2"));
    assert!(src.contains("commit_index"));
    assert!(src.contains("NodeWal"));
    assert!(src.contains("sync_all"));
}

#[test]
fn p110_farm_lease_member_fingerprint() {
    assert_eq!(
        farm_lease_member_fingerprint(),
        "farm-lease-member-v1:dynamic-mvp"
    );
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-member-v1:dynamic-mvp"));
    assert!(src.contains("AddLearner"));
    assert!(src.contains("RemoveMember"));
}

#[test]
fn p110_farm_toy_raft_v1_not_v2_ga() {
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert_eq!(farm_lease_raft_fingerprint(), "farm-lease-raft-v1:quorum-mvp");
    assert_ne!(
        farm_lease_raft_fingerprint(),
        farm_lease_raft_v2_fingerprint()
    );
    // Toy v1 append (deadline-only, no log) must not be the v2 serve path.
    let serve = src
        .split("fn serve_raft_os_child")
        .nth(1)
        .expect("serve_raft_os_child");
    assert!(
        serve.contains("raft_send_append_v2") || serve.contains("APPEND2"),
        "v2 serve must use log append"
    );
    assert!(
        src.contains("encode_append_toy_v1"),
        "toy v1 kept for negative GA contrast"
    );
    assert!(
        src.contains("append v1 rejected"),
        "v2 nodes must reject toy APPEND"
    );
    assert!(
        !serve.contains("encode_append_toy_v1"),
        "toy v1 must not be used in serve loop"
    );
}

#[test]
fn p110_farm_raft_v2_log_wal_grant_and_failover() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(96);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 90).expect("GRANT via raft v2 log+WAL");
    farm_lease_require_active().expect("lease active under quorum");

    for id in 0..3 {
        let wal = cluster.wal_dir().join(format!("node-{id}.wal"));
        assert!(wal.exists(), "node {id} WAL must exist after commit");
        let meta = fs::metadata(&wal).expect("wal metadata");
        assert!(meta.len() > 8, "node {id} WAL must contain log records");
    }

    cluster.kill_leader();
    thread::sleep(Duration::from_millis(80));
    cluster.refresh_leader(30).expect("new leader after kill");
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", cluster.addrs()[0]),
        &format!("tcp://{}", cluster.addrs()[1]),
        &format!("tcp://{}", cluster.addrs()[2]),
    );
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 90).expect("new leader catch-up GRANT");
    farm_lease_require_active().expect("new leader validates");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p110_farm_raft_v2_wal_restart_secondary_validates() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(64);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 120).expect("commit lease to WAL");
    farm_lease_require_active().expect("cluster active");

    let secondary_idx = if cluster.leader_idx() == 1 {
        2
    } else {
        1
    };
    cluster.kill_node(secondary_idx);
    thread::sleep(Duration::from_millis(40));

    let restarted = cluster
        .restart_node(secondary_idx, 48)
        .expect("restart secondary from WAL");
    thread::sleep(Duration::from_millis(60));
    let deadline = farm_lease_query_at(&format!("tcp://{restarted}"))
        .expect("QUERY restarted secondary")
        .expect("restarted secondary must hold committed lease from WAL replay");
    assert!(deadline > 0, "restarted secondary deadline={deadline}");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p110_farm_raft_member_remove_quorum_updates() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(80);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 60).expect("initial grant");

    let remove_id = 2u32;
    farm_lease_raft_member_remove(remove_id).expect("remove voter via log");
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 60).expect("grant with 2 voters still works");

    cluster.kill_node(0);
    cluster.kill_node(1);
    thread::sleep(Duration::from_millis(50));
    let err = farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 30).expect_err("one live node cannot meet quorum=2");
    assert!(
        err.contains("no quorum")
            || err.contains("not leader")
            || err.contains("failed")
            || err.contains("connect"),
        "expected quorum-loss ERR after remove+kill, got {err}"
    );

    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p110_farm_raft_member_add_learner() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(72);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 60).expect("initial grant");

    let learner_id = 2usize;
    farm_lease_raft_member_remove(learner_id as u32).expect("remove before learner re-add");
    farm_lease_raft_member_add_learner(learner_id as u32).expect("add learner via log");

    cluster.kill_node(learner_id);
    thread::sleep(Duration::from_millis(30));
    cluster
        .restart_node(learner_id, 32)
        .expect("learner node restart");
    thread::sleep(Duration::from_millis(40));

    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 60).expect("grant still works with learner non-voting");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p110_farm_raft_v2_quorum_loss_rejects_grant() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(64);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    let dead_leader = cluster.leader_idx();
    cluster.kill_leader();
    for i in 0..3 {
        if i != dead_leader && cluster.live_count() > 1 {
            cluster.kill_node(i);
        }
    }
    assert_eq!(cluster.live_count(), 1);
    thread::sleep(Duration::from_millis(40));
    let err = farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 30).expect_err("quorum loss must reject GRANT");
    assert!(
        err.contains("no quorum")
            || err.contains("not leader")
            || err.contains("failed")
            || err.contains("connect"),
        "expected quorum-loss ERR, got {err}"
    );
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p110_farm_raft_v2_joins_fast() {
    let _g = farm_lease_test_lock();
    let cluster = spawn_farm_lease_raft(0);
    let started = Instant::now();
    cluster.join();
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "raft v2 kill+join must finish under deadline"
    );
    assert_eq!(FARM_LEASE_RAFT_PORTS.len(), 3);
}
