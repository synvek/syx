# ADR: Stage-2 S2-05 — syx-mir free_vars→shells 四刀批抽（S2-I201–I204）

**Status:** Accepted (S2-I201–I204) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I195–I200 Range→Super batch](stage2-s2-05-mir-lower-expr-range-to-super-batch.md), the locked next knife is **`collect_closure_free_vars`**. This window peels free_vars plus the dependent switch→stmt chain and shells face. It does **not** claim S2-05 `[x]`, peel `lower_type_members`, or exit Stage-2.

## Decision

1. Extract four faces into dedicated modules (zero semantics):
   - **S2-I201** [`mir_collect_closure_free_vars.rs`](../../crates/syx-mir/src/mir_collect_closure_free_vars.rs) — `collect_closure_free_vars`
   - **S2-I202** [`mir_lower_switch_pattern.rs`](../../crates/syx-mir/src/mir_lower_switch_pattern.rs) — `lower_switch_pattern_match` + `bind_*`
   - **S2-I203** [`mir_lower_stmt.rs`](../../crates/syx-mir/src/mir_lower_stmt.rs) — `lower_stmt` / `stmt_span_tuple` / `lower_stmt_inner`
   - **S2-I204** [`mir_lower_shells.rs`](../../crates/syx-mir/src/mir_lower_shells.rs) — `mir_shell_fn` + dataclass/final/enum/varglobal shells + `merge_mir_functions` / `prefix_user_dep_mir_batch`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): mods + `use` / `pub use` for public shell APIs; raise helpers as needed (`type_name_is_set`, `extension_or_hosted_return_type`, `mir_local_*_for_slots`).
3. Census honesty floor for `syx-mir` `lib.rs`: **6000 → 4000**.
4. **Ladder next:** **S2-I205** = **`lower_type_members`** — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ type_members·class_init·type-infer this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change lowering semantics / ≠ open driver·semantic knives / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | four sidecars above |
| Wiring | `lib.rs` mods + shell `pub use` |
| Contract | `s2_05_mir_free_vars_to_shells_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I201–I204 row · floor 4000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I205–I208 landed:** [mir-type-members-to-infer-batch](stage2-s2-05-mir-type-members-to-infer-batch.md) — type_members/class_init/host-cast/type-infer left into modules; next default = **S2-I209 SIMD helpers**.
- Prior: [mir-lower-expr-range-to-super-batch](stage2-s2-05-mir-lower-expr-range-to-super-batch.md) · S2-I195–I200.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-range-to-super-batch](stage2-s2-05-mir-lower-expr-range-to-super-batch.md) · S2-I195–I200
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
