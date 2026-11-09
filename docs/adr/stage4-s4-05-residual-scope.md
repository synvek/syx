# ADR: Stage-4 S4-05 residual-scope（S3-I106）

**Status:** Accepted (S3-I106) · S4-05 `[~]` · Stage-4 `[~]` · **4/5**  
**Date:** 2026-11-09  
**Context:** After [S3-I105 doctor-native-sdk / full-native hinge](stage4-s4-05-doctor-native-sdk-hinge.md), Primary KPIs are green (`doctor_native_sdk_hinge_helper≥1` · `native_sdk_layout_helper≥1` · `layout_report_unit_helper≥1` · `full_native_build_helper≥1` · `syx_sdk_packaging_helper≥1`), while `syx doctor --native-sdk` CLI wire, official sysroot, marketplace, and full-profile CI remain open. This window revises **S4-05 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S4-05 `[x]`, wire CLI `--native-sdk`, change packaging/`release.yml`, declare sysroot GA, or exit Stage-4.

Do **not** confuse with legacy ID `M2-S4-05`（TLS client）.

## Decision

1. **Scoped acceptance (S4-05 stays `[~]` this window):** Under Hard OUT（≠ 接线 `doctor --native-sdk` / ≠ packaging/`release.yml` 产品刀 / ≠ 官方 sysroot GA）, honest S4-05 completion means:
   - **Primary:** I105 hinge holds（[`s4-05-doctor-native-sdk-v0`](../census/s4-05-doctor-native-sdk-v0.md)；`native_sdk_layout_report` + unit nail + full-native + packaging）；
   - **KPI:** `doctor_native_sdk_hinge_helper≥1` · `native_sdk_layout_helper≥1` · `layout_report_unit_helper≥1` · `full_native_build_helper≥1` · `syx_sdk_packaging_helper≥1`；
   - **Scaffolding:** `s4_05_open_helper` · `s4_05_census_helper` · `residual_scope_helper` green.
2. **Deferred / supersede-only:** CLI `--native-sdk` wire · release.yml smoke fix · official redistributable sysroot · marketplace · 全 profile CI · doctor UX rewrite are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`. Scope charter: [s4-05-prebuilt-residual-scope-v0](../census/s4-05-prebuilt-residual-scope-v0.md).
3. **Ladder next:** **S3-I107** = dedicated **S4-05 scoped `[x]` claim** under this acceptance — still ≠ Stage-4 exit（claim 后 checklist **5/5**）.
4. **Non-claims (hard):** ≠ S4-05 `[x]` this window / ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 接线 `doctor --native-sdk` / ≠ packaging/`release.yml` 产品刀 / ≠ 官方 sysroot / marketplace / 全 profile **GA** / ≠ 把 deferred 恢复为 claim 硬门槛（无 supersede）/ ≠ 旧 `M2-S4-05` TLS。

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Residual scope | [`docs/census/s4-05-prebuilt-residual-scope-v0.md`](../census/s4-05-prebuilt-residual-scope-v0.md) |
| Census | `residual_scope_helper≥1` · Current=**S3-I106** |
| Plan | S4-05 row stays `[~]` · scoped摘要 |
| Contract | `s4_05_residual_scope_contract` |
| Prior | [I105 doctor-native-sdk-hinge](stage4-s4-05-doctor-native-sdk-hinge.md) · [I104 open](stage4-s4-05-open.md) |

## Consequences

- Residual-scope hinge landed；S4-05 was `[~]` this window；Stage-4 was `[~]`（**4/5**）。
- **S3-I107 landed** as [stage4-s4-05-claim](stage4-s4-05-claim.md)（S4-05 scoped `[x]` · Stage-4 **5/5**）。
- Default next = **S3-I108**（Stage-4 exit-open）— not Stage-4 exit / not CLI wire / not `stage4-exit` this residual.
- Prior: [stage4-s4-05-doctor-native-sdk-hinge](stage4-s4-05-doctor-native-sdk-hinge.md) · S3-I105.
- Prebuilt census Current → **S3-I107** · Claimed scoped `[x]`。

## References

- [stage4-s4-05-doctor-native-sdk-hinge](stage4-s4-05-doctor-native-sdk-hinge.md) · S3-I105
- [stage4-s4-05-open](stage4-s4-05-open.md) · S3-I104
- [s4-05-prebuilt-residual-scope-v0](../census/s4-05-prebuilt-residual-scope-v0.md)
- [s4-05-doctor-native-sdk-v0](../census/s4-05-doctor-native-sdk-v0.md)
- [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md) · S3-I102（形态模板）
- [stage4-prebuilt-release-census](../census/stage4-prebuilt-release-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-05
