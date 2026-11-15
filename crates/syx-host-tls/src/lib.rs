//! Native TLS host domain. `syx_host_tls_install` fills the shakeable host slot.

use syx_runtime_core::register_shakeable_host_id;
use syx_stdlib::HostFnId;

/// Register every TLS-domain `HostFnId` into the shakeable host table.
pub fn install() {
    register_shakeable_host_id(HostFnId::TlsServe, syx_tls_impl::host_tls_serve);
}

/// C ABI entry the native init calls when the TLS domain is linked.
#[no_mangle]
pub extern "C" fn syx_host_tls_install() {
    install();
}
