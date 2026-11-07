# ADR: Stage-2 S2-05 — syx-semantic analyze_decl→stmt batch（S2-I242–I243）

**Status:** Accepted (S2-I242–I243) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I241 semantic collect_decls extract](stage2-s2-05-semantic-collect-decls-extract.md), the locked next knife is **`analyze_decl`**. This window batches **I242 analyze_decl** (+ satellites) with **I243 analyze_stmt** into the growing [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) sidecar. It does **not** claim S2-05 `[x]`, peel `analyze_expr`/inner/helpers, migrate tests, or exit Stage-2.

## Decision

1. Move Pass-2 decl→stmt cluster from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (same `impl SemanticAnalyzer`):
   - **S2-I242** — `analyze_decls` + `analyze_decl` + decl satellites (`resolve_type_expr` / `analyze_block` / `validate_*` / subscript helpers)
   - **S2-I243** — same file: `analyze_stmt`
2. Keep `analyze_decls` as `pub(crate)`. Raise cross-module callees still in `lib.rs` (`analyze_expr`, `analyze_expr_with_expected`, `check_assignable_with_literal`, `analyze_type_member`, `analyze_extension_method`, lifetime / protocol / pattern helpers, etc.) and moved methods still called from `lib` (`analyze_block`, `analyze_decl`, `validate_*`, subscript helpers) to `pub(crate)` as needed.
3. Leave `analyze_expr` / `analyze_expr_inner` / call·pattern·concurrency·IDE helpers and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **5800 → 4800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I244** = **`analyze_expr`** face — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel analyze_expr·inner·尾 helper this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `analyze_decls` / `analyze_decl` / `analyze_stmt` |
| Contract | `s2_05_semantic_analyze_decl_to_stmt_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I242–I243 row · semantic floor 4800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I244 landed:** [semantic-analyze-expr-extract](stage2-s2-05-semantic-analyze-expr-extract.md) — analyze_expr/inner left into semantic_analyzer; next default = **S2-I245 analyze_call**.
- Prior: [semantic-collect-decls-extract](stage2-s2-05-semantic-collect-decls-extract.md) · S2-I241.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-collect-decls-extract](stage2-s2-05-semantic-collect-decls-extract.md) · S2-I241
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
