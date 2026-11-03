# ADR: Stage-2 doctor-report tip knife (S2-I50+)

**Status:** Accepted (S2-I50)  
**Date:** 2026-11-03  
**Context:** After S2-I49, `fn *_inventory` census is **0**. Remaining S2-01 mass includes 26 `crates/syx-driver/src/*_report.rs` modules (all wired through `syx doctor`), fingerprint towers, and `lang_props_v*`. Several doctor reports have **no runtime/LSP readers and no driver-test coupling** beyond CLI flags — process-governance surface in product `src/`. Precedent: [stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md), [stage2-hostfn-construction-tip-knife](stage2-hostfn-construction-tip-knife.md).

## Decision

1. **CLI-only doctor `*_report` modules may be retired tip→deep** when they have no product runtime/LSP readers and no `syx-driver/tests` behavioral coupling, by deleting the module + `lib.rs` `pub mod` + matching `syx-cli` Doctor flag/branch/help text in the same window.
2. **S2-I50 deletes tip packs×4:**
   - `actor_ipc_report`
   - `dict_runtime_report`
   - `dist_failover_report`
   - `rt_v7_report`
3. **Expected census (S2-I50):** `syx_driver_src_report_rs` **26 → 22 (−4)**; `inventory_fn_matches` stays **0**.
4. **Later windows may continue tip packs under this ADR** (default Δ3–4) without a new decision document, while keeping `capabilities_report` / LSP- or test-coupled reports / fingerprint towers / five flags / `llvm_stub_audit_report` OUT unless separately decided.
5. **MUST STAY:** `capabilities_report`; reports with LSP/`include_str`/behavioral test coupling; fingerprint / lang_props mass; five B-09 product flags; `llvm_stub_audit_report`.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ retiring entire `syx doctor` / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- Doctor help text loses the retired `--*-report` flags.
- Historical Wave / LSP phase docs may still name deleted symbols as archival evidence.
- **S2-I50:** tip packs×4 deleted; report census **26 → 22**.
- **S2-I51:** tip packs×4 deleted (dist_actor_cluster+heap_bridge_prod+heapslot_parity+native_sdk); report census **22 → 18**.
- **S2-I52:** tip packs×4 deleted (gc+native_exec_op+inkwell+gc_generational); report census **18 → 14**; LSP CLI-string nails retired with flags.
- **S2-I53+:** pure tip packs exhausted except `heap_bridge_report`; LSP-`include_str`/CLI-string-only nails move to [stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md).
- **S2-I54:** tip 末枚 `heap_bridge_report` deleted; driver `include_str` source nails move to [stage2-doctor-report-driver-src-nail-knife](stage2-doctor-report-driver-src-nail-knife.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
