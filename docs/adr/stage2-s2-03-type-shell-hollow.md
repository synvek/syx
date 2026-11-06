# ADR: Stage-2 S2-03 type-shell hollow (S2-I137)

**Status:** Accepted (S2-I137) · S2-03 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I136 S2-03 open](stage2-s2-03-open.md), the MIR production path still called `CodeGenerator::compile_type_shells`, which only walked residual Decl clusters that were already `_ => {}` no-ops under [W473 honesty](../gates/gate-b01-noop-residual-honesty.md). Type shells are supplied by MIR `lower_*_shells` / `compile_from_mir`. This window **hollows** the entry so MIR path path-entry residual = **0**.

## Decision

1. Change `compile_type_shells` to an explicit **`Ok(())`** no-op: no `module.decls` walk, no `compile_decl` from this entry.
2. **Keep** the `compile_type_shells` symbol and the `compile_with_mir` call site (delete-fn = later window).
3. **Do not** purge `compile_decl` / dead `compile_module` AST emit bodies this window.
4. Document path-entry residual **0** under S2-03 construction — **≠** rewriting W473 as a new migrate / **≠** claiming residual Decl variants ceased to exist in the AST enum.
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ delete `compile_type_shells` symbol / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ restore governance inventory / ≠ fix Darwin wasm differential.

## Evidence

| Check | Evidence |
|-------|----------|
| Hollow body | `compile_type_shells` returns `Ok(())` with S2-I137 charter comment |
| No path `compile_decl` | `s2_03_compile_type_shells_hollow_on_mir_path` |
| Behavior floor | existing `s2_03_mir_path_run_fixture` + suite1 VM+MIR |

## Consequences

- S2-03 stays `[~]`.
- **Follow-up (done):** [S2-I138 type-shell delete](stage2-s2-03-type-shell-delete.md) removed call+fn.
- Next knife = peel `compile_decl` / `compile_module` corpse (separate windows).
- Open: [stage2-s2-03-open](stage2-s2-03-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [stage2-s2-03-type-shell-delete](stage2-s2-03-type-shell-delete.md) · S2-I138
- [gate-b01-noop-residual-honesty](../gates/gate-b01-noop-residual-honesty.md) · W473
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
