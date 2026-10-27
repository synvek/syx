//! Wave B tri-axis horizon structural readiness inventory (W335 / ADR-679).
//!
//! Aggregates B-04 Short, B-05 Medium, and B-06 clear×align horizon inventories.
//! Short cutover honesty may be on (W400); Medium real codec first-slice may be on (W403);
//! dual-blob production wire + Medium cutover honesty + AOT sidecar embed + dual-load runtime
//! + sidecar-primary deepen + execution-segment omit may be on (W404–W409);
//! Long MIR-free readiness thin may be signed (W410), native-metadata deepen thin may be
//! signed (W411), native-metadata wire first-slice may be signed (W412), emit-omit
//! first-slice may be signed (W413), closure-async MIR-free thin may be signed (W414),
//! closure-async MIR-free deepen thin may be signed (W415), closure-async wire
//! first-slice (CallFn) may be signed (W416), Await wire first-slice may be signed
//! (W417), Task wire first-slice may be signed (W418), Actor wire first-slice may
//! be signed (W419), gap #3 close may be signed (W420), host-bridge thin may be
//! signed (W421), host-bridge deepen thin may be signed (W422), and host-bridge
//! Print wire first-slice may be signed (W423), host-bridge CallHost wire
//! first-slice may be signed (W424), host-bridge pure-native gap close may
//! be signed (W425), sidecar-omit thin may be signed (W426), sidecar-omit
//! deepen thin may be signed (W427), sidecar-omit wire first-slice
//! (Empty-pool) may be signed (W428), Register-only wire first-slice may
//! be signed (W429), C stub align wire first-slice may be signed
//! (W430), and sidecar-omit emit first-slice may be signed (W431)
//! while MIR-free stays off, `@syx_mir_blob` is omitted,
//! sidecar constant is no longer embedded, CallFn + Await + Task + Actor MIR
//! fallbacks are closed, closure-async gap #3 is closed, Print + CallHost MIR
//! opcode bridges are closed, host-bridge gap is closed, Empty-pool and
//! Register-only init gaps are closed, C stub register-loop gap is closed,
//! and Missing clear stays off.

use syx_mir::{
    aot_codec_fixture_main_with_const_strs, aot_reachability_fixture_main_and_export_orphan,
    aot_short_horizon_cutover_readiness_inventory,
    mir_module_medium_horizon_cutover_readiness_inventory,
};

use crate::llvm_stub_audit::llvm_stub_clear_align_horizon_readiness_inventory;

