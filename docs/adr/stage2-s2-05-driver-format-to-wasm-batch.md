# ADR: Stage-2 S2-05 — syx-driver format→cache→wasm 三刀批抽（S2-I227–I229）

**Status:** Accepted (S2-I227–I229) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I224–I226 driver options→check→run batch](stage2-s2-05-driver-options-to-run-batch.md), the locked next knife is **`format_lint`**. This window peels format/lint, incremental compile cache (`CachedModuleUnit`), and WASM/native-MIR run helpers. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, claim driver production peel exhausted, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I227** [`format_lint.rs`](../../crates/syx-driver/src/format_lint.rs) — `format_project` / `FormatCheckReport` / `collect_sy_files` / `format_check_*` / `lint_*`（不含 `doc_project`）
   - **S2-I228** [`compile_cache.rs`](../../crates/syx-driver/src/compile_cache.rs) — `compile_with_cache` / `CachedModuleUnit` / `bincode_*` / `cache_entry_fresh`
   - **S2-I229** [`run_wasm.rs`](../../crates/syx-driver/src/run_wasm.rs) — `run_wasm_*` / `run_native_mir_file_printed`
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use`.
3. Census honesty floor for `syx-driver` `lib.rs`: **2000 → 1500**.
4. **Ladder next:** **S2-I230** = **`compile_to_program`** (~VM codegen face) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change format·cache·wasm semantics / ≠ migrate `#[cfg(test)]` / ≠ claim driver production peel exhausted / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_format_to_wasm_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I227–I229 row · driver floor 1500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I230–I232 landed:** [driver-compile-to-debug-batch](stage2-s2-05-driver-compile-to-debug-batch.md) — compile_to_program / feature_gates / debug_repl left into modules; next default = **S2-I233 production close-out**.
- Prior: [driver-options-to-run-batch](stage2-s2-05-driver-options-to-run-batch.md) · S2-I224–I226.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-options-to-run-batch](stage2-s2-05-driver-options-to-run-batch.md) · S2-I224–I226
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
