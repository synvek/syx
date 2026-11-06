# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_call_member` extract (S2-I174)

**Status:** Accepted (S2-I174) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I173 mir_lower_expr_call_ident](stage2-s2-05-mir-lower-expr-call-ident-extract.md), the locked next knife is the **Call Member** callee arm (symmetric to Ident). This window moves `match callee` → **`Expr::Member`** into a dedicated module. Call prelude, Ident delegate, and `_` fallback stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_call_member.rs`](../../crates/syx-mir/src/mir_lower_expr_call_member.rs) holding **`pub(crate) fn lower_expr_call_member`** (former Member-callee arm body).
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_call_member;` + `use` — Member arm becomes a one-call delegate (zero semantics).
3. Raise `MemberCallKind` and Member-path helpers (`resolve_host_call`, `lower_range_contains_mir`, `emit_hosted_collection_call`, …) to **`pub(crate)`** as needed; no new crate-root `pub use`.
4. Keep in `lib.rs`: `pub(crate) fn lower_expr` shell, Call prelude, Ident/`_` callee paths, and top-level `Expr::Member` / `MemberAssign` arms.
5. **Ladder next:** **S2-I175** = top-level **`Expr::Member` expression arm** in `lower_expr` — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract whole `lower_expr` or top-level Member/MemberAssign this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Call Member lowering semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_call_member.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_call_member` + Member delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_call_member_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I174 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I175 landed:** [mir-lower-expr-member-extract](stage2-s2-05-mir-lower-expr-member-extract.md) — top-level `Expr::Member` left into module; next default = **S2-I176 Expr::MemberAssign**.
- Prior: [mir-lower-expr-call-ident-extract](stage2-s2-05-mir-lower-expr-call-ident-extract.md) · S2-I173.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-call-ident-extract](stage2-s2-05-mir-lower-expr-call-ident-extract.md) · S2-I173
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
