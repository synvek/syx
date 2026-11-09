# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_index` extract (S2-I181)

**Status:** Accepted (S2-I181) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I180 mir_lower_expr_closure](stage2-s2-05-mir-lower-expr-closure-extract.md), the locked next knife is **`Expr::Index`** (read path — not IndexAssign, already extracted in I177). This window moves that arm into a dedicated module. Remaining arms (TaskBlock, CompoundAssign, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_index.rs`](../../crates/syx-mir/src/mir_lower_expr_index.rs) holding **`pub(crate) fn lower_expr_index`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_index;` + delegate arm (zero semantics).
3. Reuse already-`pub(crate)` helpers (`lower_expr`, `expr_is_string_like`, `expr_is_dict_like`, `type_name_dispatch_base`); no new raises.
4. **Ladder next:** **S2-I182** = **`Expr::TaskBlock`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract TaskBlock·CompoundAssign·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Index semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_index.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_index` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_index_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I181 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I182 landed:** [mir-lower-expr-task-block-extract](stage2-s2-05-mir-lower-expr-task-block-extract.md) — TaskBlock left into module; next default = **S2-I183 CompoundAssign**.
- Prior: [mir-lower-expr-closure-extract](stage2-s2-05-mir-lower-expr-closure-extract.md) · S2-I180.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-closure-extract](stage2-s2-05-mir-lower-expr-closure-extract.md) · S2-I180
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
