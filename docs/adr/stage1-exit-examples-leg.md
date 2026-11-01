# ADR: Stage-1 exit examples leg = VM inventory + S0 three-backend thin smoke

**Status:** Accepted (S1-I26)  
**Date:** 2026-11-01  
**Context:** Stage-1 exit triad requires book ≥85%, parse→0, and an “examples runnable” leg. [S1-10 scoped](stage1-s1-10-vm-inventory-scope.md) completed the **VM allowlist + CI** slice, while charter text still treated **拉满** and **full three-backend matrix** as blockers. Meanwhile [`scripts/s0_backend_smoke.sh`](../../scripts/s0_backend_smoke.sh) already records hello (+ hot_loop) across VM / native / wasm with PASS or clear fail-closed — but that honesty was not wired into the exit examples-leg charter.

## Decision

1. **Exit triad — examples leg (In):**
   1. **S1-10 scoped:** VM allowlist + `examples_vm_contract.sh` + CI ([ADR](stage1-s1-10-vm-inventory-scope.md));
   2. **S0 three-backend thin smoke:** `s0_backend_smoke.sh` for **hello** (and the script’s existing hot_loop thin widen): **VM must PASS**; native/wasm **PASS or explicit fail-closed** (S0-05/06).
2. **Out / defer (not required for this leg):**
   - Claiming examples **拉满** / promoting Out-table demos;
   - Running **native/wasm for all 26** allowlist files;
   - Equating this ADR with a Stage-1 exit announcement.
3. **Stage-1 exit honesty.** With book KPI + parse=0 + this examples leg, the **triad is charter-complete**. Plan SSOT flip to “阶段 1 退出” is recorded in [stage1-exit.md](stage1-exit.md) (S1-I27). S1-04 / S1-05 remain `[~]` as **post-exit residual**.

## Consequences

- `plan/02` cites this ADR for the examples leg of the exited Stage-1 triad; keep “≠ 拉满 / ≠ allowlist 全三后端”.
- Local one-shot: [`scripts/stage1_exit_examples_leg.sh`](../../scripts/stage1_exit_examples_leg.sh) (VM contract + S0 smoke). CI already runs both pieces separately.

## References

- [`docs/adr/stage1-s1-10-vm-inventory-scope.md`](stage1-s1-10-vm-inventory-scope.md)
- [`docs/examples-vm-contract.md`](../examples-vm-contract.md)
- [`scripts/s0_backend_smoke.sh`](../../scripts/s0_backend_smoke.sh)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
