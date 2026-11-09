# ADR: Stage-2 S2-05 — syx-driver compile→gates→debug 三刀批抽（S2-I230–I232）

**Status:** Accepted (S2-I230–I232) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I227–I229 driver format→cache→wasm batch](stage2-s2-05-driver-format-to-wasm-batch.md), the locked next knife is **`compile_to_program`**. This window peels that VM codegen / mono-merge cluster, the feature-gate face, and the debug/REPL surface. It does **not** claim S2-05 `[x]`, open semantic knives, migrate tests, claim driver production peel exhausted, or exit Stage-2.

## Decision

1. Extract three faces into dedicated modules (zero semantics):
   - **S2-I230** [`compile_to_program.rs`](../../crates/syx-driver/src/compile_to_program.rs) — `compile_to_program` / `inject_virtual_prelude_imports` / `merge_stdlib_mono_instances` / `collect_main_actor_await_errors` + `contextual_numeric_kinds_from_semantic` / `use_mir_*`
   - **S2-I231** [`feature_gates.rs`](../../crates/syx-driver/src/feature_gates.rs) — `result_*_enabled` / net / typealias / macro 门控簇 + `result_legacy_retirement_weeks` / `iterator_protocol_enabled` / `attached_source_template`
   - **S2-I232** [`debug_repl.rs`](../../crates/syx-driver/src/debug_repl.rs) — `debug_project*` / `debug_file*` / `debug_path*` / `debug_interactive` / `debug_dap` / `repl_eval`（不含 `doc_project`）
2. Wire [`lib.rs`](../../crates/syx-driver/src/lib.rs): mods + `pub use` / `pub(crate) use`.
3. Census honesty floor for `syx-driver` `lib.rs`: **1500 → 1200**.
4. **Ladder next:** **S2-I233** = **driver production close-out**（`doc_project` + cranelift stubs + 薄 host/gui·publish·reflect 定义面）— still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ open semantic this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change compile·gates·debug semantics / ≠ migrate `#[cfg(test)]` / ≠ claim driver production peel exhausted / ≠ claim exit after batch.

## Evidence

| Check | Evidence |
|-------|----------|
| Modules | three sidecars above |
| Wiring | `lib.rs` mods + `pub use` / `pub(crate) use` |
| Contract | `s2_05_driver_compile_to_debug_batch_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I230–I232 row · driver floor 1200 |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I233–I235 landed:** [driver-production-closeout-batch](stage2-s2-05-driver-production-closeout-batch.md) — doc_project / cranelift_stubs / thin_host left into modules; next default = **S2-I236 syx-semantic 首抽**.
- Prior: [driver-format-to-wasm-batch](stage2-s2-05-driver-format-to-wasm-batch.md) · S2-I227–I229.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-driver-format-to-wasm-batch](stage2-s2-05-driver-format-to-wasm-batch.md) · S2-I227–I229
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
