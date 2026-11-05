# ADR: Stage-2 S2-02 stdlib residual + product-caps soft retire (S2-I127)

**Status:** Accepted (S2-I127) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I126 stdlib M2 cap/ci_wires soft retire](stage2-stdlib-m2-cap-ci-wires-soft-retire.md), remaining stdlib pure soft ≈266. Soft remains a primary test path; this window does **not** claim S2-02 readiness.

## Decision

1. **Authorize deletion** of **66** pure soft `#[test]` functions under [`crates/syx-driver/tests/stdlib/`](../../crates/syx-driver/tests/stdlib/) that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `compile_and_run` / compile-run paths; and
   - match one of:
     - live in `residual_*_thin.rs` (all pure soft in those files, including `*_caps_raised`); or
     - name starts with `m2_` and pins `capabilities.toml` / `mir_op_capabilities.toml` product or mir columns (`*product*`, `*mir_op*`, `*_product_backends_real`, etc.).
2. **Keep** `inventory_meta`; `docs_and_capability`; `capability_thin` / floor / exit; props docs_honesty×3; stdlib `docs_honesty`×1; `cache_v111_*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; hasher.
3. **Keep** colocated behavioral / `run()` / loopback tests in the same files.
4. **Defer** `hostfn_*.rs` cap pins (esp. `hostfn_sql`×27); tls/net GA thin seas.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −66 soft-nail tests (residual×28 + m2 product/mir caps×38; plan estimate was ≈77); residual deletable stdlib soft still large (~160).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness complete / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- residual_*_thin + m2 product/mir caps soft subclass closes; residual soft path includes hostfn_* / tls_net / misc + KEEP tips.
- Next: hostfn cap pins or tls/net GA thin; claim-readiness only after soft is no longer a primary path.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-stdlib-m2-cap-ci-wires-soft-retire](stage2-stdlib-m2-cap-ci-wires-soft-retire.md) · S2-I126
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
