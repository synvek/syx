# ADR: Stage-2 S2-05 — syx-driver options→check→run 三刀批抽（S2-I224–I226）

**Status:** Accepted (S2-I224–I226) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I221–I223 driver compile_ctx→prelude→native batch](stage2-s2-05-driver-compile-ctx-to-native-batch.md), the locked next knife is the **`driver_options` hub**. This window peels that type hub (pure types + path helpers), the check surface, and the run/embed/exec tail. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, fold `CachedModuleUnit` into the hub, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I224** [`driver_options.rs`](../../crates/syx-driver/src/driver_options.rs) — `ModuleUnit` / `ModuleGraph` / `NativeBackend` / `DriverOptions` / `BuildProfile` / path helpers / `CompiledProject`（**不含** `CachedModuleUnit`；**禁止** compile/native/fingerprint 反向依赖）
   - **S2-I225** [`check_project.rs`](../../crates/syx-driver/src/check_project.rs) — `check_project*` / `check_file_with_options` / parse-diag walk
   - **S2-I226** [`run_exec.rs`](../../crates/syx-driver/src/run_exec.rs) — `run_project*` / embed / `run_file*` / `ensure_vm_plugin_dispatch` / `run_compiled_program*`（不含 `run_wasm*`）
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use`.
3. Census honesty floor for `syx-driver` `lib.rs`: **2500 → 2000**.
4. **Ladder next:** **S2-I227** = **`format_lint`** (~format/lint face) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change options·check·run semantics / ≠ migrate `#[cfg(test)]` / ≠ fold `CachedModuleUnit` into hub / ≠ hub reverse-deps on compile/native/fingerprint / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_options_to_run_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I224–I226 row · driver floor 2000 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I227–I229 landed:** [driver-format-to-wasm-batch](stage2-s2-05-driver-format-to-wasm-batch.md) — format_lint / compile_cache / run_wasm left into modules; next default = **S2-I230 `compile_to_program`**.
- Prior: [driver-compile-ctx-to-native-batch](stage2-s2-05-driver-compile-ctx-to-native-batch.md) · S2-I221–I223.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-compile-ctx-to-native-batch](stage2-s2-05-driver-compile-ctx-to-native-batch.md) · S2-I221–I223
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
