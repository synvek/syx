# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_compound_assign` extract (S2-I183)

**Status:** Accepted (S2-I183) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I182 mir_lower_expr_task_block](stage2-s2-05-mir-lower-expr-task-block-extract.md), the locked next knife is **`Expr::CompoundAssign`**. This window moves that arm into a dedicated module. Remaining arms (Ternary, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_compound_assign.rs`](../../crates/syx-mir/src/mir_lower_expr_compound_assign.rs) holding **`pub(crate) fn lower_expr_compound_assign`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_compound_assign;` + delegate arm (zero semantics).
3. Reuse already-available helpers (`lower_expr`, `host_fn_index_by_name`, `LowerCtx` load/store); no new raises.
4. **Ladder next:** **S2-I184** = **`Expr::Ternary`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract Ternary·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change CompoundAssign semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_compound_assign.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_compound_assign` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_compound_assign_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I183 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I184–I188 landed:** [mir-lower-expr-ternary-to-is-batch](stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md) — Ternary/Tuple/Cast/OptionalCast/Is left into modules; next default = **S2-I189 Call shell**.
- Prior: [mir-lower-expr-task-block-extract](stage2-s2-05-mir-lower-expr-task-block-extract.md) · S2-I182.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-task-block-extract](stage2-s2-05-mir-lower-expr-task-block-extract.md) · S2-I182
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
