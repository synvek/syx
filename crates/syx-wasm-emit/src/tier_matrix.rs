//! Phase 70: single source of truth for WASM AOT tier import names.
//! Phase 104 Track B: `wasm-simd-guest-v1:no-mir-exec-simd`, `wasm-i128-guest-v1:no-mir-exec-const128`, `wasm-allowlist-empty-v1:phase104`.
//! Phase 105 Track B: `wasm-simd-full-v1:aot-ops-complete`, `wasm-no-mir-exec-catchall-v1:phase105`.
//! Phase 106 Track B: `wasm-simd-lt-shuffle-v1:aot-complete`, `wasm-ga-no-mir-exec-env-v1:phase106`.

pub const TIER1_NAMES: &[&str] = &[
    "__syx_aot_load_global",
    "__syx_aot_load_local",
    "__syx_aot_store_global",
    "__syx_aot_store_local",
    "__syx_aot_const_str",
    "__syx_aot_array_build",
    "__syx_aot_dict_build",
    "__syx_aot_set_build",
    "__syx_aot_print",
    "__syx_aot_concat_str",
    "__syx_aot_to_str",
    "__syx_aot_begin_fn",
    "__syx_aot_get_return",
    "__syx_aot_set_fn_index",
];

pub const TIER2A_NAMES: &[&str] = &[
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_array_set",
    "__syx_aot_value_eq",
    "__syx_aot_value_ne",
    "__syx_aot_ref_eq",
    "__syx_aot_ref_ne",
    "__syx_aot_call_witness",
    "__syx_aot_array_get",
    "__syx_aot_array_sorted",
    "__syx_aot_dict_get",
];

pub const TIER2B_NAMES: &[&str] = &[
    "__syx_aot_make_iterator",
    "__syx_aot_iterator_next",
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
];

/// Wave A W511 / ADR-855: retain/release AOT **emit** retired; names remain as
/// host-wrap / historical inventory surface (new modules no longer import).
pub const TIER2D_NAMES: &[&str] = &[
    "__syx_aot_object_ref_retain",
    "__syx_aot_object_ref_release",
    "__syx_aot_heap_collect",
];

pub const TIER2E_NAMES: &[&str] = &[
    "__syx_aot_simd_load",
    "__syx_aot_simd_store",
    "__syx_aot_simd_shuffle",
    "__syx_aot_simd_div",
    "__syx_aot_simd_eq",
    "__syx_aot_actor_send",
    "__syx_aot_actor_register",
];

pub const TIER2F_NAMES: &[&str] = &[
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
    "__syx_aot_heap_compact",
    "__syx_aot_heap_collect",
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_simd_add",
    "__syx_aot_simd_sub",
    "__syx_aot_simd_mul",
    "__syx_aot_simd_div",
    "__syx_aot_simd_eq",
    "__syx_aot_simd_lt",
    "__syx_aot_simd_gt",
    "__syx_aot_simd_ne",
    "__syx_aot_simd_le",
    "__syx_aot_simd_ge",
    "__syx_aot_simd_and",
    "__syx_aot_simd_or",
    "__syx_aot_simd_xor",
    "__syx_aot_simd_not",
    "__syx_aot_simd_shl",
    "__syx_aot_simd_shr",
    "__syx_aot_simd_load",
    "__syx_aot_simd_store",
    "__syx_aot_simd_shuffle",
    "__syx_aot_simd_wasm_spill",
    "__syx_aot_simd_wasm_fill",
    "__syx_aot_const_numeric128",
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
];

