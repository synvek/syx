//! Shared lease authority setup for TCP farm integration tests.

use syx_driver::{
    farm_lease_acquire_tenant_token, farm_lease_auth_clear_configured, farm_lease_auth_configure,
    farm_lease_reset, farm_lease_test_lock, spawn_farm_lease_authority, FARM_LEASE_DEFAULT_TOKEN,
};

pub fn with_lease_auth<F: FnOnce()>(f: F) {
    let _g = farm_lease_test_lock();
    let (auth_addr, _auth_handle) = spawn_farm_lease_authority(64);
    farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    farm_lease_reset();
    farm_lease_acquire_tenant_token(0, FARM_LEASE_DEFAULT_TOKEN, 300)
        .expect("lease grant for tcp farm test");
    f();
    farm_lease_reset();
    farm_lease_auth_clear_configured();
}

pub fn without_lease_auth<F: FnOnce()>(f: F) {
    let _g = farm_lease_test_lock();
    std::env::remove_var("SYX_FARM_LEASE_AUTH");
    farm_lease_auth_clear_configured();
    f();
}

/// Phase 116 cross-host runtime fixtures (≥2 distinct non-loopback host keys).
pub const CROSS_HOST_RUNTIME_MEMBERS_3_JSON: &str = r#"[
    {"bind":"0.0.0.0:28770","host_key":"192.0.2.10:28770","region":"dc-west"},
    {"bind":"0.0.0.0:28771","host_key":"192.0.2.11:28771","region":"dc-east"},
    {"bind":"0.0.0.0:28772","host_key":"192.0.2.12:28772","region":"dc-west"}
]"#;

pub const CROSS_HOST_RUNTIME_MEMBERS_4_JSON: &str = r#"[
    {"bind":"0.0.0.0:28780","host_key":"192.0.2.20:28780","region":"dc-a"},
    {"bind":"0.0.0.0:28781","host_key":"192.0.2.21:28781","region":"dc-b"},
    {"bind":"0.0.0.0:28782","host_key":"192.0.2.22:28782","region":"dc-c"},
    {"bind":"0.0.0.0:28783","host_key":"192.0.2.23:28783","region":"dc-d"}
]"#;
