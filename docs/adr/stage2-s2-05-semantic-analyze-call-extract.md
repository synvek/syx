# ADR: Stage-2 S2-05 — syx-semantic `analyze_call` extract（S2-I245）

**Status:** Accepted (S2-I245) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I244 semantic analyze_expr extract](stage2-s2-05-semantic-analyze-expr-extract.md), the locked next knife is **`analyze_call`**. This window appends call-pre helpers + `analyze_call` + pattern/assignable + `analyze_expr_with_expected` into the growing [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) sidecar. It does **not** claim S2-05 `[x]`, peel type_member/concurrency/IDE, migrate tests, or exit Stage-2.

## Decision

1. Move call-pre (`check_import_member_access`…`check_call_argument_labels`) + `analyze_call` + pattern/assignable + `analyze_expr_with_expected` from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (same `impl SemanticAnalyzer`).
2. Keep `analyze_call` / `analyze_expr_with_expected` as `pub(crate)`. Raise cross-module callees still in `lib.rs` (generic/reflect/service/host/infer/sendable helpers) and module-level `type_collection_surface` / `is_retired_collection_methodcall` to `pub(crate)` as needed; raise moved `type_satisfies_as_protocol_bound` / `is_subclass_of` for lib `is_compatible`.
3. Leave `is_compatible` / `analyze_type_member` / protocol / extension / concurrency / IDE helpers and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **3800 → 2800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I246** = **`analyze_type_member`** face — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel type_member·concurrency·IDE this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `analyze_call` / `analyze_expr_with_expected` |
| Contract | `s2_05_semantic_analyze_call_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I245 row · semantic floor 2800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I246 landed:** [semantic-analyze-type-member-extract](stage2-s2-05-semantic-analyze-type-member-extract.md) — type_member cluster left into semantic_analyzer; next default = **S2-I247** infer/host→concurrency.
- Prior: [semantic-analyze-expr-extract](stage2-s2-05-semantic-analyze-expr-extract.md) · S2-I244.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyze-expr-extract](stage2-s2-05-semantic-analyze-expr-extract.md) · S2-I244
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
