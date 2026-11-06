# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_literal` extract (S2-I172)

**Status:** Accepted (S2-I172) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I171 mir_lower_ctx](stage2-s2-05-mir-lower-ctx-extract.md), the locked next knife is **`lower_expr` first extract** — one bounded sub-face, not the full ~3659-line blob. This window moves the **`Expr::Literal`** arm plus its private helper `contextual_const_bits` into a dedicated module. The `lower_expr` shell and all other arms stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_literal.rs`](../../crates/syx-mir/src/mir_lower_expr_literal.rs) holding: `contextual_const_bits` + **`pub(crate) fn lower_expr_literal`** (former `Expr::Literal` match body).
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_literal;` + `use mir_lower_expr_literal::lower_expr_literal;` — `Expr::Literal { .. }` arm becomes a one-line delegate (zero semantics).
3. Keep in `lib.rs`: `pub(crate) fn lower_expr` shell and every non-Literal arm (Call / Member / …).
4. No new public crate-root API.
5. **Ladder next:** **S2-I173** = continue `lower_expr` large-arm extract — default prefer a **bounded Call sub-face** (not the whole ~1776 Call arm) if a clear cut exists; else a single **Member** arm — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract whole `lower_expr` or Call/Member large arms this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Literal lowering semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_literal.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_literal` + delegate arm |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_literal_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I172 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I173 landed:** [mir-lower-expr-call-ident-extract](stage2-s2-05-mir-lower-expr-call-ident-extract.md) — Call Ident-callee left `lib.rs`; next default = **S2-I174 Call Member sub-face**.
- Prior: [mir-lower-ctx-extract](stage2-s2-05-mir-lower-ctx-extract.md) · S2-I171.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-ctx-extract](stage2-s2-05-mir-lower-ctx-extract.md) · S2-I171
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
