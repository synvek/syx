# ADR: Stage-2 S2-01 llvm_stub_audit product accept (S2-I134)

**Status:** Accepted (S2-I134) · S2-01 `[~]` · claim-readiness ledger **complete**  
**Date:** 2026-11-05  
**Context:** After [S2-I133 gate_b09 peel](stage2-gate-b09-narrative-peel.md), the only remaining S2-01 gate blocker on [claim inventory](stage2-claim-readiness-inventory.md) was [`llvm_stub_audit`](../../crates/syx-rt/src/llvm_stub_audit.rs). Soft OUT seas are already **0**. The module has **zero** `syx-cli` / `syx-driver/src` runtime call sites; it is a read-only classifier over `llvm_stubs.rs` that underpins Wave B **B-06** floors (`classify_llvm_stub_block`, `llvm_stub_audit_report`, count floors). Physical peel in one window would break Wave B evidence nails (`wave_b_exit_thin`, gate-b06 SSOT, historical MUST STAY ADRs). plan/02 already allows **peel or ADR acceptance**.

## Decision

1. **Accept** `llvm_stub_audit` as a **B-06 product capability** (stub classification + count floors), not process-governance / 签字 inventory.
2. **Supersede** the claim-inventory reading that listed this module as an S2-01 product-path **gate blocker**.
3. **Keep** the module in `syx-rt` with existing `pub use`; keep driver native evidence nails and B-06 floors unchanged.
4. **Close** the claim-readiness inventory ledger (**Status → complete**). Soft OUT = 0; gate OUT = 0; KEEP tip remains policy-kept.
5. **Expected:** census KPIs remain **0**; S2-01/S2-02 stay `[~]` on plan (满勾 dedicated window still required); ≠ Stage-2 exit.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ B-06 semantic upgrade / ≠ Missing clear / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-03…

## Consequences

- Claim ledger no longer blocks on `llvm_stub_audit`; next work is dedicated满勾 narrative for S2-01 (and optionally S2-02), not further soft knives.
- Open: [stage2-open](stage2-open.md) · [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-gate-b09-narrative-peel](stage2-gate-b09-narrative-peel.md) · S2-I133
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md)
- [gate-b06-llvm-stub-audit](../gates/gate-b06-llvm-stub-audit.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
