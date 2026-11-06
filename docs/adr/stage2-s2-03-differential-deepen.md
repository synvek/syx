# ADR: Stage-2 S2-03 differential deepen (S2-I141)

**Status:** Accepted (S2-I141) · S2-03 `[x]` (claimed S2-I142) · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I140 compile_decl delete](stage2-s2-03-compile-decl-delete.md), the AST corpse axis is closed. [S2-I136 open](stage2-s2-03-open.md) already **narrated** Suite-1 VM+MIR as the S2-03 differential floor, but `diff_exact` still runs wasm/native by default and suite1 oracles keep `skip.wasm=false`. On Darwin, suite1 **wasm emit fail-closed** (pre-existing `type mismatch…end of block`) makes full `diff_exact` red while VM/MIR already match the oracle. This window **binds an executable VM⟷MIR floor** independent of wasm/native.

## Decision

1. S2-03 differential floor = Suite-1 **8 fixtures**: **VM ⟷ MIR ⟷ oracle** (exact exit + stdout).
2. Add `diff_exact_vm_mir` in the differential harness (vm+mir only; no wasm/native).
3. Nail the floor in **both** `--test lang` (`s2_03_suite1_vm_mir_floor`) and `--test differential` (thin `suite1_*_vm_mir` wrappers).
4. **Honesty:** Darwin suite1 wasm fail-closed remains **OUT** of this floor — documented, **not** fixed; suite1 `*.oracle.json` **`skip.wasm` stays false** (preserve Linux CI four-backend intent).
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ fix Darwin wasm / ≠ flip suite1 `skip.wasm` / ≠ restore governance inventory / ≠ default MirRuntime / ≠ open S2-04…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Suite-1 VM+MIR floor (lang) | `s2_03_suite1_vm_mir_floor` |
| Suite-1 VM+MIR floor (differential) | `suite1_*_vm_mir` via `diff_exact_vm_mir` |
| Prior S2-03 surface nails | existing `s2_03_*` (mir_module / API / type-shell / module / decl) |
| Wasm honesty | ADR + test charter; ≠ oracle skip flip |

## Consequences

- Gap「Differential bound」→ **bound (Suite-1 VM+MIR)**; plan S2-03 claimed in [S2-I142](stage2-s2-03-claim.md).
- Next default was **S2-04** open — done in [S2-I143](stage2-s2-04-open.md); Stage-2 stays `[~]`.
- Open: [stage2-s2-03-open](stage2-s2-03-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-03-claim](stage2-s2-03-claim.md) · S2-I142
- [stage2-s2-03-compile-decl-delete](stage2-s2-03-compile-decl-delete.md) · S2-I140
- [stage2-s2-03-open](stage2-s2-03-open.md) · S2-I136
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
- Harness: [`crates/syx-driver/tests/differential`](../../crates/syx-driver/tests/differential/)
