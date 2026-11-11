//! C ABI runtime for Cranelift-generated SYX executables.

#![allow(dead_code)]
#![allow(unused_imports)]
mod abi;
mod bridge;
mod link;
mod llvm_stub_audit;
mod llvm_stubs;
mod rt_diag;

pub use abi::{
    aot_execution_segment_omitted, install_metadata_sidecar_view,
    install_runtime_from_sidecar_only, native_metadata_alternate_entry,
    reset_aot_execution_segment_omitted, reset_native_metadata_alternate_entry,
    reset_rt_internal_exec_count, reset_sidecar_install_literal_count,
    reset_sidecar_string_pool_primary, rt_await_poll_native, rt_await_suspend_native,
    rt_call_host_native, rt_direct_exec, rt_internal_exec_count, rt_print_native,
    sidecar_install_literal_count, sidecar_string_pool_primary, syx_rt_begin_function,
    syx_rt_call_fn, syx_rt_call_host, syx_rt_end_function, syx_rt_exec_op, syx_rt_frame_local_bits,
    syx_rt_frame_local_ptr, syx_rt_get_return, syx_rt_init_module, syx_rt_install_metadata_sidecar,
    syx_rt_load_mir_blob, syx_rt_method_call, syx_rt_peek_return, syx_rt_pop_bool,
    syx_rt_pop_into_local_ptr, syx_rt_pop_to_local_bits, syx_rt_push_from_local_bits,
    syx_rt_push_from_local_ptr, syx_rt_push_i32, syx_rt_push_nil, syx_rt_push_numeric,
    syx_rt_push_numeric64, syx_rt_push_str, syx_rt_run_main_fn, syx_rt_start,
    syx_rt_string_pool_base, syx_rt_take_catch, syx_rt_throw,
};
pub use bridge::{
    default_vm_runtime, install_native_module, install_native_task_hosts,
    install_native_unified_gc_bridge, native_vm_runtime_for, store_module,
    syx_rt_jit_safepoint_enabled, syx_rt_unified_gc_bridge_enabled, NativeRunContext,
};
pub use link::{
    audit_user_object_undefs, coff_undefined_externals, host_link_triple, link_executable,
    link_executable_with, link_object_bytes, native_lld_argv, native_sdk_layout_report,
    resolve_sdk_sysroot, rt_archive_path, rt_archive_path_for_profile, sysroot_looks_usable,
    user_object_undef_allowed, LinkOptions, NativeSdkLayoutReport,
};
pub use llvm_stub_audit::{
    classify_llvm_stub_block, llvm_stub_audit_report, LlvmStubAuditReport, LlvmStubKind,
};
