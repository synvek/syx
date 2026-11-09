# ADR: Stage-2 S2-02 backends hostfn_availability + ga6/ga7 meta soft retire (S2-I118)

**Status:** Accepted (S2-I118) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I117 criterion+microbench soft retire](stage2-backends-criterion-microbench-soft-retire.md), remaining backends soft-nails are the four `*_hostfn_availability.rs` modules (pure `include_str` caps/product/md/inventory pins) and path_file ga6/ga7 `inventory_meta` deferred from [S2-I116](stage2-backends-honesty-fences-retire.md). Matrix/GA `run_*` already cover HostFn behavior.

## Decision

1. **Authorize deletion** of 16 soft tests by removing entire modules under [`crates/syx-driver/tests/backends/`](../../crates/syx-driver/tests/backends/):
   - `{http,path_file,sql,tcp}_hostfn_availability.rs` and their `mod` lines in `backends/main.rs`
2. **Authorize deletion** of 2 soft tests in `path_file_native_wasm_ga.rs`:
   - `ec_m2_4_path_file_native_wasm_ga6_inventory_meta`
   - `ec_m2_4_path_file_native_wasm_ga7_inventory_meta`
3. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
4. **Keep** criterion `*_smoke_ok` / microbench `*_instant`; matrix/GA behavioral tests; docs_honesty; LSP `*_fingerprint_stub`; `cache_v111`; hasher.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −18 soft-nail tests; `driver_gate` floor unchanged.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Backends soft residual from I116–I117 axis closes; LSP stub continued in [S2-I119](stage2-lsp-fingerprint-stub-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-backends-honesty-fences-retire](stage2-backends-honesty-fences-retire.md) · S2-I116
- [stage2-backends-criterion-microbench-soft-retire](stage2-backends-criterion-microbench-soft-retire.md) · S2-I117
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
