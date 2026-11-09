# Stage-4 prebuilt / full-native release census（S3-I107）

- **Current:** S3-I107
- **Claimed:** scoped `[x]`
- **s4_05_status:** `x`
- **stage4_status:** `x`
- **stage4_checklist:** `5/5`
- **next:** S3-I108（exit-open **landed**）→ S3-I109（[stage4-exit](adr/stage4-exit.md) **landed**）→ post-exit Residual / 维护

## Purpose

Open + Primary hinge + scoped residual + **scoped claim** for plan axis **S4-05**（预编译发行物）. Checklist **5/5**；Stage-4 exit **landed**（[S3-I109](adr/stage4-exit.md)；开窗 [S3-I108](adr/stage4-exit-open.md)）. Mirror of [stage4-sequence-collection-census](stage4-sequence-collection-census.md) claim shape（S3-I103）.

**This census:** S3-I107 scoped claim — Stage-4 **`[x]` 退出** · S4-05 **`[x]`** · checklist **5/5**；exit-open **S3-I108** landed；exit **S3-I109** landed；下一 = **post-exit Residual / 维护**。

## Goal (S4-05 scoped acceptance) — claimed

Honest scoped `[x]` = **I105 Primary** green + open/census/hinge/`residual_scope_helper` + **`s4_05_claim_helper`**. CLI `--native-sdk` / official sysroot / marketplace / full-profile CI remain **deferred supersede-only**.

## Gap axes（post-claim honesty）

| Axis | Status |
|------|--------|
| full-native / syx-sdk / layout | **Claimed Primary**（I105） |
| doctor `--native-sdk` CLI | **deferred**（docs claim；CLI unwired） |
| official sysroot / marketplace / 全 profile | **deferred** |

## KPI（script `scripts/s4_05_prebuilt_release_census.sh`）

| Key | Threshold | Meaning |
|-----|-----------|---------|
| `s4_05_open_helper` | ≥1 | open ADR |
| `s4_05_census_helper` | ≥1 | this census + script |
| `doctor_native_sdk_hinge_helper` | ≥1 | hinge ADR + v0（S3-I105） |
| `residual_scope_helper` | ≥1 | residual ADR + scope-v0（S3-I106） |
| `s4_05_claim_helper` | ≥1 | claim ADR + plan S4-05 `[x]`（S3-I107） |
| `full_native_build_helper` | ≥1 | `scripts/build-full-native` |
| `syx_sdk_packaging_helper` | ≥1 | `packaging/syx-sdk` |
| `native_sdk_layout_helper` | ≥1 | `native_sdk_layout_report` |
| `layout_report_unit_helper` | ≥1 | `native_sdk_layout_report_runs` |
| `doctor_cli_helper` | ≥1 | CLI Doctor inventory |
| `doctor_native_sdk_flag_helper` | ≥1 | docs/release `--native-sdk` mentions（诚实：CLI 仍未接线） |

## Honesty note

Scoped claim does **not** assert CLI wiring. `doctor_native_sdk_flag_helper` remains a docs/release honesty counter.

## References

- [stage4-s4-05-claim](adr/stage4-s4-05-claim.md) · S3-I107
- [stage4-s4-05-residual-scope](adr/stage4-s4-05-residual-scope.md) · S3-I106
- [s4-05-prebuilt-residual-scope-v0](s4-05-prebuilt-residual-scope-v0.md)
- [stage4-s4-05-doctor-native-sdk-hinge](adr/stage4-s4-05-doctor-native-sdk-hinge.md) · S3-I105
- [s4-05-doctor-native-sdk-v0](s4-05-doctor-native-sdk-v0.md)
- [stage4-s4-05-open](adr/stage4-s4-05-open.md) · S3-I104
