# ADR: Stage-2 open (architecture convergence)

**Status:** Accepted (S2-I1)  
**Date:** 2026-11-01  
**Context:** Stages 0 and 1 have exited ([stage1-exit](stage1-exit.md)). Default credibility work shifts to Stage-2: remove process-governance mass from product `src/`, behavior-driven tests, VM←MIR honesty, parser unification, file splits, workspace slim, single public version — per [plan/02](../../plan/02-improvement-roadmap.md) S2-01…S2-07.

## Decision

1. **Stage-2 is open** as of S2-I1. Default construction axis is **S2-01 → S2-07**.
2. **First slice (this window):** S2-01 **governance surface census + baseline** only ([docs/stage2-governance-census.md](../stage2-governance-census.md) + [`scripts/s2_governance_census.sh`](../../scripts/s2_governance_census.sh)). Migration/deletion of inventory/fingerprint/report mass is **later windows**.
3. **Out / non-claims for S2-I1:**
   - Completing S2-01 `[x]` or Stage-2 exit;
   - Implementing S2-03…S2-07 in this window;
   - Unparking product umbrella / F-08 / HostFn+1;
   - Claiming examples 拉满 or mono/diamond GA.

## Consequences

- `plan/01` / `plan/02` mark Stage-2 as **`[~]`** (window label advances per slice).
- S2-01 stays **`[~]`** until product binaries no longer link process-governance inventory/fingerprint/gate narratives (acceptance in plan/02).
- **S2-I2:** deleted driver W932–W954 inventory chain; census `inventory_fn` **237 → 214** ([census](../stage2-governance-census.md)).
- **S2-I3:** deleted driver W955–W965 `syx_math_llvm_*` chain; census **214 → 203**.
- **S2-I4:** truncated codegen gate_b01+b02 inventory tail; census **203 → 186**（≠ S2-03 / B-01·B-02 产品收官）.
- **S2-I5:** deleted a06 + profiles_sdk + lsp editor interop + frontend entry-matrix; census **186 → 178**.
- **S2-I6:** deleted mir short post-cutover tail + `language_exports_*`; census **178 → 160**（kept `cutover_readiness` for wave_b）.
- **S2-I7:** deleted mir `long_horizon_*`×25 (+ companions); census **160 → 132**.
- **S2-I8:** deleted mir medium post-cutover (`honest_defer`→`exit`×11); census **132 → 121**.
- **S2-I9:** deleted mir bypass×4 + llvm post-clear tail×2; census **121 → 115**.
- **S2-I10:** deleted wave_b residual ABI×2; census **115 → 113**.
- **S2-I11:** cutover product decouple ([stage2-cutover-decouple](stage2-cutover-decouple.md)); deleted 3 rollups; census **113 → 110**.
- **S2-I12:** deleted short export/blob/absolute feed (+ inventory helper/companion); census **110 → 102**.
- **S2-I13:** deleted medium metadata/dual_blob feed×5 (+ encode/decode_probe); census **102 → 97**.
- **S2-I14:** deleted llvm clear upstream declare_diff + missing_clear; census **97 → 95**.
- **S2-I15:** renamed 10 driver test-named `*_inventory` false positives; census **95 → 85** (product `src/` still ≈85).
- **S2-I16:** retired wave_b readiness inventories ([stage2-wave-b-inventory-retire](stage2-wave-b-inventory-retire.md)); census **85 → 83**.
- **S2-I17:** HostFn←capability_align decouple ([stage2-hostfn-align-decouple](stage2-hostfn-align-decouple.md)); census **83 → 82**.
- **S2-I18:** b09 product-flag tip decouple ([stage2-b09-product-flag-decouple](stage2-b09-product-flag-decouple.md)); census **82 → 78**.
- **S2-I19:** b09 construction tip peel 两联 ([stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md)); census **78 → 72**.
- **S2-I20:** b09 construction tip dual_track+mirop 两联 (same ADR); census **72 → 66**.
- **S2-I21:** b09 construction tip objectref peel 三联 (same ADR); census **66 → 63**.
- **S2-I22:** b09 construction tip deep_field+strip_rc 两联 (same ADR); census **63 → 59**.
- **S2-I23:** b09 construction tip handle 三联 (same ADR); census **59 → 56**.
- **S2-I24:** b09 construction tip nested_field (same ADR); census **56 → 55**.
- **S2-I25:** b09 construction tip refresh+wrap+value 三联 (same ADR); census **55 → 52**.
- **S2-I26:** b09 construction tip death-path 四联 (same ADR); census **52 → 48**.
- **S2-I27:** b09 construction tip opcode retire 三联 (same ADR); census **48 → 45**.
- **S2-I28:** b09 construction tip retire-readiness 三联 (same ADR); census **45 → 42**.
- **S2-I29:** b09 construction tip migrate_readiness 单枚 (same ADR); census **42 → 41**.
- **S2-I30:** b09 construction tip heap_slots 三联 (same ADR); census **41 → 38**.
- **S2-I31:** b09 construction tip llvm wrap emit 单枚 (same ADR); census **38 → 37**.
- **S2-I32:** b09 construction tip arrayget+residual_birth 两联 (same ADR); census **37 → 35**.
- **S2-I33:** b09 construction tip llvm_wasm emit-retire 单枚 (same ADR); census **35 → 34**.
- **S2-I34:** b09 construction tip retain/method/actor 三联 (same ADR); census **34 → 31**.
- **S2-I35:** b09 construction tip call-return 两联 (same ADR); census **31 → 29**.
- **S2-I36:** b09 construction tip mutrefcell CallHost 两联 (same ADR); census **29 → 27**.
- **S2-I37:** b09 construction tip mutrefcell DictBuild 两联 (same ADR); census **27 → 25**.
- **S2-I38:** b09 construction tip objectref CallHost 两联 (same ADR); census **25 → 23**.
- **S2-I39:** b09 construction tip objectref DictBuild 单枚 (same ADR); census **23 → 22**.
- **S2-I40:** b09 construction tip objectref vm_heap_slots 单枚 (same ADR); census **22 → 21**.
- **S2-I41:** b09 construction tip objectref vm_managed_heap_probe 单枚 (same ADR); census **21 → 20**.
- **S2-I42:** b09 construction tip objectref_rc_migrate 单枚 (same ADR); census **20 → 19**.
- **S2-I43:** b09 construction tip memory_pool_dead_shell 单枚 (same ADR); census **19 → 18**.
- **S2-I44:** b09 construction tip storage_cleanup 单枚 (same ADR); census **18 → 17**.
- **S2-I45:** b09 construction tip 四联 (same ADR; default Δ3–4); census **17 → 13**.
- **S2-I46:** b09 construction tip 四联 (same ADR); census **13 → 9**.
- **S2-I47:** orphan thin + HostFn tip 三联 ([stage2-hostfn-construction-tip-knife](stage2-hostfn-construction-tip-knife.md)); census **9 → 5**.
- **S2-I48:** HostFn tip 四联 (same ADR); census **5 → 1**.
- **S2-I49:** HostFn thin 末刀 (same ADR); census **1 → 0** (inventory KPI zero; ≠ S2-01 `[x]`).
- **S2-I50:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](stage2-doctor-report-tip-knife.md)); report census **26 → 22**.
- **S2-I51:** doctor-report tip 四联 (same ADR); report census **22 → 18**.
- **S2-I52:** doctor-report tip 四联 (same ADR); report census **18 → 14**.
- **S2-I53:** doctor-report LSP-nail 四联 ([stage2-doctor-report-lsp-nail-knife](stage2-doctor-report-lsp-nail-knife.md)); report census **14 → 10**.
- **S2-I54:** tip 末枚 + driver-src-nail 三联 ([stage2-doctor-report-driver-src-nail-knife](stage2-doctor-report-driver-src-nail-knife.md)); report census **10 → 6**.
- **S2-I55:** LSP 末枚 + run-test-nail 三联 ([stage2-doctor-report-run-test-nail-knife](stage2-doctor-report-run-test-nail-knife.md)); report census **6 → 2**.
- **S2-I56:** run-test-nail 末枚 `gui_backend_report` (same ADR); report census **2 → 1**.
- **S2-I57:** fingerprint orphan tip 四联 ([stage2-fingerprint-tip-knife](stage2-fingerprint-tip-knife.md)); `fingerprint_fn` **655 → 651**.
- **S2-I58:** fingerprint tip nail-co-delete 四联 (same ADR); `heap_v76_phase104` track a–d; `fingerprint_fn` **651 → 647**.
- **S2-I59:** fingerprint census test-noise tip×31 (same ADR; LSP phase86–90 `#[test] *_fingerprint`); `fingerprint_fn` **647 → 616**; later test-noise default **Δ16–32**.
- **S2-I60:** fingerprint census test-noise tip×34 (same ADR; LSP phase91–95); `fingerprint_fn` **616 → 582**.
- **S2-I61:** fingerprint census test-noise tip×29 (same ADR; LSP phase96–100); `fingerprint_fn` **582 → 553**.
- **S2-I62:** fingerprint census test-noise tip×24 (same ADR; LSP remaining phase79–85/101–103/131–139); `fingerprint_fn` **553 → 529**; LSP phase-real `*_fingerprint` test-noise axis closed.
- **S2-I63:** fingerprint census test-noise tip×29 (same ADR; driver `driver_gate` + `property_observers` + `native_gui_parity`); `fingerprint_fn` **529 → 500**; driver tests fingerprint test-noise axis opened.
- **S2-I64:** fingerprint census test-noise tip×30 (same ADR; driver `tests/farm` + `gui` + `wasm`); `fingerprint_fn` **500 → 470**.
- **S2-I65:** fingerprint census test-noise tip×29 (same ADR; driver `tests/runtime` + `data`); `fingerprint_fn` **470 → 441**.
- **S2-I66:** fingerprint census test-noise tip×36 (same ADR; driver `tests/platform`); `fingerprint_fn` **441 → 405**.
- **S2-I67:** fingerprint census test-noise tip×22 (same ADR; driver `tests/native` + `concurrency`); `fingerprint_fn` **405 → 383**; driver tests fingerprint test-noise axis closed.
- **S2-I68:** fingerprint tip orphan+sparse×5 (same ADR; orphan×3 + frontend/macro nails×2); `fingerprint_fn` **383 → 378**; tip-knife orphan axis closed; aggregator fold deferred.
- **S2-I69:** fingerprint aggregator fold Δ6 ([stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md); v3/v4/v48–v51 → `heap_v52`); `fingerprint_fn` **378 → 372**.
- **S2-I70:** fingerprint aggregator fold Δ16 (same ADR; v53…v68 → `heap_v69`; soft-nail retarget); `fingerprint_fn` **372 → 356**.
- **S2-I71:** fingerprint aggregator fold Δ16 (same ADR; v70…v78_dx → `heap_v79_track_a`; soft-nail retarget×6); `fingerprint_fn` **356 → 340**.
- **S2-I72:** fingerprint aggregator fold Δ16 (same ADR; v79_b…v82_b → `heap_v82_track_c`; soft-nail-free); `fingerprint_fn` **340 → 324**.
- **S2-I73:** fingerprint aggregator fold Δ16 (same ADR; v82_d…v85_d → `heap_v85_dx`; soft-nail-free); `fingerprint_fn` **324 → 308**.
- **S2-I74:** fingerprint aggregator fold Δ16 (same ADR; v86_a…v89_a → `heap_v89_track_b`; soft-nail-free); `fingerprint_fn` **308 → 292**.
- **S2-I75:** fingerprint aggregator fold Δ16 (same ADR; v89_c…v93_c → `heap_v93_track_d`; soft-nail retarget); `fingerprint_fn` **292 → 276**.
- **S2-I76:** fingerprint aggregator fold Δ16 (same ADR; v93_dx…v96_dx → `heap_v97_track_a`; soft-nail-free); `fingerprint_fn` **276 → 260**.
- **S2-I77:** fingerprint aggregator fold Δ16 (same ADR; v97_b…v100_b → `heap_v100_track_c`; soft-nail-free); `fingerprint_fn` **260 → 244**.
- **S2-I78:** fingerprint aggregator fold Δ16 (same ADR; v100_d…v104_d → `heap_v104_dx`; soft-nail retarget); `fingerprint_fn` **244 → 228**.
- **S2-I79:** fingerprint aggregator fold Δ16 (same ADR; v103_a…v106_e → `heap_v106_dx`; soft-nail-free); `fingerprint_fn` **228 → 212**.
- **S2-I80:** fingerprint aggregator fold Δ16 (same ADR; v107_a…v109_dx → `heap_v110_track_a`; soft-nail retarget); `fingerprint_fn` **212 → 196**.
- **S2-I81:** fingerprint aggregator fold Δ4 (same ADR; v110 c/d/e/dx → `heap_v110_track_a`; soft-nail narrowed); `fingerprint_fn` **196 → 192**; contiguous pre-`v111` foldable tips exhausted.
- **S2-I82:** earlier kept-tip re-fold Δ8 (same ADR; v52…v97_a → `heap_v100_track_c`; soft-nail retarget); `fingerprint_fn` **192 → 184**.
- **S2-I83:** earlier kept-tip re-fold Δ3 (same ADR; v104_dx/v106_dx/v110_a → `heap_v100_track_c`; soft-nail retarget); `fingerprint_fn` **184 → 181**; pre-`v111` earlier-tip axis exhausted.
- **S2-I84:** capabilities product surface 另策 ([stage2-capabilities-report-surface](stage2-capabilities-report-surface.md); `capabilities_report.rs` → `capabilities.rs`); `syx_driver_src_report_rs` **1 → 0**; `fingerprint_fn` **181**.
- **S2-I85:** fingerprint v111 fold Δ4 ([stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md); c/d/inherit/dx → `track_l`; soft-nail narrowed); `fingerprint_fn` **181 → 177**.
- **S2-I86:** fingerprint foundation fold Δ12 ([stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md); foundation×12 → `v100_c`; soft-nail retarget); `fingerprint_fn` **177 → 165**.
- **S2-I87:** fingerprint track_l fold Δ1 ([stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md); `track_l` → `v100_c`; soft-nail retarget); `fingerprint_fn` **165 → 164**; on-agg single tip.
- **S2-I88:** product-sea cdn_bgp rename Δ11 ([stage2-fingerprint-product-sea-cdn-bgp-rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md); sticky/peer-announce `*_fingerprint` → `*_digest`); `fingerprint_fn` **164 → 153**.
- **S2-I89:** product-sea async dist/cdn gate rename Δ8 ([stage2-fingerprint-product-sea-async-dist-rename](stage2-fingerprint-product-sea-async-dist-rename.md); main gates `*_fingerprint` → `*_digest`); `fingerprint_fn` **153 → 145**; syx-async product-sea fingerprint surface cleared.
- **S2-I90:** product-sea select channel rename Δ14 ([stage2-fingerprint-product-sea-select-channel-rename](stage2-fingerprint-product-sea-select-channel-rename.md); parser+vm `select_*_fingerprint` → `*_digest`); `fingerprint_fn` **145 → 131**.
- **S2-I91:** product-sea gui shape-core rename Δ9 ([stage2-fingerprint-product-sea-gui-shape-rename](stage2-fingerprint-product-sea-gui-shape-rename.md); shape `*_fingerprint` → `*_digest`; façades kept); `fingerprint_fn` **131 → 122**.
- **S2-I92:** product-sea gui shape façade rename Δ9 ([stage2-fingerprint-product-sea-gui-shape-facade-rename](stage2-fingerprint-product-sea-gui-shape-facade-rename.md); `gui_*_fingerprint` → `gui_*_digest`; C-ABI/VM host kept); `fingerprint_fn` **122 → 113**.
- **S2-I93:** product-sea gui render leaf rename Δ14 ([stage2-fingerprint-product-sea-gui-leaf-rename](stage2-fingerprint-product-sea-gui-leaf-rename.md); leaf+façade `*_fingerprint` → `*_digest`; C-ABI/VM host kept); `fingerprint_fn` **113 → 99**.

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · Stage 2
- [`docs/adr/stage1-exit.md`](stage1-exit.md)
- [`docs/adr/stage2-cutover-decouple.md`](stage2-cutover-decouple.md) · S2-I11
- [`docs/adr/stage2-wave-b-inventory-retire.md`](stage2-wave-b-inventory-retire.md) · S2-I16
- [`docs/adr/stage2-hostfn-align-decouple.md`](stage2-hostfn-align-decouple.md) · S2-I17
- [`docs/adr/stage2-b09-product-flag-decouple.md`](stage2-b09-product-flag-decouple.md) · S2-I18
- [`docs/adr/stage2-b09-construction-tip-knife.md`](stage2-b09-construction-tip-knife.md) · S2-I19
- [`docs/adr/stage2-hostfn-construction-tip-knife.md`](stage2-hostfn-construction-tip-knife.md) · S2-I47
- [`docs/adr/stage2-doctor-report-tip-knife.md`](stage2-doctor-report-tip-knife.md) · S2-I50
- [`docs/adr/stage2-doctor-report-lsp-nail-knife.md`](stage2-doctor-report-lsp-nail-knife.md) · S2-I53
- [`docs/adr/stage2-doctor-report-driver-src-nail-knife.md`](stage2-doctor-report-driver-src-nail-knife.md) · S2-I54
- [`docs/adr/stage2-doctor-report-run-test-nail-knife.md`](stage2-doctor-report-run-test-nail-knife.md) · S2-I55
- [`docs/adr/stage2-fingerprint-tip-knife.md`](stage2-fingerprint-tip-knife.md) · S2-I57
- [`docs/adr/stage2-fingerprint-aggregator-fold.md`](stage2-fingerprint-aggregator-fold.md) · S2-I69
- [`docs/adr/stage2-capabilities-report-surface.md`](stage2-capabilities-report-surface.md) · S2-I84
- [`docs/adr/stage2-fingerprint-v111-fold.md`](stage2-fingerprint-v111-fold.md) · S2-I85
- [`docs/adr/stage2-fingerprint-foundation-fold.md`](stage2-fingerprint-foundation-fold.md) · S2-I86
- [`docs/adr/stage2-fingerprint-track-l-fold.md`](stage2-fingerprint-track-l-fold.md) · S2-I87
- [`docs/adr/stage2-fingerprint-product-sea-cdn-bgp-rename.md`](stage2-fingerprint-product-sea-cdn-bgp-rename.md) · S2-I88
- [`docs/adr/stage2-fingerprint-product-sea-async-dist-rename.md`](stage2-fingerprint-product-sea-async-dist-rename.md) · S2-I89
- [`docs/adr/stage2-fingerprint-product-sea-select-channel-rename.md`](stage2-fingerprint-product-sea-select-channel-rename.md) · S2-I90
- [`docs/adr/stage2-fingerprint-product-sea-gui-shape-rename.md`](stage2-fingerprint-product-sea-gui-shape-rename.md) · S2-I91
- [`docs/adr/stage2-fingerprint-product-sea-gui-shape-facade-rename.md`](stage2-fingerprint-product-sea-gui-shape-facade-rename.md) · S2-I92
- [`docs/adr/stage2-fingerprint-product-sea-gui-leaf-rename.md`](stage2-fingerprint-product-sea-gui-leaf-rename.md) · S2-I93
- Analysis: [`plan/ProjectAnalysisAndImprovementPlan.zh-CN.md`](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §4.2.1 / 阶段2




