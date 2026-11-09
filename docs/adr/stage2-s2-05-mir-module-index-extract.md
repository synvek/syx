# ADR: Stage-2 S2-05 — syx-mir `mir_module_index` extract (S2-I170)

**Status:** Accepted (S2-I170) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I169 mir_opt](stage2-s2-05-mir-opt-extract.md), the locked next knife is the first **mir_lower** slice: the AST→index face. This window moves [`ModuleIndex`](../../crates/syx-mir/src/mir_module_index.rs) and its index-build satellites into a dedicated module. `LowerCtx` / `lower_expr` stay in `lib.rs`. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs) holding: `ModuleIndex` (+ `from_module` / observers / property accessors) and satellites `extension_init_literal_fields`, `register_extension_init_literal_defaults`, `platform_int_extension_aliases`, `remap_platform_int_extension_call`, `mir_type_expr_name` — all **`pub(crate)`** where lowering in `lib.rs` needs them.
2. Keep in [`lib.rs`](../../crates/syx-mir/src/lib.rs): `LowerCtx`, module orchestration, `lower_expr`, and the bulk of MIR emission.
3. No new public crate-root API (index surface remains crate-private).
4. **Ladder next:** **S2-I171** = extract **`LowerCtx`** (and tightly coupled helpers) — still ≠ S2-05 `[x]`.
5. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract `LowerCtx`/`lower_expr` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change index-build semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_module_index.rs` |
| Wiring | `lib.rs` `mod mir_module_index` + `use mir_module_index::{…}` |
| Contract | `s2_05_mir_module_index_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I170 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I171 landed:** [mir-lower-ctx-extract](stage2-s2-05-mir-lower-ctx-extract.md) — `LowerCtx` left `lib.rs`; next default = **S2-I172 lower_expr first extract**.
- Prior: [mir-opt-extract](stage2-s2-05-mir-opt-extract.md) · S2-I169.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-opt-extract](stage2-s2-05-mir-opt-extract.md) · S2-I169
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
