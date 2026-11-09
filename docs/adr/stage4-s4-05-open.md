# Stage-4 S4-05 open — prebuilt / full-native release inventory（S3-I104）

- **Status:** Accepted（open only）
- **Date:** 2026-11-09
- **Deciders:** Synswx core
- **Tags:** stage4, S4-05, S3-I104, open, prebuilt, full-native, syx-sdk, doctor-native-sdk

## Context

Stage-4 checklist is **4/5** after [S3-I103](stage4-s4-04-claim.md) claimed S4-04. Plan axis **S4-05**（预编译发行物 / full-native SDK + doctor 修复提示）was still `[ ]`. Inventory is already thick: `scripts/build-full-native`, `packaging/syx-sdk`, `native_sdk_layout_report` in `syx-rt`. Honest gap: docs / `release.yml` still invoke `syx doctor --native-sdk`, but CLI `Doctor` has no such flag.

Do **not** confuse with legacy ID `M2-S4-05`（TLS client）.

## Decision

1. Flip plan **S4-05** `[ ]` → `[~]`. Stage-4 stays `[~]` · checklist **still 4/5**（claim 前不抬）.
2. Land open census + script + thin contract only — **zero** CLI / packaging / release product knife.
3. Gap axes for residual ladder:
   - **full-native 构建** — `scripts/build-full-native`（+ `.ps1`）
   - **`syx-sdk` 发行包装** — `packaging/syx-sdk` · `.github/workflows/release.yml`
   - **`native_sdk_layout_report`** — library inventory in `syx-rt`（already present）
   - **`syx doctor --native-sdk` CLI 脱节** — docs/release claim vs CLI Doctor flags（诚实缺口）
   - **sysroot / marketplace / 全 profile GA** — still deferred
4. **Ladder next:** **S3-I105** = first construct hinge — elevate **doctor-native-sdk / full-native SDK layout** evidence to machine-checkable Primary（mirror I101）. Official redistributable sysroot / marketplace / full-profile CI / doctor UX rewrite → residual.
5. Hard OUT this window: ≠ S4-05 `[x]` · ≠ wire `doctor --native-sdk` · ≠ packaging/release behavior change · ≠ Stage-4 exit · ≠ official sysroot GA · ≠ marketplace / full-profile CI · ≠ HostFn/F-08/Wave · ≠ legacy `M2-S4-05` TLS.

## Consequences

- S4-05 plan-open `[~]`; Stage-4 `[~]` · **4/5**.
- **S3-I105 landed** as [stage4-s4-05-doctor-native-sdk-hinge](stage4-s4-05-doctor-native-sdk-hinge.md)（Primary layout / full-native hinge）。
- **S3-I106 landed** as [stage4-s4-05-residual-scope](stage4-s4-05-residual-scope.md)（scoped residual）。
- **S3-I107 landed** as [stage4-s4-05-claim](stage4-s4-05-claim.md)（S4-05 scoped `[x]` · Stage-4 **5/5**）。
- Default next = **S3-I108**（Stage-4 exit-open）— not Stage-4 exit / not CLI wire / not `stage4-exit` this open.
- Prior claim: [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103.
- Census: [stage4-prebuilt-release-census](../stage4-prebuilt-release-census.md) · `Current=S3-I107` · Claimed scoped `[x]`。

## References

- [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103
- [stage4-prebuilt-release-census](../stage4-prebuilt-release-census.md)
- [`scripts/build-full-native`](../../scripts/build-full-native)
- [`packaging/syx-sdk`](../../packaging/syx-sdk)
- [`crates/syx-rt/src/link.rs`](../../crates/syx-rt/src/link.rs) · `native_sdk_layout_report`
- [plan/02](../../plan/02-improvement-roadmap.md) S4-05
