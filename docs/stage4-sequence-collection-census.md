# Stage-4 Sequence/Collection census (S4-04)

**ADR:** [open](adr/stage4-s4-04-open.md) · [iterator-forin-hinge](adr/stage4-s4-04-iterator-forin-hinge.md) · [residual-scope](adr/stage4-s4-04-residual-scope.md) · [**claim**](adr/stage4-s4-04-claim.md) · **Window:** S3-I103 · **Current:** S3-I103 · **Claimed:** scoped `[x]`

**Goal (S4-04 scoped acceptance):** Primary = I101 custom Iterator for-in + I102 residual scaffolding；`makeIterator` / 用户 map·filter / Collection Index = **deferred supersede-only** — **≠** Stage-4 exit / **≠** semantics·stdlib·codegen product knife this window.  
**This census:** S3-I103 scoped claim — Stage-4 **`[~]`** · S4-04 **`[x]`** · checklist **4/5**；下一 = **S3-I104** S4-05 open（**I104 open landed** → tip now **S3-I105** doctor-native-sdk hinge；本 census Current 仍钉 I103）。

## Command

```bash
bash scripts/s4_04_sequence_collection_census.sh
```

## Classification (hard)

| Class | Counts as S4-04 debt? | Notes |
|-------|----------------------|-------|
| 自定义 Iterator for-in 钉测 | **Primary** | I101 |
| parity fixture Counter | **Primary** | I101 |
| `collections.protocol` honesty | **Primary** | I101 |
| scoped acceptance charter | **Claimed hinge** | I102 |
| S4-04 scoped `[x]` | **Claimed** | I103 |
| Sequence.makeIterator | **Deferred / residual** | ≠ claim blocker |
| 内建 Array map/filter HostOp | **Inventory** | ≠ 用户 Sequence GA |
| full Collection Index | **Deferred / residual** | capabilities notes |
| S4-05 / Stage-4 exit | **No（OUT）** | |
| 旧 M2-S4-04 HTTP streaming | **N/A OUT** | ≠ 本轴 |

## KPIs（S3-I103 · Scoped claim · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `s4_04_claim_helper` | **≥1** | I103 claim hinge |
| `residual_scope_helper` | **≥1** | I102 |
| `custom_iterator_forin_hinge_helper` | **≥1** | I101 Primary |
| `custom_iterator_parity_fixture_helper` | **≥1** | parity fixture |
| `s4_04_open_helper` | **≥1** | I100 |
| `s4_04_census_helper` | **≥1** | census anchors |
| `iterator_forin_test_helper` | **≥1** | for-in tests |
| `collections_protocol_helper` | **≥1** | `collections.protocol` |
| `make_iterator_surface_helper` | **≥1** | deferred honesty |
| `collection_host_map_filter_helper` | **≥1** | inventory（≠ 用户 GA） |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S4-04 | **`[x]`** scoped；Primary 绿；makeIterator / 用户 map·filter / Index deferred supersede-only |
| Stage-4 | **`[~]`** · **4/5** |
| Next | **S3-I104** S4-05 open — ≠ Stage-4 exit / ≠ S4-05 construct this window |

## Ladder (locked)

1. **S3-I99:** S4-03 scoped `[x]` — landed.
2. **S3-I100:** S4-04 open — landed.
3. **S3-I101:** S4-04 custom-Iterator for-in hinge — landed.
4. **S3-I102:** S4-04 residual-scope — landed.
5. **S3-I103:** S4-04 scoped `[x]` claim — **this window**.
6. **S3-I104:** S4-05 open — next。

## Non-claims

- **≠** Stage-4 exit / write `stage4-exit` this window  
- **≠** 改 Sequence/Collection 语义 / stdlib 协议 / codegen  
- **≠** `makeIterator` / 用户 map·filter / Collection Index **GA**  
- **≠** S4-05 施工 / HostFn+1 / F-08 / Wave  
- **≠** 旧 `M2-S4-04` HTTP 轴  

## References

- [stage4-s4-04-claim](adr/stage4-s4-04-claim.md) · S3-I103
- [stage4-s4-04-residual-scope](adr/stage4-s4-04-residual-scope.md) · S3-I102
- [s4-04-sequence-residual-scope-v0](s4-04-sequence-residual-scope-v0.md)
- [stage4-s4-04-iterator-forin-hinge](adr/stage4-s4-04-iterator-forin-hinge.md) · S3-I101
- [s4-04-iterator-forin-v0](s4-04-iterator-forin-v0.md)
- [stage4-s4-04-open](adr/stage4-s4-04-open.md) · S3-I100
- [plan/02](../plan/02-improvement-roadmap.md) S4-04
- [`packages/internal/capabilities.toml`](../packages/internal/capabilities.toml)
- [`crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs`](../crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs)
- [`crates/syx-driver/tests/lang/protocol_iterator.rs`](../crates/syx-driver/tests/lang/protocol_iterator.rs)
