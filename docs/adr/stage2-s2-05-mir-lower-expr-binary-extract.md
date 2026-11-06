# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_binary` extract (S2-I178)

**Status:** Accepted (S2-I178) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I177 mir_lower_expr_index_assign](stage2-s2-05-mir-lower-expr-index-assign-extract.md), the locked next knife is **`Expr::Binary`**. This window moves that arm into a dedicated module and raises five Binary-local helpers to `pub(crate)`. Remaining arms (Unary, Call shell, …) stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_binary.rs`](../../crates/syx-mir/src/mir_lower_expr_binary.rs) holding **`pub(crate) fn lower_expr_binary`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_binary;` + delegate arm (zero semantics).
3. Raise to **`pub(crate)`**: `expr_is_opaque_operand`, `infer_expr_simd_type`, `binary_numeric_kind`, `binop_operator_symbol`, `infer_expr_type_name`.
4. **Ladder next:** **S2-I179** = **`Expr::Unary`** arm — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract Unary·Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Binary semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_binary.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_binary` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_binary_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I178 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I179 landed:** [mir-lower-expr-unary-extract](stage2-s2-05-mir-lower-expr-unary-extract.md) — Unary left into module; next default = **S2-I180 Closure**.
- Prior: [mir-lower-expr-index-assign-extract](stage2-s2-05-mir-lower-expr-index-assign-extract.md) · S2-I177.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-index-assign-extract](stage2-s2-05-mir-lower-expr-index-assign-extract.md) · S2-I177
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
