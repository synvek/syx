# ADR: Stage-2 S2-03 compile_module delete (S2-I139)

**Status:** Accepted (S2-I139) · S2-03 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I138 type-shell delete](stage2-s2-03-type-shell-delete.md), the only remaining AST module entry was dead `CodeGenerator::compile_module` (`#[allow(dead_code)]`, zero callers) plus its helpers `preregister_decl_functions` / `ensure_function_slot`. Production MIR path is `compile_with_mir` → `compile_from_mir*` only. This window deletes that **trio**.

## Decision

1. Delete `compile_module`, `preregister_decl_functions`, and `ensure_function_slot`.
2. **Keep** `compile_decl` and its type-shell helpers as retained corpse (no production entry after this delete) — mark with `#[allow(dead_code)]` until peel windows; peel in later windows.
3. **Keep** `compile_function_decl` (live via `compile_expr` closures / TaskBlock).
4. Leave an S2-I139 charter comment above `compile_decl`.
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ purge `compile_decl` this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ restore governance inventory / ≠ fix Darwin wasm / ≠ residual cluster count→0 migrate claim.

## Evidence

| Check | Evidence |
|-------|----------|
| Trio absent | `s2_03_compile_module_absent_on_mir_path` |
| Live lowering kept | same test asserts `compile_decl` + `compile_function_decl` present |
| Behavior floor | existing `s2_03_*` + suite1 VM+MIR |

## Consequences

- S2-03 stays `[~]`; next knife was peel `compile_decl` — done in [S2-I140](stage2-s2-03-compile-decl-delete.md).
- Open: [stage2-s2-03-open](stage2-s2-03-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-03-compile-decl-delete](stage2-s2-03-compile-decl-delete.md) · S2-I140
- [stage2-s2-03-type-shell-delete](stage2-s2-03-type-shell-delete.md) · S2-I138
- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
