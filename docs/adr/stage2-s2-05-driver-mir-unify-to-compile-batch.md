# ADR: Stage-2 S2-05 — syx-driver mir_unify→test→compile 三刀批抽（S2-I218–I220）

**Status:** Accepted (S2-I218–I220) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I215–I217 driver fingerprint→lsp batch](stage2-s2-05-driver-fingerprint-to-lsp-batch.md), the locked next knife is **`mir_unify`**. This window peels MIR merge/unify, the `syx test` runner surface, and compile_linked/resolve/project compile. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, peel `DriverOptions` / prelude glue, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I218** [`mir_unify.rs`](../../crates/syx-driver/src/mir_unify.rs) — `stdlib_mir_enabled` / `finalize_unified_mir` / merge helpers / `build_project_mir` / `build_unified_mir` / `build_mir_from_graph`
   - **S2-I219** [`test_runner.rs`](../../crates/syx-driver/src/test_runner.rs) — `TestCase*` / `TestReport` / `TestOutputFormat` / `test_project*` / discover·hooks / `run_file_entry_with_options`
   - **S2-I220** [`compile_linked.rs`](../../crates/syx-driver/src/compile_linked.rs) — `compile_linked*` / `compile_file*` / `compile_project*` / `resolve_module` / package-unit merge（**不含** `compile_file_with_context`）
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use` for public and sibling-needed APIs.
3. Census honesty floor for `syx-driver` `lib.rs`: **5500 → 4000**.
4. **Ladder next:** **S2-I221** = **`compile_file_with_context`** (~single-file compile-with-graph face) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change MIR merge·test·compile semantics / ≠ migrate `#[cfg(test)]` / ≠ peel `DriverOptions` hub / prelude·inject glue / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_mir_unify_to_compile_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I218–I220 row · driver floor 4000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I221–I223 landed:** [driver-compile-ctx-to-native-batch](stage2-s2-05-driver-compile-ctx-to-native-batch.md) — compile_file_context / prelude_glue / native_build left into modules; next default = **S2-I224 `driver_options` hub**.
- Prior: [driver-fingerprint-to-lsp-batch](stage2-s2-05-driver-fingerprint-to-lsp-batch.md) · S2-I215–I217.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-fingerprint-to-lsp-batch](stage2-s2-05-driver-fingerprint-to-lsp-batch.md) · S2-I215–I217
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
