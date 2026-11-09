# ADR: Stage-2 S2-05 plan claim (S2-I259)

**Status:** Accepted (S2-I259) · S2-05 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I256–I258 analyzer concurrency/IDE/infer batch](stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md), the S2-05 construction ladder opened in [S2-I165](stage2-s2-05-open.md) meets the scoped acceptance bar: three-crate **production** peels exhausted; semantic analyzer secondary peels down to an entry/builtins hub; census + `s2_05_*` contracts document the path. This dedicated window flips the plan checkbox only — aligned with [S2-I164 S2-04 claim](stage2-s2-04-claim.md) / [S2-I142 S2-03 claim](stage2-s2-03-claim.md). It does **not** exit Stage-2, migrate `#[cfg(test)]` out of hubs, or open S2-06 product construction.

## Evidence

| Axis | Evidence |
|------|----------|
| Census baseline → current | [stage2-file-split-census](../census/stage2-file-split-census.md) · open combined **31222** → ~**3513** (−89%) |
| `syx-mir` production peel | I166–I214 · production close-out; hub + tests residual OK |
| `syx-driver` production peel | I215–I235 · [driver-production-closeout-batch](stage2-s2-05-driver-production-closeout-batch.md) |
| `syx-semantic` lib + analyzer | I236–I249 lib/helpers · I250–I258 analyzer faces · hub = entry/builtins |
| Contract chain | `s2_05_*` lang contracts (census through concurrency-ide-infer batch) |
| Honesty OUT | **≠** single-file LOC→0 · **≠** tests migrated · **≠** Stage-2 exit |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-05 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until S2-06…S2-07 acceptance (or explicit Stage-2 exit ADR).
3. **Scoped `[x]` meaning:** maintainable module layout via ownership/layer peels for mir/driver/semantic **production** faces; hubs may retain orchestration + `#[cfg(test)]` / host stubs; semantic `semantic_analyzer.rs` may remain an **entry/builtins hub** — **not** 「every residual LOC emigrated」 / **not** 「tests-in-lib forbidden」.
4. Update [stage2-s2-05-open](stage2-s2-05-open.md) / [stage2-open](stage2-open.md) / [concurrency-ide-infer-batch](stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md) Consequences; census Current=I259; next default = **S2-06 open**.
5. **Non-claims (hard):** ≠ Stage-2 exit / ≠ migrate `#[cfg(test)]` / ≠ peel remaining builtins this window / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 construction this window / ≠ change analysis/MIR/driver semantics.

## Consequences

- S2-05 plan-claimed; Stage-2 stays `[~]` (S2-01…05 `[x]`; S2-06…07 open).
- Decision §4 locked next = **S2-06 open**; **landed** as [s2-06-open](stage2-s2-06-open.md)（S2-I260）→ [s2-06-exclude-syx-gui](stage2-s2-06-exclude-syx-gui.md)（S2-I261）→ [s2-06-claim](stage2-s2-06-claim.md)（S2-I262）；next = **S2-I263 S2-07 open**.
- Prior: [concurrency-ide-infer-batch](stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md) · S2-I256–I258.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-open](stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch](stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md) · S2-I256–I258
- [stage2-s2-04-claim](stage2-s2-04-claim.md) · S2-I164 (claim pattern)
- [stage2-file-split-census](../census/stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
