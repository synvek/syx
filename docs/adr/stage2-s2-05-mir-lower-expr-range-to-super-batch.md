# ADR: Stage-2 S2-05 — syx-mir `lower_expr` Range→Super 六刀批抽（S2-I195–I200）

**Status:** Accepted (S2-I195–I200) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I189–I194 Call→Macro batch](stage2-s2-05-mir-lower-expr-call-to-macro-batch.md), the locked next knife is **Range cluster**. This window lands the full Range helper family plus five micro arms, clearing remaining inline `lower_expr` match bodies. It does **not** claim S2-05 `[x]`, peel `collect_closure_free_vars`, or exit Stage-2.

## Decision

1. Extract six faces into dedicated modules (zero semantics; crate-private):
   - **S2-I195** [`mir_lower_expr_range.rs`](../../crates/syx-mir/src/mir_lower_expr_range.rs) — `range_bounds_from_expr`, `lower_range_contains_mir`, `lower_range_dict`, `lower_range_dict_dyn`, plus arm wrappers `lower_expr_{half_open_range,closed_range,range_dyn}`
   - **S2-I196** [`mir_lower_expr_ident.rs`](../../crates/syx-mir/src/mir_lower_expr_ident.rs) — `lower_expr_ident`
   - **S2-I197** [`mir_lower_expr_group.rs`](../../crates/syx-mir/src/mir_lower_expr_group.rs) — `lower_expr_group`
   - **S2-I198** [`mir_lower_expr_borrow.rs`](../../crates/syx-mir/src/mir_lower_expr_borrow.rs) — `lower_expr_borrow`
   - **S2-I199** [`mir_lower_expr_keypath.rs`](../../crates/syx-mir/src/mir_lower_expr_keypath.rs) — `lower_expr_keypath`
   - **S2-I200** [`mir_lower_expr_super.rs`](../../crates/syx-mir/src/mir_lower_expr_super.rs) — `lower_expr_super`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): six `mod` + delegate arms; remove Range helpers from `lib`.
3. Update [`mir_lower_expr_call_member.rs`](../../crates/syx-mir/src/mir_lower_expr_call_member.rs) to import bounds/contains from `mir_lower_expr_range`.
4. **Ladder next:** **S2-I201** = **`collect_closure_free_vars`** — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ `collect_closure_free_vars` or stmt/switch this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change arm semantics / ≠ open driver·semantic knives / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | six `mir_lower_expr_{range,ident,group,borrow,keypath,super}.rs` |
| Wiring | `lib.rs` six `mod` + delegates |
| Range helpers | defined in `mir_lower_expr_range.rs` only |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_range_to_super_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I195–I200 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I201–I204 landed:** [mir-free-vars-to-shells-batch](stage2-s2-05-mir-free-vars-to-shells-batch.md) — free_vars/switch/stmt/shells left into modules; next default = **S2-I205 lower_type_members**.
- Prior: [mir-lower-expr-call-to-macro-batch](stage2-s2-05-mir-lower-expr-call-to-macro-batch.md) · S2-I189–I194.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-call-to-macro-batch](stage2-s2-05-mir-lower-expr-call-to-macro-batch.md) · S2-I189–I194
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
