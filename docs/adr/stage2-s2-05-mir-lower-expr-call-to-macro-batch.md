# ADR: Stage-2 S2-05 — syx-mir `lower_expr` Call→MacroInvoke 六刀批抽（S2-I189–I194）

**Status:** Accepted (S2-I189–I194) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I184–I188 Ternary→Is batch](stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md), the locked next knife is **`Expr::Call` shell**. This acceleration window lands Call shell plus five medium inline arms. It does **not** claim S2-05 `[x]`, peel Range helpers, or exit Stage-2.

## Decision

1. Extract six faces into dedicated modules (zero semantics; crate-private):
   - **S2-I189** [`mir_lower_expr_call.rs`](../../crates/syx-mir/src/mir_lower_expr_call.rs) — `lower_expr_call` (Call shell)
   - **S2-I190** [`mir_lower_expr_array.rs`](../../crates/syx-mir/src/mir_lower_expr_array.rs) — `lower_expr_array`
   - **S2-I191** [`mir_lower_expr_await.rs`](../../crates/syx-mir/src/mir_lower_expr_await.rs) — `lower_expr_await`
   - **S2-I192** [`mir_lower_expr_assign.rs`](../../crates/syx-mir/src/mir_lower_expr_assign.rs) — `lower_expr_assign`
   - **S2-I193** [`mir_lower_expr_type_self.rs`](../../crates/syx-mir/src/mir_lower_expr_type_self.rs) — `lower_expr_type_self`
   - **S2-I194** [`mir_lower_expr_macro_invoke.rs`](../../crates/syx-mir/src/mir_lower_expr_macro_invoke.rs) — `lower_expr_macro_invoke`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): six `mod` + delegate arms.
3. **Raises (remain in lib):**
   - Call: `classify_member_call`, `host_simd_lowers_own_args`, `keypath_from_callee` → `pub(crate)`
   - Assign: `emit_integer_narrow_cast_if_needed` → `pub(crate)`
4. Census honesty floor for `syx-mir` `lib.rs`: **6500 → 6000**.
5. **Ladder next:** **S2-I195** = **Range cluster** (`HalfOpen`/`Closed`/`RangeDyn` + `lower_range_dict*`) — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ Range helpers or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change arm semantics / ≠ open driver·semantic knives / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | six `mir_lower_expr_{call,array,await,assign,type_self,macro_invoke}.rs` |
| Wiring | `lib.rs` six `mod` + delegates |
| Helpers | Call 3 + Assign 1 `pub(crate)` in `lib.rs` |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_call_to_macro_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I189–I194 row · floor 6000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I195–I200 landed:** [mir-lower-expr-range-to-super-batch](stage2-s2-05-mir-lower-expr-range-to-super-batch.md) — Range family + Ident/Group/Borrow/KeyPath/Super left into modules; next default = **S2-I201 collect_closure_free_vars**.
- Prior: [mir-lower-expr-ternary-to-is-batch](stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md) · S2-I184–I188.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-ternary-to-is-batch](stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md) · S2-I184–I188
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
