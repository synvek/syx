# ADR: Stage-4 S4-04 Iterator for-in hinge（S3-I101）

**Status:** Accepted (S3-I101) · S4-04 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I100 S4-04 open](stage4-s4-04-open.md), census showed custom Iterator for-in / `collections.protocol` inventory green while Stage-4 still lacked a **Primary construct hinge**. Existing multi-backend Counter for-in nails and parity fixture already pass. This window elevates that evidence into a **machine-checkable hinge** (charter + census KPIs + contract). It does **not** claim S4-04 `[x]`, change Sequence/Collection semantics / stdlib / codegen, declare `makeIterator` / 用户 map·filter GA, or exit Stage-4.

## Decision

1. **Publish** charter [`docs/census/s4-04-iterator-forin-v0.md`](../census/s4-04-iterator-forin-v0.md)（Primary = custom Iterator for-in；`makeIterator` / 用户 map·filter / Collection Index deferred）。
2. **Nail Primary KPIs** on existing evidence（no product edit）：
   - `iterator_forin_test_helper≥1` · `collection_sequence_iterator` / `protocol_iterator`
   - `custom_iterator_parity_fixture_helper≥1` · `protocol_iterator_custom.sy`
   - `collections_protocol_helper≥1` · `collections.protocol` honesty
   - `custom_iterator_forin_hinge_helper≥1` · this hinge ADR/charter
3. **Ladder next:** **S3-I102** = S4-04 residual-scope — still ≠ S4-04 `[x]`.
4. **Non-claims (hard):** ≠ S4-04 `[x]` / ≠ Stage-4 exit / ≠ 改语义/stdlib/codegen / ≠ `makeIterator` / 用户 map·filter / Collection Index **GA** / ≠ S4-05 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 旧 `M2-S4-04` HTTP / ≠ language semantics greenfield.

## Evidence

| Check | Evidence |
|-------|----------|
| Charter | [`docs/census/s4-04-iterator-forin-v0.md`](../census/s4-04-iterator-forin-v0.md) |
| Census KPI | `custom_iterator_forin_hinge_helper≥1` · iterator_forin / parity fixture / collections.protocol |
| Contract | `s4_04_iterator_forin_hinge_contract` |
| Prior | [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100 |

## Consequences

- Custom-Iterator for-in Primary hinge landed；S4-04 stays `[~]` this window；Stage-4 stays `[~]`（**3/5**）.
- **S3-I102 landed** as [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md)（scoped acceptance · deferred makeIterator/map·filter/Index）。
- **S3-I103 landed** as [stage4-s4-04-claim](stage4-s4-04-claim.md)（S4-04 scoped `[x]` · Stage-4 **4/5**）。
- Default next = **S3-I104**（S4-05 open）— not Stage-4 exit / not `makeIterator` GA.
- Prior: [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100.
- Sequence/collection census Current → **S3-I103** · Claimed scoped `[x]`。

## References

- [stage4-s4-04-claim](stage4-s4-04-claim.md) · S3-I103
- [stage4-s4-04-residual-scope](stage4-s4-04-residual-scope.md) · S3-I102
- [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100


- [s4-04-iterator-forin-v0](../census/s4-04-iterator-forin-v0.md)
- [stage4-sequence-collection-census](../census/stage4-sequence-collection-census.md)
- [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97（hinge 形态）
- [plan/02](../../plan/02-improvement-roadmap.md) S4-04
- [`crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs`](../../crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs)
- [`crates/syx-driver/tests/lang/protocol_iterator.rs`](../../crates/syx-driver/tests/lang/protocol_iterator.rs)
- [`packages/internal/capabilities.toml`](../../packages/internal/capabilities.toml)
