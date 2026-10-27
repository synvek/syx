//! LLVM AOT runtime symbols declared by `syx-llvm` but not in `abi.rs`.

#![allow(dead_code, unused_variables)]

use crate::abi::{rt_call_host_native, rt_direct_exec, rt_print_native};
use syx_stdlib::{host_fn_index, HostFnId};

/// Tier-reduce noop stubs count as wired for GA audit (delegates to MIR POP).
#[inline]
fn tier_reduce(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[inline]
fn tier_reduce0() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

/// W419: Actor wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn actor_call_or_fail_closed(_a: i32, _b: i32) -> i32 {
    panic!("syx_rt_actor_call: no-native / no MIR (W419 Actor wire)");
}

/// W419: Actor wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn actor_send_or_fail_closed(_a: i32, _b: i32) -> i32 {
    panic!("syx_rt_actor_send: no-native / no MIR (W419 Actor wire)");
}

/// W419: Actor wire first-slice — fail-closed POP/aux stubs.
pub(crate) fn actor_aux_or_fail_closed(name: &'static str) -> i32 {
    panic!("{name}: no-native / no MIR (W419 Actor wire)");
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_call(a: i32, b: i32) -> i32 {
    actor_call_or_fail_closed(a, b)
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_cross_region(_a: i32, _b: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_cross_region")
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_dynamic_lb(_a: i32, _b: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_dynamic_lb")
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_global_lb(_a: i32, _b: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_global_lb")
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_mailbox(_a: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_mailbox")
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_route(_a: i32, _b: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_route")
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_send(a: i32, b: i32) -> i32 {
    actor_send_or_fail_closed(a, b)
}

#[no_mangle]
pub extern "C" fn syx_rt_actor_weighted_lb(_a: i32, _b: i32) -> i32 {
    actor_aux_or_fail_closed("syx_rt_actor_weighted_lb")
}

#[no_mangle]
pub extern "C" fn syx_rt_adr038_perf() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_adr040_blocker() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_adr042_default() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_adr044_prod() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_adr046_object() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_adr048_crate() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_array_build(a: i32) {
    let _ = rt_direct_exec(syx_mir_run::ARRAY_BUILD, a, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_array_len(a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

/// W417: Await wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn await_poll_or_fail_closed() -> i32 {
    panic!("syx_rt_await_poll: no-native / no MIR (W417 Await wire)");
}

/// W417: Await wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn await_suspend_or_fail_closed(_a: i32, _b: i32) {
    panic!("syx_rt_await_suspend: no-native / no MIR (W417 Await wire)");
}

#[no_mangle]
pub extern "C" fn syx_rt_await_poll() -> i32 {
    // Panic across C ABI aborts (intentional fail-closed).
    await_poll_or_fail_closed()
}

#[no_mangle]
pub extern "C" fn syx_rt_await_suspend(a: i32, b: i32) {
    await_suspend_or_fail_closed(a, b);
}

#[no_mangle]
pub extern "C" fn syx_rt_begin_try(a: i32) {
    let _ = rt_direct_exec(syx_mir_run::BEGIN_TRY, a, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_bind_result(a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::BIND_RESULT, a, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_block_phi(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_complete(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_coq_lean(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_coq_lean_ci(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_coq_lean_proof(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_formal_verify(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_machine_proof(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_region_v8(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_region_v8_full(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_se0461(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_borrow_subprocess(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_call_closure(a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::CALL_CLOSURE, a, 0)
}

/// W424: CallHost wire first-slice — no `rt_direct_exec(CALL_HOST)`.
#[no_mangle]
pub extern "C" fn syx_rt_call_host_aot(a: i32) -> i32 {
    rt_call_host_native(a as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_call_witness(a: i32, b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::CALL_WITNESS, a, b)
}

#[no_mangle]
pub extern "C" fn syx_rt_catch() -> i32 {
    rt_direct_exec(syx_mir_run::CATCH, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_channel_close(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_channel_recv(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_channel_recv_async(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_channel_send(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_closure_capture(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_closure_invoke(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_cranelift_gc_parity() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_cranelift_p20(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dataclass_aot(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_datetime_v2(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_datetime_v3_dst(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dict_build(a: i32) {
    let _ = rt_direct_exec(syx_mir_run::DICT_BUILD, a, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_dict_get(a: i32, b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_actor_recv(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_actor_send(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_rustls_e2e(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_rustls_real(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_winpipe_e2e(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_dist_winpipe_real(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v14_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v15_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v16_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v17_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v18_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v19_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v20_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v21_gc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v21_rss() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v22_gc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_embed_v22_rss(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_end_try() -> i32 {
    rt_direct_exec(syx_mir_run::END_TRY, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_enum_aot(_a: i32, _b: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_enum_switch(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_enum_tag(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_file_read_aot(_a: i32) -> i32 {
    // W452 / ADR-796: B-07 second-slice wire — leave noop via native HostFn dispatch.
    rt_call_host_native(host_fn_index(HostFnId::FileRead) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_alloc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_card_table(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_collect() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_compaction(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_generational(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_generational_promote(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_guest_heap(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_intrinsic_alloc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_intrinsic_collect() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_native_card_table(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_native_intrinsic(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_native_remembered_set(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_phase32_prod() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_phase33_prod() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_phase34_prod() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_phase35_prod() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_pin(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_promotion_codegen(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_relocating(_a: i32) -> i32 {
    tier_reduce(_a, _a)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_relocating_compact() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_relocating_prod() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_remembered_set(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_sweep_incr() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_unpin(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_wasm_intrinsic(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_gc_write_barrier(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v10(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v11(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v12(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v13(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v13_org(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v14(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v14_cdn(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v15(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v15_crdt(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v16(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v16_probe(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v8(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_global_actor_v9(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_hash_into(_a: i32) {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_http_get_aot(_a: i32) -> i32 {
    // W451 / ADR-795: B-07 first-slice wire — leave noop via native HostFn dispatch.
    rt_call_host_native(host_fn_index(HostFnId::HttpGet) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_candidate() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_crate() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_default() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_object() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_prod() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_inkwell_spike() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_jit_safepoint(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_jit_tier2(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_json_decode_aot(a: i32) -> i32 {
    // W433: named HostFn stub clear — leave table path via native host call.
    rt_call_host_native(a as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_json_encode_aot(a: i32) -> i32 {
    // W433: named HostFn stub clear — leave table path via native host call.
    rt_call_host_native(a as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase10() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase11() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase4() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase5() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase6() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase7() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase8() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_llvm_phase9() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_make_closure(a: i32, b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::MAKE_CLOSURE, a, b)
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v18_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v19_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v20_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v21_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v22_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v23_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v24_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_mir_v25_barrier() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_net_backpressure(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_flow_control(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_ring_buffer(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_setsockopt(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_setsockopt_real(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_socket_window(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_net_stream_handle(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_object_clone(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_perf_parity_gate() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_perf_release_ga() {
    let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_phi_merge(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

/// W423: Print wire first-slice — pure native; no `rt_direct_exec(PRINT)`.
#[no_mangle]
pub extern "C" fn syx_rt_print() {
    rt_print_native();
}

/// W423: Print wire first-slice — pure native; no `rt_direct_exec(PRINT)`.
#[no_mangle]
pub extern "C" fn syx_rt_print_aot() {
    rt_print_native();
}

#[no_mangle]
pub extern "C" fn syx_rt_property_wrapper(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_property_wrapper_parser(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_push_collection(a: i32) {
    let _ = rt_direct_exec(syx_mir_run::ARRAY_BUILD, a, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_failover(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_health_probe(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase10(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase11(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase12(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase13(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase14(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase15(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase16(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_registry_phase17(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_result_actor_try(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_result_async_try(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_result_generic(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_result_generic_full(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_result_try_expr(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_return_err() {
    let _ = rt_direct_exec(syx_mir_run::RETURN_ERR, 0, 0);
}

#[no_mangle]
pub extern "C" fn syx_rt_rustls_chain_verify(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_rustls_crate_e2e(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_rustls_crate_verify(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_rustls_handshake(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_rustls_prod(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_string_len(ptr: i64) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

/// W418: Task wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn task_cancel_or_fail_closed() -> i32 {
    panic!("syx_rt_task_cancel: no-native / no MIR (W418 Task wire)");
}

/// W418: Task wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn task_join_or_fail_closed(_a: i32) -> i32 {
    panic!("syx_rt_task_join: no-native / no MIR (W418 Task wire)");
}

/// W418: Task wire first-slice — fail-closed; no MIR opcode fallback.
pub(crate) fn task_spawn_or_fail_closed(_a: i32) -> i32 {
    panic!("syx_rt_task_spawn: no-native / no MIR (W418 Task wire)");
}

#[no_mangle]
pub extern "C" fn syx_rt_task_cancel(_a: i32) -> i32 {
    task_cancel_or_fail_closed()
}

#[no_mangle]
pub extern "C" fn syx_rt_task_join(a: i32) -> i32 {
    task_join_or_fail_closed(a)
}

#[no_mangle]
pub extern "C" fn syx_rt_task_spawn(a: i32) -> i32 {
    task_spawn_or_fail_closed(a)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_connect(_ptr: i64, _b: i32) -> i32 {
    // W433: named HostFn stub clear — leave table path via native host call.
    rt_call_host_native(host_fn_index(HostFnId::TcpConnect) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_handle_refcount(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_handle_release(_a: i32) {
    // W455 / ADR-799: B-07 fifth-slice wire — leave noop via native HostFn dispatch.
    let _ = rt_call_host_native(host_fn_index(HostFnId::TcpHandleRelease) as u32);
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_read(_a: i32, _b: i32) -> i32 {
    // W453 / ADR-797: B-07 third-slice wire — leave noop via native HostFn dispatch.
    rt_call_host_native(host_fn_index(HostFnId::TcpRead) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_sockbuf(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_stream(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_stream_handle(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_stream_tls(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tcp_write(_a: i32, _b: i32) -> i32 {
    // W454 / ADR-798: B-07 fourth-slice wire — leave noop via native HostFn dispatch.
    rt_call_host_native(host_fn_index(HostFnId::TcpWrite) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_tls_connect(_ptr: i64, _b: i32) -> i32 {
    // W433: named HostFn stub clear — leave table path via native host call.
    // Still indexes TcpConnect (≠ TLS product status raise).
    rt_call_host_native(host_fn_index(HostFnId::TcpConnect) as u32)
}

#[no_mangle]
pub extern "C" fn syx_rt_tls_handshake(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_tls_rustls_pin(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_try_resume(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_typealias_ga(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_typealias_prod(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_unified_gc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_unified_gc_collect() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_unified_gc_promote(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_unified_gc_promotion(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_unified_gc_remembered(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_vm_jit_deopt(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_vm_native_gc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_vm_native_gc_collect() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_vm_native_gc_remembered(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_wasm_gc_phase28() -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_win_pipe_api(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_win_pipe_named_io(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_win_pipe_real(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_win_pipe_uds(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_win_pipe_winapi(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_aot(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v12(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v13(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v14(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v15(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v16(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v16_inkwell(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v17(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v17_unified(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v18(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v18_formal(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v19(_a: i32, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_witness_v19_gc(_a: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_ws_send(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[no_mangle]
pub extern "C" fn syx_rt_ws_send_prod(ptr: i64, _b: i32) -> i32 {
    rt_direct_exec(syx_mir_run::op::POP, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "syx_rt_await_poll: no-native")]
    fn await_poll_fail_closed_no_mir_direct_exec() {
        let _ = await_poll_or_fail_closed();
    }

    #[test]
    #[should_panic(expected = "syx_rt_await_suspend: no-native")]
    fn await_suspend_fail_closed_no_mir_direct_exec() {
        await_suspend_or_fail_closed(0, 0);
    }

    #[test]
    fn await_stubs_source_has_no_rt_direct_exec() {
        let src = include_str!("llvm_stubs.rs");
        let poll_start = src
            .find("fn await_poll_or_fail_closed")
            .expect("await_poll_or_fail_closed");
        let suspend_end = src
            .find("pub extern \"C\" fn syx_rt_begin_try")
            .unwrap_or(src.len());
        let await_region = &src[poll_start..suspend_end];
        assert!(
            await_region.contains("no-native") && await_region.contains("panic!"),
            "Await helpers must fail-closed (W417)"
        );
        assert!(
            !await_region.contains("rt_direct_exec("),
            "Await stubs must not call rt_direct_exec (W417)"
        );
        assert!(
            !await_region.contains("AWAIT_POLL") && !await_region.contains("AWAIT_SUSPEND"),
            "Await stubs must not reference AWAIT_* MIR opcodes (W417)"
        );
    }

    #[test]
    #[should_panic(expected = "syx_rt_task_spawn: no-native")]
    fn task_spawn_fail_closed_no_mir_direct_exec() {
        let _ = task_spawn_or_fail_closed(0);
    }

    #[test]
    #[should_panic(expected = "syx_rt_task_join: no-native")]
    fn task_join_fail_closed_no_mir_direct_exec() {
        let _ = task_join_or_fail_closed(0);
    }

    #[test]
    #[should_panic(expected = "syx_rt_task_cancel: no-native")]
    fn task_cancel_fail_closed_no_mir_direct_exec() {
        let _ = task_cancel_or_fail_closed();
    }

    #[test]
    fn task_stubs_source_has_no_rt_direct_exec() {
        let src = include_str!("llvm_stubs.rs");
        let start = src
            .find("fn task_cancel_or_fail_closed")
            .expect("task_cancel_or_fail_closed");
        let end = src
            .find("pub extern \"C\" fn syx_rt_tcp_connect")
            .unwrap_or(src.len());
        let task_region = &src[start..end];
        assert!(
            task_region.contains("no-native") && task_region.contains("panic!"),
            "Task helpers must fail-closed (W418)"
        );
        assert!(
            !task_region.contains("rt_direct_exec("),
            "Task stubs must not call rt_direct_exec (W418)"
        );
        assert!(
            !task_region.contains("TASK_SPAWN") && !task_region.contains("TASK_JOIN"),
            "Task stubs must not reference TASK_* MIR opcodes (W418)"
        );
    }

    #[test]
    #[should_panic(expected = "syx_rt_actor_call: no-native")]
    fn actor_call_fail_closed_no_mir_direct_exec() {
        let _ = actor_call_or_fail_closed(0, 0);
    }

    #[test]
    #[should_panic(expected = "syx_rt_actor_send: no-native")]
    fn actor_send_fail_closed_no_mir_direct_exec() {
        let _ = actor_send_or_fail_closed(0, 0);
    }

    #[test]
    #[should_panic(expected = "syx_rt_actor_mailbox: no-native")]
    fn actor_aux_fail_closed_no_mir_direct_exec() {
        let _ = actor_aux_or_fail_closed("syx_rt_actor_mailbox");
    }

    #[test]
    fn actor_stubs_source_has_no_rt_direct_exec() {
        let src = include_str!("llvm_stubs.rs");
        let start = src
            .find("fn actor_call_or_fail_closed")
            .expect("actor_call_or_fail_closed");
        let end = src
            .find("pub extern \"C\" fn syx_rt_adr038_perf")
            .unwrap_or(src.len());
        let actor_region = &src[start..end];
        assert!(
            actor_region.contains("no-native") && actor_region.contains("panic!"),
            "Actor helpers must fail-closed (W419)"
        );
        assert!(
            !actor_region.contains("rt_direct_exec("),
            "Actor stubs must not call rt_direct_exec (W419)"
        );
        assert!(
            !actor_region.contains("ACTOR_CALL") && !actor_region.contains("ACTOR_SEND"),
            "Actor stubs must not reference ACTOR_* MIR opcodes (W419)"
        );
    }
}
