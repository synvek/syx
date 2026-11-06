# ADR: Stage-2 S2-05 — syx-mir `mir_aot` extract (S2-I168)

**Status:** Accepted (S2-I168) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I167 mir_codec](stage2-s2-05-mir-codec-extract.md), the locked next knife is the **zero-semantics** AOT shaken / compact / omit / size-inventory face from [`syx-mir`](../../crates/syx-mir). `encode_module_compact_for_aot` depends on `module_with_*` helpers that sat **after** the MIR opt block in `lib.rs`, so this window joins **two non-contiguous slices** into [`mir_aot.rs`](../../crates/syx-mir/src/mir_aot.rs) and leaves `dead_code_elim` / `optimize` in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_aot.rs`](../../crates/syx-mir/src/mir_aot.rs) from:
   - **Slice A:** `direct_call_targets_from_instructions` → `aot_size_fixture_call_indirect_full_emit` (reachable / llvm_emit / stub / shaken / compact / size report / size fixtures).
   - **Slice B:** `aot_codec_fixture_*` + `module_with_debug_locals_stripped` / `module_with_execution_string_table_stripped` / `module_with_native_instructions_omitted` (closes compact path).
2. Keep in [`lib.rs`](../../crates/syx-mir/src/lib.rs): `const_fold`, `dead_code_elim`, full `optimize` / LICM / opt-v* gates, lowering.
3. Preserve public API via `mod mir_aot;` + `pub use mir_aot::{…}` at crate root; `encode_module` remains from `mir_codec`.
4. **Ladder next:** **S2-I169** = extract **mir_opt** (`dead_code_elim` → `optimize` / LICM / opt-v* helpers) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract `optimize` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change AOT shaken/compact semantics or default path.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_aot.rs` |
| Re-export | `lib.rs` `mod mir_aot` + `pub use mir_aot::{…}` |
| Contract | `s2_05_mir_aot_extract_contract` |
| Census | [stage2-file-split-census](../stage2-file-split-census.md) I168 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- Default next construction window = **S2-I169 mir_opt extract**.
- **S2-I169:** [mir-opt-extract](stage2-s2-05-mir-opt-extract.md) landed — `mir_opt.rs`; next = **S2-I170 mir_lower first extract**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Prior: [mir-codec-extract](stage2-s2-05-mir-codec-extract.md) · S2-I167.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-types-extract](stage2-s2-05-mir-types-extract.md) · S2-I166
- [stage2-s2-05-mir-codec-extract](stage2-s2-05-mir-codec-extract.md) · S2-I167
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
