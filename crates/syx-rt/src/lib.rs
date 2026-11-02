//! C ABI runtime for Cranelift-generated SYX executables.

#![allow(dead_code)]
#![allow(unused_imports)]
mod abi;
mod gate_b09_gc_narrative;
mod bridge;
mod link;
mod llvm_stub_audit;
mod llvm_stubs;

pub use abi::{
    aot_execution_segment_omitted, install_metadata_sidecar_view,
    install_runtime_from_sidecar_only, native_metadata_alternate_entry,
    reset_aot_execution_segment_omitted, reset_native_metadata_alternate_entry,
    reset_rt_internal_exec_count, reset_sidecar_install_literal_count,
    reset_sidecar_string_pool_primary, rt_await_poll_native, rt_await_suspend_native,
    rt_call_host_native, rt_direct_exec, rt_internal_exec_count, rt_print_native,
    sidecar_install_literal_count, sidecar_string_pool_primary,
    syx_rt_begin_function, syx_rt_call_fn, syx_rt_call_host, syx_rt_end_function, syx_rt_exec_op,
    syx_rt_frame_local_bits, syx_rt_frame_local_ptr, syx_rt_get_return, syx_rt_init_module,
    syx_rt_install_metadata_sidecar, syx_rt_load_mir_blob, syx_rt_method_call,
    syx_rt_peek_return, syx_rt_pop_bool,
    syx_rt_pop_into_local_ptr, syx_rt_pop_to_local_bits, syx_rt_push_from_local_bits,
    syx_rt_push_from_local_ptr, syx_rt_push_i32, syx_rt_push_nil, syx_rt_push_numeric,
    syx_rt_push_numeric64, syx_rt_push_str,
    syx_rt_run_main_fn, syx_rt_start, syx_rt_string_pool_base, syx_rt_take_catch, syx_rt_throw,
};
pub use gate_b09_gc_narrative::{
    gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory,
    gate_b09_dual_heap_converge_first_slice_inventory, gate_b09_enable_readiness_thin_inventory,
    gate_b09_gc_narrative_deepen_inventory, gate_b09_gc_narrative_thin_inventory,
    gate_b09_memory_pool_dead_shell_first_slice_inventory,
    gate_b09_mir_runtime_root_set_first_slice_inventory,
    gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory,
    gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory,
    gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory,
    gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory,
    gate_b09_objectref_memory_pool_storage_honesty_thin_inventory,
    gate_b09_objectref_actor_call_heap_slots_first_slice_inventory,
    gate_b09_objectref_method_call_heap_slots_first_slice_inventory,
    gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory,
    objectref_rc_migrate_enabled,
    storage_cleanup_enabled,
    unified_mir_runtime_heap_enabled,
    b09_axis_exit_enabled,
    gc_ga_claimed,
    gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory,
    gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory,
    gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory,
    gate_b09_objectref_rc_migrate_first_slice_inventory,
    gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory,
    gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory,
    gate_b09_objectref_vm_heap_slots_first_slice_inventory,
    gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory,
    gate_b09_storage_cleanup_first_slice_inventory, gate_b09_vm_gccollect_root_set_first_slice_inventory,
    gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory,
    GateB09ActorMailboxValueQueueRootSetFirstSliceInventory, GateB09DualHeapConvergeFirstSliceInventory,
    GateB09EnableReadinessThinInventory, GateB09GcNarrativeDeepenInventory, GateB09GcNarrativeThinInventory,
    GateB09MemoryPoolDeadShellFirstSliceInventory, GateB09MirRuntimeRootSetFirstSliceInventory,
    GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory,
    GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory,
    GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory,
    GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory,
    GateB09ObjectRefMemoryPoolStorageHonestyThinInventory,
    GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory,
    GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory,
    GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory,
    GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory,
    GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory,
    GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory, GateB09ObjectRefRcMigrateFirstSliceInventory,
    GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory,
    GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory, GateB09ObjectRefVmHeapSlotsFirstSliceInventory,
    GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory, GateB09StorageCleanupFirstSliceInventory,
    GateB09VmGcCollectRootSetFirstSliceInventory, GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory,
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
    classify_llvm_stub_block, gate_hostfn_real_lowering_horizon_deepen_thin_inventory,
    gate_hostfn_real_lowering_horizon_exit_inventory,
    gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory,
    gate_hostfn_real_lowering_horizon_first_slice_wire_inventory,
    gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory,
    gate_hostfn_real_lowering_horizon_second_slice_wire_inventory,
    gate_hostfn_real_lowering_horizon_thin_inventory,
    gate_hostfn_real_lowering_horizon_third_slice_wire_inventory, llvm_stub_audit_report,
    GateHostFnRealLoweringHorizonDeepenThinInventory, GateHostFnRealLoweringHorizonExitInventory,
    GateHostFnRealLoweringHorizonFifthSliceWireInventory,
    GateHostFnRealLoweringHorizonFirstSliceWireInventory,
    GateHostFnRealLoweringHorizonFourthSliceWireInventory,
    GateHostFnRealLoweringHorizonSecondSliceWireInventory, GateHostFnRealLoweringHorizonThinInventory,
    GateHostFnRealLoweringHorizonThirdSliceWireInventory, LlvmStubAuditReport, LlvmStubKind,
};
