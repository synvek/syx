# ADR: Stage-2 S2-03 compile_decl corpse delete (S2-I140)

**Status:** Accepted (S2-I140) · S2-03 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I139 compile_module delete](stage2-s2-03-compile-module-delete.md), the AST module entry trio is gone. Remaining corpse was `CodeGenerator::compile_decl` plus ten type-shell helpers (`compile_dataclass_*`, `compile_type_members`, `compile_class_init`, `compile_enum_case_init`, `self_param*`, `merged_class_fields`) — all `#[allow(dead_code)]`, zero production callers. Live closures still need `compile_function_decl` via `compile_expr`. This window deletes the **entire corpse cluster** in one knife (~749 LOC).

## Decision

1. Delete `compile_decl` and all S2-I139 corpse helpers:
   `compile_dataclass_{init,copy,hash_into,describe}`, `compile_type_members`,
   `compile_class_init`, `compile_enum_case_init`, `self_param`, `self_param_typed`,
   `merged_class_fields`.
2. **Keep** `compile_function_decl` (live via `compile_expr` closures / TaskBlock) and
   `register_param_var_type`.
3. **Keep** MIR-side `Decl` metadata scans and `syx-mir` live copies of similarly named helpers.
4. Leave an S2-I140 charter comment at the former corpse site.
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ restore governance inventory / ≠ fix Darwin wasm / ≠ deepen differential claim / ≠ residual cluster count→0 migrate claim.

## Evidence

| Check | Evidence |
|-------|----------|
| Corpse absent | `s2_03_compile_decl_absent_on_mir_path` |
| Live lowering kept | same test asserts `compile_function_decl` present |
| Module trio still absent | `s2_03_compile_module_absent_on_mir_path` (I139 charter retained) |
| Behavior floor | existing `s2_03_*` + suite1 VM+MIR |

## Consequences

- S2-03 stays `[~]`; next knife was deepen differential — done in [S2-I141](stage2-s2-03-differential-deepen.md); then dedicated **S2-03 `[x]`** claim.
- Open: [stage2-s2-03-open](stage2-s2-03-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-03-differential-deepen](stage2-s2-03-differential-deepen.md) · S2-I141
- [stage2-s2-03-compile-module-delete](stage2-s2-03-compile-module-delete.md) · S2-I139
- [stage2-s2-03-type-shell-delete](stage2-s2-03-type-shell-delete.md) · S2-I138
- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
