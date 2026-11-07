# ADR: Stage-2 S2-05 — syx-semantic free-helpers extract（S2-I249）

**Status:** Accepted (S2-I249) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I247–I248 semantic production close-out](stage2-s2-05-semantic-production-closeout-batch.md), the locked next knife is **helpers/claim-prep**. This window peels free helpers into [`semantic_helpers.rs`](../../crates/syx-semantic/src/semantic_helpers.rs). It does **not** claim S2-05 `[x]`, migrate `#[cfg(test)]`, or exit Stage-2.

## Decision

1. Move free helpers (`as_optional_surface` / `CHANNEL_SELECT_SURFACE` / `is_forbidden_internal_import`…`expr_span` / `expr_span_start`) into `semantic_helpers.rs`.
2. Keep crate hub in [`lib.rs`](../../crates/syx-semantic/src/lib.rs): `mod` + `pub use` / `pub(crate) use` reexports + `MAX_COLLECTED_DIAGNOSTICS` + `#[cfg(test)]`.
3. Census honesty floor for `syx-semantic` `lib.rs`: **500 → 300**. Driver floor stays **1100**. Sidecar count **7 → 8**.
4. **Ladder next:** **S2-I250** = dedicated **S2-05 `[x]` claim** (scoped: three-crate production peel exhausted; tests-in-lib OK) — still ≠ Stage-2 exit.
5. **Non-claims (hard):** ≠ S2-05 `[x]` this window / ≠ Stage-2 exit / ≠ migrate `#[cfg(test)]` / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change helper semantics / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `semantic_helpers.rs` holds `has_attr` / `expr_span` |
| Contract | `s2_05_semantic_helpers_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I249 row · semantic floor 300 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Decision §4 locked next = **S2-I250 S2-05 claim**; **superseded** by [semantic-analyzer-core-faces-batch](stage2-s2-05-semantic-analyzer-core-faces-batch.md)（I250–I252 analyzer peel；claim deferred）.
- Residual in semantic `lib.rs`: hub + unit tests only; giant face relocated to `semantic_analyzer.rs`.
- Prior: [semantic-production-closeout-batch](stage2-s2-05-semantic-production-closeout-batch.md) · S2-I247–I248.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-production-closeout-batch](stage2-s2-05-semantic-production-closeout-batch.md) · S2-I247–I248
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
