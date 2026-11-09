# ADR: Stage-2 doctor-report LSP-nail knife (S2-I53+)

**Status:** Accepted (S2-I53)  
**Date:** 2026-11-03  
**Context:** After S2-I52, pure zero-coupling doctor reports under [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md) are exhausted except `heap_bridge_report` (Δ1). Remaining mass includes reports whose **only extra coupling** is LSP `include_str` of the `*_report.rs` file and/or CLI-string nails in `lsp_phase*_real.rs` — no `syx-driver/tests` behavioral `run_*` / product-runtime readers. Precedent: S2-I52 retiring CLI-string nails with tip packs; [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md).

## Decision

1. **Doctor `*_report` modules may be retired** when they have **no** `syx-driver/tests` behavioral coupling and **no** product runtime/LSP `run_*` readers beyond CLI Doctor, even if LSP phases `include_str` the report source or assert CLI flag strings — by deleting the module + `lib.rs` `pub mod` + CLI wiring + updating/removing those LSP nails in the **same window**.
2. **S2-I53 deletes tip packs×4:**
   - `cache_remote_report`
   - `rt_exec_report`
   - `heap_host_report`
   - `farm_report`
3. **Expected census (S2-I53):** `syx_driver_src_report_rs` **14 → 10 (−4)**; `inventory_fn_matches` stays **0**.
4. **Later windows may continue packs under this ADR** (default Δ3–4) without a new decision document, while keeping `capabilities_report` / driver-test-coupled reports / fingerprint / five flags / `llvm_stub_audit_report` OUT unless separately decided.
5. **MUST STAY:** `capabilities_report`; reports with driver `run_*` / `include_str` behavioral tests; fingerprint / lang_props mass; five B-09 product flags; `llvm_stub_audit_report`.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ retiring entire `syx doctor` / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- LSP phase76/77 nails for deleted modules/flags are retired or retargeted.
- Doctor help text loses the retired `--*-report` flags (`--cache-remote-report`, `--rt-exec-report`, `--heap-host-report`, `--farm-report`).
- **S2-I53:** tip packs×4 deleted (cache_remote+rt_exec+heap_host+farm); report census **14 → 10**; inventory stays **0**.
- **S2-I54+:** tip 末枚 + driver-src `include_str` nails move to [stage2-doctor-report-driver-src-nail-knife](stage2-doctor-report-driver-src-nail-knife.md); `rt_runtime_report` remains for later LSP-nail continuation.
- **S2-I55:** `rt_runtime_report` deleted (LSP residual); `run_*` test-coupled reports move to [stage2-doctor-report-run-test-nail-knife](stage2-doctor-report-run-test-nail-knife.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
