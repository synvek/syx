# ADR: Stage-2 S2-05 — syx-driver production close-out 三刀批抽（S2-I233–I235）

**Status:** Accepted (S2-I233–I235) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I230–I232 driver compile→gates→debug batch](stage2-s2-05-driver-compile-to-debug-batch.md), the locked next knife is **driver production close-out**. This window peels the last movable production definition clusters (`doc_project`, Cranelift stubs, thin host/gui/publish/reflect/mono/desugar). It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, claim `lib.rs` production peel fully exhausted (hub + `#[cfg(test)]` remain), or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I233** [`doc_project.rs`](../../crates/syx-driver/src/doc_project.rs) — `doc_project`
   - **S2-I234** [`cranelift_stubs.rs`](../../crates/syx-driver/src/cranelift_stubs.rs) — Cranelift retirement stubs + `take_macro_warnings`
   - **S2-I235** [`thin_host.rs`](../../crates/syx-driver/src/thin_host.rs) — `ensure_gui_*` / `publish_*` / `reflect_*` / `backfill_mono_witness_cache` / `compile_native_object` / `GLOBALS_PACKAGES_REL` / `should_desugar_top_level_stmts` / `farm_lease_test_lock`
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use`. Keep `ensure_host_rt_link_stubs` / `FORCE_HOST_RT_STUBS` at crate root.
3. Census honesty floor for `syx-driver` `lib.rs`: **1200 → 1100**.
4. Honesty note: **movable production fn/const clusters cleared**; residual = `pub use` hub + in-crate tests + host-rt link stubs — **≠** `lib.rs` production peel fully exhausted.
5. **Ladder next:** **S2-I236** = **syx-semantic 首抽** — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change close-out face semantics / ≠ migrate `#[cfg(test)]` / ≠ claim driver **lib** production peel fully exhausted / ≠ claim exit after batch / ≠ move `ensure_host_rt_link_stubs`.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_production_closeout_batch_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I233–I235 row · driver floor 1100 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I236–I237 landed:** [semantic-types-to-codec-batch](stage2-s2-05-semantic-types-to-codec-batch.md) — semantic_types / semantic_type_codec left into modules; next default = **S2-I238 semantic feature_gates**.
- Prior: [driver-compile-to-debug-batch](stage2-s2-05-driver-compile-to-debug-batch.md) · S2-I230–I232.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-compile-to-debug-batch](stage2-s2-05-driver-compile-to-debug-batch.md) · S2-I230–I232
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
