//! Phase 101: farm worker pool + tenant-scoped cache (TCP GA; file:// archived).

use syx_pkg::SyxLockfile;

#[test]
fn build_farm_multitenant_phase98_worker_pool_field() {
    let lock = SyxLockfile {
        farm_worker_pool: vec!["id-a".into(), "id-b".into()],
        tenant_id: Some("acme".into()),
        ..Default::default()
    };
    assert_eq!(lock.farm_worker_pool.len(), 2);
    assert_eq!(lock.tenant_id.as_deref(), Some("acme"));
}

#[test]
fn build_farm_multitenant_phase98_lease_ttl_default() {
    assert_eq!(SyxLockfile::default().farm_lease_ttl_secs, 300);
}

#[test]
fn p98_build_farm_multitenant_lease_ttl_enforce() {
    let lock = syx_pkg::SyxLockfile::default();
    assert!(lock.farm_lease_ttl_secs > 0);
}
