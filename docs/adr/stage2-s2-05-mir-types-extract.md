# ADR: Stage-2 S2-05 — syx-mir `mir_types` first extract (S2-I166)

**Status:** Accepted (S2-I166) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I165 S2-05 open](stage2-s2-05-open.md), the locked first knife is a **zero-semantics** extract from [`syx-mir`](../../crates/syx-mir) — the largest remaining `lib.rs` face. This window moves the IR **data surface** into [`mir_types.rs`](../../crates/syx-mir/src/mir_types.rs) and re-exports it from the crate root. It does **not** claim S2-05 `[x]`, change MIR semantics/opt defaults, or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_types.rs`](../../crates/syx-mir/src/mir_types.rs) holding: `MirOp`, `is_suite1_pilot_mir_op`, `MirInstr`, `MirBlock`, `MirFunction`, `MirModule`, `mir_module_aot_export_root_refs`.
2. Keep interleaved lowering helpers (`MemberCallKind`, SIMD/member helpers, `ModuleIndex`, `LowerCtx`, opt, codec) in [`lib.rs`](../../crates/syx-mir/src/lib.rs).
3. Preserve public API via `mod mir_types;` + `pub use mir_types::{…}` at crate root (downstream `syx_mir::MirOp` etc. unchanged).
4. **Ladder next:** **S2-I167** = extract **codec/blob** face (`encode_module` / dual-blob / sidecar helpers) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract `ModuleIndex`·`LowerCtx`·opt this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ MIR semantics or opt-default change.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_types.rs` |
| Re-export | `lib.rs` `mod mir_types` + `pub use mir_types::{…}` |
| Contract | `s2_05_mir_types_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I166 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Default next construction window = **S2-I167 codec/blob extract**.
- **S2-I167:** [mir-codec-extract](stage2-s2-05-mir-codec-extract.md) landed — `mir_codec.rs`; next = **S2-I168 AOT shaken/compact**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Prior: [s2-05-open](stage2-s2-05-open.md) · S2-I165.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
