# ADR: Stage-2 S2-05 — syx-semantic `analyze_expr` extract（S2-I244）

**Status:** Accepted (S2-I244) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I242–I243 analyze_decl→stmt batch](stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md), the locked next knife is **`analyze_expr`**. This window appends `analyze_expr` + pre-helpers + `analyze_expr_inner` into the growing [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) sidecar. It does **not** claim S2-05 `[x]`, peel `analyze_call`/pattern/concurrency/IDE, migrate tests, or exit Stage-2.

## Decision

1. Move `analyze_expr` + `record_expr_type`…`unify_element_types` + `analyze_expr_inner` from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (same `impl SemanticAnalyzer`).
2. Keep `analyze_expr` as `pub(crate)`. Raise cross-module callees still in `lib.rs` (`analyze_call`, `check_assignable`, `expr_type`, `is_compatible`, `promote_numeric*`, import/actor/extension/service helpers, etc.) and free helpers (`expr_span`, `retired_ops_hint`) to `pub(crate)` as needed.
3. Leave `analyze_call` / call-pre helpers / pattern / `analyze_expr_with_expected` / type_member / protocol / concurrency / IDE helpers and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **4800 → 3800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I245** = **`analyze_call`** face — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel analyze_call·pattern·concurrency·IDE this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `analyze_expr` / `analyze_expr_inner` |
| Contract | `s2_05_semantic_analyze_expr_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I244 row · semantic floor 3800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I245 landed:** [semantic-analyze-call-extract](stage2-s2-05-semantic-analyze-call-extract.md) — analyze_call cluster left into semantic_analyzer; next default = **S2-I246 analyze_type_member**.
- Prior: [semantic-analyze-decl-to-stmt-batch](stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md) · S2-I242–I243.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyze-decl-to-stmt-batch](stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md) · S2-I242–I243
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
