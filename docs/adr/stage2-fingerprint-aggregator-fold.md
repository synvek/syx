# ADR: Stage-2 fingerprint aggregator fold (S2-I69+)

**Status:** Accepted (S2-I69; amended S2-I70…S2-I83)  
**Date:** 2026-11-03  
**Context:** After S2-I68, tip-knife orphan / driver / sparse-nail axes are closed. Remaining S2-01 `fingerprint_fn` mass is dominated by on-aggregator `heap_*_fingerprint` helpers in `compute_cache_version_tag`. Bare peel (drop `format!` slots and discard hasher bytes) would lose cache-invalidation inputs. Precedent: [stage2-fingerprint-tip-knife](stage2-fingerprint-tip-knife.md) (explicitly deferred fold).

## Decision

1. **Fold** = merge N tip helpers' hasher input statements (`hasher.update` byte literals, interleaved `HostFnId::all()` loops, **and** any nested `farm_lease_*_fingerprint()` call expressions used as hasher inputs — product API **definitions** stay) into **one successor** or an **adjacent kept helper**; then delete the folded definitions and remove their `{}`/args from `compute_cache_version_tag` in the **same window**. **Bare peel is forbidden.**
2. Soft-nail / `driver_gate` / LSP `contains("…_fingerprint")` names **must not** be deleted without co-editing those nails **in the same window** (retarget `contains` to the kept merge-target name, or delete the assert).
3. Default pack size on this fold axis: **Δ8–16** `fingerprint_fn` census drop (prefer keeping the merge-target name to avoid +1). Smaller Δ3–8 packs remain allowed when soft-nail/HostFn risk is high.
4. **S2-I69 first pack Δ6** — merge into `heap_v52_phase80_fingerprint` (kept name); delete:
   - `heap_v3_llvm_tier3_fingerprint`
   - `heap_v4_inkwell_fingerprint`
   - `heap_v48_phase76_fingerprint`
   - `heap_v49_phase77_fingerprint`
   - `heap_v50_phase78_fingerprint`
   - `heap_v51_phase79_fingerprint`
5. **Byte order inside `heap_v52_phase80_fingerprint` after fold:** v3 updates → v4 → v48 → v49 → v50 → v51 → original v52 updates.
6. **Expected census (S2-I69):** `fingerprint_fn_matches` **378 → 372 (−6)**; `syx_driver_src_report_rs` **1**; `inventory_fn_matches` **0**.
7. **S2-I70 pack Δ16** — merge into `heap_v69_phase97_fingerprint` (kept name); delete contiguous `heap_v53_phase81_fingerprint` … `heap_v68_phase96_fingerprint`. Same-window soft-nail retarget: `driver_gate` `contains("heap_v58_phase86_fingerprint")` → `contains("heap_v69_phase97_fingerprint")`. **Byte order:** v53 → … → v68 → original v69 statements. Expected census: **372 → 356 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` or `heap_v70+`.
8. **S2-I71 pack Δ16** — merge into `heap_v79_phase107_track_a_gui_fingerprint` (kept name); delete aggregator-contiguous tip×16:
   - `heap_v70_phase98_fingerprint` … `heap_v75_phase103_fingerprint`
   - `heap_v77_phase105_track_a_gui_fingerprint` / `_track_b_wasm_` / `_track_c_` / `_track_d_` / `_dx_`
   - `heap_v78_phase106_track_a_gui_fingerprint` / `_track_b_wasm_` / `_track_c_select_reflect_` / `_track_d_farm_` / `_dx_`
   Same-window soft-nail retarget → merge target: `driver_gate` (v72/v73/v74/v75/v77_track_d) + `syx-lsp` `lsp_phase100_real` (v72). **Byte order:** v70 → … → v75 → v77_a…dx → v78_a…dx → original v79_track_a. Expected census: **356 → 340 (−16)**; report **1**; inventory **0**. Does **not** delete `farm_lease_*` product API defs; does **not** touch `heap_v52` / `heap_v69` / `heap_v79` other tracks+dx / `heap_v111_*`.
9. **S2-I72 pack Δ16** — merge into `heap_v82_phase110_track_c_farm_fingerprint` (kept name); delete aggregator-contiguous tip×16:
   - `heap_v79_phase107_track_b_wasm_fingerprint` / `_track_c_select_reflect_` / `_track_d_farm_dist_` / `_dx_`
   - `heap_v80_phase108` track a/b/c/d/dx
   - `heap_v81_phase109` track a/b/c/d/dx
   - `heap_v82_phase110_track_a_gui_fingerprint` / `_track_b_wasm_`
   No soft-nail edits (band clean). **Byte order:** v79_b → … → v82_b → original v82_c. Expected census: **340 → 324 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82` track_d+dx / `v83+` / `heap_v111_*`.
