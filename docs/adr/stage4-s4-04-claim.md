# ADR: Stage-4 S4-04 plan claim（S3-I103）

**Status:** Accepted (S3-I103) · S4-04 `[x]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I102 residual-scope](stage4-s4-04-residual-scope.md), Hard OUT construct ([I101 Iterator for-in hinge](stage4-s4-04-iterator-forin-hinge.md)), open ([I100](stage4-s4-04-open.md)), and scoped acceptance (I102) are landed. Primary KPIs report **`custom_iterator_forin_hinge_helper≥1`**, **`custom_iterator_parity_fixture_helper≥1`**, **`iterator_forin_test_helper≥1`**, **`collections_protocol_helper≥1`**, **`residual_scope_helper≥1`**, with honest deferred axes（`Sequence.makeIterator` · 用户 map·filter · Collection Index）remaining **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with [S3-I99 S4-03 claim](stage4-s4-03-claim.md). It does **not** exit Stage-4, change Sequence/Collection semantics / stdlib / codegen, or restore deferred axes as hard gates.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I100](stage4-s4-04-open.md) · [census](../stage4-sequence-collection-census.md) |
| Construct I101 | [`s4-04-iterator-forin-v0`](../s4-04-iterator-forin-v0.md) · for-in KPIs |
| Residual-scope | [S3-I102](stage4-s4-04-residual-scope.md) · scoped acceptance · [scope-v0](../s4-04-sequence-residual-scope-v0.md) |
| Primary KPI | `custom_iterator_forin_hinge_helper≥1` · parity≥1 · iterator_forin≥1 · collections.protocol≥1 · `residual_scope_helper≥1` · `s4_04_claim_helper≥1` |
| Contract | `s4_04_claim_contract` · `s4_04_sequence_collection_census_contract` |
| Honesty OUT | **≠** Stage-4 exit · **≠** write `stage4-exit` this window · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ 改语义/stdlib/codegen / ≠ `makeIterator` GA / ≠ 用户 map·filter GA / ≠ Collection Index GA）, **Sequence/Collection 协议** workstream is complete when I101 hinge + I102 scaffolding hold and Primary KPIs stay green — **not** 「`makeIterator` 已 GA」 / **not** 「用户类型 map/filter 已通用」 / **not** 「Collection Index 已全量」 / **not** 「Stage-4 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S4-04 → `[x]`**.
2. Keep **Stage-4 → `[~]`** until an explicit Stage-4 exit ADR（checklist **4/5** items claimed；exit still separate）。
3. Update sequence/collection census Current=**S3-I103** · **Claimed: scoped `[x]`**；next default = **S3-I104 S4-05 open** — **≠** Stage-4 exit / **≠** S4-05 construct this window.
4. **Non-claims (hard):** ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage4-exit` this window / ≠ 改语义/stdlib/codegen / ≠ `makeIterator`/用户 map·filter/Index **GA** / ≠ S4-05 施工 / ≠ 旧 `M2-S4-04` HTTP / ≠ language semantics.

## Consequences

- S4-04 plan-claimed `[x]`; Stage-4 stays `[~]`（checklist **4/5**）.
- **S3-I104 landed** as [stage4-s4-05-open](stage4-s4-05-open.md)（S4-05 open · census baseline）.
- Default next = **S3-I105**（doctor-native-sdk / full-native hinge）— not Stage-4 exit / not S4-05 claim / not `doctor --native-sdk` product wire this prior claim.
- Prior: [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md) · S3-I102 · construct [S3-I101](stage4-s4-04-iterator-forin-hinge.md) · open [S3-I100](stage4-s4-04-open.md).
- Sequence/collection census Current → **S3-I103** · Claimed scoped `[x]`。
- Prebuilt census: [stage4-prebuilt-release-census](../stage4-prebuilt-release-census.md) · Current **S3-I104** · next **S3-I105**。

## References

- [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md) · S3-I102
- [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · S3-I101
- [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100
- [s4-04-sequence-residual-scope-v0](../s4-04-sequence-residual-scope-v0.md)
- [s4-04-iterator-forin-v0](../s4-04-iterator-forin-v0.md)
- [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99（claim 窗形态）
- [stage4-sequence-collection-census](../stage4-sequence-collection-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-04
- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml)
