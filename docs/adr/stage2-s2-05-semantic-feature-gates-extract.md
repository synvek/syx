# ADR: Stage-2 S2-05 — syx-semantic `feature_gates` extract（S2-I238）

**Status:** Accepted (S2-I238) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I236–I237 semantic types→codec batch](stage2-s2-05-semantic-types-to-codec-batch.md), the locked next knife is **semantic feature_gates**. This window moves the contiguous gate / routing / prover face (~L78–839) into [`feature_gates.rs`](../../crates/syx-semantic/src/feature_gates.rs). The sidecar includes env toggles **and** routing scaffold / prover I/O / `reflect_type_lower` — zero behavior change, **≠** “pure env-only” cluster. It does **not** claim S2-05 `[x]`, open `SemanticAnalyzer` peels, migrate tests, or exit Stage-2.

## Decision

1. Create [`crates/syx-semantic/src/feature_gates.rs`](../../crates/syx-semantic/src/feature_gates.rs) holding the full continuous cluster: `global_actor_*`, `borrow_regions_*`, formal/prover/CI helpers, `reflect_type_lower`, `typealias_*`, `property_wrapper_*`, plus private `coq_/lean_*_smoke`.
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): `mod feature_gates;` + `pub use feature_gates::*;` so `syx_semantic::typealias_*` / `global_actor_*` paths stay stable.
3. Leave `SemanticAnalyzer`, thin helpers (L44–73), `expr_span*`, and `#[cfg(test)]` in `lib.rs`.
4. Census honesty floor for `syx-semantic` `lib.rs`: **8500 → 7700**. Driver floor stays **1100**; mir floor stays **2000**.
5. **Ladder next:** **S2-I239** = **SemanticAnalyzer 首面**（struct + analyze 入口 / collecting 首块）— still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel `SemanticAnalyzer` this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change gate behavior / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-semantic/src/feature_gates.rs` |
| Re-export | `lib.rs` `mod feature_gates` + `pub use feature_gates::*` |
| Contract | `s2_05_semantic_feature_gates_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I238 row · semantic floor 7700 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I239–I240 landed:** [semantic-analyzer-entry-to-builtins-batch](stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md) — SemanticAnalyzer entry/builtins left into module.
- **S2-I241 landed:** [semantic-collect-decls-extract](stage2-s2-05-semantic-collect-decls-extract.md) — next default = **S2-I242 analyze_decl**.
- Prior: [semantic-types-to-codec-batch](stage2-s2-05-semantic-types-to-codec-batch.md) · S2-I236–I237.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-types-to-codec-batch](stage2-s2-05-semantic-types-to-codec-batch.md) · S2-I236–I237
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
