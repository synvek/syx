# ADR: S1-10 scoped = VM inventory + CI (≠ examples 拉满 / 三后端)

**Status:** Accepted (S1-I21)  
**Date:** 2026-11-01  
**Context:** Stage-1 exit triad historically tied S1-10 to “examples runnable rate full” plus three-backend matrix. Engineering already shipped a **VM allowlist** (26 files), `scripts/examples_vm_contract.sh`, CI hook, and an explicit OUT table — but `plan/02` stayed `[~]` because the charter still said 拉满 / 三后端.

## Decision

1. **S1-10 acceptance is scoped.** Completing S1-10 means:
   - Frozen **VM-only** allowlist documented in [`docs/examples-vm-contract.md`](../examples-vm-contract.md);
   - `scripts/examples_vm_contract.sh` asserts `syx check` + `syx run` (default VM) for every allowlisted path;
   - CI runs that script after `cargo build -p syx-cli`;
   - OUT-of-inventory categories are listed honestly (GUI, HTTP, linalg/HostFn, wasm backends, multi-module, gitignored locals, etc.).
2. **Out / defer (not required for S1-10 `[x]`):**
   - Claiming “examples runnable rate full” / 拉满;
   - native / wasm **three-backend** matrix for the allowlist or for Out demos;
   - Promoting Out-table demos into the contract without a new ADR.
3. **Stage-1 exit honesty.** Under this ADR, the exit triad’s “examples” leg counts as **VM inventory + CI complete**. Full 拉满 / 三后端 remain **exit blockers or defer items** until a superseding ADR. **S1-10 scoped `[x]` alone does not authorize claiming Stage-1 exit.**

## Consequences

- `plan/02` marks **S1-10 `[x]` (scoped)**; narrative must keep “≠ 拉满 / ≠ 三后端 / ≠ 阶段1退出”.
- Capability / marketing text must not equate VM allowlist CI with full examples GA.
- Expanding inventory or adding native/wasm contract gates requires updating the contract doc (and preferably a follow-on ADR).

## References

- [`docs/examples-vm-contract.md`](../examples-vm-contract.md)
- [`scripts/examples_vm_contract.sh`](../../scripts/examples_vm_contract.sh)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · S1-10
