//! Slim runtime core: `Value`, numeric rules, protocols, and hasher — no VM interpreter.

mod bigint;
mod dict;
mod hasher;
mod managed_heap;
mod numeric;
mod protocols;
mod runtime_bridge;
mod simd;
mod value;

pub use bigint::{
    bigint_add, bigint_cmp, bigint_description, bigint_from_i128, bigint_from_string, bigint_mul,
    bigint_sub, BIGINT_TYPE,
};
pub use dict::{
    dict_build_from_pairs, dict_contains, dict_get, dict_insert, dict_keys, dict_len, dict_remove,
    dict_values,
};
pub use hasher::{
    combine_value, finalize_state, hash_kind_tag, hash_value, hasher_from_value, hasher_value,
    HasherState,
};
pub use managed_heap::{
    collect_global, compact_global, gc_churn_stdout_lines, gc_gen_compact_cycles,
    gc_gen_compact_record, global_heap, heap_root_bridge_attach_count, heap_root_bridge_enabled,
    heap_root_bridge_peek, heap_root_bridge_publish, heap_root_bridge_subscribe, heap_v10,
    heap_v11, heap_v11_bridge_prod_enabled, heap_v13_generational, heap_v9_generational,
    record_gc_churn_stdout, relocate_count, relocate_global, reset_gc_churn_stdout,
    reset_gc_gen_compact_cycles, reset_relocate_count, ManagedHeap,
};
pub use numeric::{
    display_numeric, i128_from_lo_hi, lo_hi_from_i128, lo_hi_from_u128, numeric_abs, numeric_add,
    numeric_bitand, numeric_bitnot, numeric_bitor, numeric_bitxor, numeric_cast, numeric_cmp,
    numeric_div, numeric_max, numeric_min, numeric_mod, numeric_mul, numeric_neg, numeric_shl,
    numeric_shr, numeric_sub, platform_int_value, try_numeric_cast, u128_from_lo_hi,
    value_from_numeric, value_from_numeric128, value_kind,
};
pub use protocols::{
    ensure_object_id, is_hasher, make_object_identifier, value_compare, value_compare_int,
    value_debug_description, value_description, value_equals, value_identical, value_object_id,
    OBJECT_ID_KEY,
};
pub use runtime_bridge::{RuntimeBridge, VmRuntime};
pub use simd::{
    simd_abs, simd_add_sat, simd_all_true, simd_and, simd_any_true, simd_avgr_u, simd_binop,
    simd_bitmask, simd_broadcast, simd_div, simd_eq, simd_equals, simd_extract_lane,
    simd_from_array, simd_ge, simd_gt, simd_le, simd_load_from_array, simd_lt, simd_max, simd_min,
    simd_ne, simd_neg, simd_not, simd_or, simd_popcnt, simd_reduce_add, simd_replace_lane,
    simd_shl, simd_shr, simd_shuffle_i8, simd_store_to_array, simd_sub_sat, simd_umax, simd_umin,
    simd_xor, SimdBinOp,
};
pub use value::{cooperative_yield, HeapSlotKind, IteratorState, Value};
