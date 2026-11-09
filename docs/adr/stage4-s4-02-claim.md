# ADR: Stage-4 S4-02 plan claim（S3-I95）

**Status:** Accepted (S3-I95) · S4-02 `[x]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I94 residual-scope](stage4-s4-02-residual-scope.md), Hard OUT construct ([I93 file-registry hinge](stage4-s4-02-file-registry-hinge.md)), open ([I92](stage4-s4-02-open.md)), and scoped acceptance (I94) are landed. Primary KPIs report **`file_registry_hinge_helper≥1`**, **`file_registry_integration_test_count≥1`**, **`file_registry_evergreen_roundtrip_count≥2`**, **`registry_semver_r001_count≥1`**, **`residual_scope_helper≥1`**, with honest deferred axes（公网 GA / lock v16 trim / CLI UX rewrite / multi-publisher CDN）remaining **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with [S3-I91 S4-01 claim](stage4-s4-01-claim.md). It does **not** exit Stage-4, change `syx-pkg`/CLI, or restore deferred axes as hard gates.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I92](stage4-s4-02-open.md) · [census](../census/stage4-package-registry-census.md) |
| Construct I93 | [`s4-02-file-registry-v0`](../census/s4-02-file-registry-v0.md) · file:// round-trip KPIs |
| Residual-scope | [S3-I94](stage4-s4-02-residual-scope.md) · scoped acceptance · [scope-v0](../census/s4-02-registry-residual-scope-v0.md) |
| Primary KPI | `file_registry_hinge_helper≥1` · integration≥1 · evergreen≥2 · R001≥1 · `residual_scope_helper≥1` · `s4_02_claim_helper≥1` |
| Contract | `s4_02_claim_contract` · `s4_02_package_registry_census_contract` |
| Honesty OUT | **≠** Stage-4 exit · **≠** write `stage4-exit` this window · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ 改 `syx-pkg`/CLI / ≠ lock v16 全面裁剪 / ≠ 公网 `packages.sy.dev` GA / ≠ 多 publisher/CDN GA）, **包注册表最小可用** workstream is complete when I93 hinge + I94 scaffolding hold and Primary KPIs stay green — **not** 「公网 registry 已 GA」 / **not** 「lock v16 已裁剪」 / **not** 「CLI 五动词 UX 已重写」 / **not** 「Stage-4 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S4-02 → `[x]`**.
2. Keep **Stage-4 → `[~]`** until an explicit Stage-4 exit ADR（checklist **4/5** items claimed；exit still separate）。
3. Update package-registry census Current=**S3-I95** · **Claimed: scoped `[x]`**；next default = **S3-I96 S4-03 open** — **≠** Stage-4 exit / **≠** S4-03 construct this window.
4. **Non-claims (hard):** ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage4-exit` this window / ≠ 改 `syx-pkg`/CLI / ≠ lock v16 全面裁剪 / ≠ 公网 registry GA / ≠ S4-03…05 施工 / ≠ language semantics.

## Consequences

- S4-02 plan-claimed `[x]`; Stage-4 stays `[~]`（checklist **4/5**）.
- **S3-I96 landed** as [stage4-s4-03-open](stage4-s4-03-open.md)（S4-03 `[~]` open）。
- **S3-I97 landed** as [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md)（cancel/TaskGroup Primary）。
- Default next = **S3-I98**（S4-03 residual-scope）— not S4-03 `[x]` / not Stage-4 exit / not `withTaskGroup` GA.
- Prior: [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md) · S3-I94 · construct [S3-I93](stage4-s4-02-file-registry-hinge.md) · open [S3-I92](stage4-s4-02-open.md).
- Package-registry census Current → **S3-I95** · Claimed scoped `[x]`；structured-concurrency census Current → **S3-I96**.

## References

- [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96
- [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md) · S3-I94
- [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · S3-I93
- [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92
- [s4-02-registry-residual-scope-v0](../census/s4-02-registry-residual-scope-v0.md)
- [s4-02-file-registry-v0](../census/s4-02-file-registry-v0.md)
- [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91（claim 窗形态）
- [stage4-package-registry-census](../census/stage4-package-registry-census.md)
- [stage4-structured-concurrency-census](../census/stage4-structured-concurrency-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-02
- [`crates/syx-pkg`](../../crates/syx-pkg)
