# ADR: Stage-2 S2-02 backends honesty_fences + colocated soft retire (S2-I116)

**Status:** Accepted (S2-I116) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I115 driver_gate residual soft retire](stage2-driver-gate-manifest-audit-codable-residual-retire.md), remaining S2-02 soft-nail work moves outside `driver_gate`. Backends matrix/GA files still carry colocated `include_str` soft cells: `*_honesty_fences`, `*_inventory_meta`, and `*_ci_wires` that pin toml/ci/stubs rather than exercise `.sy` / `run_*` behavior.

## Decision

1. **Authorize deletion** of 28 pure soft tests under [`crates/syx-driver/tests/backends/`](../../crates/syx-driver/tests/backends/) (`*_native_wasm_{matrix,ga}.rs`):
   - `*_honesty_fences` ×10
   - colocated `*_inventory_meta` ×10 (honesty siblings only; **not** `path_file` ga6/ga7)
   - colocated `*_ci_wires` ×8
2. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
3. **Keep** all matrix/GA behavioral `run_*` / hostfn_availability tests; docs_honesty markers; LSP `*_fingerprint_stub`; criterion/microbench soft deferred to [S2-I117](stage2-backends-criterion-microbench-soft-retire.md); `cache_v111`; hasher.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −28 soft-nail tests; `driver_gate` floor count unchanged.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Backends soft cell shrinks; criterion/microbench continued in [S2-I117](stage2-backends-criterion-microbench-soft-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-driver-gate-manifest-audit-codable-residual-retire](stage2-driver-gate-manifest-audit-codable-residual-retire.md) · S2-I115
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
