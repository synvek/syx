# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_task_block` extract (S2-I182)

**Status:** Accepted (S2-I182) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I181 mir_lower_expr_index](stage2-s2-05-mir-lower-expr-index-extract.md), the locked next knife is **`Expr::TaskBlock`**. This window moves that arm into a dedicated module. Remaining arms (CompoundAssign, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_task_block.rs`](../../crates/syx-mir/src/mir_lower_expr_task_block.rs) holding **`pub(crate) fn lower_expr_task_block`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_task_block;` + delegate arm (zero semantics).
3. Reuse already-`pub(crate)` `lower_function_with_params`; no new raises.
4. **Ladder next:** **S2-I183** = **`Expr::CompoundAssign`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract CompoundAssign·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change TaskBlock semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_task_block.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_task_block` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_task_block_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I182 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I183 landed:** [mir-lower-expr-compound-assign-extract](stage2-s2-05-mir-lower-expr-compound-assign-extract.md) — CompoundAssign left into module; next default = **S2-I184 Ternary**.
- Prior: [mir-lower-expr-index-extract](stage2-s2-05-mir-lower-expr-index-extract.md) · S2-I181.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-index-extract](stage2-s2-05-mir-lower-expr-index-extract.md) · S2-I181
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
