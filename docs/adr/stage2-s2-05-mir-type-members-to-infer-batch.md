# ADR: Stage-2 S2-05 — syx-mir type_members→infer 四刀批抽（S2-I205–I208）

**Status:** Accepted (S2-I205–I208) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I201–I204 free_vars→shells batch](stage2-s2-05-mir-free-vars-to-shells-batch.md), the locked next knife is **`lower_type_members`**. This window peels type_members, class_init/vtable, host/cast helpers, and type-infer helpers. It does **not** claim S2-05 `[x]`, peel SIMD helpers / `lower_function_with_params`, open driver·semantic knives, or exit Stage-2.

## Decision

1. Extract four faces into dedicated modules (zero semantics):
   - **S2-I205** [`mir_lower_type_members.rs`](../../crates/syx-mir/src/mir_lower_type_members.rs) — `merged_class_fields` / `lower_type_members` / protocol stubs / `self_param*` / convenience init shell
   - **S2-I206** [`mir_lower_class_init.rs`](../../crates/syx-mir/src/mir_lower_class_init.rs) — `lower_class_init` / witnesses / vtable slots / mut-ref wrap / `mir_instruction_count`
   - **S2-I207** [`mir_host_cast_helpers.rs`](../../crates/syx-mir/src/mir_host_cast_helpers.rs) — `resolve_host_call` / collection host emit / integer·float numeric cast helpers
   - **S2-I208** [`mir_type_infer_helpers.rs`](../../crates/syx-mir/src/mir_type_infer_helpers.rs) — `infer_expr_init_type` / `expr_is_*` / object·generic infer / `extension_or_hosted_return_type` / `type_name_is_*` (cycle-break co-location with infer)
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): mods + `pub(crate) use` re-exports; raise `lower_class_init` / `register_param_local_type` / helpers as needed.
3. Census honesty floor for `syx-mir` `lib.rs`: **4000 → 2900** (plan mid-estimate targeted 3000 / lib≈3100–3250; landed **2960**, so floor tracks actual peel while S2-05 stays `[~]`).
4. **Ladder next:** **S2-I209** = **SIMD helpers** — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ SIMD·`lower_function_with_params`·orchestrator 大拆 this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change lowering semantics / ≠ open driver·semantic knives / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | four sidecars above |
| Wiring | `lib.rs` mods + `pub(crate) use` for host/infer helpers |
| Contract | `s2_05_mir_type_members_to_infer_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I205–I208 row · floor 2900 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I209–I212 landed:** [mir-simd-to-support-batch](stage2-s2-05-mir-simd-to-support-batch.md) — SIMD/lower_fn/member-dispatch/support left into modules; next default = **S2-I213 `lower_expr` shell**.
- Prior: [mir-free-vars-to-shells-batch](stage2-s2-05-mir-free-vars-to-shells-batch.md) · S2-I201–I204.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-free-vars-to-shells-batch](stage2-s2-05-mir-free-vars-to-shells-batch.md) · S2-I201–I204
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
