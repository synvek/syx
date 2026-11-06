# ADR: Stage-2 S2-03 type-shell delete (S2-I138)

**Status:** Accepted (S2-I138) · S2-03 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I137 type-shell hollow](stage2-s2-03-type-shell-hollow.md), `CodeGenerator::compile_type_shells` was an explicit `Ok(())` with a retained call site in `compile_with_mir`. This window **deletes** the call and the method — MIR shells continue via `lower_*_shells` / `compile_from_mir` only.

## Decision

1. Remove `gen.compile_type_shells(module)?` from `compile_with_mir`.
2. Delete `pub fn compile_type_shells` entirely (I137 hollow body + docs).
3. Leave a one-line charter comment at the former call site pointing at this ADR.
4. **Do not** purge `compile_decl` / dead `compile_module` this window.
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ restore governance inventory / ≠ fix Darwin wasm differential / ≠ residual cluster count→0 migrate claim.

## Evidence

| Check | Evidence |
|-------|----------|
| Symbol absent | `s2_03_compile_type_shells_absent_on_mir_path` |
| Behavior floor | `s2_03_mir_path_run_fixture` + suite1 VM+MIR |

## Consequences

- S2-03 stays `[~]`.
- **Follow-up (done):** [S2-I139 compile_module delete](stage2-s2-03-compile-module-delete.md) removed dead module entry trio.
- Next knife = peel `compile_decl` corpse (protect live `compile_function_decl`←`compile_expr`).
- Open: [stage2-s2-03-open](stage2-s2-03-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-03-type-shell-hollow](stage2-s2-03-type-shell-hollow.md) · S2-I137
- [stage2-s2-03-compile-module-delete](stage2-s2-03-compile-module-delete.md) · S2-I139
- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
