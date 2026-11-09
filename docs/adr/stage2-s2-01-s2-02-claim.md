# ADR: Stage-2 S2-01 · S2-02 plan claim (S2-I135)

**Status:** Accepted (S2-I135) · S2-01 `[x]` · S2-02 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I134 llvm_stub product accept](stage2-llvm-stub-audit-product-accept.md), the [claim-readiness inventory](stage2-claim-readiness-inventory.md) ledger is **complete** (soft OUT = 0; gate OUT = 0). Census KPIs are **0**. Soft-nail is no longer the primary test path. This dedicated window flips plan checkboxes only — it does **not** exit Stage-2.

## Evidence

### S2-01 (process governance out of product `src/`)

| Check | Evidence |
|-------|----------|
| Census KPIs all **0** | [`scripts/s2_governance_census.sh`](../../scripts/s2_governance_census.sh) · [census](../census/stage2-governance-census.md) |
| Claim ledger complete | [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md) |
| Gate peel / accept | `gate_b09` peeled ([I133](stage2-gate-b09-narrative-peel.md)); `llvm_stub_audit` accepted as B-06 product ([I134](stage2-llvm-stub-audit-product-accept.md)) |
| No inventory/fingerprint as release KPI | census + claim ledger |

### S2-02 (behavior-driven tests)

| Check | Evidence |
|-------|----------|
| Soft OUT ≈ **0** | I108–I132 knives; [S2-02 open](stage2-s2-02-open.md) |
| Soft not primary path | behavioral / `run()` / contract matrices retained |
| KEEP tip policy-kept | inventory/docs/capability/tls·tcp_accept charter; cache_v111; docs_honesty; soft_debt; floors 450/350; LSP stubs; hasher; B-06 audit API |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-01 → `[x]`** and **S2-02 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until S2-03…S2-07 acceptance (or explicit Stage-2 exit ADR).
3. Update claim inventory Status to note plan S2-01/S2-02 claimed under this ADR.
4. **Non-claims (hard):** ≠ Stage-2 exit / ≠ open or complete S2-03…S2-07 / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ delete KEEP tip / ≠ change B-06 floors.

## Consequences

- P1 dual track (S2-01/S2-02) plan-claimed; Stage-2 default next = **S2-03** open window.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md)
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md)
- [stage2-llvm-stub-audit-product-accept](stage2-llvm-stub-audit-product-accept.md) · S2-I134
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01 · S2-02
