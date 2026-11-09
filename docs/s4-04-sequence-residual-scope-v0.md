# S4-04 Sequence/Collection residual-scope v0（S3-I102）

**ADR:** [residual-scope](adr/stage4-s4-04-residual-scope.md) · **Window:** S3-I102  
**Purpose:** Bound scoped acceptance vs deferred axes for S4-04「Sequence/Collection 协议」— documentation only.  
**≠** change Sequence/Collection semantics / stdlib / codegen · **≠** `makeIterator` GA · **≠** 用户 map·filter GA.

## Primary（already landed — claim-ready）

| Item | Status |
|------|--------|
| Custom Iterator for-in multi-backend | [I101](adr/stage4-s4-04-iterator-forin-hinge.md) · [v0](s4-04-iterator-forin-v0.md) |
| Parity fixture Counter for-in | census Primary greens |
| `collections.protocol` honesty | capabilities notes still open on deferred axes |
| Open inventory | [I100](adr/stage4-s4-04-open.md) |

## Deferred / supersede-only（not blockers for scoped `[x]`）

| Axis | Meaning |
|------|---------|
| `Sequence.makeIterator` | User Sequence entry GA |
| 用户类型通用 map/filter | Beyond built-in Array HostOp |
| full Collection Index | capabilities notes「still open」 |

Restoring any as a **hard claim gate** requires a **superseding ADR**.

## Out（Hard）

- S4-04 `[x]` this window · Stage-4 exit · S4-05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- 旧 `M2-S4-04` HTTP · Language semantics greenfield  

## Ladder

- **S3-I102:** this charter + residual-scope ADR  
- **S3-I103:** dedicated S4-04 scoped `[x]` claim under Primary above — **landed**（[claim ADR](adr/stage4-s4-04-claim.md)）  
- **S3-I104:** S4-05 open（next tip）  

