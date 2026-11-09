# ADR: Stage-2 S2-05 — syx-mir `mir_codec` extract (S2-I167)

**Status:** Accepted (S2-I167) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I166 mir_types](stage2-s2-05-mir-types-extract.md), the locked next knife is the **zero-semantics** codec / dual-blob / sidecar / AOT-embed framing face from [`syx-mir`](../../crates/syx-mir). This window moves that surface into [`mir_codec.rs`](../../crates/syx-mir/src/mir_codec.rs) and re-exports it from the crate root. It does **not** claim S2-05 `[x]`, change blob wire format, or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_codec.rs`](../../crates/syx-mir/src/mir_codec.rs) holding: `encode_module` / `decode_module`, blob stats, metadata sidecar encode/decode, SYXD dual-blob package + peel, AOT-embed framing (`encode_module_aot_embed_*`, `aot_embed_execution_omitted`), `mir_module_from_sidecar_fn_stubs`, dual-blob helpers.
2. Keep in [`lib.rs`](../../crates/syx-mir/src/lib.rs): `module_with_*` strips, `aot_codec_fixture_*`, reachable / `llvm_emit` / shaken / compact / AOT size pipeline.
3. Preserve public API via `mod mir_codec;` + `pub use mir_codec::{…}` at crate root.
4. **Ladder next:** **S2-I168** = AOT shaken/compact extract (`reachable` / `llvm_emit` / `shaken` / `compact` / `module_with_native_instructions_omitted`) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract AOT llvm/shaken/compact this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change encode/decode wire semantics or default blob format.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_codec.rs` |
| Re-export | `lib.rs` `mod mir_codec` + `pub use mir_codec::{…}` |
| Contract | `s2_05_mir_codec_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I167 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Default next construction window = **S2-I168 AOT shaken/compact extract**.
- **S2-I168:** [mir-aot-extract](stage2-s2-05-mir-aot-extract.md) landed — `mir_aot.rs`; next = **S2-I169 mir_opt**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Prior: [mir-types-extract](stage2-s2-05-mir-types-extract.md) · S2-I166.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-types-extract](stage2-s2-05-mir-types-extract.md) · S2-I166
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
