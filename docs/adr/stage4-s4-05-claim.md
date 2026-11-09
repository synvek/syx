# ADR: Stage-4 S4-05 plan claim（S3-I107）

**Status:** Accepted (S3-I107) · S4-05 `[x]` · Stage-4 `[~]` · **5/5**  
**Date:** 2026-11-09  
**Context:** After [S3-I106 residual-scope](stage4-s4-05-residual-scope.md), Hard OUT construct ([I105 doctor-native-sdk / full-native hinge](stage4-s4-05-doctor-native-sdk-hinge.md)), open ([I104](stage4-s4-05-open.md)), and scoped acceptance (I106) are landed. Primary KPIs report **`doctor_native_sdk_hinge_helper≥1`**, **`native_sdk_layout_helper≥1`**, **`layout_report_unit_helper≥1`**, **`full_native_build_helper≥1`**, **`syx_sdk_packaging_helper≥1`**, **`residual_scope_helper≥1`**, with honest deferred axes（CLI `--native-sdk` · official sysroot · marketplace · 全 profile CI · doctor UX）remaining **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with [S3-I103 S4-04 claim](stage4-s4-04-claim.md). It does **not** exit Stage-4, wire CLI `--native-sdk`, change packaging/`release.yml`, or restore deferred axes as hard gates.

Do **not** confuse with legacy ID `M2-S4-05`（TLS client）.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I104](stage4-s4-05-open.md) · [census](../census/stage4-prebuilt-release-census.md) |
| Construct I105 | [`s4-05-doctor-native-sdk-v0`](../census/s4-05-doctor-native-sdk-v0.md) · layout/full-native KPIs |
| Residual-scope | [S3-I106](stage4-s4-05-residual-scope.md) · scoped acceptance · [scope-v0](../census/s4-05-prebuilt-residual-scope-v0.md) |
| Primary KPI | `doctor_native_sdk_hinge_helper≥1` · layout≥1 · unit≥1 · full-native≥1 · packaging≥1 · `residual_scope_helper≥1` · `s4_05_claim_helper≥1` |
| Contract | `s4_05_claim_contract` · `s4_05_prebuilt_release_census_contract` |
| Honesty OUT | **≠** Stage-4 exit · **≠** write `stage4-exit` this window · **≠** wire `doctor --native-sdk` · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ 接线 `doctor --native-sdk` / ≠ packaging/`release.yml` 产品刀 / ≠ 官方 sysroot GA / ≠ marketplace GA）, **预编译发行物** workstream is complete when I105 hinge + I106 scaffolding hold and Primary KPIs stay green — **not** 「`syx doctor --native-sdk` 已接线」 / **not** 「官方 redistributable sysroot 已 GA」 / **not** 「marketplace / 全 profile CI 已产品化」 / **not** 「Stage-4 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S4-05 → `[x]`**.
2. Keep **Stage-4 → `[~]`** until an explicit Stage-4 exit ADR（checklist **5/5** items claimed；exit still separate）。
3. Update prebuilt census Current=**S3-I107** · **Claimed: scoped `[x]`**；next default = **S3-I108 Stage-4 exit-open** — **≠** Stage-4 exit / **≠** write `stage4-exit` this window.
4. **Non-claims (hard):** ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage4-exit*` this window / ≠ 接线 `doctor --native-sdk` / ≠ packaging/`release.yml` 产品刀 / ≠ 官方 sysroot / marketplace / 全 profile **GA** / ≠ 旧 `M2-S4-05` TLS。

## Consequences

- S4-05 plan-claimed `[x]`; Stage-4 stays `[~]`（checklist **5/5**）.
- **S3-I108 landed** as [stage4-exit-open](stage4-exit-open.md)（Stage-4 exit-open census）。
- Default next = **S3-I109**（write `stage4-exit`）— not Stage-4 `[x] 退出` this claim / not CLI wire / not product knife.
- Prior: [stage4-s4-05-residual-scope](stage4-s4-05-residual-scope.md) · S3-I106 · construct [S3-I105](stage4-s4-05-doctor-native-sdk-hinge.md) · open [S3-I104](stage4-s4-05-open.md).
- Prebuilt census Current → **S3-I107** · Claimed scoped `[x]`。
- Exit census: [stage4-exit-census](../census/stage4-exit-census.md) · Current **S3-I108** · exit_status=open。

## References

- [stage4-s4-05-residual-scope](stage4-s4-05-residual-scope.md) · S3-I106
- [stage4-s4-05-doctor-native-sdk-hinge](stage4-s4-05-doctor-native-sdk-hinge.md) · S3-I105
- [stage4-s4-05-open](stage4-s4-05-open.md) · S3-I104
- [s4-05-prebuilt-residual-scope-v0](../census/s4-05-prebuilt-residual-scope-v0.md)
- [s4-05-doctor-native-sdk-v0](../census/s4-05-doctor-native-sdk-v0.md)
- [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103（claim 窗形态）
- [stage4-prebuilt-release-census](../census/stage4-prebuilt-release-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-05
- [stage3-exit-open](stage3-exit-open.md) · S3-I86（下一 tip 形态）
