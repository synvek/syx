# ADR: Stage-2 fingerprint tip knife (S2-I57+)

**Status:** Accepted (S2-I57; amended S2-I58…S2-I68)  
**Date:** 2026-11-03  
**Context:** After S2-I56, doctor-report tip/nail packs are exhausted except product `capabilities_report`. Remaining S2-01 mass is dominated by fingerprint towers (`fingerprint_fn`≈655 / `fingerprint_const`≈241). Many historical `heap_v*_fingerprint` helpers are **orphan** (definition only; masked by `#![allow(dead_code)]`; not on `compute_cache_version_tag`). Census also counts **`#[test] fn *_fingerprint`** (e.g. `syx-lsp/src/lsp_phase*_real.rs` include_str nails) as false positives. Precedent: [stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md), [stage2-doctor-report-run-test-nail-knife](stage2-doctor-report-run-test-nail-knife.md).

## Decision

1. **Orphan `*_fingerprint` helpers may be retired** when they have **no** product/runtime callers. Window-end must leave **no** test `include_str`/`contains` nails of the deleted fn names — pure orphans delete the body alone; helpers pinned only by `driver_gate`/`include_str` nails may be retired **in the same window** by deleting the function **and** removing those nails (parallel to doctor-report run-test-nail).
2. **Census false-positive `#[test] fn *_fingerprint`** (including `syx-lsp/src/lsp_phase*_real.rs` include_str nails) **may be deleted** as tip packs — remove the test function only; do **not** rewrite product cache strings. Default pack size on this test-noise axis: **Δ16–32** (replaces Δ3–4 as the default for that axis). Product aggregator tip peels remain separately judged (still require `format!` `{}`/arg alignment).
3. **S2-I57 deletes orphan tip packs×4** (not on cache aggregator):
   - `heap_v90_phase118_track_a_gui_fingerprint`
   - `heap_v90_phase118_track_b_wasm_fingerprint`
   - `heap_v90_phase118_track_c_farm_fingerprint`
   - `heap_v90_phase118_track_d_dist_select_fingerprint`
4. **Expected census (S2-I57):** `fingerprint_fn_matches` **655 → 651 (−4)**; `syx_driver_src_report_rs` stays **1**; `inventory_fn_matches` stays **0**.
5. **Later windows may continue packs under this ADR** without a new decision document. Peeling `compute_cache_version_tag` aggregator tips requires same-window `format!` `{}`/arg alignment and is allowed under this ADR only when the tip has no external readers beyond the aggregator.
6. **S2-I58 deletes nail-co-delete tip packs×4** (not on cache aggregator; same-window narrow of `p104_driver_heap_v76_fingerprint`):
   - `heap_v76_phase104_track_a_gui_fingerprint`
   - `heap_v76_phase104_track_b_wasm_fingerprint`
   - `heap_v76_phase104_track_c_fingerprint`
   - `heap_v76_phase104_track_d_fingerprint`
