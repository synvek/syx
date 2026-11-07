# ADR: Stage-2 S2-05 — syx-driver fingerprint→lsp 三刀批抽（S2-I215–I217）

**Status:** Accepted (S2-I215–I217) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I213–I214 mir lower_expr→orchestrator close-out](stage2-s2-05-mir-lower-expr-to-module-batch.md), MIR production peel is exhausted. The locked next knife is **syx-driver 首抽**. This window peels the largest clean driver faces: cache fingerprint kit, farm TCP/remote cache IO, and LSP IDE navigation. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I215** [`cache_fingerprint.rs`](../../crates/syx-driver/src/cache_fingerprint.rs) — `cache_version_tag` / `heap_v100_phase128_track_c_wasm_digest` / phase seeds / digests / `module_hash` cluster（跳过夹心 feature gates）
   - **S2-I216** [`farm_tcp.rs`](../../crates/syx-driver/src/farm_tcp.rs) — `default_farm_cache_url` / farm TCP codec·handle / remote fetch·push / purge·incremental path
   - **S2-I217** [`lsp_ide.rs`](../../crates/syx-driver/src/lsp_ide.rs) — diagnostics / analyze / definition·references·workspace / import suggestions（`lsp_completion` 旁路不动）
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + private `use` for fingerprint helpers + `pub use` for farm/LSP public APIs.
3. Census honesty floor for `syx-driver` `lib.rs`: **8000 → 5500**.
4. **Ladder next:** **S2-I218** = **`mir_unify`** (~driver MIR merge/unify face) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change fingerprint·cache·LSP semantics / ≠ migrate `#[cfg(test)]` / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `use` / `pub use` |
| Contract | `s2_05_driver_fingerprint_to_lsp_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I215–I217 row · driver floor 5500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I218–I220 landed:** [driver-mir-unify-to-compile-batch](stage2-s2-05-driver-mir-unify-to-compile-batch.md) — mir_unify / test_runner / compile_linked left into modules; next default = **S2-I221 `compile_file_with_context`**.
- Prior: [mir-lower-expr-to-module-batch](stage2-s2-05-mir-lower-expr-to-module-batch.md) · S2-I213–I214.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-lower-expr-to-module-batch](stage2-s2-05-mir-lower-expr-to-module-batch.md) · S2-I213–I214
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
