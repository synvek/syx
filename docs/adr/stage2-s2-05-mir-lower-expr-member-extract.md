# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_member` extract (S2-I175)

**Status:** Accepted (S2-I175) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I174 mir_lower_expr_call_member](stage2-s2-05-mir-lower-expr-call-member-extract.md), the locked next knife is the **top-level `Expr::Member`** arm in `lower_expr` (field/property read — not the Call Member-callee path). This window moves that arm into a dedicated module. `MemberAssign` and other arms stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_expr_member.rs`](../../crates/syx-mir/src/mir_lower_expr_member.rs) holding **`pub(crate) fn lower_expr_member`** (former top-level Member arm body).
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_member;` + `use` — Member arm becomes a one-call delegate (zero semantics).
3. Raise helpers as needed (`extension_call_op`, `collection_method_mir_op`, `is_static_type_member_call`, …) to **`pub(crate)`**; no new crate-root `pub use`.
4. Keep in `lib.rs`: `pub(crate) fn lower_expr` shell, **`Expr::MemberAssign`**, and remaining expr arms.
5. **Ladder next:** **S2-I176** = **`Expr::MemberAssign`** arm — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract `MemberAssign` or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change Member read lowering semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_member.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_member` + Member delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_member_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I175 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I176 landed:** [mir-lower-expr-member-assign-extract](stage2-s2-05-mir-lower-expr-member-assign-extract.md) — MemberAssign left into module; next default = **S2-I177 IndexAssign**.
- Prior: [mir-lower-expr-call-member-extract](stage2-s2-05-mir-lower-expr-call-member-extract.md) · S2-I174.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-call-member-extract](stage2-s2-05-mir-lower-expr-call-member-extract.md) · S2-I174
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
