# ADR: Stage-2 S2-02 stdlib M2 cap/ci_wires soft retire (S2-I126)

**Status:** Accepted (S2-I126) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I125 cross-crate surface soft retire](stage2-cross-crate-surface-soft-retire.md), the largest remaining soft-nail sea is under [`crates/syx-driver/tests/stdlib/`](../../crates/syx-driver/tests/stdlib/). Soft remains a primary test path overall; this window does **not** claim S2-02 readiness.

## Decision

1. **Authorize deletion** of **79** pure soft `#[test]` functions under `tests/stdlib/` (+ security `ci_wires_capability_generators`) that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `compile_and_run` / compile-run paths; and
   - match one of:
     - name starts with `m2_` and body pins `host_fn_capabilities.toml`; or
     - name contains `ci_wires`; or
     - name starts with `m2_` and body pins `.github/workflows/ci.yml` and/or `syx-stdlib/.../registry.rs` (ci/registry line); or
     - name contains `surface`.
2. **Keep** `inventory_meta`; `docs_and_capability`; `capability_thin` / floor / exit; props docs_honesty×3; `cache_v111_*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; hasher.
3. **Keep** colocated behavioral / `run()` / loopback tests in the same files.
4. **Defer** `hostfn_*.rs` cap pins (esp. `hostfn_sql`×27); `residual_*_thin`; `m2_product_caps`; tls/net GA thin seas.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −79 soft-nail tests; residual stdlib pure soft still large (~250+).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness complete / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- M2 host_fn_capabilities + CI wire + surface soft subclass closes; residual soft path includes hostfn_sql / residual_thin / product_caps / platform misc + KEEP tips.
- Next: hostfn cap pins or residual+net thin; claim-readiness only after soft is no longer a primary path.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-cross-crate-surface-soft-retire](stage2-cross-crate-surface-soft-retire.md) · S2-I125
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
