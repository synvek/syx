//! Dynamic Host plugin loader — ADR-071 P0+P1.

mod abi;
mod codegen;
mod handles;
mod loader;
mod macro_plugin;
mod manifest;
mod registry;
mod v2;

pub use abi::{
    SYX_PLUGIN_API_VERSION, SYX_PLUGIN_API_VERSION_V2, SYX_PLUGIN_API_VERSION_V2M,
    SYX_PLUGIN_BUF_F64LE, SYX_PLUGIN_BUF_HANDLE, SYX_PLUGIN_BUF_UTF8,
    SYX_PLUGIN_V2M_GROW_INITIAL_CAP, SYX_PLUGIN_V2_OUT_CAP,
};
pub use codegen::{generate_syx_module, write_codegen_module};
pub use handles::{
    add_handles, buf_to_values, drop_handle, hadamard_handles, mul_handles, pack_handle,
    pin_f64_le_bytes, pin_f64_values, pin_shaped, pin_zeros, solve_handles, sub_handles, sum_handle,
    unpack_handle, value_as_dim, value_as_handle,
};
pub use loader::{
    install_plugin, load_from_env, load_from_exe_plugin_dirs, load_library, load_path,
    plugin_install_roots, plugin_paths_from_env, plugins_enabled, register_echo_plugin_in_process,
    register_faer_add_in_process, register_faer_from_in_process, register_faer_hadamard_in_process,
    register_faer_mul_in_process, register_faer_solve_in_process, register_faer_sub_in_process,
    register_faer_to_in_process, register_faer_zeros_in_process, register_linalg_system_hosts,
    register_mycrypto_plugin_in_process, register_v2probe_dot_in_process,
    register_v2probe_drop_in_process, register_v2probe_grow_in_process,
    register_v2probe_pin_in_process, register_v2probe_sum_in_process,
    register_v2probe_sumh_in_process, system_faerlin_present, unload_plugin,
};
pub use macro_plugin::{expand_macro_json, macro_plugin_path_from_env};
pub use manifest::{
    verify_ed25519_signature, verify_signature, verify_syxplugin_bundle, PluginHostManifest,
    PluginManifest, SyxPluginBundle,
};
pub use registry::{
    dispatch_plugin_host, loaded_plugin_ids, plugin_host_base_index, plugin_host_index,
    plugin_lock, plugin_slot_poisoned, plugin_unload_epoch, poison_plugin_slot,
    register_compile_hosts_from_manifest, register_compile_plugin_manifests, register_runtime_host,
    runtime_hosts, runtime_plugin_loaded, set_plugin_lock, validate_compile_plugin_lock,
    validate_plugin_lock, LoadedPluginHost, PluginLock, PluginLockEntry,
};
pub use v2::{dot_f64_arrays, grow_copy_f64, pack_f64_le, sum_f64_array, unpack_f64_le};

pub const PLUGIN_HEADER: &str = include_str!("../include/syx_plugin.h");