7. **Expected census (S2-I58):** `fingerprint_fn_matches` **651 → 647 (−4)**; report **1**; inventory **0**. Keeps `heap_v76_phase104_dx_fingerprint` and `heap_v90_phase118_dx_fingerprint`.
8. **S2-I59 deletes LSP census test-noise tip packs×31** (`#[test] fn *_fingerprint` in `lsp_phase86_real`…`lsp_phase90_real` only). Expected census: `fingerprint_fn_matches` **647 → 616 (−31)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag`.
9. **S2-I60 deletes LSP census test-noise tip packs×34** (`#[test] fn *_fingerprint` in `lsp_phase91_real`…`lsp_phase95_real`; contiguous phase band, slightly above default Δ32). Expected census: `fingerprint_fn_matches` **616 → 582 (−34)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag`.
10. **S2-I61 deletes LSP census test-noise tip packs×29** (`#[test] fn *_fingerprint` in `lsp_phase96_real`…`lsp_phase100_real`). Expected census: `fingerprint_fn_matches` **582 → 553 (−29)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag`.
11. **S2-I62 deletes remaining LSP census test-noise tip packs×24** (phase79–85 / 101–103 / 131–139 `#[test] fn *_fingerprint`). Expected census: `fingerprint_fn_matches` **553 → 529 (−24)**; report **1**; inventory **0**. **LSP `lsp_phase*_real.rs` `#[test] *_fingerprint` axis closed.** Does **not** touch `compute_cache_version_tag` or driver `tests/**` fingerprint tests.
12. **S2-I63 deletes driver-tests census tip packs×29** (`#[test] fn *_fingerprint` in `driver_gate.rs`×16 + `property_observers.rs`×8 + `native_gui_parity.rs`×5). Expected census: `fingerprint_fn_matches` **529 → 500 (−29)**; report **1**; inventory **0**. **Driver tests fingerprint test-noise axis opened.** Does **not** touch `compute_cache_version_tag` or product `src/` fingerprint bodies.
13. **S2-I64 deletes driver-tests census tip packs×30** (`#[test] fn *_fingerprint` in `tests/farm`×18 + `tests/gui`×7 + `tests/wasm`×5). Expected census: `fingerprint_fn_matches` **500 → 470 (−30)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag` or product `src/` fingerprint bodies.
14. **S2-I65 deletes driver-tests census tip packs×29** (`#[test] fn *_fingerprint` in `tests/runtime`×15 + `tests/data`×14). Expected census: `fingerprint_fn_matches` **470 → 441 (−29)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag` or product `src/` fingerprint bodies.
15. **S2-I66 deletes driver-tests census tip packs×36** (`#[test] fn *_fingerprint` in `tests/platform` only; slightly above default Δ32, same precedent as S2-I60). Expected census: `fingerprint_fn_matches` **441 → 405 (−36)**; report **1**; inventory **0**. Does **not** touch `compute_cache_version_tag` or product `src/` fingerprint bodies.
16. **S2-I67 deletes remaining driver-tests census tip packs×22** (`#[test] fn *_fingerprint` in `tests/native`×13 + `tests/concurrency`×9). Expected census: `fingerprint_fn_matches` **405 → 383 (−22)**; report **1**; inventory **0**. **Driver `tests/**` `#[test] *_fingerprint` axis closed.** Does **not** touch `compute_cache_version_tag` or product `src/` fingerprint bodies.
17. **S2-I68 deletes orphan+sparse tip packs×5** (product orphans×3 not on aggregator: `heap_v75_phase103_track_a_fingerprint`, `heap_v76_phase104_dx_fingerprint`, `heap_v90_phase118_dx_fingerprint`; plus non-driver `#[test] fn *_fingerprint`×2: `pipeline_order_fingerprint`, `p120_macros_exports_schema_fingerprint`). Expected census: `fingerprint_fn_matches` **383 → 378 (−5)**; report **1**; inventory **0**. **Tip-knife orphan axis closed.** Does **not** peel/fold `compute_cache_version_tag`.
18. **MUST STAY:** `capabilities_report`; fingerprints on the cache aggregator that remain product cache surface; `farm_lease_*_fingerprint` product APIs; near-tip `heap_v111_*`; five B-09 product flags; `llvm_stub_audit_report`. Do not confuse with wasm const `PHASE118_TRACK_B_WASM_FINGERPRINTS`.
19. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- **S2-I57:** orphan packs×4 deleted; `fingerprint_fn` **655 → 651**; cache aggregator unchanged.
- **S2-I58:** v76 phase104 track a–d×4 deleted + `p104_driver_heap_v76_fingerprint` narrowed to `dx` only; `fingerprint_fn` **651 → 647**; cache aggregator unchanged.
- **S2-I59:** LSP phase86–90 `#[test] *_fingerprint`×31 deleted; `fingerprint_fn` **647 → 616**; cache aggregator unchanged; later test-noise windows default **Δ16–32**.
- **S2-I60:** LSP phase91–95 `#[test] *_fingerprint`×34 deleted; `fingerprint_fn` **616 → 582**; cache aggregator unchanged.
- **S2-I61:** LSP phase96–100 `#[test] *_fingerprint`×29 deleted; `fingerprint_fn` **582 → 553**; cache aggregator unchanged.
- **S2-I62:** LSP remaining `#[test] *_fingerprint`×24 deleted; `fingerprint_fn` **553 → 529**; LSP phase-real fingerprint test-noise axis closed; driver tests fingerprint mass remains.
- **S2-I63:** driver tests tip×29 deleted (`driver_gate` + `property_observers` + `native_gui_parity`); `fingerprint_fn` **529 → 500**; driver tests fingerprint test-noise axis opened.
- **S2-I64:** driver tests tip×30 deleted (`tests/farm` + `gui` + `wasm`); `fingerprint_fn` **500 → 470**.
- **S2-I65:** driver tests tip×29 deleted (`tests/runtime` + `data`); `fingerprint_fn` **470 → 441**.
- **S2-I66:** driver tests tip×36 deleted (`tests/platform`); `fingerprint_fn` **441 → 405**.
- **S2-I67:** driver tests tip×22 deleted (`tests/native` + `concurrency`); `fingerprint_fn` **405 → 383**; driver tests fingerprint test-noise axis closed.
- **S2-I68:** orphan×3 + non-driver nails×2 deleted; `fingerprint_fn` **383 → 378**; tip-knife orphan axis closed; aggregator unchanged (fold deferred to separate ADR).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