10. **S2-I73 pack Δ16** — merge into `heap_v85_phase113_dx_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v82_phase110_track_d_dist_select_fingerprint` / `_dx_`
    - `heap_v83_phase111` track a/b/c/d/dx
    - `heap_v84_phase112` track a/b/c/d/dx
    - `heap_v85_phase113` track a/b/c/d
    No soft-nail edits (band clean). **Byte order:** v82_d → v82_dx → v83_a…dx → v84_a…dx → v85_a…d → original v85_dx. Expected census: **324 → 308 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v86+` / `heap_v111_*`.
11. **S2-I74 pack Δ16** — merge into `heap_v89_phase117_track_b_wasm_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v86_phase114` track a/b/c/d/dx
    - `heap_v87_phase115` track a/b/c/d/dx
    - `heap_v88_phase116` track a/b/c/d/dx
    - `heap_v89_phase117_track_a_gui_fingerprint`
    No soft-nail edits (band clean; v91/v92 nails out-of-pack). **Byte order:** v86_a → … → v89_a → original v89_b. Expected census: **308 → 292 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89` c/d/dx / `v90+` / `heap_v111_*`.
12. **S2-I75 pack Δ16** — merge into `heap_v93_phase121_track_d_farm_dist_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v89_phase117_track_c_farm_fingerprint` / `_track_d_dist_select_` / `_dx_`
    - `heap_v91_phase119` track a/b/c/d/dx
    - `heap_v92_phase120` track a/b/c/d/dx
    - `heap_v93_phase121_track_a_macro_service_fingerprint` / `_track_b_gui_` / `_track_c_wasm_`
    Same-window soft-nail retarget → merge target: `evergreen/smoke` (`heap_v91_phase119_track_b_wasm_fingerprint`) + `service_di_mvp` (`heap_v92_phase120`). **Byte order:** v89_c → v89_d → v89_dx → v91_a…dx → v92_a…dx → v93_a → v93_b → v93_c → original v93_d. Expected census: **292 → 276 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89_track_b` / `v93_dx` / `v94+` / `heap_v111_*`.
13. **S2-I76 pack Δ16** — merge into `heap_v97_phase125_track_a_macro_service_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v93_phase121_dx_fingerprint`
    - `heap_v94_phase122` track a/b/c/d/dx
    - `heap_v95_phase123` track a/b/c/d/dx
    - `heap_v96_phase124` track a/b/c/d/dx
    No soft-nail edits (band clean; `v93_track_d` nails stay; `v101` nail out-of-pack). **Byte order:** v93_dx → v94_a…dx → v95_a…dx → v96_a…dx → original v97_a. Expected census: **276 → 260 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89_track_b` / `v93_track_d` / `v97` b/c/d/dx / `v98+` / `heap_v111_*`.
14. **S2-I77 pack Δ16** — merge into `heap_v100_phase128_track_c_wasm_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v97_phase125` track b/c/d/dx
    - `heap_v98_phase126` track a/b/c/d/dx
    - `heap_v99_phase127` track a/b/c/d/dx
    - `heap_v100_phase128_track_a_macro_service_fingerprint` / `_track_b_gui_`
    No soft-nail edits (band clean; `v101`/`v102` nails out-of-pack). **Byte order:** v97_b → … → v97_dx → v98_a…dx → v99_a…dx → v100_a → v100_b → original v100_c. Expected census: **260 → 244 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89_track_b` / `v93_track_d` / `v97_track_a` / `v100` d/dx / `v101+` / `heap_v111_*`.
15. **S2-I78 pack Δ16** — merge into `heap_v104_phase132_dx_fingerprint` (kept name); delete aggregator-contiguous tip×16 (agg order; `v104` before `v103`):
    - `heap_v100_phase128_track_d_farm_dist_fingerprint` / `_dx_`
    - `heap_v101_phase129` track a/b/c/d/dx
    - `heap_v102_phase130` track a/b/c/d/dx
    - `heap_v104_phase132` track a/b/c/d
    Same-window soft-nail retarget → merge target: `codable_associated_enum_e2e` (`heap_v101_phase129_track_a_macro_service_fingerprint`) + `token_expr_mvp` (`heap_v102_phase130_track_a_macro_service_fingerprint`). **Byte order:** v100_d → v100_dx → v101_a…dx → v102_a…dx → v104_a…d → original v104_dx. Expected census: **244 → 228 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89_track_b` / `v93_track_d` / `v97_track_a` / `v100_track_c` / `v103` / `v105+` / `heap_v111_*`.
