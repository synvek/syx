# ADR: Stage-2 S2-05 — syx-semantic SemanticAnalyzer 入口→builtins 两刀批抽（S2-I239–I240）

**Status:** Accepted (S2-I239–I240) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I238 semantic feature_gates extract](stage2-s2-05-semantic-feature-gates-extract.md), the locked next knife is **SemanticAnalyzer 首面**. This window peels the analyzer struct, analyze/collecting entry orchestration, and builtins/support helpers into one growing sidecar. It does **not** claim S2-05 `[x]`, move `collect_decls` / expr faces, migrate tests, or exit Stage-2.

## Decision

1. Create [`crates/syx-semantic/src/semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs):
   - **S2-I239** — `SemanticAnalyzer` struct + `analyze*` / `analyze_collecting*` entry cluster
   - **S2-I240** — same file: `push_diagnostic` → `register_nested_type_path` (builtins / promote / assoc / member surface)
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): `mod semantic_analyzer;` + `pub use semantic_analyzer::SemanticAnalyzer`. Raise cross-module callees still in `lib.rs` to `pub(crate)` as needed (zero semantics).
3. Leave `collect_decls` / `collect_nested_type_decl`, `analyze_expr`/`analyze_stmt`, tail helpers, and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **7700 → 6800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I241** = **`collect_decls`** face — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel `collect_decls` or expr this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-semantic/src/semantic_analyzer.rs` |
| Re-export | `lib.rs` `mod semantic_analyzer` + `pub use …::SemanticAnalyzer` |
| Contract | `s2_05_semantic_analyzer_entry_to_builtins_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I239–I240 row · semantic floor 6800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I241 landed:** [semantic-collect-decls-extract](stage2-s2-05-semantic-collect-decls-extract.md) — collect_decls left into semantic_analyzer; next default = **S2-I242 analyze_decl**.
- Prior: [semantic-feature-gates-extract](stage2-s2-05-semantic-feature-gates-extract.md) · S2-I238.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-feature-gates-extract](stage2-s2-05-semantic-feature-gates-extract.md) · S2-I238
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
