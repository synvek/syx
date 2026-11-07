# ADR: Stage-2 S2-05 — syx-semantic production close-out batch（S2-I247–I248）

**Status:** Accepted (S2-I247–I248) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I246 semantic analyze_type_member extract](stage2-s2-05-semantic-analyze-type-member-extract.md), the locked next knife is **infer/host→concurrency**. This window batches **I247** (infer/host + concurrency) with **I248** (IDE surface) to clear all remaining `SemanticAnalyzer` **production** methods from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs). It does **not** claim S2-05 `[x]`, migrate `#[cfg(test)]` / free helpers, or exit Stage-2.

## Decision

1. Move remaining production `impl SemanticAnalyzer` methods (`infer_enum_case_type_args`…`members_for_type`) from `lib.rs` into `semantic_analyzer.rs`; delete the emptied `impl` shell from `lib.rs`.
   - **S2-I247** — infer/host + concurrency
   - **S2-I248** — IDE (`symbol_type`…`members_for_type`)
2. Raise module-level `attr_has_ident_arg` / `module_uses_task_detach` / `stmts_use_task_detach` / `expr_uses_task_detach` to `pub(crate)`.
3. Leave free helpers (`has_attr`, `expr_span`, …) and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **1800 → 500**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I249** = free-helpers peel or claim-prep — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ migrate tests·claim this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `check_concurrency_rules` / `members_for_type` |
| Contract | `s2_05_semantic_production_closeout_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I247–I248 row · semantic floor 500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Semantic **production** peel of `SemanticAnalyzer` inherent methods is exhausted.
- **S2-I249 landed:** [semantic-helpers-extract](stage2-s2-05-semantic-helpers-extract.md) — free helpers left into `semantic_helpers.rs`; next default = **S2-I250 S2-05 claim**.
- Prior: [semantic-analyze-type-member-extract](stage2-s2-05-semantic-analyze-type-member-extract.md) · S2-I246.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyze-type-member-extract](stage2-s2-05-semantic-analyze-type-member-extract.md) · S2-I246
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
