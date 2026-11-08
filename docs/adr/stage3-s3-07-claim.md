# ADR: Stage-3 S3-07 plan claim（S3-I85）

**Status:** Accepted (S3-I85) · S3-07 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I84 residual-scope](stage3-s3-07-residual-scope.md), Hard OUT construct (I83 default-vm), open (I82), and scoped acceptance (I84) are landed. Primary KPIs report **`init_default_template_native=0`**, **`init_default_template_vm≥1`**, **`init_vm_named_template_count≥1`**, and **`init_integration_test_count≥1`**; UsageGuide rewrite / Init crate extract / CLI-subprocess end-to-end / `_`→hard-error remain **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with S3-01…06 scoped claim. It does **not** exit Stage-3 or restore deferred axes as hard gates.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I82](stage3-s3-07-open.md) · init-template census |
| Construct I83 | default-vm (`default_value = "vm"` · explicit vm/native) |
| Residual-scope | [S3-I84](stage3-s3-07-residual-scope.md) · scoped acceptance · deferred UsageGuide/crate |
| Primary KPI | `init_default_template_native=0` · `init_vm_named_template_count≥1` · `init_integration_test_count≥1` |
| Contract | `s3_07_claim_contract` · `s3_07_init_template_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** write `stage3-exit` this window · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ UsageGuide 大改 / ≠ 抽 Init crate / ≠ CLI 子进程 init→run 硬门槛 / ≠ `_`→hard error / ≠ further Init product knives beyond I83）, **`syx init` 默认 VM 模板** workstream is complete when I83 hinge + I84 scaffolding hold and Primary KPIs stay green — **not** 「UsageGuide 全量对齐」 / **not** 「Init 抽 crate」 / **not** 「Stage-3 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-07 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR（checklist **7/7 ≈ 100%** items claimed；exit still separate）。
3. Update init-template census Current=**S3-I85** · **Claimed: scoped `[x]`**；next default = **S3-I86 Stage-3 exit open** — **≠** Stage-3 exit this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage3-exit` this window / ≠ UsageGuide 大改 / ≠ 抽 Init crate / ≠ change `Init` this window / ≠ reopen S3-06 LSP / S3-05 lint / S3-04 fib&lt;0.5 / ≠ language semantics.

## Consequences

- S3-07 plan-claimed `[x]`; Stage-3 stays `[~]`（checklist **7/7**）.
- Default next = **S3-I86**（Stage-3 exit open）— not Stage-3 exit itself.
- Prior: [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md) · S3-I84 · construct [S3-I83](stage3-s3-07-default-vm.md) · open [S3-I82](stage3-s3-07-open.md).
- Census Current → **S3-I85** · Claimed scoped `[x]`.

## References

- [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md) · S3-I84
- [stage3-s3-07-default-vm](stage3-s3-07-default-vm.md) · S3-I83
- [stage3-s3-07-open](stage3-s3-07-open.md) · S3-I82
- [stage3-s3-06-claim](stage3-s3-06-claim.md) · S3-I81（claim 窗形态）
- [stage3-init-template-census](../stage3-init-template-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-07
