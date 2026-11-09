# ADR: Stage-4 S4-01 plan claim（S3-I91）

**Status:** Accepted (S3-I91) · S4-01 `[x]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I90 residual-scope](stage4-s4-01-residual-scope.md), Hard OUT construct ([I89 SSOT](stage4-s4-01-stability-ssot.md)), open ([I88](stage4-open.md)), and scoped acceptance (I90) are landed. Primary KPIs report **`stability_ssot_package_rows≥19`**, **`stability_ssot_helper≥1`**, **`residual_scope_helper≥1`**, with honest deferred zeros **`manifest_stability_field_count=0`** / **`syx_doctest_subcommand_count=0`**; manifest wiring / driver SSOT read / `syx doctest` / packages doctest gate remain **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with Stage-3 scoped claims. It does **not** exit Stage-4 or restore deferred axes as hard gates.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I88](stage4-open.md) · [census](../stage4-stdlib-stability-doctest-census.md) |
| Construct I89 | [`stability.toml`](../../packages/internal/stability.toml) · [v0](../stdlib-stability-v0.md) |
| Residual-scope | [S3-I90](stage4-s4-01-residual-scope.md) · scoped acceptance · [scope-v0](../s4-01-manifest-doctest-scope-v0.md) |
| Primary KPI | `stability_ssot_package_rows≥19` · `stability_ssot_helper≥1` · `residual_scope_helper≥1` |
| Contract | `s4_01_claim_contract` · `s4_01_stdlib_stability_doctest_census_contract` |
| Honesty OUT | **≠** Stage-4 exit · **≠** write `stage4-exit` this window · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ 改 `manifest.json` schema / ≠ driver 读 SSOT / ≠ 实现 `syx doctest` / ≠ packages doctest 闸 / ≠ 批量 `packages/**/*.sy` / ≠ further product knives beyond I89）, **标准库稳定性标注 + doctest** workstream is complete when I89 hinge + I90 scaffolding hold and Primary KPIs stay green — **not** 「manifest 已接线」 / **not** 「`syx doctest` 已产品化」 / **not** 「Stage-4 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S4-01 → `[x]`**.
2. Keep **Stage-4 → `[~]`** until an explicit Stage-4 exit ADR（checklist **1/5** items claimed；exit still separate）。
3. Update stdlib stability/doctest census Current=**S3-I91** · **Claimed: scoped `[x]`**；next default = **S3-I92 S4-02 open** — **≠** Stage-4 exit / **≠** S4-02 construct this window.
4. **Non-claims (hard):** ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage4-exit` this window / ≠ 改 `manifest.json` / ≠ driver 读 SSOT / ≠ `syx doctest` / ≠ 批量包源码 / ≠ S4-02…05 施工 / ≠ language semantics.

## Consequences

- S4-01 plan-claimed `[x]`; Stage-4 stays `[~]`（checklist **1/5**）.
- Default next = **S3-I92**（S4-02 open）— not Stage-4 exit / not S4-02 construct / not `syx doctest`.
- Prior: [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md) · S3-I90 · construct [S3-I89](stage4-s4-01-stability-ssot.md) · open [S3-I88](stage4-open.md).
- Census Current → **S3-I91** · Claimed scoped `[x]`.

## References

- [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md) · S3-I90
- [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md) · S3-I89
- [stage4-open](stage4-open.md) · S3-I88
- [s4-01-manifest-doctest-scope-v0](../s4-01-manifest-doctest-scope-v0.md)
- [stage3-s3-07-claim](stage3-s3-07-claim.md) · S3-I85（claim 窗形态）
- [stage4-stdlib-stability-doctest-census](../stage4-stdlib-stability-doctest-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-01
- [`packages/internal/stability.toml`](../../packages/internal/stability.toml)
