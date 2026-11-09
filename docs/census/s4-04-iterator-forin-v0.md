# S4-04 custom Iterator for-in v0（S3-I101）

**ADR:** [iterator-forin-hinge](../adr/stage4-s4-04-iterator-forin-hinge.md) · **Window:** S3-I101  
**Purpose:** Charter for Stage-4 **Primary** acceptance of S4-04「Sequence/Collection 协议」— elevate existing custom Iterator for-in evidence to a credibility hinge.  
**≠** change Sequence/Collection semantics / stdlib / codegen · **≠** `makeIterator` GA · **≠** 用户 map·filter Primary this window.

## Primary（in scope for later scoped `[x]`）

| Axis | Meaning | Evidence anchors |
|------|---------|------------------|
| Custom Iterator for-in | User type `: Iterator` + `for-in` multi-backend | [`collection_sequence_iterator.rs`](../../crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs)；[`protocol_iterator.rs`](../../crates/syx-driver/tests/lang/protocol_iterator.rs) |
| Parity fixture | Counter for-in → sum `6` | [`protocol_iterator_custom.sy`](../../crates/syx-driver/tests/fixtures/parity/protocol_iterator_custom.sy) |
| `collections.protocol` honesty | implemented + notes still open | [`capabilities.toml`](../../packages/internal/capabilities.toml) |

## Deferred / residual / supersede-only（not blockers for Primary）

| Axis | Notes |
|------|-------|
| `Sequence.makeIterator` | capabilities / mir_op inventory；GA OUT |
| 用户类型通用 map/filter | 内建 Array HostOp ≠ 用户 Sequence GA |
| full Collection Index | notes「still open」 |
| 旧 `M2-S4-04` HTTP streaming | **N/A** — ≠ 本轴 |

## Out（Hard）

- S4-04 `[x]` this window · Stage-4 exit · S4-05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Language semantics / stdlib protocol / codegen product knife  

## Ladder

- **S3-I101:** this charter + hinge ADR + census Primary KPIs  
- **S3-I102:** residual-scope（scoped acceptance vs deferred makeIterator/map·filter/Index）— **landed**（[residual ADR](../adr/stage4-s4-04-residual-scope.md)）  
- **S3-I103:** dedicated S4-04 scoped `[x]` claim — **landed**（[claim ADR](../adr/stage4-s4-04-claim.md)）  
- **S3-I104:** S4-05 open（next tip）  


