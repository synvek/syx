# ADR: Stage-2 doctor-report driver-src-nail knife (S2-I54+)

**Status:** Accepted (S2-I54)  
**Date:** 2026-11-03  
**Context:** After S2-I53, tip packs are exhausted except `heap_bridge_report` (Δ1), and LSP-nail under [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md) has only `rt_runtime_report` left (Δ1; `gc_relocate_report` blocked by `run_*`). Remaining doctor-report mass includes modules whose **only** driver-test coupling is `include_str!(…*_report.rs)` source nails — **no** `run_*` behavioral calls and **no** product-runtime readers beyond CLI Doctor. Precedent: S2-I53 LSP-nail heavier ADR; [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md).

## Decision

1. **Doctor `*_report` modules may be retired** when they have **no** `syx-driver/tests` `run_*` behavioral calls and **no** product runtime/LSP `run_*` readers beyond CLI Doctor, even if driver tests `include_str` the report source (and/or LSP/CLI assert flag strings) — by deleting the module + `lib.rs` `pub mod` + CLI wiring + updating/removing those source nails in the **same window**.
2. **S2-I54 deletes tip + driver-src-nail packs×4:**
   - `heap_bridge_report` (tip residual; [tip ADR](stage2-doctor-report-tip-knife.md))
   - `native_profile_report`
   - `farm_billing_report`
   - `dist_cdn_route_report`
3. **Expected census (S2-I54):** `syx_driver_src_report_rs` **10 → 6 (−4)**; `inventory_fn_matches` stays **0**.
4. **Later windows may continue packs under this ADR** (default Δ3–4) without a new decision document, while keeping `capabilities_report` / `run_*`-coupled reports / fingerprint / five flags / `llvm_stub_audit_report` OUT unless separately decided.
5. **MUST STAY:** `capabilities_report`; reports with driver `run_*` behavioral tests; fingerprint / lang_props mass; five B-09 product flags; `llvm_stub_audit_report`.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ retiring entire `syx doctor` / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- Doctor help text loses the retired `--*-report` flags (`--heap-bridge-report`, `--native-profile-report`, `--farm-billing-report`, `--dist-cdn-route-report`).
- Driver/LSP source nails for deleted modules/flags are retired or retargeted.
- **S2-I54:** tip + driver-src-nail packs×4 deleted; report census **10 → 6**; inventory stays **0**.
- **S2-I55+:** `run_*` test-coupled doctor reports move to [stage2-doctor-report-run-test-nail-knife](stage2-doctor-report-run-test-nail-knife.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md)
- [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