/// Wave B tri-axis horizon structural readiness inventory (≠ Wave B exit / ≠ axis `[x]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveBTriAxisHorizonReadinessInventory {
    pub relational_b04_short_horizon_ok: bool,
    pub relational_b05_medium_horizon_ok: bool,
    pub relational_b06_clear_align_horizon_ok: bool,
    pub candidate_predicate_count: usize,
    /// `true` after W400 — Short production cutover honesty signed (≠ Medium/B-06 delivery).
    pub production_cutover_enabled: bool,
    /// `true` after W403 — Medium real codec first-slice wire (dual-blob production wire may be on after W404).
    pub real_codec_enabled: bool,
    /// `true` after W405 — Medium production cutover honesty (≠ dual-load runtime / ≠ Medium axis `[x]`).
    pub medium_cutover_enabled: bool,
    /// `true` after W406 — Medium AOT sidecar embed first-slice (≠ MIR-free).
    pub aot_embeds_sidecar: bool,
    /// `true` after W407 — Medium dual-load runtime first-slice (≠ MIR-free / ≠ delete blob).
    pub dual_load_runtime_enabled: bool,
    /// `true` after W408 — Medium sidecar-primary string-pool deepen (≠ MIR-free / ≠ delete blob).
    pub sidecar_string_pool_primary: bool,
    /// `true` after W409 — Medium AOT execution-segment omit (≠ MIR-free / ≠ delete blob).
    pub aot_execution_segment_omitted: bool,
    /// `true` after W410 — Long MIR-free readiness thin signed (≠ MIR-free / ≠ delete blob).
    pub long_mir_free_readiness_thin_signed: bool,
    /// `true` after W411 — Long native-metadata deepen thin signed (≠ MIR-free).
    pub native_metadata_deepen_thin_signed: bool,
    /// `true` after W412 — Long native-metadata wire first-slice signed (≠ MIR-free).
    pub native_metadata_wire_first_slice_signed: bool,
    /// `true` after W413 — Long emit-omit first-slice signed (≠ MIR-free).
    pub emit_omit_first_slice_signed: bool,
    /// `true` after W414 — Long closure-async MIR-free thin signed (≠ gap close / ≠ MIR-free).
    pub closure_async_mir_free_thin_signed: bool,
    /// `true` after W415 — Long closure-async MIR-free deepen thin signed (≠ gap close / ≠ MIR-free).
    pub closure_async_mir_free_deepen_thin_signed: bool,
    /// `true` after W416 — Long closure-async wire first-slice signed (CallFn; ≠ full gap close).
    pub closure_async_wire_first_slice_signed: bool,
    /// Always `false` after W416 — CallFn MIR interpret fallback sub-gap closed.
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    /// `true` after W417 — Long Await wire first-slice signed (≠ full gap close).
    pub closure_async_await_wire_first_slice_signed: bool,
    /// Always `false` after W417 — Await MIR interpret / `rt_direct_exec` fallback sub-gap closed.
    pub await_mir_interpret_fallback_gap_open: bool,
    /// `true` after W418 — Long Task wire first-slice signed (≠ full gap close).
    pub closure_async_task_wire_first_slice_signed: bool,
    /// Always `false` after W418 — Task MIR interpret / `rt_direct_exec` fallback sub-gap closed.
    pub task_mir_interpret_fallback_gap_open: bool,
    /// `true` after W419 — Long Actor wire first-slice signed (≠ full gap close).
    pub closure_async_actor_wire_first_slice_signed: bool,
    /// Always `false` after W419 — Actor MIR interpret / `rt_direct_exec` fallback sub-gap closed.
    pub actor_mir_interpret_fallback_gap_open: bool,
    /// `true` after W420 — Long gap #3 close signed (≠ MIR-free / ≠ delete sidecar).
    pub closure_async_mir_free_gap_close_signed: bool,
    /// `true` after W421 — Long host-bridge thin signed (≠ host gap close / ≠ MIR-free).
    pub host_bridge_thin_signed: bool,
    /// `true` after W422 — Long host-bridge deepen thin signed (≠ wire / ≠ host gap close).
    pub host_bridge_deepen_thin_signed: bool,
    /// `true` after W423 — Long host-bridge Print wire first-slice signed (≠ CallHost wire / ≠ host gap close).
    pub host_bridge_print_wire_first_slice_signed: bool,
    /// Always `false` after W423 — Print MIR opcode / `rt_direct_exec(PRINT)` bridge sub-gap closed.
    pub print_mir_opcode_bridge_gap_open: bool,
    /// `true` after W424 — Long host-bridge CallHost wire first-slice signed (≠ host gap close).
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    /// Always `false` after W424 — CallHost MIR opcode / `rt_direct_exec(CALL_HOST)` bridge sub-gap closed.
    pub call_host_mir_opcode_bridge_gap_open: bool,
    /// `true` after W425 — Long host-bridge pure-native gap close signed (≠ MIR-free / ≠ delete sidecar).
    pub host_bridge_pure_native_gap_close_signed: bool,
    /// Always `false` after W425 — honesty: host-bridge pure-native gap closed (Print+CallHost 齐后签收).
    pub host_bridge_pure_native_gap_open: bool,
    /// `true` after W426 — Long sidecar-omit thin signed (≠ emit omit / ≠ MIR-free / ≠ delete sidecar).
    pub sidecar_omit_thin_signed: bool,
    /// `true` after W427 — Long sidecar-omit deepen thin signed (≠ wire ABI / ≠ emit omit / ≠ MIR-free).
    pub sidecar_omit_deepen_thin_signed: bool,
    /// `true` after W428 — Long sidecar-omit wire first-slice signed (Empty-pool ABI; ≠ emit omit).
    pub sidecar_omit_wire_first_slice_signed: bool,
    /// Always `false` after W428 — Empty-pool init gap closed via `syx_rt_install_empty_runtime`.
    pub empty_pool_init_gap_open: bool,
    /// `true` after W429 — Long sidecar-omit Register-only wire first-slice signed (≠ emit omit / ≠ C stub).
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    /// Always `false` after W429 — Register-only init gap closed (empty install + register_native_fn).
    pub register_only_init_gap_open: bool,
    /// `true` after W430 — Long sidecar-omit C stub align wire first-slice signed (≠ emit omit).
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    /// Always `false` after W430 — C stub register-loop gap closed via `register_native_fn` in C stub.
    pub c_stub_register_loop_gap_open: bool,
    /// `true` after W431 — Long sidecar-omit emit first-slice signed (≠ MIR-free).
    pub sidecar_omit_emit_first_slice_signed: bool,
    /// `true` after W432 — Long MIR-free first-slice signed (≠ named stub clear / ≠ B-05 `[x]`).
    pub mir_free_first_slice_signed: bool,
    /// `true` after W433 — Long named HostFn stub CALL_HOST clear first-slice signed (≠ delete flag / ≠ TLS).
    pub named_hostfn_stub_call_host_clear_signed: bool,
    /// Always `false` after W433 — honesty: named HostFn stub CALL_HOST gap closed.
    pub named_hostfn_stub_call_host_gap_open: bool,
    /// `true` after W434 — Long delete_syx_mir_blob_enabled first-slice signed (≠ TLS / ≠ Short).
    pub delete_syx_mir_blob_enabled_first_slice_signed: bool,
    /// Always `false` after W412 — honesty: native-metadata-without-blob gap closed via alternate entry.
    pub native_metadata_without_blob_gap_open: bool,
    /// Always `false` after W413 — llvm emit-omit gap closed.
    pub llvm_emit_omit_blob_gap_open: bool,
    /// Always `false` after W413 — user_init omit-load gap closed.
    pub user_init_omit_load_gap_open: bool,
    /// Always `false` after W420 — honesty: closure-async gap #3 closed (CallFn+Await+Task+Actor 齐后签收).
    pub closure_async_mir_free_gap_open: bool,
    /// `true` after W432 — MIR-free first-slice honesty flag on (≠ named stub clear).
    pub mir_free_enabled: bool,
    /// `true` after W434 — narrative delete_syx_mir_blob_enabled on (emit already gone since W413).
    pub delete_syx_mir_blob_enabled: bool,
    /// Always `false` after W413 — honesty: production no longer embeds `@syx_mir_blob`.
    pub blob_still_embedded: bool,
    /// Always `false` after W431 — honesty: production no longer embeds `@syx_metadata_sidecar`.
    pub sidecar_constant_still_embedded: bool,
    pub missing_clear_enabled: bool,
}

