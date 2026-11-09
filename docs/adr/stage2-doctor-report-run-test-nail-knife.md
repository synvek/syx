# ADR: Stage-2 doctor-report run-test-nail knife (S2-I55+)

**Status:** Accepted (S2-I55)  
**Date:** 2026-11-03  
**Context:** After S2-I54, tip/driver-src-nail packs are exhausted; LSP-nail under [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md) has only `rt_runtime_report` left (Δ1). Remaining doctor-report mass (except product `capabilities_report` and GUI-adjacent `gui_backend_report`) is coupled only by `syx-driver/tests` calling `run_*` (plus LSP/CLI string/`include_str` nails) — **no** product runtime readers beyond CLI Doctor. Precedent: S2-I54 driver-src-nail heavier ADR; [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md).

## Decision

1. **Doctor `*_report` modules may be retired** when they have **no** product runtime/LSP product readers beyond CLI Doctor, even if `syx-driver/tests` call `run_*` (and/or LSP/CLI assert flag strings / `include_str` the report source) — by deleting the module + `lib.rs` `pub mod` + CLI wiring + updating/removing those test nails (retarget to underlying APIs or delete the doctor-only assert) in the **same window**.
2. **S2-I55 deletes LSP residual + run-test-nail packs×4:**
   - `rt_runtime_report` (LSP-nail residual; [LSP-nail ADR](stage2-doctor-report-lsp-nail-knife.md))
   - `gc_relocate_report`
   - `reflect_report`
   - `hsm_rotation_report`
3. **Expected census (S2-I55):** `syx_driver_src_report_rs` **6 → 2 (−4)**; `inventory_fn_matches` stays **0**.
4. **Later windows may continue packs under this ADR** (default Δ3–4; e.g. `gui_backend_report` if still test-only) without a new decision document, while keeping `capabilities_report` / fingerprint / five flags / `llvm_stub_audit_report` OUT unless separately decided.
5. **MUST STAY:** `capabilities_report`; fingerprint / lang_props mass; five B-09 product flags; `llvm_stub_audit_report`.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ retiring entire `syx doctor` / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- Doctor help text loses the retired `--*-report` flags (`--rt-runtime-report`, `--gc-relocate-report`, `--reflect-report`, `--hsm-rotation-report`, `--gui-backend-report`).
- Driver/LSP nails for deleted modules/flags are retired or retargeted to underlying APIs.
- **S2-I55:** packs×4 deleted; report census **6 → 2**; inventory stays **0**.
- **S2-I56:** `gui_backend_report` deleted; GUI/driver nails retargeted to `syx_vm::gui_*`; report census **2 → 1**; inventory stays **0**.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md)
- [stage2-doctor-report-driver-src-nail-knife](stage2-doctor-report-driver-src-nail-knife.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
