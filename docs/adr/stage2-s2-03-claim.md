# ADR: Stage-2 S2-03 plan claim (S2-I142)

**Status:** Accepted (S2-I142) · S2-03 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I141 differential deepen](stage2-s2-03-differential-deepen.md), the S2-03 construction ladder opened in [S2-I136](stage2-s2-03-open.md) is complete: AST corpse axis retired (I137–I140) and Suite-1 VM⟷MIR differential floor bound (I141). This dedicated window flips the plan checkbox only — aligned with [S2-I135 S2-01·S2-02 claim](stage2-s2-01-s2-02-claim.md). It does **not** exit Stage-2.

## Evidence

| Axis | Evidence |
|------|----------|
| Production MIR path | `s2_03_production_compile_attaches_mir_module` · `s2_03_mir_path_run_fixture` |
| Public AST compile API absent | `s2_03_public_ast_compile_api_absent` ([I136](stage2-s2-03-open.md) / W482) |
| Type-shell hook deleted | [I137](stage2-s2-03-type-shell-hollow.md) / [I138](stage2-s2-03-type-shell-delete.md) · `s2_03_compile_type_shells_absent_on_mir_path` |
| `compile_module` trio deleted | [I139](stage2-s2-03-compile-module-delete.md) · `s2_03_compile_module_absent_on_mir_path` |
| `compile_decl` corpse deleted; `compile_function_decl` kept | [I140](stage2-s2-03-compile-decl-delete.md) · `s2_03_compile_decl_absent_on_mir_path` |
| Differential meaningful (Suite-1 VM⟷MIR) | [I141](stage2-s2-03-differential-deepen.md) · `s2_03_suite1_vm_mir_floor` · `suite1_*_vm_mir` |
| Honesty OUT | Darwin suite1 wasm fail-closed documented; **≠** four-backend GA claim |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-03 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until S2-04…S2-07 acceptance (or explicit Stage-2 exit ADR).
3. Update [stage2-s2-03-open](stage2-s2-03-open.md) / [stage2-open](stage2-open.md) / [differential-deepen](stage2-s2-03-differential-deepen.md) Consequences to note plan claim under this ADR; next default = **S2-04** open.
4. **Non-claims (hard):** ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ fix Darwin wasm / ≠ default MirRuntime / ≠ open or complete S2-04…07 construction this window / ≠ restore governance inventory in `src/` / ≠ residual Decl cluster count→0 migrate claim.

## Consequences

- S2-03 plan-claimed; Stage-2 default next = **S2-04** (unified CST→AST parser) open window.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [stage2-s2-03-type-shell-hollow](stage2-s2-03-type-shell-hollow.md) · S2-I137
- [stage2-s2-03-type-shell-delete](stage2-s2-03-type-shell-delete.md) · S2-I138
- [stage2-s2-03-compile-module-delete](stage2-s2-03-compile-module-delete.md) · S2-I139
- [stage2-s2-03-compile-decl-delete](stage2-s2-03-compile-decl-delete.md) · S2-I140
- [stage2-s2-03-differential-deepen](stage2-s2-03-differential-deepen.md) · S2-I141
- [stage2-s2-01-s2-02-claim](stage2-s2-01-s2-02-claim.md) · S2-I135 (claim pattern)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
