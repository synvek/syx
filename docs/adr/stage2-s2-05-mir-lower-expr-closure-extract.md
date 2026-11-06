# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_closure` extract (S2-I180)

**Status:** Accepted (S2-I180) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I179 mir_lower_expr_unary](stage2-s2-05-mir-lower-expr-unary-extract.md), the locked next knife is **`Expr::Closure`**. This window moves that arm into a dedicated module and raises `collect_closure_free_vars` / `lower_function_with_params` to `pub(crate)` (helpers stay in `lib.rs`). Remaining arms (Index, TaskBlock, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_closure.rs`](../../crates/syx-mir/src/mir_lower_expr_closure.rs) holding **`pub(crate) fn lower_expr_closure`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_closure;` + delegate arm (zero semantics).
3. Raise to **`pub(crate)`** (remain in `lib.rs`): `collect_closure_free_vars`, `lower_function_with_params`.
4. **Ladder next:** **S2-I181** = **`Expr::Index`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract Index·TaskBlock·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Closure semantics / ≠ move `lower_function_with_params` out of `lib.rs`.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_closure.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_closure` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_closure_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I180 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I181 landed:** [mir-lower-expr-index-extract](stage2-s2-05-mir-lower-expr-index-extract.md) — Index left into module; next default = **S2-I182 TaskBlock**.
- Prior: [mir-lower-expr-unary-extract](stage2-s2-05-mir-lower-expr-unary-extract.md) · S2-I179.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-unary-extract](stage2-s2-05-mir-lower-expr-unary-extract.md) · S2-I179
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
