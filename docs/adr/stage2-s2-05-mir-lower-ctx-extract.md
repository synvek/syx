# ADR: Stage-2 S2-05 — syx-mir `mir_lower_ctx` extract (S2-I171)

**Status:** Accepted (S2-I171) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I170 mir_module_index](stage2-s2-05-mir-module-index-extract.md), the locked next knife is **`LowerCtx`**. This window moves the `LowerCtx` struct and its `impl` into a dedicated module. `lower_expr` (and the bulk of MIR emission) stay in `lib.rs`, with `lower_expr` / `class_name_from_mangled` raised to **`pub(crate)`** for one-way submodule visibility. It does **not** claim S2-05 `[x]` or exit Stage-2.

## Decision

1. Create [`crates/syx-mir/src/mir_lower_ctx.rs`](../../crates/syx-mir/src/mir_lower_ctx.rs) holding: `LowerCtx` (+ all `impl LowerCtx` methods) — type and methods **`pub(crate)`**.
2. Wire [`lib.rs`](../../crates/syx-mir/src/lib.rs): `mod mir_lower_ctx;` + `use mir_lower_ctx::LowerCtx;` (no new crate-root `pub use`).
3. Raise `fn lower_expr` / `fn class_name_from_mangled` → **`pub(crate)`** in `lib.rs` so `LowerCtx` helpers (`lower_rhs_for_field_store`, enclosing-class paths) keep calling them without a cyclic module edge.
4. Keep in `lib.rs`: `lower_expr` body, init emit foursome / `register_param_local_type`, module orchestration, and the rest of MIR emission.
5. **Ladder next:** **S2-I172** = **`lower_expr` first extract** — cut one clear sub-face (not the full 3000+ blob) — still ≠ S2-05 `[x]`.
6. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ extract whole `lower_expr` or init emit foursome this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 / ≠ change `LowerCtx` semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Module | `crates/syx-mir/src/mir_lower_ctx.rs` |
| Wiring | `lib.rs` `mod mir_lower_ctx` + `use mir_lower_ctx::LowerCtx` |
| Visibility | `pub(crate) fn lower_expr` / `pub(crate) fn class_name_from_mangled` remain in `lib.rs` |
| Contract | `s2_05_mir_lower_ctx_extract_contract` |
| Census | [stage2-file-split-census](../census/stage2-file-split-census.md) I171 row |

## Consequences

- S2-05 stays `[~]`; Stage-2 stays `[~]`.
- **S2-I172 landed:** [mir-lower-expr-literal-extract](stage2-s2-05-mir-lower-expr-literal-extract.md) — `Expr::Literal` + `contextual_const_bits` left `lib.rs`; next default = **S2-I173 lower_expr large-arm continuation** (prefer Call sub-face).
- Prior: [mir-module-index-extract](stage2-s2-05-mir-module-index-extract.md) · S2-I170.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-module-index-extract](stage2-s2-05-mir-module-index-extract.md) · S2-I170
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
