# ADR: Stage-2 S2-05 — syx-mir `mir_opt` extract (S2-I169)

**Status:** Accepted (S2-I169) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I168 mir_aot](stage2-s2-05-mir-aot-extract.md), the locked next knife is the **zero-semantics** MIR optimization pipeline from [`syx-mir`](../../crates/syx-mir). This window moves `const_fold` through `loop_licm` into [`mir_opt.rs`](../../crates/syx-mir/src/mir_opt.rs). Lowering helpers `flatten_blocks` / `resolve_jump_targets` stay in `lib.rs` as `pub(crate)` for `resync_flat_from_blocks`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_opt.rs`](../../crates/syx-mir/src/mir_opt.rs) holding: `const_fold`, `dead_code_elim`, `optimize` / `optimize_mir_module`, DCE/unreachable/inline/escape helpers, all `mir_opt_v*` gates, LICM/CSE/`loop_licm`.
2. Keep in [`lib.rs`](../../crates/syx-mir/src/lib.rs): `ModuleIndex` / `LowerCtx` / lowering; `flatten_blocks` / `resolve_jump_targets` as `pub(crate)`.
3. Preserve public API via `mod mir_opt;` + `pub use mir_opt::{…}` at crate root.
4. **Ladder next:** **S2-I170** = **mir_lower first extract** (clear `ModuleIndex` / `LowerCtx` or one coherent lower boundary) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract lowering this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change optimize defaults or opt-v* gate behavior.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_opt.rs` |
| Re-export | `lib.rs` `mod mir_opt` + `pub use mir_opt::{…}` |
| Contract | `s2_05_mir_opt_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I169 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Default next construction window = **S2-I170 mir_lower first extract**.
- **S2-I170:** [mir-module-index-extract](stage2-s2-05-mir-module-index-extract.md) landed — `mir_module_index.rs`; next = **S2-I171 LowerCtx**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Prior: [mir-aot-extract](stage2-s2-05-mir-aot-extract.md) · S2-I168.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-aot-extract](stage2-s2-05-mir-aot-extract.md) · S2-I168
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
