# ADR: Stage-2 S2-05 — syx-mir lower_expr→orchestrator 收口批抽（S2-I213–I214）

**Status:** Accepted (S2-I213–I214) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I209–I212 SIMD→support batch](stage2-s2-05-mir-simd-to-support-batch.md), the locked next knife is **`lower_expr` shell**. This window peels the remaining production faces (`lower_expr` dispatch + `lower_module*` orchestrator) and clears orphan docs. **MIR production peel is exhausted** (lib residual ≈ preamble + `#[cfg(test)]`). It does **not** claim S2-05 `[x]`, migrate tests, open driver·semantic knives this window, or exit Stage-2.

## Decision

1. Extract two faces into dedicated modules (zero semantics):
   - **S2-I213** [`mir_lower_expr.rs`](../../crates/syx-mir/src/mir_lower_expr.rs) — `lower_expr` match shell
   - **S2-I214** [`mir_lower_module.rs`](../../crates/syx-mir/src/mir_lower_module.rs) — `extension_methods_for_module` + three `lower_module*` entrypoints; delete orphan parked-doc stubs in `lib.rs`
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): mods + `pub(crate) use lower_expr` + **`pub use`** public orchestrator APIs.
3. Census honesty floor for `syx-mir` `lib.rs`: **keep 2000** (tests still dominate LOC).
4. **Ladder next:** **S2-I215** = **syx-driver 首抽** (largest remaining S2-05 face) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ open driver·semantic this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change lowering semantics / ≠ migrate `#[cfg(test)]` / ≠ claim exit after batch / ≠ floor→1500.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | two sidecars above |
| Wiring | `lib.rs` mods + `pub(crate) use` / `pub use` |
| Contract | `s2_05_mir_lower_expr_to_module_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I213–I214 row · floor 2000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **MIR production peel exhausted** (≠ S2-05 complete; driver/semantic/tests remain).
- **S2-I215–I217 landed:** [driver-fingerprint-to-lsp-batch](stage2-s2-05-driver-fingerprint-to-lsp-batch.md) — fingerprint/farm_tcp/lsp_ide left into modules; next default = **S2-I218 `mir_unify`**.
- Prior: [mir-simd-to-support-batch](stage2-s2-05-mir-simd-to-support-batch.md) · S2-I209–I212.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-simd-to-support-batch](stage2-s2-05-mir-simd-to-support-batch.md) · S2-I209–I212
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
