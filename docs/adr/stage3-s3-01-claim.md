# ADR: Stage-3 S3-01 plan claim（S3-I41）

**Status:** Accepted (S3-I41) · S3-01 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I40 noise/claim-prep](stage3-s3-01-noise-claim-prep.md), claim axes ① bare · ② filtered positionless · ③ miette are aligned; face positionless KPIs are 0. This dedicated window flips the plan checkbox only — aligned with Stage-2 scoped claim pattern. It does **not** exit Stage-3 or open S3-02 construction.

## Evidence

| Axis | Evidence |
|------|----------|
| ① user bare = 0 | census `host_*` / `host_residual_bare` / semantic bare = **0**（I2–I23） |
| ② filtered helper = 0 | `positionless_helper_calls=0`（I40）；noise/policy transparent |
| ③ miette | `miette_help_impl=1` · `with_span_label_wired=1`（I25） |
| Face gates | all `positionless_*_bare=0`（I26–I39） |
| Claim-prep ④ | this dedicated claim window（I24 §4） |
| Prior | [noise-claim-prep](stage3-s3-01-noise-claim-prep.md) · S3-I40 |
| Contract | `s3_01_claim_contract` · `s3_01_span_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-02 open/complete this window · **≠** convert |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-01 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR.
3. **Scoped `[x]` meaning:** user-facing diagnostics carry Span; real positionless helper KPIs = 0; miette label/help wired; I40 noise/policy carve-out **OK** — **not** 「zero noise names」 / **not** 「MIR IR `span: None` = 0」 / **not** 「change diagnostics ctor defaults」.
4. Update census Current=I41；stage3-open / I40 + chain ADRs Consequences；next default = **S3-I42 S3-02 open**.
5. **Non-claims (hard):** ≠ Stage-3 exit this window / ≠ open or complete S3-02 construction this window / ≠ convert / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ change diagnostics defaults / ≠ MIR IR semantics.

## Consequences

- S3-01 plan-claimed `[x]`; Stage-3 stays `[~]`.
- Default next = **S3-I42 S3-02 open** — not Stage-3 exit.
- Prior: [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md) · S3-I40.
- Census Current → **S3-I41**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md) · S3-I24
- [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md) · S3-I40
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
