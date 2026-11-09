# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_unary` extract (S2-I179)

**Status:** Accepted (S2-I179) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I178 mir_lower_expr_binary](stage2-s2-05-mir-lower-expr-binary-extract.md), the locked next knife is **`Expr::Unary`**. This window moves that arm into a dedicated module and raises `unary_numeric_kind` to `pub(crate)`. Remaining arms (Closure, Call shell, Index, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_unary.rs`](../../crates/syx-mir/src/mir_lower_expr_unary.rs) holding **`pub(crate) fn lower_expr_unary`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_unary;` + delegate arm (zero semantics).
3. Raise **`unary_numeric_kind`** to **`pub(crate)`**.
4. **Ladder next:** **S2-I180** = **`Expr::Closure`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract Closure·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Unary semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_unary.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_unary` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_unary_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I179 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I180 landed:** [mir-lower-expr-closure-extract](stage2-s2-05-mir-lower-expr-closure-extract.md) — Closure left into module; next default = **S2-I181 Index**.
- Prior: [mir-lower-expr-binary-extract](stage2-s2-05-mir-lower-expr-binary-extract.md) · S2-I178.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-binary-extract](stage2-s2-05-mir-lower-expr-binary-extract.md) · S2-I178
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
