# S4-02 registry residual-scope v0（S3-I94）

**ADR:** [residual-scope](../adr/stage4-s4-02-residual-scope.md) · **Window:** S3-I94  
**Purpose:** Bound scoped acceptance vs deferred axes for S4-02「包注册表最小可用」— documentation only.  
**≠** change [`syx-pkg`](../../crates/syx-pkg) / CLI · **≠** public registry GA · **≠** trim lock v16.

## Primary（already landed — claim-ready）

| Item | Status |
|------|--------|
| `file://` registry round-trip | [I93](../adr/stage4-s4-02-file-registry-hinge.md) · [v0](s4-02-file-registry-v0.md) |
| Lock exists + semver resolve KPIs | census Primary greens |
| Open inventory (CLI verbs) | [I92](../adr/stage4-s4-02-open.md) |

## Deferred / supersede-only（not blockers for scoped `[x]`）

| Axis | Meaning |
|------|---------|
| Public `packages.sy.dev` GA | Production registry product claim |
| Lock schema v16 field trim | MVP vs full lock surface honesty |
| CLI five-verb UX rewrite | Polish beyond verb existence |
| Multi-publisher / CDN GA | Beyond minimal local registry |

Restoring any as a **hard claim gate** requires a **superseding ADR**.

## Out（Hard）

- S4-02 `[x]` this window · Stage-4 exit · S4-03…05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Language semantics changes  

## Ladder

- **S3-I94:** this charter + residual-scope ADR  
- **S3-I95:** dedicated S4-02 scoped `[x]` claim under Primary above — **landed**（[claim ADR](../adr/stage4-s4-02-claim.md)）  
- **S3-I96:** S4-03 open（next tip）  
