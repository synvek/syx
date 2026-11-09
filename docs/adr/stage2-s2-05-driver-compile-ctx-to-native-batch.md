# ADR: Stage-2 S2-05 — syx-driver compile_ctx→prelude→native 三刀批抽（S2-I221–I223）

**Status:** Accepted (S2-I221–I223) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I218–I220 driver mir_unify→test→compile batch](stage2-s2-05-driver-mir-unify-to-compile-batch.md), the locked next knife is **`compile_file_with_context`**. This window peels that face (plus dep helpers), the prelude/inject/export glue cluster, and the native/wasm build surface. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, peel the `DriverOptions`/`ModuleUnit` type hub, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I221** [`compile_file_context.rs`](../../crates/syx-driver/src/compile_file_context.rs) — `compile_file_with_context` + `dep_decl_has_attr` / `collect_dep_function_actors`
   - **S2-I222** [`prelude_glue.rs`](../../crates/syx-driver/src/prelude_glue.rs) — `core_prelude_*` / `inject_*` / `ensure_prelude_*` / `export_symbols` / `insert_export_*`
   - **S2-I223** [`native_build.rs`](../../crates/syx-driver/src/native_build.rs) — `build_native*` / link·stamp helpers / `build_wasm*` / `run_native_project_with_options`（不含 `run_wasm*`）
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use` for public and sibling-needed APIs.
3. Census honesty floor for `syx-driver` `lib.rs`: **4000 → 2500**.
4. **Ladder next:** **S2-I224** = **`driver_options` hub** (`ModuleUnit` / `ModuleGraph` / `DriverOptions` / `CompiledProject`) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change compile·prelude·native semantics / ≠ migrate `#[cfg(test)]` / ≠ peel `DriverOptions`/`ModuleUnit`/`ModuleGraph` type hub this window / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_compile_ctx_to_native_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I221–I223 row · driver floor 2500 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I224–I226 landed:** [driver-options-to-run-batch](stage2-s2-05-driver-options-to-run-batch.md) — driver_options / check_project / run_exec left into modules; next default = **S2-I227 `format_lint`**.
- Prior: [driver-mir-unify-to-compile-batch](stage2-s2-05-driver-mir-unify-to-compile-batch.md) · S2-I218–I220.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-mir-unify-to-compile-batch](stage2-s2-05-driver-mir-unify-to-compile-batch.md) · S2-I218–I220
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
