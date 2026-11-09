# ADR: Stage-4 S4-04 open — Sequence/Collection protocol（S3-I100）

**Status:** Accepted (S3-I100) · Stage-4 `[~]` · S4-04 `[~]` · S4-01/`02`/`03` `[x]`  
**Date:** 2026-11-09  
**Context:** After [S3-I99 S4-03 scoped claim](stage4-s4-03-claim.md), default credibility work advances to [plan/02 S4-04](../../plan/02-improvement-roadmap.md): **Sequence/Collection 协议**（用户类型 for-in / map/filter）. Iterator for-in 子集与 `collections.protocol` inventory 已厚；this window **opens** S4-04 with a sequence/collection **census baseline** — it does **not** claim S4-04 `[x]`, change Sequence/Collection semantics / stdlib protocol / codegen, declare `makeIterator` / 用户 map·filter GA, or exit Stage-4. **≠** 旧 ID `M2-S4-04`（HTTP streaming）。

## Gap: Sequence/Collection vs plan S4-04

| Claim | Today (honest) | plan/02 S4-04 target |
|-------|----------------|----------------------|
| 自定义 Iterator for-in | [`collection_sequence_iterator`](../../crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs)；[`protocol_iterator`](../../crates/syx-driver/tests/lang/protocol_iterator.rs)；Counter + witness 多后端钉测 | 用户类型 for-in 可验收加深（非从 0） |
| `collections.protocol` | [`capabilities.toml`](../../packages/internal/capabilities.toml) `implemented`；notes：**Sequence.makeIterator / full Collection Index still open** | 诚实面加深 / scoped KPI |
| Sequence.makeIterator | [`mir_op_capabilities`](../../packages/internal/mir_op_capabilities.toml) `MakeIterator` inventory；书/矩阵 **partial** | 用户 Sequence 入口 GA（缺口） |
| 用户类型 map/filter | 内建 [`CollectionHostOp::Map/Filter`](../../crates/syx-stdlib/src/collection_hosts.rs) ≠ 用户 Sequence 通用 | 用户类型通用 map/filter |
| Collection Index | capabilities notes **still open** | 全量 Index 面 |
| Stage-4 claim | **无** S4-04 scoped `[x]` / **无** 本轴 census | census → construct → claim |

Baseline: [stage4-sequence-collection-census](../stage4-sequence-collection-census.md).

## Decision

1. **Open S4-04** as the default construction axis; flip plan S4-04 `[ ]` → `[~]`.
2. Keep **Stage-4 → `[~]`** with checklist **3/5**（S4-01/`02`/`03` already claimed；本窗不抬分母）.
3. **S3-I100 first slice** — publish sequence/collection census (+ script); add a thin contract under `syx-driver/tests/`（**no** Sequence/Collection semantics / stdlib / codegen product edit this window）.
4. **Ladder:** **S3-I101** = first construct（**custom Iterator for-in KPI hinge** — elevate 现有多后端证据为 machine-checkable Primary，仿 I97）；`makeIterator` / 用户 map·filter / Collection Index → residual / supersede-only → dedicated **S4-04 `[x]` claim**.
5. **Non-claims (hard):** ≠ S4-04 `[x]` / ≠ S4-04 construct（改语义/stdlib/codegen）/ ≠ Stage-4 exit / ≠ `makeIterator` / 用户 map·filter / Collection Index **GA** / ≠ S4-05 施工 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ language semantics greenfield / ≠ 旧 `M2-S4-04` HTTP 轴。

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Sequence/collection census | [`docs/stage4-sequence-collection-census.md`](../stage4-sequence-collection-census.md) · `scripts/s4_04_sequence_collection_census.sh` |
| Contract nail | `s4_04_sequence_collection_census_contract` |
| Plan status | Stage-4 `[~]` · S4-04 `[~]` · checklist **3/5** under this ADR |

## Consequences

- S4-04 moves `[ ]` → `[~]`；Stage-4 stays `[~]`（**3/5**）.
- **S3-I101 landed** as [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md)（custom Iterator for-in Primary hinge）。
- **S3-I102 landed** as [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md)（scoped acceptance）。
- **S3-I103 landed** as [stage4-s4-04-claim](stage4-s4-04-claim.md)（S4-04 scoped `[x]` · Stage-4 **4/5**）。
- **S3-I104 landed** as [stage4-s4-05-open](stage4-s4-05-open.md)（S4-05 open）。
- Default next = **S3-I105**（doctor-native-sdk / full-native hinge）— not Stage-4 exit / not `makeIterator` GA.
- Prior: [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99 · [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96.
- Sequence/collection census Current → **S3-I103** · Claimed scoped `[x]`。

## References

- [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103
- [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md) · S3-I102
- [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · S3-I101
- [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99



- [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96（开窗形态）
- [stage4-sequence-collection-census](../stage4-sequence-collection-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-04
- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml) `collections.protocol`
- [`crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs`](../../crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs)
- [`crates/syx-stdlib/src/collection_hosts.rs`](../../crates/syx-stdlib/src/collection_hosts.rs)
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §阶段 4