/// Phase 86: tier-2I = tier-2H + StringBuilder + reflect + net closure hosts.
pub const TIER2I_NAMES: &[&str] = &[
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
    "__syx_aot_heap_compact",
    "__syx_aot_heap_collect",
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_dict_get",
    "__syx_aot_dict_build",
    "__syx_aot_set_build",
    "__syx_aot_begin_try",
    "__syx_aot_catch",
    "__syx_aot_end_try",
    "__syx_aot_throw",
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
    "__syx_aot_simd_add",
    "__syx_aot_simd_sub",
    "__syx_aot_simd_mul",
    "__syx_aot_simd_div",
    "__syx_aot_simd_eq",
    "__syx_aot_simd_lt",
    "__syx_aot_simd_gt",
    "__syx_aot_simd_ne",
    "__syx_aot_simd_le",
    "__syx_aot_simd_ge",
    "__syx_aot_simd_and",
    "__syx_aot_simd_or",
    "__syx_aot_simd_xor",
    "__syx_aot_simd_not",
    "__syx_aot_simd_shl",
    "__syx_aot_simd_shr",
    "__syx_aot_simd_load",
    "__syx_aot_simd_store",
    "__syx_aot_simd_shuffle",
    "__syx_aot_simd_wasm_spill",
    "__syx_aot_simd_wasm_fill",
    "__syx_aot_const_numeric128",
    "__syx_fmt_print",
    "__syx_fmt_format",
    "__syx_process_args",
    "__syx_channel_send",
    "__syx_channel_create",
    "__syx_channel_recv",
    "__syx_string_builder_new",
    "__syx_string_builder_append",
    "__syx_string_builder_to_string",
    "__syx_reflect_type",
    "__syx_reflect_specialization",
];

/// Phase 85: tier-2H = tier-2G + SIMD110 + Try/Catch complete + stdlib prelude minimum.
pub const TIER2H_NAMES: &[&str] = &[
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
    "__syx_aot_heap_compact",
    "__syx_aot_heap_collect",
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_dict_get",
    "__syx_aot_dict_build",
    "__syx_aot_set_build",
    "__syx_aot_begin_try",
    "__syx_aot_catch",
    "__syx_aot_end_try",
    "__syx_aot_throw",
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
    "__syx_aot_simd_add",
    "__syx_aot_simd_sub",
    "__syx_aot_simd_mul",
    "__syx_aot_simd_div",
    "__syx_aot_simd_eq",
    "__syx_aot_simd_lt",
    "__syx_aot_simd_gt",
    "__syx_aot_simd_ne",
    "__syx_aot_simd_le",
    "__syx_aot_simd_ge",
    "__syx_aot_simd_and",
    "__syx_aot_simd_or",
    "__syx_aot_simd_xor",
    "__syx_aot_simd_not",
    "__syx_aot_simd_shl",
    "__syx_aot_simd_shr",
    "__syx_aot_simd_load",
    "__syx_aot_simd_store",
    "__syx_aot_simd_shuffle",
    "__syx_aot_simd_wasm_spill",
    "__syx_aot_simd_wasm_fill",
    "__syx_aot_const_numeric128",
    "__syx_fmt_print",
    "__syx_fmt_format",
    "__syx_process_args",
    "__syx_channel_send",
    "__syx_channel_create",
    "__syx_channel_recv",
];

/// Phase 84: tier-2G = tier-2F + dict/try + stdlib hot hosts for browser hard run.
pub const TIER2G_NAMES: &[&str] = &[
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
    "__syx_aot_heap_compact",
    "__syx_aot_heap_collect",
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_dict_get",
    "__syx_aot_dict_build",
    "__syx_aot_set_build",
    "__syx_aot_begin_try",
    "__syx_aot_catch",
    "__syx_aot_end_try",
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
    "__syx_fmt_print",
    "__syx_process_args",
    "__syx_channel_send",
];

pub const TIER2C_NAMES: &[&str] = &[
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_send",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
    "__syx_aot_begin_try",
    "__syx_aot_catch",
    "__syx_aot_throw",
    "__syx_aot_end_try",
    "__syx_aot_simd_add",
    "__syx_aot_simd_sub",
    "__syx_aot_simd_mul",
    "__syx_aot_simd_div",
    "__syx_aot_simd_eq",
    "__syx_aot_simd_lt",
    "__syx_aot_simd_gt",
    "__syx_aot_simd_ne",
    "__syx_aot_simd_le",
    "__syx_aot_simd_ge",
    "__syx_aot_simd_and",
    "__syx_aot_simd_or",
    "__syx_aot_simd_xor",
    "__syx_aot_simd_not",
    "__syx_aot_simd_shl",
    "__syx_aot_simd_shr",
    "__syx_aot_simd_load",
    "__syx_aot_simd_store",
    "__syx_aot_simd_shuffle",
    "__syx_aot_simd_wasm_spill",
    "__syx_aot_simd_wasm_fill",
    "__syx_aot_const_numeric128",
    "__syx_aot_exec_op",
    "__syx_aot_run_op",
];

