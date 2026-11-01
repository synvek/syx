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

- `plan/01` / `plan/02` mark Stage-2 as **`[~]` S2-I1**.
- S2-01 stays **`[~]`** until product binaries no longer link process-governance inventory/fingerprint/gate narratives (acceptance in plan/02).

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · Stage 2
- [`docs/adr/stage1-exit.md`](stage1-exit.md)
- Analysis: [`plan/ProjectAnalysisAndImprovementPlan.zh-CN.md`](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §4.2.1 / 阶段2
