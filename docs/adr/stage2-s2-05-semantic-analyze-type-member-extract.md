# ADR: Stage-2 S2-05 — syx-semantic `analyze_type_member` extract（S2-I246）

**Status:** Accepted (S2-I246) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I245 semantic analyze_call extract](stage2-s2-05-semantic-analyze-call-extract.md), the locked next knife is **`analyze_type_member`**. This window appends `is_compatible` + `analyze_type_member` + protocol/extension satellites into the growing [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) sidecar. It does **not** claim S2-05 `[x]`, peel infer/host/concurrency/IDE, migrate tests, or exit Stage-2.

## Decision

1. Move `is_compatible` + `analyze_type_member` + satellites through `is_associated_type_constraint` from [`lib.rs`](../../crates/syx-semantic/src/lib.rs) into [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs) (same `impl SemanticAnalyzer`).
2. Keep moved `pub(crate)` entry points. Raise cross-module callees still in `lib.rs` only as `cargo check` requires.
3. Leave `infer_enum_case_type_args`…host/infer, concurrency, IDE helpers, module-level free helpers, and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **2800 → 1800**. Driver floor stays **1100**.
5. **Ladder next:** **S2-I247** = infer/host→concurrency cluster — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel concurrency·IDE·claim this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change analysis semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_analyzer.rs` holds `analyze_type_member` / `is_compatible` |
| Contract | `s2_05_semantic_analyze_type_member_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I246 row · semantic floor 1800 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I247–I248 landed:** [semantic-production-closeout-batch](stage2-s2-05-semantic-production-closeout-batch.md) — remaining production impl left into semantic_analyzer; next default = **S2-I249** helpers/claim-prep.
- Prior: [semantic-analyze-call-extract](stage2-s2-05-semantic-analyze-call-extract.md) · S2-I245.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyze-call-extract](stage2-s2-05-semantic-analyze-call-extract.md) · S2-I245
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
