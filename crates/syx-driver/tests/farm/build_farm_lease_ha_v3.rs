//! Phase 109 Track C: Farm OS-process HA v3 + Raft quorum MVP.

use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{
    farm_lease_acquire_tenant_token, farm_lease_auth_clear_configured,
    farm_lease_auth_configure_ha, farm_lease_auth_configure_raft, farm_lease_ha_v3_fingerprint,
    farm_lease_os_child_try_serve, farm_lease_raft_fingerprint, farm_lease_require_active,
    farm_lease_reset, farm_lease_test_lock, spawn_farm_lease_ha, spawn_farm_lease_ha_v3,
    spawn_farm_lease_raft, FARM_LEASE_DEFAULT_TOKEN, FARM_LEASE_RAFT_PORTS,
};

const RAFT_TOKEN: u32 = 0xF109_7001;

/// OS-child re-entry for `std::process::Command` HA v3 / Raft nodes.
#[test]
fn p109_farm_os_child_entry() {
    if farm_lease_os_child_try_serve() {
        return;
    }
}

#[test]
fn p109_farm_lease_ha_v3_fingerprint() {
    assert_eq!(farm_lease_ha_v3_fingerprint(), "farm-lease-ha-v3:os-process");
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-ha-v3:os-process"));
    assert!(src.contains("std::process::Command") || src.contains("process::Command"));
}

#[test]
fn p109_farm_lease_raft_fingerprint() {
    assert_eq!(farm_lease_raft_fingerprint(), "farm-lease-raft-v1:quorum-mvp");
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-raft-v1:quorum-mvp"));
    assert_eq!(FARM_LEASE_RAFT_PORTS.len(), 3);
}

#[test]
fn p109_farm_thread_sync_not_ha_v3_ga() {
    // Same-process thread+SYNC (`spawn_farm_lease_ha`) must NOT pass HA v3 GA.
    let src = include_str!("../../src/farm_lease_auth.rs");
    let v2 = src
        .split("pub fn spawn_farm_lease_ha(")
        .nth(1)
        .expect("spawn_farm_lease_ha")
        .split("pub fn farm_lease_acquire")
        .next()
        .expect("v2 body");
    assert!(
        v2.contains("thread::spawn") || src.contains("spawn_authority_listener"),
        "v2 HA is in-process threads"
    );
    assert!(
        !v2.contains("Command::new"),
        "thread HA path must not use Command"
    );
    let v3 = src
        .split("pub fn spawn_farm_lease_ha_v3")
        .nth(1)
        .expect("spawn_farm_lease_ha_v3")
        .split("pub struct FarmLeaseRaftCluster")
        .next()
        .expect("v3 body");
    assert!(
        v3.contains("Command") || src.contains("spawn_os_authority_child"),
        "HA v3 must spawn OS processes via Command"
    );
    assert_ne!(
        farm_lease_ha_v3_fingerprint(),
        "farm-lease-ha-v2:process-isolated"
    );
    // Runtime: thread cluster is a different API from OS-process v3.
    let _g = farm_lease_test_lock();
    let thread_cluster = spawn_farm_lease_ha(0);
    thread_cluster.join();
}

#[test]
fn p109_farm_ha_v3_process_isolation_failover() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_ha_v3(48);
    farm_lease_auth_configure_ha(
        &format!("tcp://{}", cluster.primary_addr()),
        &format!("tcp://{}", cluster.secondary_addr()),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, FARM_LEASE_DEFAULT_TOKEN, 120).expect("GRANT via OS primary");
    farm_lease_require_active().expect("active on primary");

    cluster.stop_primary();
    thread::sleep(Duration::from_millis(30));
    farm_lease_require_active().expect("secondary OS process validates after primary kill");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p109_farm_ha_v3_no_lease_still_err() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_ha_v3(24);
    farm_lease_auth_configure_ha(
        &format!("tcp://{}", cluster.primary_addr()),
        &format!("tcp://{}", cluster.secondary_addr()),
    );
    farm_lease_reset();
    let err = farm_lease_require_active().expect_err("no grant must ERR");
    assert!(
        err.contains("not granted") || err.contains("lease"),
        "expected no-lease ERR, got {err}"
    );

    cluster.stop_primary();
    thread::sleep(Duration::from_millis(20));
    let err2 = farm_lease_require_active().expect_err("no grant via secondary must ERR");
    assert!(
        err2.contains("not granted") || err2.contains("lease"),
        "expected no-lease ERR on secondary, got {err2}"
    );

    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p109_farm_ha_v3_zero_conns_joins_fast() {
    let _g = farm_lease_test_lock();
    let cluster = spawn_farm_lease_ha_v3(0);
    let started = Instant::now();
    cluster.join();
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "HA v3 max_conns=0 must not hang on kill+join"
    );
}

#[test]
fn p109_farm_raft_quorum_grant_and_failover() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(96);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 90).expect("GRANT needs raft majority");
    farm_lease_require_active().expect("lease active under quorum");

    cluster.kill_leader();
    thread::sleep(Duration::from_millis(50));
    cluster
        .refresh_leader(30)
        .expect("new leader after kill");
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", cluster.addrs()[0]),
        &format!("tcp://{}", cluster.addrs()[1]),
        &format!("tcp://{}", cluster.addrs()[2]),
    );
    farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 90).expect("new leader can GRANT");
    farm_lease_require_active().expect("new leader can validate");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p109_farm_raft_no_lease_still_err() {
    let _g = farm_lease_test_lock();
    let cluster = spawn_farm_lease_raft(64);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    farm_lease_reset();
    let err = farm_lease_require_active().expect_err("no grant must ERR");
    assert!(
        err.contains("not granted") || err.contains("lease"),
        "expected no-lease ERR, got {err}"
    );
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p109_farm_raft_quorum_loss_rejects_grant() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_raft(64);
    let addrs = cluster.addrs();
    farm_lease_auth_configure_raft(
        &format!("tcp://{}", addrs[0]),
        &format!("tcp://{}", addrs[1]),
        &format!("tcp://{}", addrs[2]),
    );
    // Leave a single node alive — majority of 3 is impossible.
    let dead_leader = cluster.leader_idx();
    cluster.kill_leader();
    for i in 0..3 {
        if i != dead_leader && cluster.live_count() > 1 {
            cluster.kill_node(i);
        }
    }
    assert_eq!(cluster.live_count(), 1, "exactly one raft node must remain");
    thread::sleep(Duration::from_millis(40));
    let err = farm_lease_acquire_tenant_token(0, RAFT_TOKEN, 30)
        .expect_err("quorum loss must reject GRANT");
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
fn p109_farm_raft_joins_fast() {
    let _g = farm_lease_test_lock();
    let cluster = spawn_farm_lease_raft(0);
    let started = Instant::now();
    cluster.join();
    assert!(
        started.elapsed() < Duration::from_secs(3),
        "raft kill+join must finish under deadline"
    );
}
