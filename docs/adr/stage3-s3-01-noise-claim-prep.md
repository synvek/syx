# ADR: Stage-3 S3-01 census 噪声收紧 + claim-prep ② 对齐（S3-I40）

**Status:** Accepted (S3-I40) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I39 residual](stage3-s3-01-positionless-residual.md), all **real** positionless `*_error(` faces are gated =0. Raw `positionless_helper_calls` still showed ~31 from census noise / policy wrappers — not remaining convert debt. This window aligns KPI ② — still **≠** S3-01 `[x]`.

## Decision

1. **Filter** `positionless_helper_calls`：在 raw `[a-z_][a-z0-9_]*_error\(` 宇宙上减去文档化 **noise** + **policy**；契约钉过滤后 **=0**。
2. **Echo** 次级透明 KPI：`positionless_helper_noise` · `positionless_helper_policy`（不钉 =0）。
3. **Whitelist（硬）：**
   - Noise：`set_error` · `from_error` · `last_os_error` · `host_log_error` · `registered_body_error` · `diagnostics_on_parse_error` · `*_is/has/no_parse/non_error` · `p[0-9]+_*_error`
   - Policy（无 SF 的 DUMMY 包装，② carve-out）：`lockfile_error` · `reflect_specialization_miss_error`
4. **Claim-prep ②：** 以 **filtered** `positionless_helper_calls=0` 计为对齐；① bare / ③ miette 已过。
5. **Ladder next：** **S3-I41** = 专用 **S3-01 `[x]` claim** 窗（**landed**）→ **S3-I42 S3-02 open**。
6. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ convert 噪声名 / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Filtered KPI | `positionless_helper_calls=0` |
| Transparency | `positionless_helper_noise≈26` · `positionless_helper_policy≈5` |
| Face gates | all prior positionless_* = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I40 assertions) |

## Consequences

- S3-01 was `[~]` under this window; **S3-I41 landed** [stage3-s3-01-claim](stage3-s3-01-claim.md) → S3-01 `[x]`; Stage-3 stays `[~]`.
- Claim axis ①②③ aligned under I40; ④ claim = I41.
- **S3-I43 landed** as [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md).
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（debug stack display）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next after I47 = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md) · S3-I39.
- Census Current → **S3-I41**（via I41 claim）.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md) · S3-I24
- [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md) · S3-I39
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
