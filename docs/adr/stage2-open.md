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

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · Stage 2
- [`docs/adr/stage1-exit.md`](stage1-exit.md)
- [`docs/adr/stage2-cutover-decouple.md`](stage2-cutover-decouple.md) · S2-I11
- Analysis: [`plan/ProjectAnalysisAndImprovementPlan.zh-CN.md`](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §4.2.1 / 阶段2
