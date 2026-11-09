# ADR: Stage-2 S2-05 — syx-mir SIMD→support 四刀批抽（S2-I209–I212）

**Status:** Accepted (S2-I209–I212) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I205–I208 type_members→infer batch](stage2-s2-05-mir-type-members-to-infer-batch.md), the locked next knife is **SIMD helpers**. This window peels SIMD, `lower_function_with_params`, member-call dispatch, and lower support helpers. It does **not** claim S2-05 `[x]`, peel `lower_expr` shell / orchestrator, open driver·semantic knives, migrate `#[cfg(test)]`, or exit Stage-2.

## Decision

1. Extract four faces into dedicated modules (zero semantics):
   - **S2-I209** [`mir_simd_helpers.rs`](../../crates/syx-mir/src/mir_simd_helpers.rs) — `simd_*` / `infer_simd_*` / `host_simd_*` / `parse_simd_type` / `infer_expr_simd_type`
   - **S2-I210** [`mir_lower_function.rs`](../../crates/syx-mir/src/mir_lower_function.rs) — `lower_function_with_params` / `register_param_local_type`
   - **S2-I211** [`mir_member_call_dispatch.rs`](../../crates/syx-mir/src/mir_member_call_dispatch.rs) — `MemberCallKind` / `classify_member_call` / `method_id_mir_op` / `expr_as_i32_literal`
   - **S2-I212** [`mir_lower_support.rs`](../../crates/syx-mir/src/mir_lower_support.rs) — `flatten_blocks` / `resolve_jump_targets` / `mir_local_*_for_slots` / `expr_is_opaque_operand` / `keypath_from_callee`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): mods + `pub(crate) use` re-exports.
3. Census honesty floor for `syx-mir` `lib.rs`: **2900 → 2000**.
4. **Ladder next:** **S2-I213** = **`lower_expr` shell** — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ `lower_expr` shell·orchestrator 大拆 this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change lowering semantics / ≠ open driver·semantic knives / ≠ migrate `#[cfg(test)]` / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | four sidecars above |
| Wiring | `lib.rs` mods + `pub(crate) use` |
| Contract | `s2_05_mir_simd_to_support_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I209–I212 row · floor 2000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I213–I214 landed:** [mir-lower-expr-to-module-batch](stage2-s2-05-mir-lower-expr-to-module-batch.md) — lower_expr shell + orchestrator left into modules; mir production peel exhausted; next default = **S2-I215 syx-driver 首抽**.
- Prior: [mir-type-members-to-infer-batch](stage2-s2-05-mir-type-members-to-infer-batch.md) · S2-I205–I208.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-type-members-to-infer-batch](stage2-s2-05-mir-type-members-to-infer-batch.md) · S2-I205–I208
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