16. **S2-I79 pack Δ16** — merge into `heap_v106_phase134_dx_fingerprint` (kept name); delete aggregator-contiguous tip×16 (agg order; `v103` after `v104_dx`, before `v105`):
    - `heap_v103_phase131` track a/b/c/d/dx
    - `heap_v105_phase133` track a/b/c/d/e/dx
    - `heap_v106_phase134` track a/b/c/d/e
    No soft-nail edits (band clean; `v104_dx` nails stay; `v110` nail out-of-pack). **Byte order:** v103_a…dx → v105_a…dx → v106_a…e → original v106_dx. Expected census: **228 → 212 (−16)**; report **1**; inventory **0**. Does **not** touch `heap_v52` / `v69` / `v79_track_a` / `v82_track_c` / `v85_dx` / `v89_track_b` / `v93_track_d` / `v97_track_a` / `v100_track_c` / `v104_dx` / `v107+` / `heap_v111_*`.
17. **S2-I80 pack Δ16** — merge into `heap_v110_phase138_track_a_props_fingerprint` (kept name); delete aggregator-contiguous tip×16:
    - `heap_v107_phase135` track a/b/c/d/e/dx
    - `heap_v108_phase136` track a/c/d/e/dx
    - `heap_v109_phase137` track a/c/d/e/dx
    Same-window soft-nail retarget → merge target: `cache_v108_invalidation` (`heap_v108_*`×5) + `cache_v109_invalidation` (`heap_v109_*`×5). **Byte order:** v107_a…dx → v108_a…dx → v109_a…dx → original v110_a. Expected census: **212 → 196 (−16)**; report **1**; inventory **0**. Does **not** touch prior tips / `v110` c/d/e/dx / **`heap_v111_*`**.
18. **S2-I81 pack Δ4** — merge into adjacent kept helper `heap_v110_phase138_track_a_props_fingerprint`; delete aggregator-contiguous tip×4:
    - `heap_v110_phase138_track_c_wasm_fingerprint`
    - `heap_v110_phase138_track_d_os_network_fingerprint`
    - `heap_v110_phase138_track_e_macro_concurrency_fingerprint`
    - `heap_v110_phase138_dx_fingerprint`
    Same-window soft-nail: `cache_v110_invalidation` `cache_v110_tag_active` drop c/d/e/dx `contains` (keep `track_a` + `syx-cache-v110`). **Byte order:** original v110_a → v110_c → v110_d → v110_e → v110_dx. Expected census: **196 → 192 (−4)**; report **1**; inventory **0**. Does **not** touch **`heap_v111_*`**. After this pack, contiguous foldable tips before `v111` are **exhausted**.
19. **S2-I82 pack Δ8 (earlier kept-tip re-fold)** — merge into `heap_v100_phase128_track_c_wasm_fingerprint` (kept name); delete earlier kept tips×8:
    - `heap_v52_phase80_fingerprint`
    - `heap_v69_phase97_fingerprint`
    - `heap_v79_phase107_track_a_gui_fingerprint`
    - `heap_v82_phase110_track_c_farm_fingerprint`
    - `heap_v85_phase113_dx_fingerprint`
    - `heap_v89_phase117_track_b_wasm_fingerprint`
    - `heap_v93_phase121_track_d_farm_dist_fingerprint`
    - `heap_v97_phase125_track_a_macro_service_fingerprint`
    Same-window soft-nail retarget → merge target: `driver_gate` (v69/v79_a) + `syx-lsp` `lsp_phase100_real` (v79_a) + `evergreen/smoke` (v93_d) + `service_di_mvp` (v93_d). **Byte order:** v52 → v69 → v79_a → v82_c → v85_dx → v89_b → v93_d → v97_a → original v100_c. Expected census: **192 → 184 (−8)**; report **1**; inventory **0**. Does **not** touch foundation non-tip slots / `v104_dx` / `v106_dx` / `v110_a` / **`heap_v111_*`** / `capabilities_report`.
