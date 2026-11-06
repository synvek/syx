# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_call_ident` extract (S2-I173)

**Status:** Accepted (S2-I173) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I172 mir_lower_expr_literal](stage2-s2-05-mir-lower-expr-literal-extract.md), the locked next knife is a **bounded Call sub-face** (not the full ~1776-line `Expr::Call` arm). This window moves the `match callee` **`Expr::Ident`** path into a dedicated module. Call prelude (early args / `unbox_targets`) and `Member` / `_` callee arms stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_call_ident.rs`](../../crates/syx-mir/src/mir_lower_expr_call_ident.rs) holding **`pub(crate) fn lower_expr_call_ident`** (former Ident-callee arm body).
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_call_ident;` + `use` — Ident arm becomes a one-call delegate (zero semantics). Prelude still computes `lower_args_early` / `unbox_targets` and passes them in.
3. Raise Call-Ident helpers needed across the module boundary to **`pub(crate)`** in `lib.rs` (`expr_as_i32_literal`, `infer_*`, `simd_reduce_add_shape`, …) without new crate-root `pub use`.
4. Keep in `lib.rs`: `pub(crate) fn lower_expr` shell, Call prelude, Member/`_` callee arms, and all other expr arms.
5. **Ladder next:** **S2-I174** = **Call Member 子面** (`match callee` → `Expr::Member`) — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract whole `Expr::Call` or Member/MemberAssign large arms this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Call Ident lowering semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_call_ident.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_call_ident` + Ident delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_call_ident_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I173 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I174 landed:** [mir-lower-expr-call-member-extract](stage2-s2-05-mir-lower-expr-call-member-extract.md) — Call Member-callee left into module; next default = **S2-I175 Expr::Member arm**.
- Prior: [mir-lower-expr-literal-extract](stage2-s2-05-mir-lower-expr-literal-extract.md) · S2-I172.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-literal-extract](stage2-s2-05-mir-lower-expr-literal-extract.md) · S2-I172
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
