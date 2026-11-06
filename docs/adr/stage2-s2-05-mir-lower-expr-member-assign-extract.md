# ADR: Stage-2 S2-05 — syx-mir `mir_lower_expr_member_assign` extract (S2-I176)

**Status:** Accepted (S2-I176) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I175 mir_lower_expr_member](stage2-s2-05-mir-lower-expr-member-extract.md), the locked next knife is **`Expr::MemberAssign`**. This window also **repaired** post-I175 `lib.rs` wiring after an accidental monolith overwrite, then moved the MemberAssign arm into a dedicated module. `IndexAssign` and remaining arms stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create/keep [`crates/syx-mir/src/mir_lower_expr_member_assign.rs`](../../crates/syx-mir/src/mir_lower_expr_member_assign.rs) holding **`pub(crate) fn lower_expr_member_assign`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_expr_member_assign;` + delegate arm (zero semantics).
3. Restore post-I175 module graph (types/codec/aot/opt/module_index/lower_ctx/literal/call_ident/call_member/expr_member) before/with this extract.
4. Raise `lower_stmt` to **`pub(crate)`** as needed for assign observers; no new crate-root `pub use`.
5. **Ladder next:** **S2-I177** = **`Expr::IndexAssign`** arm — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract IndexAssign or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change MemberAssign semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_expr_member_assign.rs` |
| Wiring | `lib.rs` `mod mir_lower_expr_member_assign` + delegate |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_member_assign_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I176 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I177 landed:** [mir-lower-expr-index-assign-extract](stage2-s2-05-mir-lower-expr-index-assign-extract.md) — IndexAssign left into module; next default = **S2-I178 Binary**.
- Prior: [mir-lower-expr-member-extract](stage2-s2-05-mir-lower-expr-member-extract.md) · S2-I175.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-member-extract](stage2-s2-05-mir-lower-expr-member-extract.md) · S2-I175
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