20. **S2-I83 pack Δ3 (earlier kept-tip re-fold)** — merge into `heap_v100_phase128_track_c_wasm_fingerprint` (kept name); delete earlier kept tips×3:
    - `heap_v104_phase132_dx_fingerprint`
    - `heap_v106_phase134_dx_fingerprint`
    - `heap_v110_phase138_track_a_props_fingerprint`
    Same-window soft-nail retarget → merge target: `codable_associated_enum_e2e` + `token_expr_mvp` (v104) + `driver_gate` + `cache_v108_invalidation` + `cache_v109_invalidation` + `cache_v110_invalidation` (v110_a). **Byte order:** original v100_c → v104_dx → v106_dx → v110_a. Expected census: **184 → 181 (−3)**; report **1**; inventory **0**. Does **not** touch foundation non-tip slots / **`heap_v111_*`** / `capabilities_report`. After this pack, earlier kept-tip re-fold pre-`v111` tips are **exhausted**.
21. Later windows may continue fold packs under this ADR without a new decision document. **S2-I84:** capabilities product surface migrated out of `*_report.rs` census — see [stage2-capabilities-report-surface](stage2-capabilities-report-surface.md). **S2-I85:** `heap_v111_*` fold authorized under separate gate — see [stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md). **S2-I86:** foundation×12 fold authorized under separate gate — see [stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md). **S2-I87:** `track_l` fold authorized under separate gate — see [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md).
22. **MUST STAY:** capabilities product API (`run_capabilities_report` / CLI / LSP; module `capabilities` after S2-I84); `farm_lease_*_fingerprint` product APIs; remaining on-agg helpers not folded under a dedicated gate (after S2-I87: kept `heap_v100_…` only); five B-09 product flags; `llvm_stub_audit_report`.
23. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ emit·ABI rewrite / ≠ tip-knife orphan or test-noise packs / ≠ product umbrella / F-08.

## Consequences

- **S2-I69:** fold Δ6 landed; outer cache tag slot count decreases and `heap_v52` hex changes (expected invalidation); aggregator fold axis opened.
- **S2-I70:** fold Δ16 landed into `heap_v69`; soft-nail retargeted; fold default pack raised to **Δ8–16**.
- **S2-I71:** fold Δ16 landed into `heap_v79_track_a`; soft-nails retargeted (driver_gate×5 + lsp×1).
- **S2-I72:** fold Δ16 landed into `heap_v82_track_c`; soft-nail-free band.
- **S2-I73:** fold Δ16 landed into `heap_v85_dx`; soft-nail-free band.
- **S2-I74:** fold Δ16 landed into `heap_v89_track_b`; soft-nail-free band.
- **S2-I75:** fold Δ16 landed into `heap_v93_track_d`; soft-nails retargeted (evergreen smoke + service_di).
- **S2-I76:** fold Δ16 landed into `heap_v97_track_a`; soft-nail-free band.
- **S2-I77:** fold Δ16 landed into `heap_v100_track_c`; soft-nail-free band.
- **S2-I78:** fold Δ16 landed into `heap_v104_dx`; soft-nails retargeted (codable_associated_enum + token_expr).
- **S2-I79:** fold Δ16 landed into `heap_v106_dx`; soft-nail-free band.
- **S2-I80:** fold Δ16 landed into `heap_v110_track_a`; soft-nails retargeted (cache_v108 + cache_v109).
- **S2-I81:** fold Δ4 landed into `heap_v110_track_a` (c/d/e/dx); soft-nail narrowed (cache_v110); contiguous pre-`v111` foldable tips exhausted.
- **S2-I82:** earlier kept-tip re-fold Δ8 into `heap_v100_track_c`; soft-nails retargeted (driver_gate + lsp + smoke + service_di).
- **S2-I83:** earlier kept-tip re-fold Δ3 into `heap_v100_track_c` (v104_dx/v106_dx/v110_a); soft-nails retargeted (codable + token_expr + driver_gate + cache_v108–110); pre-`v111` earlier-tip axis exhausted.
- **S2-I84:** capabilities product surface renamed out of `*_report.rs` census ([stage2-capabilities-report-surface](stage2-capabilities-report-surface.md)); fold axis unchanged.
- **S2-I85:** `heap_v111_*` Δ4 fold into `track_l` under [stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md); soft-nail narrowed (`cache_v111_tag_active`).
- **S2-I86:** foundation×12 fold into `heap_v100_track_c` under [stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md); soft-nails retargeted; `module_hash` digests renamed off census.
- **S2-I87:** `track_l` Δ1 fold into `heap_v100_track_c` under [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md); soft-nail retargeted (`cache_v111`); on-agg single tip.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-tip-knife](stage2-fingerprint-tip-knife.md)
- [stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md) · S2-I85
- [stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md) · S2-I86
- [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md) · S2-I87
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
