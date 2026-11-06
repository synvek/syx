# ADR: Stage-2 S2-03 open — VM←MIR / B-01·B-02 product honesty (S2-I136)

**Status:** Accepted (S2-I136) · S2-03 `[x]` (claimed S2-I142) · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I135 S2-01·S2-02 claim](stage2-s2-01-s2-02-claim.md), Stage-2 default work shifts to [plan/02 S2-03](../../plan/02-improvement-roadmap.md): VM consumes MIR (product-level B-01/B-02 honesty) — retire AST→bytecode as a living path; make differential testing meaningful. Wave B already signed **narrow** B-01/B-02 `[x]` (ADR-818 / ADR-823 / ADR-826); S2-I4 deleted codegen GateB01/B02 inventory + native companions (**≠** product exit). This window **opens** the workstream and rebuilds **behavioral** proof — it does **not** claim S2-03 `[x]`.

## Wave B narrow vs plan S2-03

| Claim | Wave B (narrow) | plan/02 S2-03 |
|-------|-----------------|---------------|
| Primary path `compile_with_mir` | **met** | required |
| Public `compile` / `compile_into*` deleted | **met** (W482) | required |
| Active AST type-shell emit = 0 | **met** (W473) | required |
| `compile_type_shells` / private `compile_decl` present | shell hook **deleted** (I138); `compile_module` **deleted** (I139); `compile_decl` corpse **deleted** (I140) | **claimed** ([I142](stage2-s2-03-claim.md)) |
| VM executes MIR-lowered **bytecode** | true | honesty: still bytecode interpreter |
| Differential bound as S2-03 floor | not bound | **bound** (I141: Suite-1 VM⟷MIR; [deepen](stage2-s2-03-differential-deepen.md)) |
| Product behavior nails post S2-I4 | **gap** (inventory deleted) | this window restores via **tests/** only |

## Decision

1. **Open S2-03** as an active Stage-2 workstream; flip plan S2-03 `[ ]` → `[~]`.
2. **S2-I136 first slice** — document the gap table above; add behavioral contract tests under `syx-driver/tests/` (no `src/` governance inventory); bind [`tests/differential`](../../crates/syx-driver/tests/differential/) as the S2-03 differential floor.
3. **Proof style (post S2-I4):** docs gates + behavioral tests — **not** restoring `GateB01*Inventory` / `GateB02*Inventory` into product `src/` (S2-01 KPI 0).
4. **Ladder:** **S2-I137** hollow → **S2-I138** delete-fn → **S2-I139** compile_module delete ([compile-module-delete](stage2-s2-03-compile-module-delete.md)) → **S2-I140** compile_decl delete ([compile-decl-delete](stage2-s2-03-compile-decl-delete.md)) → **S2-I141** differential deepen ([differential-deepen](stage2-s2-03-differential-deepen.md)) → **S2-I142** claim ([claim](stage2-s2-03-claim.md)).
5. **Non-claims (hard):** ≠ S2-03 `[x]` / ≠ Stage-2 exit / ≠ default MirRuntime / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-04…07 / ≠ restore governance inventory in `src/`.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Production compile attaches MIR | `s2_03_production_compile_attaches_mir_module` |
| Public AST compile API absent | `s2_03_public_ast_compile_api_absent` (codegen surface honesty) |
| MIR path runnable | `s2_03_mir_path_run_fixture` |
| Differential floor (S2-03 meaning) | Suite-1 **VM + MIR** axes agree on oracle stdout before later backends; harness = [`tests/differential`](../../crates/syx-driver/tests/differential/); CI job retained |
| Honesty — wasm/native | Darwin suite1 **wasm** emit may fail-closed (`type mismatch…end of block`) — **pre-existing** (same class as exit-leg wasm FAIL-CLOSED); **≠** introduced by S2-I136; full four-backend green is **later** S2-03 construction, not open-window claim |

## Consequences

- S2-03 moved `[ ]` → `[~]` (I136); claimed `[x]` in [I142](stage2-s2-03-claim.md); Stage-2 stays `[~]`.
- **S2-I137:** [type-shell hollow](stage2-s2-03-type-shell-hollow.md) — `compile_type_shells`→`Ok(())`; path entry 0; symbol kept.
- **S2-I138:** [type-shell delete](stage2-s2-03-type-shell-delete.md) — call+fn removed.
- **S2-I139:** [compile-module delete](stage2-s2-03-compile-module-delete.md) — trio removed; `compile_decl` corpse kept.
- **S2-I140:** [compile-decl delete](stage2-s2-03-compile-decl-delete.md) — corpse cluster removed; `compile_function_decl` kept.
- **S2-I141:** [differential deepen](stage2-s2-03-differential-deepen.md) — Suite-1 VM⟷MIR floor bound; wasm honesty OUT.
- **S2-I142:** [claim](stage2-s2-03-claim.md) — plan S2-03 `[x]`; Stage-2 stays `[~]`.
- Next default was **S2-04** open — done in [S2-I143](stage2-s2-04-open.md); Stage-2 stays `[~]`.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-01-s2-02-claim](stage2-s2-01-s2-02-claim.md) · S2-I135
- [stage2-s2-03-type-shell-hollow](stage2-s2-03-type-shell-hollow.md) · S2-I137
- [stage2-s2-03-type-shell-delete](stage2-s2-03-type-shell-delete.md) · S2-I138
- [stage2-s2-03-compile-module-delete](stage2-s2-03-compile-module-delete.md) · S2-I139
- [stage2-s2-03-compile-decl-delete](stage2-s2-03-compile-decl-delete.md) · S2-I140
- [stage2-s2-03-differential-deepen](stage2-s2-03-differential-deepen.md) · S2-I141
- [stage2-s2-03-claim](stage2-s2-03-claim.md) · S2-I142
- [gate-b01-mir-consumer-thin](../gates/gate-b01-mir-consumer-thin.md)
- [gate-b01-noop-residual-honesty](../gates/gate-b01-noop-residual-honesty.md)
- [gate-b02-compile-api-delete](../gates/gate-b02-compile-api-delete.md)
- [plan/archive/gates/gate-b01-mir-consumer-exit](../../plan/archive/gates/gate-b01-mir-consumer-exit.md) · Wave B narrow
- [plan/02](../../plan/02-improvement-roadmap.md) S2-03
