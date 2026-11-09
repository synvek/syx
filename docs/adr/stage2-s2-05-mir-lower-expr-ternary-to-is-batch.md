# ADR: Stage-2 S2-05 — syx-mir `lower_expr` Ternary→Is 五刀批抽（S2-I184–I188）

**Status:** Accepted (S2-I184–I188) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I183 mir_lower_expr_compound_assign](stage2-s2-05-mir-lower-expr-compound-assign-extract.md), acceleration window lands **five** zero-semantics `lower_expr` arms in one construction window. It does **not** claim S2-05 `[x]`, peel Call shell, or exit Stage-2.

## Decision

1. Extract five arms into dedicated modules (zero semantics; crate-private):
   - **S2-I184** [`mir_lower_expr_ternary.rs`](../../crates/syx-mir/src/mir_lower_expr_ternary.rs) — `lower_expr_ternary`
   - **S2-I185** [`mir_lower_expr_tuple.rs`](../../crates/syx-mir/src/mir_lower_expr_tuple.rs) — `lower_expr_tuple`
   - **S2-I186** [`mir_lower_expr_cast.rs`](../../crates/syx-mir/src/mir_lower_expr_cast.rs) — `lower_expr_cast`
   - **S2-I187** [`mir_lower_expr_optional_cast.rs`](../../crates/syx-mir/src/mir_lower_expr_optional_cast.rs) — `lower_expr_optional_cast`
   - **S2-I188** [`mir_lower_expr_is.rs`](../../crates/syx-mir/src/mir_lower_expr_is.rs) — `lower_expr_is`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): five `mod` + delegate arms.
3. **S2-I186 raise:** `infer_expr_init_type`, `emit_integer_numeric_cast_if_needed`, `emit_float_to_int_numeric_cast_if_needed` → `pub(crate)` (remain in `lib.rs`).
4. **Ladder next:** **S2-I189** = **`Expr::Call` shell** peel — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ Call shell or whole `lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change arm semantics / ≠ open driver·semantic knives / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | five `mir_lower_expr_{ternary,tuple,cast,optional_cast,is}.rs` |
| Wiring | `lib.rs` five `mod` + delegates |
| Cast helpers | three `pub(crate)` in `lib.rs` |
| Shell | `pub(crate) fn lower_expr` remains in `lib.rs` |
| Contract | `s2_05_mir_lower_expr_ternary_to_is_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I184–I188 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I189–I194 landed:** [mir-lower-expr-call-to-macro-batch](stage2-s2-05-mir-lower-expr-call-to-macro-batch.md) — Call shell + Array/Await/Assign/TypeSelf/MacroInvoke left into modules; next default = **S2-I195 Range cluster**.
- Prior: [mir-lower-expr-compound-assign-extract](stage2-s2-05-mir-lower-expr-compound-assign-extract.md) · S2-I183.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-compound-assign-extract](stage2-s2-05-mir-lower-expr-compound-assign-extract.md) · S2-I183
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
