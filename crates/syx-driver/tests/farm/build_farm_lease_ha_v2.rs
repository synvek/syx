//! Phase 108 Track D: farm lease process-isolated HA (independent AuthorityState + SYNC).

use std::thread;
use std::time::Duration;

use syx_driver::{
    farm_lease_acquire_tenant_token, farm_lease_auth_clear_configured, farm_lease_auth_configure_ha,
    farm_lease_require_active, farm_lease_reset, farm_lease_test_lock,
    spawn_farm_lease_ha, FARM_LEASE_DEFAULT_TOKEN,
};

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
