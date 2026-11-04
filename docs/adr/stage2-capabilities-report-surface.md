# ADR: Stage-2 capabilities report surface (S2-I84)

**Status:** Accepted (S2-I84)  
**Date:** 2026-11-04  
**Context:** After S2-I83, tip-knife and pre-`v111` earlier-tip aggregator fold axes are exhausted. Remaining S2-01 `syx_driver_src_report_rs` census is **1** — solely `crates/syx-driver/src/capabilities_report.rs`. That module is a **product** surface (CLI `syx doctor --capabilities` / `--ir-capabilities`, LSP, website/hosted/marketing helpers), not a process-governance tip eligible for doctor-report tip-knife delete. Fold ADR Decision §21 deferred this to a separate capabilities 另策. Census counts `find … -maxdepth 1 -name '*_report.rs'`, so a rename out of that glob clears the KPI without retiring the API.

## Decision

1. **S2-I84** renames product module file `capabilities_report.rs` → `capabilities.rs` and `pub mod capabilities_report` → `pub mod capabilities` in the same window.
2. **MUST STAY (API):** `run_capabilities_report` / `run_ir_capabilities_report` / `print_*` / `CapabilitiesReport` types; CLI flags; LSP / website / hosted / marketing callers (update import paths only).
3. **Expected census:** `syx_driver_src_report_rs` **1 → 0 (−1)**; `fingerprint_fn_matches` **181**; `inventory_fn_matches` **0**.
4. Soft-nail / `include_str` / A-10·website doc paths that name `capabilities_report.rs` **must** retarget to `capabilities.rs` in the same window.
5. Does **not** touch fingerprint aggregator / foundation / `heap_v111_*` / `farm_lease_*` defs / HostFn.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ deleting doctor capabilities / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Doctor-report tip-knife MUST STAY on the **filename** `*_report.rs` no longer applies; product capabilities live under `capabilities` mod.
- Fold ADR MUST STAY wording for `capabilities_report` means the **product API surface**, not the census filename.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md)
- [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
