//! Phase 108 Track D: farm lease process-isolated HA (independent AuthorityState + SYNC).

use std::thread;
use std::time::Duration;

use syx_driver::{
    farm_lease_acquire_tenant_token, farm_lease_auth_clear_configured, farm_lease_auth_configure_ha,
    farm_lease_ha_fingerprint, farm_lease_require_active, farm_lease_reset, farm_lease_test_lock,
    spawn_farm_lease_ha, FARM_LEASE_DEFAULT_TOKEN,
};

#[test]
fn p108_farm_lease_ha_v2_fingerprint() {
    assert_eq!(
        farm_lease_ha_fingerprint(),
        "farm-lease-ha-v2:process-isolated"
    );
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(src.contains("farm-lease-ha-v2:process-isolated"));
    assert!(src.contains("SYNC\\0") || src.contains("b\"SYNC\\0\"") || src.contains("SYNC\0"));
}

#[test]
fn p108_farm_ha_shared_mutex_path_not_ga() {
    // Shared Arc<Mutex> clone across primary/secondary must NOT be the HA GA path.
    let src = include_str!("../../src/farm_lease_auth.rs");
    assert!(
        src.contains("independent") || src.contains("Independent"),
        "HA v2 must document independent AuthorityState"
    );
    assert!(
        src.contains("primary_state") && src.contains("secondary_state"),
        "HA v2 must use separate primary_state / secondary_state"
    );
    // Anti-pattern from P107: one `state` Arc cloned to both listeners as GA.
    let spawn_fn = src
        .split("pub fn spawn_farm_lease_ha")
        .nth(1)
        .expect("spawn_farm_lease_ha")
        .split("pub fn farm_lease_acquire")
        .next()
        .expect("spawn body");
    assert!(
        !spawn_fn.contains("state.clone()"),
        "shared-mutex state.clone() path must not pass HA GA"
    );
    assert!(
        spawn_fn.contains("AuthorityState::default()"),
        "each node needs its own AuthorityState::default()"
    );
}

#[test]
fn p108_farm_ha_secondary_validates_after_primary_stop() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_ha(48);
    farm_lease_auth_configure_ha(
        &format!("tcp://{}", cluster.primary_addr()),
        &format!("tcp://{}", cluster.secondary_addr()),
    );
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, FARM_LEASE_DEFAULT_TOKEN, 120).expect("grant via primary");
    farm_lease_require_active().expect("active on primary");

    cluster.stop_primary();
    thread::sleep(Duration::from_millis(20));
    // Secondary has independent state populated via SYNC — not a shared Mutex.
    farm_lease_require_active().expect("secondary must validate active lease after primary stop");

    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p108_farm_ha_no_lease_still_err() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_ha(24);
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
fn p108_farm_ha_grant_on_secondary_after_failover() {
    let _g = farm_lease_test_lock();
    let mut cluster = spawn_farm_lease_ha(24);
    farm_lease_auth_configure_ha(
        &format!("tcp://{}", cluster.primary_addr()),
        &format!("tcp://{}", cluster.secondary_addr()),
    );
    farm_lease_reset();
    cluster.stop_primary();
    thread::sleep(Duration::from_millis(20));
    farm_lease_acquire_tenant_token(0, FARM_LEASE_DEFAULT_TOKEN, 60)
        .expect("GRANT via secondary after primary stop");
    farm_lease_require_active().expect("active lease on secondary");
    farm_lease_reset();
    farm_lease_auth_clear_configured();
    cluster.join();
}

#[test]
fn p108_farm_ha_zero_conns_joins_fast() {
    let _g = farm_lease_test_lock();
    let cluster = spawn_farm_lease_ha(0);
    let started = std::time::Instant::now();
    cluster.join();
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "max_conns=0 must not hang on accept"
    );
}
