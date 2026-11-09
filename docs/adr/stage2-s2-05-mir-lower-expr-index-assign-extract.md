# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_index_assign` extract (S2-I177)

**Status:** Accepted (S2-I177) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I176 mir_lower_expr_member_assign](stage2-s2-05-mir-lower-expr-member-assign-extract.md), the locked next knife is **`Expr::IndexAssign`**. This window moves that arm into a dedicated module. Remaining arms (Binary, Unary, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_index_assign.rs`](../../crates/syx-mir/src/mir_lower_expr_index_assign.rs) holding **`pub(crate) fn lower_expr_index_assign`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_index_assign;` + delegate arm (zero semantics).
3. Reuse already-`pub(crate)` helpers (`lower_expr`, `type_name_dispatch_base`); no new crate-root `pub use`.
4. **Ladder next:** **S2-I178** = **`Expr::Binary`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract Binary·Unary·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change IndexAssign semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_index_assign.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_index_assign` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_index_assign_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I177 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I178 landed:** [mir-lower-expr-binary-extract](stage2-s2-05-mir-lower-expr-binary-extract.md) — Binary left into module; next default = **S2-I179 Unary**.
- Prior: [mir-lower-expr-member-assign-extract](stage2-s2-05-mir-lower-expr-member-assign-extract.md) · S2-I176.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-member-assign-extract](stage2-s2-05-mir-lower-expr-member-assign-extract.md) · S2-I176
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
