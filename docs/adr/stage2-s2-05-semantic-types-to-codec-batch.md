# ADR: Stage-2 S2-05 — syx-semantic types→codec 两刀首开批（S2-I236–I237）

**Status:** Accepted (S2-I236–I237) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I233–I235 driver production close-out](stage2-s2-05-driver-production-closeout-batch.md), the locked next knife is **syx-semantic 首抽**. This window peels the IR data face (`TypeId`/`Symbol`/`Scope`) and the TypeId↔name/TypeExpr codec, aligning with mir types→codec but batched in one window. It does **not** claim S2-05 `[x]`, open feature gates / analyzer peels, migrate tests, or exit Stage-2.

## Decision

1. Extract two faces into dedicated modules (zero semantics):
   - **S2-I236** [`semantic_types.rs`](../../crates/syx-semantic/src/semantic_types.rs) — `TypeId` / `Symbol` / `Scope`
   - **S2-I237** [`semantic_type_codec.rs`](../../crates/syx-semantic/src/semantic_type_codec.rs) — `type_name_to_id` / `type_expr_to_id*` / `type_id_to_name` / collection·normalize helpers / `simd_lanes_from_type_name`
2. Wire [`lib.rs`](../../crates/syx-semantic/src/lib.rs): mods + `pub use` (crate-root API unchanged).
3. Census honesty floor for `syx-semantic` `lib.rs`: **新建 8500**（open 8889 → landed ~8555）. Driver floor stays **1100**; mir floor stays **2000**.
4. **Ladder next:** **S2-I238** = **semantic feature_gates**（~env 门控簇 L199–960）— still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ peel feature_gates or `SemanticAnalyzer` this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change TypeId·codec semantics / ≠ migrate `#[cfg(test)]` / ≠ touch driver·mir peels / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | two sidecars above |
| Wiring | `lib.rs` mods + `pub use` |
| Contract | `s2_05_semantic_types_to_codec_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I236–I237 row · semantic floor 8500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I238 landed:** [semantic-feature-gates-extract](stage2-s2-05-semantic-feature-gates-extract.md) — feature_gates left into module; next default = **S2-I239 SemanticAnalyzer 首面**.
- Prior: [driver-production-closeout-batch](stage2-s2-05-driver-production-closeout-batch.md) · S2-I233–I235.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-types-extract](stage2-s2-05-mir-types-extract.md) · S2-I166（types 先例）
- [stage2-s2-05-driver-production-closeout-batch](stage2-s2-05-driver-production-closeout-batch.md) · S2-I233–I235
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
