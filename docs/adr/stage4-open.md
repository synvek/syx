# ADR: Stage-4 open — ecosystem & language enhancement（S3-I88）

**Status:** Accepted (S3-I88) · Stage-4 `[~]` · S4-01 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I87 Stage-3 exit](stage3-exit.md), default credibility work shifts to [plan/02 Stage-4](../../plan/02-improvement-roadmap.md): ecosystem and language enhancement. The locked first axis is **S4-01**（标准库稳定性标注 + doctest）. This window **opens** Stage-4 with a stdlib stability / doctest **census baseline** — it does **not** claim S4-01 `[x]`, introduce machine-readable stability schema, implement `syx doctest`, or exit Stage-4.

## Gap: stdlib stability vs plan S4-01

| Claim | Today (honest) | plan/02 S4-01 target |
|-------|----------------|----------------------|
| 每包 stable / preview / experimental | [`packages/README.md`](../../packages/README.md) 五档文字（Stable/MVP/Partial/Experimental/Deferred）；**无** 机器可读三元 | 三元 + SSOT |
| Manifest / package stability 字段 | [`packages/internal/manifest.json`](../../packages/internal/manifest.json) **无** `stability`；capabilities 另有 experimental 面 | 包级可机器盘点 |
| stable 包 API + doctest | 书块 [`docs/doctest-annotations.md`](../doctest-annotations.md) + `book_pass_rate.py`（S1-09）；**packages/** 无 doctest 闸 | stable 须 API 文档 + doctest |
| `syx doctest` 子命令 | **不存在**（CLI 无 Doctest） | 产品化（可后窗） |
| 示例缺失 API | 多为标注/doctest 债，非「stdlib API 全缺」 | 补齐诚实缺口 |

Baseline: [stage4-stdlib-stability-doctest-census](../stage4-stdlib-stability-doctest-census.md).

## Decision

1. **Open Stage-4** as an active credibility workstream; flip plan Stage-4 `[ ]` → `[~]`.
2. **Open S4-01** as the default construction axis; flip S4-01 `[ ]` → `[~]`.
3. **S3-I88 first slice** — publish stdlib stability / doctest census (+ script); add a thin contract under `syx-driver/tests/`（**no** packages / CLI / doctest product edit this window）.
4. **Ladder:** **S3-I89** = first construct（priority: 稳定性 schema / 首包试点标注，由本窗 census 数字驱动）→ residual/claim-prep（可压缩）→ dedicated **S4-01 `[x]` claim**.
5. **Non-claims (hard):** ≠ S4-01 `[x]` / ≠ Stage-4 exit / ≠ 改 `manifest.json` schema / ≠ 批量改 `packages/**` 加标注 / ≠ 实现 `syx doctest` / ≠ 开 S4-02…05 施工 / ≠ HostFn+1 / ≠ F-08 / ≠ 产品伞 unpark / ≠ change language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Stability/doctest census | [`docs/stage4-stdlib-stability-doctest-census.md`](../stage4-stdlib-stability-doctest-census.md) · `scripts/s4_01_stdlib_stability_doctest_census.sh` |
| Contract nail | `s4_01_stdlib_stability_doctest_census_contract` |
| Plan status | Stage-4 `[~]` · S4-01 `[~]` · checklist **0/5** under this ADR |

## Consequences

- Stage-4 and S4-01 move `[ ]` → `[~]`；S4-01 later claimed `[x]` under I91.
- **S3-I89 landed** as [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md)（stability.toml SSOT）。
- **S3-I90 landed** as [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md)（scoped acceptance）。
- **S3-I91 landed** as [stage4-s4-01-claim](stage4-s4-01-claim.md)（S4-01 `[x]`）。
- Default next = **S3-I92**（S4-02 open）— not Stage-4 exit / not S4-02 construct.
- Prior: [stage3-exit](stage3-exit.md) · S3-I87 · [stage3-open](stage3-open.md) · S3-I1.
- Stdlib stability/doctest census Current → **S3-I91** · Claimed scoped `[x]`.

## References

- [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91
- [stage4-s4-01-residual-scope](stage4-s4-01-residual-scope.md) · S3-I90
- [stage4-s4-01-stability-ssot](stage4-s4-01-stability-ssot.md) · S3-I89
- [stage3-exit](stage3-exit.md) · S3-I87
- [stage3-open](stage3-open.md) · S3-I1（开窗形态）
- [stage4-stdlib-stability-doctest-census](../stage4-stdlib-stability-doctest-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) Stage-4 / S4-01
- [`packages/README.md`](../../packages/README.md)
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §阶段 4

