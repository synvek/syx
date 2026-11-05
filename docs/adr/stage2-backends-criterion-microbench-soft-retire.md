# ADR: Stage-2 S2-02 backends criterion + microbench soft retire (S2-I117)

**Status:** Accepted (S2-I117) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I116 backends honesty cell retire](stage2-backends-honesty-fences-retire.md), remaining backends soft-nails include criterion/microbench `include_str` fixture/harness/inventory/ci pins that do not exercise `run_*` behavior. Criterion `*_smoke_ok` and microbench `*_instant` already cover behavioral floors.

## Decision

1. **Authorize deletion** of 24 pure soft tests under [`crates/syx-driver/tests/backends/`](../../crates/syx-driver/tests/backends/) (`{http,path_file,sql,tcp}_{criterion,microbench}.rs`):
   - criterion: `*_fixture_source` / `*_harness_wired` / `*_inventory_meta` / `*_ci_wires` ×4 domains = 16
   - microbench: `*_fixture_source` / `*_inventory_meta` ×4 domains = 8
2. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
3. **Keep** criterion `*_smoke_ok`×4; microbench `*_instant`×4; all `*_hostfn_availability`×16; path_file ga6/ga7 `inventory_meta`; matrix/GA behavioral tests; docs_honesty; LSP `*_fingerprint_stub`; `cache_v111`; hasher.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −24 soft-nail tests; `driver_gate` floor count unchanged.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Backends criterion/microbench soft shrinks; hostfn_availability + ga6/ga7 meta continued in [S2-I118](stage2-backends-hostfn-availability-ga67-meta-retire.md).
- Verify hygiene (pre-existing, not soft-nail): `examples/sql_microbench/main2.sy` moved to `alt/` (same-package dual-`main`); catch `e.message` → `print(e)` so KEEP `*_smoke_ok` / `*_instant` stay green.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-backends-honesty-fences-retire](stage2-backends-honesty-fences-retire.md) · S2-I116
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
