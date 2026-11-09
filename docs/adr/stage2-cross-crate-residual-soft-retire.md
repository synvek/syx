# ADR: Stage-2 S2-02 cross-crate residual soft retire (S2-I131)

**Status:** Accepted (S2-I131) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I130 platform residual soft](stage2-platform-residual-soft-retire.md), stdlib+platform deletable pure soft ≈0. Remaining accel sea is cross-crate driver tests (six dirs + lang/wasm/security/concurrency/evergreen neighbors ≈75; claim ledger previously ≈72). [I125](stage2-cross-crate-surface-soft-retire.md) already retired fingerprint/surface/playwright subset. This window retires the residual pure soft. Soft remains as KEEP tips; **≠** S2-02 readiness / **≠** claim-readiness complete.

## Decision

1. **Authorize deletion** of ≈**71–75** pure soft `#[test]` functions under:
   - [`crates/syx-driver/tests/gui|runtime|data|native|browser|farm/`](../../crates/syx-driver/tests/)
   - [`crates/syx-driver/tests/lang|wasm|security|concurrency|evergreen/`](../../crates/syx-driver/tests/)
   that contain `include_str!(…).contains(…)`, do **not** invoke `run_*` / `compile_and_run`, and are **not** KEEP-named below.
2. **Keep:** colocated behavioral / `run()` / e2e; platform `cache_v111_*` · docs_honesty×3 · `soft_debt_zero` · floors 450/350; stdlib KEEP tip; LSP KEEP stubs×10 · semantic `*_contract` · hasher.
3. **Do not** delete `syx-rt` gate sources; **do not** retire LSP non-KEEP ≈6 (deferred).
4. **Expected:** census KPIs remain **0**; ≈ −75 soft nails; target-dir deletable pure soft ≈0; claim inventory cross-crate sea → **0**, still **incomplete**.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness **complete** / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Cross-crate residual soft sea closes; residual OUT soft ≈ LSP×6 + KEEP tip + S2-01 gate peel.
- Open: [stage2-open](stage2-open.md) · [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md)
- [stage2-cross-crate-surface-soft-retire](stage2-cross-crate-surface-soft-retire.md) · S2-I125
- [stage2-platform-residual-soft-retire](stage2-platform-residual-soft-retire.md) · S2-I130
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
