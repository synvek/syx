# ADR: Stage-4 S4-05 doctor-native-sdk / full-native hinge（S3-I105）

**Status:** Accepted (S3-I105) · S4-05 `[~]` · Stage-4 `[~]` · **4/5**  
**Date:** 2026-11-09  
**Context:** After [S3-I104 S4-05 open](stage4-s4-05-open.md), census showed `build-full-native` / `packaging/syx-sdk` / `native_sdk_layout_report` inventory green while S4-05 still lacked a **Primary construct hinge**. Library layout report + unit test already exist; docs/release claim `syx doctor --native-sdk` but CLI remains unwired. This window elevates layout / full-native evidence into a **machine-checkable hinge** (charter + census KPIs + contract). It does **not** claim S4-05 `[x]`, wire CLI `--native-sdk`, change packaging/`release.yml` behavior, declare official sysroot GA, or exit Stage-4.

Do **not** confuse with legacy ID `M2-S4-05`（TLS client）.

## Decision

1. **Publish** charter [`docs/s4-05-doctor-native-sdk-v0.md`](../s4-05-doctor-native-sdk-v0.md)（Primary = native SDK layout / full-native 预编译发行物；CLI 接线 / sysroot / marketplace deferred）。
2. **Nail Primary KPIs** on existing evidence（no product edit）：
   - `native_sdk_layout_helper≥1` · `fn native_sdk_layout_report` in `syx-rt`
   - `layout_report_unit_helper≥1` · `native_sdk_layout_report_runs`
   - `full_native_build_helper≥1` · `scripts/build-full-native`
   - `syx_sdk_packaging_helper≥1` · `packaging/syx-sdk`
   - `doctor_native_sdk_hinge_helper≥1` · this hinge ADR/charter
3. **Ladder next:** **S3-I106** = S4-05 residual-scope — still ≠ S4-05 `[x]`。
4. **Non-claims (hard):** ≠ S4-05 `[x]` / ≠ Stage-4 exit / ≠ 接线 `doctor --native-sdk` / ≠ packaging/`release.yml` 产品刀 / ≠ 官方 sysroot GA / ≠ marketplace / 全 profile CI / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 旧 `M2-S4-05` TLS。

## Evidence

| Check | Evidence |
|-------|----------|
| Charter | [`docs/s4-05-doctor-native-sdk-v0.md`](../s4-05-doctor-native-sdk-v0.md) |
| Census KPI | `doctor_native_sdk_hinge_helper≥1` · layout / full-native / packaging |
| Contract | `s4_05_doctor_native_sdk_hinge_contract` |
| Prior | [stage4-s4-05-open](stage4-s4-05-open.md) · S3-I104 |

## Consequences

- doctor-native-sdk / full-native Primary hinge landed；S4-05 stays `[~]`；Stage-4 stays `[~]`（**4/5**）。
- **S3-I106 landed** as [stage4-s4-05-residual-scope](stage4-s4-05-residual-scope.md)（scoped acceptance · deferred CLI/sysroot/marketplace）。
- **S3-I107 landed** as [stage4-s4-05-claim](stage4-s4-05-claim.md)（S4-05 scoped `[x]` · Stage-4 **5/5**）。
- Default next = **S3-I108**（Stage-4 exit-open）— not Stage-4 exit / not CLI wire / not `stage4-exit` this hinge.
- Prior: [stage4-s4-05-open](stage4-s4-05-open.md) · S3-I104.
- Prebuilt census Current → **S3-I107** · Claimed scoped `[x]`。

## References

- [stage4-s4-05-open](stage4-s4-05-open.md) · S3-I104
- [s4-05-doctor-native-sdk-v0](../s4-05-doctor-native-sdk-v0.md)
- [stage4-prebuilt-release-census](../stage4-prebuilt-release-census.md)
- [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · S3-I101（hinge 形态）
- [`crates/syx-rt/src/link.rs`](../../crates/syx-rt/src/link.rs) · `native_sdk_layout_report`
- [`scripts/build-full-native`](../../scripts/build-full-native)
- [`packaging/syx-sdk`](../../packaging/syx-sdk)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-05
