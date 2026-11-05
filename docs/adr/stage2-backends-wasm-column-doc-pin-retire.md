# ADR: Stage-2 S2-02 backends wasm column / doc pin soft retire (S2-I122)

**Status:** Accepted (S2-I122) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I121 LSP historical pure soft retire](stage2-lsp-historical-pure-soft-retire.md), remaining large soft-nail mass in backends includes pure `include_str!(…).contains(…)` column / capability-table pins under `*_native_wasm_{matrix,ga}.rs` (toml / generated md / product columns). Behavioral `run_*` matrix·GA tests in the same files already cover host behavior. Continues the backends soft axis of [S2-I116–I118](stage2-backends-hostfn-availability-ga67-meta-retire.md).

## Decision

1. **Authorize deletion** of **47** pure column/doc-pin `#[test]` functions under [`crates/syx-driver/tests/backends/`](../../crates/syx-driver/tests/backends/) matching `*_native_wasm_{matrix,ga}.rs` that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `run(` compile-run paths.
2. **Keep** all `run_*` / matrix·GA behavioral tests in those eight files.
3. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
4. **Do not** change LSP floors / hybrid×36 / hasher / `cache_v111`.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −47 soft-nail tests.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Backends wasm column soft sea closes; follow-on hybrid fingerprint stub retire: [S2-I123](stage2-lsp-hybrid-fingerprint-stub-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-lsp-historical-pure-soft-retire](stage2-lsp-historical-pure-soft-retire.md) · S2-I121
- [stage2-backends-hostfn-availability-ga67-meta-retire](stage2-backends-hostfn-availability-ga67-meta-retire.md) · S2-I118
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