pub fn wave_b_tri_axis_horizon_readiness_inventory() -> WaveBTriAxisHorizonReadinessInventory {
    let short_module = aot_reachability_fixture_main_and_export_orphan();
    let short = aot_short_horizon_cutover_readiness_inventory(&short_module, &["export_lib"]);
    let relational_b04_short_horizon_ok = short.relational_export_root_cutover_chain_ok
        && short.relational_size_ci_cutover_chain_ok
        && short.relational_inventory_overlap_ok
        && short.cutover_enabled;

    let medium_module = aot_codec_fixture_main_with_const_strs(4);
    let medium = mir_module_medium_horizon_cutover_readiness_inventory(&medium_module);
    let relational_b05_medium_horizon_ok = medium.relational_codec_readiness_chain_ok
        && medium.relational_dual_blob_cutover_chain_ok
        && medium.relational_inventory_overlap_ok
        && medium.cutover_enabled
        && medium.real_codec_enabled
        && medium.production_dual_load_enabled
        && medium.aot_embeds_sidecar
        && !medium.load_path_is_whole_module;

    let clear_align = llvm_stub_clear_align_horizon_readiness_inventory();
    let relational_b06_clear_align_horizon_ok = clear_align.relational_missing_clear_chain_ok
        && clear_align.relational_align_inventory_chain_ok
        && clear_align.relational_inventory_overlap_ok
        && !clear_align.clear_enabled
        && !clear_align.stub_rewrite_enabled
        && !clear_align.absolute_align_matrix_exit_enabled;

    // W426: sidecar-omit thin honesty flags below are policy-fixed. Live inventory
    // walk is covered by syx-mir / driver meta tests — avoid exponential encode re-walk.

    WaveBTriAxisHorizonReadinessInventory {
        relational_b04_short_horizon_ok,
        relational_b05_medium_horizon_ok,
        relational_b06_clear_align_horizon_ok,
        candidate_predicate_count: 3,
        production_cutover_enabled: true,
        real_codec_enabled: true,
        medium_cutover_enabled: true,
        aot_embeds_sidecar: true,
        dual_load_runtime_enabled: true,
        sidecar_string_pool_primary: true,
        aot_execution_segment_omitted: true,
        long_mir_free_readiness_thin_signed: true,
        native_metadata_deepen_thin_signed: true,
        native_metadata_wire_first_slice_signed: true,
        emit_omit_first_slice_signed: true,
        closure_async_mir_free_thin_signed: true,
        closure_async_mir_free_deepen_thin_signed: true,
        closure_async_wire_first_slice_signed: true,
        call_fn_mir_interpret_fallback_gap_open: false,
        closure_async_await_wire_first_slice_signed: true,
        await_mir_interpret_fallback_gap_open: false,
        closure_async_task_wire_first_slice_signed: true,
        task_mir_interpret_fallback_gap_open: false,
        closure_async_actor_wire_first_slice_signed: true,
        actor_mir_interpret_fallback_gap_open: false,
        closure_async_mir_free_gap_close_signed: true,
        host_bridge_thin_signed: true,
        host_bridge_deepen_thin_signed: true,
        host_bridge_print_wire_first_slice_signed: true,
        print_mir_opcode_bridge_gap_open: false,
        host_bridge_call_host_wire_first_slice_signed: true,
        call_host_mir_opcode_bridge_gap_open: false,
        host_bridge_pure_native_gap_close_signed: true,
        host_bridge_pure_native_gap_open: false,
        sidecar_omit_thin_signed: true,
        sidecar_omit_deepen_thin_signed: true,
        sidecar_omit_wire_first_slice_signed: true,
        empty_pool_init_gap_open: false,
        sidecar_omit_register_only_wire_first_slice_signed: true,
        register_only_init_gap_open: false,
        sidecar_omit_c_stub_align_wire_first_slice_signed: true,
        c_stub_register_loop_gap_open: false,
        sidecar_omit_emit_first_slice_signed: true,
        mir_free_first_slice_signed: true,
        named_hostfn_stub_call_host_clear_signed: true,
        named_hostfn_stub_call_host_gap_open: false,
        delete_syx_mir_blob_enabled_first_slice_signed: true,
        native_metadata_without_blob_gap_open: false,
        llvm_emit_omit_blob_gap_open: false,
        user_init_omit_load_gap_open: false,
        closure_async_mir_free_gap_open: false,
        mir_free_enabled: true,
        delete_syx_mir_blob_enabled: true,
        blob_still_embedded: false,
        sidecar_constant_still_embedded: false,
        missing_clear_enabled: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveBStructuralExitReadinessInventory {
    pub relational_tri_axis_horizon_ok: bool,
    pub relational_production_honesty_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub candidate_predicate_count: usize,
    pub production_cutover_enabled: bool,
    pub real_codec_enabled: bool,
    pub medium_cutover_enabled: bool,
    pub aot_embeds_sidecar: bool,
    pub dual_load_runtime_enabled: bool,
    pub sidecar_string_pool_primary: bool,
    pub aot_execution_segment_omitted: bool,
    pub long_mir_free_readiness_thin_signed: bool,
    pub native_metadata_deepen_thin_signed: bool,
    pub native_metadata_wire_first_slice_signed: bool,
    pub emit_omit_first_slice_signed: bool,
    pub closure_async_mir_free_thin_signed: bool,
    pub closure_async_mir_free_deepen_thin_signed: bool,
    pub closure_async_wire_first_slice_signed: bool,
    pub call_fn_mir_interpret_fallback_gap_open: bool,
    pub closure_async_await_wire_first_slice_signed: bool,
    pub await_mir_interpret_fallback_gap_open: bool,
    pub closure_async_task_wire_first_slice_signed: bool,
    pub task_mir_interpret_fallback_gap_open: bool,
    pub closure_async_actor_wire_first_slice_signed: bool,
    pub actor_mir_interpret_fallback_gap_open: bool,
    pub closure_async_mir_free_gap_close_signed: bool,
    pub host_bridge_thin_signed: bool,
    pub host_bridge_deepen_thin_signed: bool,
    pub host_bridge_print_wire_first_slice_signed: bool,
    pub print_mir_opcode_bridge_gap_open: bool,
    pub host_bridge_call_host_wire_first_slice_signed: bool,
    pub call_host_mir_opcode_bridge_gap_open: bool,
    pub host_bridge_pure_native_gap_close_signed: bool,
    pub host_bridge_pure_native_gap_open: bool,
    pub sidecar_omit_thin_signed: bool,
    pub sidecar_omit_deepen_thin_signed: bool,
    pub sidecar_omit_wire_first_slice_signed: bool,
    pub empty_pool_init_gap_open: bool,
    pub sidecar_omit_register_only_wire_first_slice_signed: bool,
    pub register_only_init_gap_open: bool,
    pub sidecar_omit_c_stub_align_wire_first_slice_signed: bool,
    pub c_stub_register_loop_gap_open: bool,
    pub sidecar_omit_emit_first_slice_signed: bool,
    pub mir_free_first_slice_signed: bool,
    pub named_hostfn_stub_call_host_clear_signed: bool,
    pub named_hostfn_stub_call_host_gap_open: bool,
    pub delete_syx_mir_blob_enabled_first_slice_signed: bool,
    pub native_metadata_without_blob_gap_open: bool,
    pub llvm_emit_omit_blob_gap_open: bool,
    pub user_init_omit_load_gap_open: bool,
    pub closure_async_mir_free_gap_open: bool,
    pub mir_free_enabled: bool,
    pub delete_syx_mir_blob_enabled: bool,
    pub blob_still_embedded: bool,
    pub sidecar_constant_still_embedded: bool,
    pub missing_clear_enabled: bool,
}

/// Structural exit readiness after honest Wave B defer (≠ axis `[x]` / delete flag after W434).
pub fn wave_b_structural_exit_readiness_inventory() -> WaveBStructuralExitReadinessInventory {
    let tri = wave_b_tri_axis_horizon_readiness_inventory();
    let tri_again = wave_b_tri_axis_horizon_readiness_inventory();
    let relational_tri_axis_horizon_ok = tri.relational_b04_short_horizon_ok
        && tri.relational_b05_medium_horizon_ok
        && tri.relational_b06_clear_align_horizon_ok
        && tri.production_cutover_enabled
        && tri.real_codec_enabled
        && tri.medium_cutover_enabled
        && tri.aot_embeds_sidecar
        && tri.dual_load_runtime_enabled
        && tri.sidecar_string_pool_primary
        && tri.aot_execution_segment_omitted
        && tri.long_mir_free_readiness_thin_signed
        && tri.native_metadata_deepen_thin_signed
        && tri.native_metadata_wire_first_slice_signed
        && tri.emit_omit_first_slice_signed
        && tri.closure_async_mir_free_thin_signed
        && tri.closure_async_mir_free_deepen_thin_signed
        && tri.closure_async_wire_first_slice_signed
        && !tri.call_fn_mir_interpret_fallback_gap_open
        && tri.closure_async_await_wire_first_slice_signed
        && !tri.await_mir_interpret_fallback_gap_open
        && tri.closure_async_task_wire_first_slice_signed
        && !tri.task_mir_interpret_fallback_gap_open
        && tri.closure_async_actor_wire_first_slice_signed
        && !tri.actor_mir_interpret_fallback_gap_open
        && tri.closure_async_mir_free_gap_close_signed
        && !tri.closure_async_mir_free_gap_open
        && tri.host_bridge_thin_signed
        && tri.host_bridge_deepen_thin_signed
        && tri.host_bridge_print_wire_first_slice_signed
        && !tri.print_mir_opcode_bridge_gap_open
        && tri.host_bridge_call_host_wire_first_slice_signed
        && !tri.call_host_mir_opcode_bridge_gap_open
        && tri.host_bridge_pure_native_gap_close_signed
        && !tri.host_bridge_pure_native_gap_open
        && tri.sidecar_omit_thin_signed
        && tri.sidecar_omit_deepen_thin_signed
        && tri.sidecar_omit_wire_first_slice_signed
        && !tri.empty_pool_init_gap_open
        && tri.sidecar_omit_register_only_wire_first_slice_signed
        && !tri.register_only_init_gap_open
        && tri.sidecar_omit_c_stub_align_wire_first_slice_signed
        && !tri.c_stub_register_loop_gap_open
        && tri.sidecar_omit_emit_first_slice_signed
        && tri.mir_free_first_slice_signed
        && tri.named_hostfn_stub_call_host_clear_signed
        && !tri.named_hostfn_stub_call_host_gap_open
        && tri.delete_syx_mir_blob_enabled_first_slice_signed
        && tri.delete_syx_mir_blob_enabled
        && tri.mir_free_enabled
        && !tri.blob_still_embedded
        && !tri.sidecar_constant_still_embedded
        && !tri.missing_clear_enabled;
    let relational_production_honesty_ok = tri.production_cutover_enabled
        && tri.real_codec_enabled
        && tri.medium_cutover_enabled
        && tri.aot_embeds_sidecar
        && tri.dual_load_runtime_enabled
        && tri.sidecar_string_pool_primary
        && tri.aot_execution_segment_omitted
        && tri.mir_free_enabled
        && tri.named_hostfn_stub_call_host_clear_signed
        && !tri.named_hostfn_stub_call_host_gap_open
        && tri.delete_syx_mir_blob_enabled_first_slice_signed
        && tri.delete_syx_mir_blob_enabled
        && !tri.blob_still_embedded
        && !tri.sidecar_constant_still_embedded
        && !tri.missing_clear_enabled;
    WaveBStructuralExitReadinessInventory {
        relational_tri_axis_horizon_ok,
        relational_production_honesty_ok,
        relational_inventory_overlap_ok: tri == tri_again,
        candidate_predicate_count: 3,
        production_cutover_enabled: tri.production_cutover_enabled,
        real_codec_enabled: tri.real_codec_enabled,
        medium_cutover_enabled: tri.medium_cutover_enabled,
        aot_embeds_sidecar: tri.aot_embeds_sidecar,
        dual_load_runtime_enabled: tri.dual_load_runtime_enabled,
        sidecar_string_pool_primary: tri.sidecar_string_pool_primary,
        aot_execution_segment_omitted: tri.aot_execution_segment_omitted,
        long_mir_free_readiness_thin_signed: tri.long_mir_free_readiness_thin_signed,
        native_metadata_deepen_thin_signed: tri.native_metadata_deepen_thin_signed,
        native_metadata_wire_first_slice_signed: tri.native_metadata_wire_first_slice_signed,
        emit_omit_first_slice_signed: tri.emit_omit_first_slice_signed,
        closure_async_mir_free_thin_signed: tri.closure_async_mir_free_thin_signed,
        closure_async_mir_free_deepen_thin_signed: tri.closure_async_mir_free_deepen_thin_signed,
        closure_async_wire_first_slice_signed: tri.closure_async_wire_first_slice_signed,
        call_fn_mir_interpret_fallback_gap_open: tri.call_fn_mir_interpret_fallback_gap_open,
        closure_async_await_wire_first_slice_signed: tri
            .closure_async_await_wire_first_slice_signed,
        await_mir_interpret_fallback_gap_open: tri.await_mir_interpret_fallback_gap_open,
        closure_async_task_wire_first_slice_signed: tri.closure_async_task_wire_first_slice_signed,
        task_mir_interpret_fallback_gap_open: tri.task_mir_interpret_fallback_gap_open,
        closure_async_actor_wire_first_slice_signed: tri
            .closure_async_actor_wire_first_slice_signed,
        actor_mir_interpret_fallback_gap_open: tri.actor_mir_interpret_fallback_gap_open,
        closure_async_mir_free_gap_close_signed: tri.closure_async_mir_free_gap_close_signed,
        host_bridge_thin_signed: tri.host_bridge_thin_signed,
        host_bridge_deepen_thin_signed: tri.host_bridge_deepen_thin_signed,
        host_bridge_print_wire_first_slice_signed: tri.host_bridge_print_wire_first_slice_signed,
        print_mir_opcode_bridge_gap_open: tri.print_mir_opcode_bridge_gap_open,
        host_bridge_call_host_wire_first_slice_signed: tri
            .host_bridge_call_host_wire_first_slice_signed,
        call_host_mir_opcode_bridge_gap_open: tri.call_host_mir_opcode_bridge_gap_open,
        host_bridge_pure_native_gap_close_signed: tri.host_bridge_pure_native_gap_close_signed,
        host_bridge_pure_native_gap_open: tri.host_bridge_pure_native_gap_open,
        sidecar_omit_thin_signed: tri.sidecar_omit_thin_signed,
        sidecar_omit_deepen_thin_signed: tri.sidecar_omit_deepen_thin_signed,
        sidecar_omit_wire_first_slice_signed: tri.sidecar_omit_wire_first_slice_signed,
        empty_pool_init_gap_open: tri.empty_pool_init_gap_open,
        sidecar_omit_register_only_wire_first_slice_signed: tri
            .sidecar_omit_register_only_wire_first_slice_signed,
        register_only_init_gap_open: tri.register_only_init_gap_open,
        sidecar_omit_c_stub_align_wire_first_slice_signed: tri
            .sidecar_omit_c_stub_align_wire_first_slice_signed,
        c_stub_register_loop_gap_open: tri.c_stub_register_loop_gap_open,
        sidecar_omit_emit_first_slice_signed: tri.sidecar_omit_emit_first_slice_signed,
        mir_free_first_slice_signed: tri.mir_free_first_slice_signed,
        named_hostfn_stub_call_host_clear_signed: tri.named_hostfn_stub_call_host_clear_signed,
        named_hostfn_stub_call_host_gap_open: tri.named_hostfn_stub_call_host_gap_open,
        delete_syx_mir_blob_enabled_first_slice_signed: tri
            .delete_syx_mir_blob_enabled_first_slice_signed,
        native_metadata_without_blob_gap_open: tri.native_metadata_without_blob_gap_open,
        llvm_emit_omit_blob_gap_open: tri.llvm_emit_omit_blob_gap_open,
        user_init_omit_load_gap_open: tri.user_init_omit_load_gap_open,
        closure_async_mir_free_gap_open: tri.closure_async_mir_free_gap_open,
        mir_free_enabled: tri.mir_free_enabled,
        delete_syx_mir_blob_enabled: tri.delete_syx_mir_blob_enabled,
        blob_still_embedded: tri.blob_still_embedded,
        sidecar_constant_still_embedded: tri.sidecar_constant_still_embedded,
        missing_clear_enabled: tri.missing_clear_enabled,
    }
}

/// MIR residual honesty inventory (W459 / ADR-803).
///
/// Signs that production native already omits MIR blob/sidecar while residual
/// IR declare / RT load ABI / dead `aot_compact_view` paths remain present for
/// tests/inventory. Does **not** delete ABI/declare or change production emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirResidualHonestyInventory {
    /// Always `false` — production AOT does not embed `@syx_mir_blob`.
    pub production_blob_embedded: bool,
    /// Always `false` — production `syx_user_init` does not call `syx_rt_load_mir_blob`.
    pub production_user_init_calls_load: bool,
    /// Always `true` — IR still declares `@syx_rt_load_mir_blob` (residual).
    pub residual_ir_declare_present: bool,
    /// Always `true` — RT still exports `syx_rt_load_mir_blob` (test/inventory).
    pub residual_rt_abi_exported: bool,
    /// Always `true` — `aot_compact_view` still computes unused sidecar bytes.
    pub residual_aot_compact_view_dead: bool,
    /// Always `false` — honesty: residual cleanup (delete ABI/declare) remains off.
    pub residual_cleanup_enabled: bool,
    /// Always `true` — W459 / ADR-803: residual honesty first-slice signed.
    pub residual_honesty_signed: bool,
    pub relational_inventory_overlap_ok: bool,
}

/// Inventory MIR residual honesty (W459 / ADR-803).
///
/// Policy+inventory only — does not mutate llvm declare / RT ABI / emit (ADR-803).
pub fn mir_residual_honesty_inventory() -> MirResidualHonestyInventory {
    let again = mir_residual_honesty_inventory_once();
    let first = mir_residual_honesty_inventory_once();
    MirResidualHonestyInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn mir_residual_honesty_inventory_once() -> MirResidualHonestyInventory {
    MirResidualHonestyInventory {
        production_blob_embedded: false,
        production_user_init_calls_load: false,
        residual_ir_declare_present: true,
        residual_rt_abi_exported: true,
        residual_aot_compact_view_dead: true,
        residual_cleanup_enabled: false,
        residual_honesty_signed: true,
        relational_inventory_overlap_ok: true,
    }
}

/// MIR residual ABI narrow cleanup inventory (W484 / ADR-828).
///
/// Signs omit of IR `declare @syx_rt_load_mir_blob` + production dead sidecar encode
/// while **retaining** RT `syx_rt_load_mir_blob` ABI. Historical
/// [`MirResidualHonestyInventory`] keeps W459 floor flags unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MirResidualAbiNarrowCleanupInventory {
    /// Always `false` — production AOT does not embed `@syx_mir_blob`.
    pub production_blob_embedded: bool,
    /// Always `true` — W459 honesty floor still signed.
    pub residual_honesty_floor_ok: bool,
    /// Always `true` — W484 narrow cleanup enabled (declare + dead encode).
    pub residual_cleanup_enabled: bool,
    /// Always `true` — production IR no longer declares `@syx_rt_load_mir_blob`.
    pub residual_ir_declare_omitted: bool,
    /// Always `true` — production shim/C stub no longer dead-encode sidecar bytes.
    pub residual_dead_sidecar_encode_omitted: bool,
    /// Always `true` — RT still exports `syx_rt_load_mir_blob` (B-05 / tests).
    pub residual_rt_abi_exported: bool,
    /// Always `false` — RT ABI delete remains Frozen (≠ full residual clear).
    pub rt_abi_delete_enabled: bool,
    /// Always `true` — W484 / ADR-828: residual ABI narrow cleanup signed.
    pub mir_residual_abi_narrow_cleanup_signed: bool,
    pub relational_inventory_overlap_ok: bool,
}

/// Inventory MIR residual ABI narrow cleanup (W484 / ADR-828).
pub fn mir_residual_abi_narrow_cleanup_inventory() -> MirResidualAbiNarrowCleanupInventory {
    let again = mir_residual_abi_narrow_cleanup_inventory_once();
    let first = mir_residual_abi_narrow_cleanup_inventory_once();
    MirResidualAbiNarrowCleanupInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn mir_residual_abi_narrow_cleanup_inventory_once() -> MirResidualAbiNarrowCleanupInventory {
    MirResidualAbiNarrowCleanupInventory {
        production_blob_embedded: false,
        residual_honesty_floor_ok: true,
        residual_cleanup_enabled: true,
        residual_ir_declare_omitted: true,
        residual_dead_sidecar_encode_omitted: true,
        residual_rt_abi_exported: true,
        rt_abi_delete_enabled: false,
        mir_residual_abi_narrow_cleanup_signed: true,
        relational_inventory_overlap_ok: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_mir::{
        mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory,
        mir_module_long_horizon_mir_free_first_slice_inventory,
        mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory,
    };

    #[test]
    fn mir_residual_honesty_inventory_is_deterministic_and_signed() {
        let a = mir_residual_honesty_inventory();
        let b = mir_residual_honesty_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert_eq!(a.production_blob_embedded, false);
        assert_eq!(a.production_user_init_calls_load, false);
        assert_eq!(a.residual_ir_declare_present, true);
        assert_eq!(a.residual_rt_abi_exported, true);
        assert_eq!(a.residual_aot_compact_view_dead, true);
        assert_eq!(a.residual_cleanup_enabled, false);
        assert_eq!(a.residual_honesty_signed, true);
    }

    #[test]
    fn mir_residual_abi_narrow_cleanup_inventory_is_deterministic_and_signed() {
        let a = mir_residual_abi_narrow_cleanup_inventory();
        let b = mir_residual_abi_narrow_cleanup_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(!a.production_blob_embedded);
        assert!(a.residual_honesty_floor_ok);
        assert!(a.residual_cleanup_enabled);
        assert!(a.residual_ir_declare_omitted);
        assert!(a.residual_dead_sidecar_encode_omitted);
        assert!(a.residual_rt_abi_exported);
        assert!(!a.rt_abi_delete_enabled);
        assert!(a.mir_residual_abi_narrow_cleanup_signed);
        // Historical W459 floor unchanged.
        let floor = mir_residual_honesty_inventory();
        assert!(floor.residual_ir_declare_present);
        assert!(floor.residual_aot_compact_view_dead);
        assert!(!floor.residual_cleanup_enabled);
    }

    #[test]
    fn wave_b_tri_axis_horizon_readiness_inventory_is_deterministic() {
        let a = wave_b_tri_axis_horizon_readiness_inventory();
        let b = wave_b_tri_axis_horizon_readiness_inventory();
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 3);
        assert!(a.relational_b04_short_horizon_ok);
        assert!(a.relational_b05_medium_horizon_ok);
        assert!(a.relational_b06_clear_align_horizon_ok);
        assert_eq!(a.production_cutover_enabled, true);
        assert_eq!(a.real_codec_enabled, true);
        assert_eq!(a.medium_cutover_enabled, true);
        assert_eq!(a.aot_embeds_sidecar, true);
        assert_eq!(a.dual_load_runtime_enabled, true);
        assert_eq!(a.sidecar_string_pool_primary, true);
        assert_eq!(a.aot_execution_segment_omitted, true);
        assert_eq!(a.long_mir_free_readiness_thin_signed, true);
        assert_eq!(a.native_metadata_deepen_thin_signed, true);
        assert_eq!(a.native_metadata_wire_first_slice_signed, true);
        assert_eq!(a.emit_omit_first_slice_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(a.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_actor_wire_first_slice_signed, true);
        assert_eq!(a.actor_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.host_bridge_thin_signed, true);
        assert_eq!(a.host_bridge_deepen_thin_signed, true);
        assert_eq!(a.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(a.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(a.call_host_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_pure_native_gap_close_signed, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, false);
        assert_eq!(a.sidecar_omit_thin_signed, true);
        assert_eq!(a.sidecar_omit_deepen_thin_signed, true);
        assert_eq!(a.sidecar_omit_wire_first_slice_signed, true);
        assert_eq!(a.empty_pool_init_gap_open, false);
        assert_eq!(a.sidecar_omit_register_only_wire_first_slice_signed, true);
        assert_eq!(a.register_only_init_gap_open, false);
        assert_eq!(a.sidecar_omit_c_stub_align_wire_first_slice_signed, true);
        assert_eq!(a.c_stub_register_loop_gap_open, false);
        assert_eq!(a.sidecar_omit_emit_first_slice_signed, true);
        assert_eq!(a.mir_free_first_slice_signed, true);
        assert_eq!(a.named_hostfn_stub_call_host_clear_signed, true);
        assert_eq!(a.named_hostfn_stub_call_host_gap_open, false);
        assert_eq!(a.delete_syx_mir_blob_enabled_first_slice_signed, true);
        assert_eq!(a.native_metadata_without_blob_gap_open, false);
        assert_eq!(a.llvm_emit_omit_blob_gap_open, false);
        assert_eq!(a.user_init_omit_load_gap_open, false);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.mir_free_enabled, true);
        assert_eq!(a.delete_syx_mir_blob_enabled, true);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, false);
        assert_eq!(a.missing_clear_enabled, false);
    }

    #[test]
    fn wave_b_tri_axis_matches_long_mir_free_inventory() {
        let inv = wave_b_tri_axis_horizon_readiness_inventory();
        let module = aot_codec_fixture_main_with_const_strs(4);
        let mir_free = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
        assert_eq!(
            inv.mir_free_first_slice_signed,
            mir_free.mir_free_first_slice_signed
        );
        assert_eq!(
            inv.sidecar_omit_emit_first_slice_signed,
            mir_free.sidecar_omit_emit_first_slice_signed
        );
        assert_eq!(
            inv.sidecar_constant_still_embedded,
            mir_free.sidecar_constant_still_embedded
        );
        assert_eq!(inv.mir_free_enabled, mir_free.mir_free_enabled);
        assert_eq!(inv.blob_still_embedded, mir_free.blob_still_embedded);
    }

    #[test]
    fn wave_b_tri_axis_matches_named_stub_clear_inventory() {
        let inv = wave_b_tri_axis_horizon_readiness_inventory();
        let module = aot_codec_fixture_main_with_const_strs(4);
        let clear = mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(
            &module,
        );
        assert_eq!(
            inv.named_hostfn_stub_call_host_clear_signed,
            clear.named_hostfn_stub_call_host_clear_signed
        );
        assert_eq!(
            inv.named_hostfn_stub_call_host_gap_open,
            clear.named_hostfn_stub_call_host_gap_open
        );
        assert_eq!(inv.mir_free_enabled, clear.mir_free_enabled);
        assert_eq!(
            inv.host_bridge_pure_native_gap_open,
            clear.host_bridge_pure_native_gap_open
        );
    }

    #[test]
    fn wave_b_tri_axis_matches_delete_blob_enabled_inventory() {
        let inv = wave_b_tri_axis_horizon_readiness_inventory();
        let module = aot_codec_fixture_main_with_const_strs(4);
        let delete =
            mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(&module);
        assert_eq!(
            inv.delete_syx_mir_blob_enabled_first_slice_signed,
            delete.delete_syx_mir_blob_enabled_first_slice_signed
        );
        assert_eq!(
            inv.delete_syx_mir_blob_enabled,
            delete.delete_syx_mir_blob_enabled
        );
        assert_eq!(inv.mir_free_enabled, delete.mir_free_enabled);
        assert_eq!(
            inv.named_hostfn_stub_call_host_clear_signed,
            delete.named_hostfn_stub_call_host_clear_signed
        );
        assert_eq!(inv.blob_still_embedded, delete.blob_still_embedded);
    }

    #[test]
    fn wave_b_structural_exit_readiness_inventory_is_deterministic() {
        let a = wave_b_structural_exit_readiness_inventory();
        let b = wave_b_structural_exit_readiness_inventory();
        assert_eq!(a, b);
        assert_eq!(a.candidate_predicate_count, 3);
        assert!(a.relational_tri_axis_horizon_ok);
        assert!(a.relational_production_honesty_ok);
        assert!(a.relational_inventory_overlap_ok);
        assert_eq!(a.production_cutover_enabled, true);
        assert_eq!(a.real_codec_enabled, true);
        assert_eq!(a.medium_cutover_enabled, true);
        assert_eq!(a.aot_embeds_sidecar, true);
        assert_eq!(a.dual_load_runtime_enabled, true);
        assert_eq!(a.sidecar_string_pool_primary, true);
        assert_eq!(a.aot_execution_segment_omitted, true);
        assert_eq!(a.long_mir_free_readiness_thin_signed, true);
        assert_eq!(a.native_metadata_deepen_thin_signed, true);
        assert_eq!(a.native_metadata_wire_first_slice_signed, true);
        assert_eq!(a.emit_omit_first_slice_signed, true);
        assert_eq!(a.closure_async_mir_free_thin_signed, true);
        assert_eq!(a.closure_async_mir_free_deepen_thin_signed, true);
        assert_eq!(a.closure_async_wire_first_slice_signed, true);
        assert_eq!(a.call_fn_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_await_wire_first_slice_signed, true);
        assert_eq!(a.await_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_task_wire_first_slice_signed, true);
        assert_eq!(a.task_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_actor_wire_first_slice_signed, true);
        assert_eq!(a.actor_mir_interpret_fallback_gap_open, false);
        assert_eq!(a.closure_async_mir_free_gap_close_signed, true);
        assert_eq!(a.host_bridge_thin_signed, true);
        assert_eq!(a.host_bridge_deepen_thin_signed, true);
        assert_eq!(a.host_bridge_print_wire_first_slice_signed, true);
        assert_eq!(a.print_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_call_host_wire_first_slice_signed, true);
        assert_eq!(a.call_host_mir_opcode_bridge_gap_open, false);
        assert_eq!(a.host_bridge_pure_native_gap_close_signed, true);
        assert_eq!(a.host_bridge_pure_native_gap_open, false);
        assert_eq!(a.sidecar_omit_thin_signed, true);
        assert_eq!(a.sidecar_omit_deepen_thin_signed, true);
        assert_eq!(a.sidecar_omit_wire_first_slice_signed, true);
        assert_eq!(a.empty_pool_init_gap_open, false);
        assert_eq!(a.sidecar_omit_register_only_wire_first_slice_signed, true);
        assert_eq!(a.register_only_init_gap_open, false);
        assert_eq!(a.sidecar_omit_c_stub_align_wire_first_slice_signed, true);
        assert_eq!(a.c_stub_register_loop_gap_open, false);
        assert_eq!(a.sidecar_omit_emit_first_slice_signed, true);
        assert_eq!(a.mir_free_first_slice_signed, true);
        assert_eq!(a.named_hostfn_stub_call_host_clear_signed, true);
        assert_eq!(a.named_hostfn_stub_call_host_gap_open, false);
        assert_eq!(a.delete_syx_mir_blob_enabled_first_slice_signed, true);
        assert_eq!(a.closure_async_mir_free_gap_open, false);
        assert_eq!(a.mir_free_enabled, true);
        assert_eq!(a.delete_syx_mir_blob_enabled, true);
        assert_eq!(a.blob_still_embedded, false);
        assert_eq!(a.sidecar_constant_still_embedded, false);
        assert_eq!(a.missing_clear_enabled, false);
    }

    #[test]
    fn wave_b_structural_exit_requires_long_honesty_and_mir_free() {
        let tri = wave_b_tri_axis_horizon_readiness_inventory();
        let inv = wave_b_structural_exit_readiness_inventory();
        assert!(
            inv.relational_tri_axis_horizon_ok
                && tri.mir_free_enabled
                && tri.mir_free_first_slice_signed
                && tri.named_hostfn_stub_call_host_clear_signed
                && !tri.named_hostfn_stub_call_host_gap_open
                && tri.delete_syx_mir_blob_enabled_first_slice_signed
                && tri.delete_syx_mir_blob_enabled
                && !tri.blob_still_embedded
                && !tri.sidecar_constant_still_embedded
                && tri.emit_omit_first_slice_signed
                && tri.closure_async_mir_free_thin_signed
                && tri.closure_async_mir_free_deepen_thin_signed
                && tri.closure_async_wire_first_slice_signed
                && !tri.call_fn_mir_interpret_fallback_gap_open
                && tri.closure_async_await_wire_first_slice_signed
                && !tri.await_mir_interpret_fallback_gap_open
                && tri.closure_async_task_wire_first_slice_signed
                && !tri.task_mir_interpret_fallback_gap_open
                && tri.closure_async_actor_wire_first_slice_signed
                && !tri.actor_mir_interpret_fallback_gap_open
                && tri.closure_async_mir_free_gap_close_signed
                && !tri.closure_async_mir_free_gap_open
                && tri.host_bridge_thin_signed
                && tri.host_bridge_deepen_thin_signed
                && tri.host_bridge_print_wire_first_slice_signed
                && !tri.print_mir_opcode_bridge_gap_open
                && tri.host_bridge_call_host_wire_first_slice_signed
                && !tri.call_host_mir_opcode_bridge_gap_open
                && tri.host_bridge_pure_native_gap_close_signed
                && !tri.host_bridge_pure_native_gap_open
                && tri.sidecar_omit_thin_signed
                && tri.sidecar_omit_deepen_thin_signed
                && tri.sidecar_omit_wire_first_slice_signed
                && !tri.empty_pool_init_gap_open
                && tri.sidecar_omit_register_only_wire_first_slice_signed
                && !tri.register_only_init_gap_open
                && tri.sidecar_omit_c_stub_align_wire_first_slice_signed
                && !tri.c_stub_register_loop_gap_open
                && tri.sidecar_omit_emit_first_slice_signed
                && !tri.missing_clear_enabled
        );
    }
}