pub const NATIVE_TIER2_RT_NAMES: &[&str] = &[
    "syx_rt_dict_get",
    "syx_rt_dict_set",
    "syx_rt_make_iterator",
    "syx_rt_iterator_next",
    "syx_rt_make_closure",
    "syx_rt_call_closure",
    "syx_rt_await_poll",
    "syx_rt_await_suspend",
    "syx_rt_actor_call",
    "syx_rt_actor_send",
    "syx_rt_actor_register",
    "syx_rt_task_spawn",
    "syx_rt_task_join",
    "syx_rt_begin_try",
    "syx_rt_catch",
    "syx_rt_throw",
    "syx_rt_end_try",
    "syx_rt_simd_add",
    "syx_rt_simd_sub",
    "syx_rt_simd_mul",
    "syx_rt_object_ref_retain",
    "syx_rt_object_ref_release",
    "syx_rt_heap_collect",
];

/// Phase 90: tier-3 = tier-2I direct imports without `__syx_aot_exec_op`.
pub const TIER3_NAMES: &[&str] = &[
    "__syx_aot_make_closure",
    "__syx_aot_call_closure",
    "__syx_aot_await_poll",
    "__syx_aot_await_suspend",
    "__syx_aot_heap_compact",
    "__syx_aot_heap_collect",
    "__syx_aot_object_get",
    "__syx_aot_object_set",
    "__syx_aot_dict_get",
    "__syx_aot_dict_build",
    "__syx_aot_set_build",
    "__syx_aot_begin_try",
    "__syx_aot_catch",
    "__syx_aot_end_try",
    "__syx_aot_call_fn",
    "__syx_aot_call_host",
    "__syx_aot_actor_call",
    "__syx_aot_actor_register",
    "__syx_aot_task_spawn",
    "__syx_aot_task_join",
    "__syx_fmt_print",
    "__syx_process_args",
    "__syx_channel_send",
    "__syx_aot_load_global",
    "__syx_aot_store_global",
    "__syx_aot_const_str",
    "__syx_aot_array_build",
    "__syx_aot_print",
    "__syx_aot_array_get",
    "__syx_aot_array_set",
    "__syx_aot_value_eq",
    "__syx_aot_call_witness",
];

pub fn tier_for_import(name: &str) -> Option<&'static str> {
    if name == "mir_exec" {
        return Some("mir_exec");
    }
    if TIER1_NAMES.contains(&name) {
        return Some("tier1");
    }
    if TIER2A_NAMES.contains(&name) {
        return Some("tier2a");
    }
    if TIER2B_NAMES.contains(&name) {
        return Some("tier2b");
    }
    if TIER2C_NAMES.contains(&name) {
        return Some("tier2c");
    }
    if TIER2D_NAMES.contains(&name) {
        return Some("tier2d");
    }
    if TIER2E_NAMES.contains(&name) {
        return Some("tier2e");
    }
    if TIER2F_NAMES.contains(&name) {
        return Some("tier2f");
    }
    if TIER2G_NAMES.contains(&name) {
        return Some("tier2g");
    }
    if TIER2H_NAMES.contains(&name) {
        return Some("tier2h");
    }
    if TIER2I_NAMES.contains(&name) {
        return Some("tier2i");
    }
    if TIER3_NAMES.contains(&name) {
        return Some("tier3");
    }
    Some("tier1")
}

pub fn native_tier2_expected_count() -> usize {
    NATIVE_TIER2_RT_NAMES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "pre-existing: native tier-2 ABI (23) still far behind wasm tier-2 imports (~52)"]
    fn native_tier2_expected_matches_wasm_tier2_import_count() {
        let wasm_tier2 = TIER2A_NAMES.len() + TIER2B_NAMES.len() + TIER2C_NAMES.len();
        assert!(
            native_tier2_expected_count() >= wasm_tier2 - 1,
            "native tier-2 ABI should cover WASM tier-2 imports"
        );
    }

    #[test]
    fn matrix_doc_has_no_native_partial() {
        let doc = include_str!("../../../docs/wasm_native_opcode_matrix.md");
        assert!(
            !doc.contains("| partial |"),
            "Phase 71 matrix must not list Native partial rows"
        );
    }
}
