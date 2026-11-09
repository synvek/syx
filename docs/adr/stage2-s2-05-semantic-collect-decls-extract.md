# ADR: Stage-2 S2-05 — syx-semantic `collect_decls` extract（S2-I241）

**Status:** Accepted (S2-I241) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I239–I240 SemanticAnalyzer entry→builtins batch](stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md), the locked next knife is **`collect_decls`**. This window appends `collect_nested_type_decl` + `collect_decls` into the growing [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) sidecar. It does **not** claim S2-05 `[x]`, peel `analyze_decl`/stmt/expr, migrate tests, or exit Stage-2.

## Decision

1. Move `collect_nested_type_decl` + `collect_decls` from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (same `impl SemanticAnalyzer`).
2. Keep `collect_decls` as `pub(crate)`. Raise cross-module callees still in `lib.rs` (`error_at`, `check_prelude_name_shadow`, `collect_function_isolation`, `resolve_type_expr`, `is_forbidden_internal_import`) to `pub(crate)` as needed.
3. Leave `analyze_decls` / `analyze_decl` / `analyze_stmt` / `analyze_expr`, tail helpers, and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **6800 → 5800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I242** = **`analyze_decl`** face — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel analyze_decl/stmt/expr this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `collect_decls` |
| Contract | `s2_05_semantic_collect_decls_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I241 row · semantic floor 5800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I242–I243 landed:** [semantic-analyze-decl-to-stmt-batch](stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md) — analyze_decl→stmt left into semantic_analyzer; next default = **S2-I244 analyze_expr**.
- Prior: [semantic-analyzer-entry-to-builtins-batch](stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md) · S2-I239–I240.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyzer-entry-to-builtins-batch](stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md) · S2-I239–I240
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
