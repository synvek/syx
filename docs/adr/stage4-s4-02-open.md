# ADR: Stage-4 S4-02 open — package registry minimal（S3-I92）

**Status:** Accepted (S3-I92) · Stage-4 `[~]` · S4-02 `[~]` · S4-01 `[x]`  
**Date:** 2026-11-09  
**Context:** After [S3-I91 S4-01 scoped claim](stage4-s4-01-claim.md), default credibility work advances to [plan/02 S4-02](../../plan/02-improvement-roadmap.md): **包注册表最小可用**（add/update/publish + lock + semver）. This window **opens** S4-02 with a package-registry **census baseline** — it does **not** claim S4-02 `[x]`, change `syx-pkg`/CLI product behavior, declare public registry GA, or exit Stage-4.

## Gap: package registry vs plan S4-02

| Claim | Today (honest) | plan/02 S4-02 target |
|-------|----------------|----------------------|
| `syx add` / `update` / `publish` | CLI verbs exist（[`syx-cli`](../../crates/syx-cli/src/main.rs)）+ [`syx-pkg`](../../crates/syx-pkg) APIs | 五动词「最小可用」scoped 可宣称 |
| Lock | [`syx.lock`](../../crates/syx-pkg/src/lockfile.rs) schema **v16**（远超 MVP 字段面） | 最小锁面 + 诚实 KPI |
| Semver | `VersionReq` / resolver（`manifest` + `semver` crate） | 端到端 resolve 验收口 |
| Registry | `file://` + HTTP client；默认 `packages.sy.dev` 路径存在 | **最小注册表**（file/local index 先行） |
| Stage-4 claim | **无** S4-02 scoped `[x]` / **无** 本轴 census | census → construct → claim |

Baseline: [stage4-package-registry-census](../stage4-package-registry-census.md).

## Decision

1. **Open S4-02** as the default construction axis; flip plan S4-02 `[ ]` → `[~]`.
2. Keep **Stage-4 → `[~]`** with checklist **1/5**（S4-01 already claimed；本窗不抬分母）.
3. **S3-I92 first slice** — publish package-registry census (+ script); add a thin contract under `syx-driver/tests/`（**no** `syx-pkg` / CLI product edit this window）.
4. **Ladder:** **S3-I93** = first construct（**file:// registry round-trip + lock/semver KPI hinge**，由本窗 census 数字驱动）→ residual/claim-prep（可压缩）→ dedicated **S4-02 `[x]` claim**.
5. **Non-claims (hard):** ≠ S4-02 `[x]` / ≠ S4-02 construct（改 `syx-pkg`/CLI 行为）/ ≠ Stage-4 exit / ≠ 公网 registry GA / ≠ S4-03…05 施工 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ restore S4-01 deferred as gate / ≠ language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Package-registry census | [`docs/stage4-package-registry-census.md`](../stage4-package-registry-census.md) · `scripts/s4_02_package_registry_census.sh` |
| Contract nail | `s4_02_package_registry_census_contract` |
| Plan status | Stage-4 `[~]` · S4-02 `[~]` · checklist **1/5** under this ADR |

## Consequences

- S4-02 moves `[ ]` → `[~]`；Stage-4 was `[~]`（**1/5**）at open.
- **S3-I93 landed** as [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md)（file:// Primary hinge）。
- **S3-I94 landed** as [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md)（scoped acceptance）。
- **S3-I95 landed** as [stage4-s4-02-claim](stage4-s4-02-claim.md)（S4-02 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I96**（S4-03 open）— not Stage-4 exit / not public registry GA.
- Prior: [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91 · [stage4-open](stage4-open.md) · S3-I88.
- Package-registry census Current → **S3-I95** · Claimed scoped `[x]`。

## References

- [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95
- [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md) · S3-I94
- [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · S3-I93
- [stage4-s4-01-claim](stage4-s4-01-claim.md) · S3-I91
- [stage4-open](stage4-open.md) · S3-I88（开窗形态）
- [stage4-package-registry-census](../stage4-package-registry-census.md)
- [s4-02-file-registry-v0](../s4-02-file-registry-v0.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-02
- [`crates/syx-pkg`](../../crates/syx-pkg)
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §阶段 4
