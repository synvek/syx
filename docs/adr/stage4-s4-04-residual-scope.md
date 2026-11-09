# ADR: Stage-4 S4-04 residual-scope（S3-I102）

**Status:** Accepted (S3-I102) · S4-04 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I101 Iterator for-in hinge](stage4-s4-04-iterator-forin-hinge.md), Primary KPIs are green (`custom_iterator_forin_hinge_helper≥1` · `custom_iterator_parity_fixture_helper≥1` · `iterator_forin_test_helper≥1` · `collections_protocol_helper≥1`), while `Sequence.makeIterator`, 用户类型通用 map/filter, and full Collection Index remain open. This window revises **S4-04 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S4-04 `[x]`, change Sequence/Collection semantics / stdlib / codegen, declare `makeIterator` GA, or exit Stage-4.

## Decision

1. **Scoped acceptance (S4-04 stays `[~]` this window):** Under Hard OUT（≠ 改语义/stdlib/codegen / ≠ `makeIterator` GA / ≠ 用户 map·filter GA / ≠ Collection Index GA）, honest S4-04 completion means:
   - **Primary:** I101 hinge holds（[`s4-04-iterator-forin-v0`](../s4-04-iterator-forin-v0.md)；自定义 Iterator for-in 多后端 + parity fixture + `collections.protocol` honesty）；
   - **KPI:** `custom_iterator_forin_hinge_helper≥1` · `custom_iterator_parity_fixture_helper≥1` · `iterator_forin_test_helper≥1` · `collections_protocol_helper≥1`；
   - **Scaffolding:** `s4_04_open_helper` · `s4_04_census_helper` · `residual_scope_helper` green.
2. **Deferred / supersede-only:** `Sequence.makeIterator` · 用户类型通用 map/filter · 全量 Collection Index are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`. Scope charter: [s4-04-sequence-residual-scope-v0](../s4-04-sequence-residual-scope-v0.md).
3. **Ladder next:** **S3-I103** = dedicated **S4-04 scoped `[x]` claim** under this acceptance — still ≠ Stage-4 exit.
4. **Non-claims (hard):** ≠ S4-04 `[x]` this window / ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 改语义/stdlib/codegen / ≠ `makeIterator`/用户 map·filter/Index **GA** / ≠ 开 S4-05 / ≠ 旧 `M2-S4-04` HTTP / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Residual scope | [`docs/s4-04-sequence-residual-scope-v0.md`](../s4-04-sequence-residual-scope-v0.md) |
| Census | `residual_scope_helper≥1` · Current=**S3-I102** |
| Plan | S4-04 row stays `[~]` · scoped摘要 |
| Contract | `s4_04_residual_scope_contract` |
| Prior | [I101 iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · [I100 open](stage4-s4-04-open.md) |

## Consequences

- Residual-scope hinge landed；S4-04 was `[~]` this window；Stage-4 was `[~]`（**3/5**）.
- **S3-I103 landed** as [stage4-s4-04-claim](stage4-s4-04-claim.md)（S4-04 scoped `[x]` · Stage-4 **4/5**）。
- Default next = **S3-I104**（S4-05 open）— not Stage-4 exit / not S4-05 construct / not `makeIterator` GA.
- Prior: [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · S3-I101.
- Sequence/collection census Current → **S3-I103** · Claimed scoped `[x]`。

## References

- [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103
- [stage4-s4-04-iterator-forin-hinge](stage4-s4-04-iterator-forin-hinge.md) · S3-I101

- [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100
- [s4-04-sequence-residual-scope-v0](../s4-04-sequence-residual-scope-v0.md)
- [s4-04-iterator-forin-v0](../s4-04-iterator-forin-v0.md)
- [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md) · S3-I98（形态模板）
- [stage4-sequence-collection-census](../stage4-sequence-collection-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-04
- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml)
