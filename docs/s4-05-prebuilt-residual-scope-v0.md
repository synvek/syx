# S4-05 prebuilt / full-native residual-scope v0（S3-I106）

**ADR:** [residual-scope](adr/stage4-s4-05-residual-scope.md) · **Window:** S3-I106  
**Purpose:** Bound scoped acceptance vs deferred axes for S4-05「预编译发行物」— documentation only.  
**≠** wire `syx doctor --native-sdk` · **≠** packaging/`release.yml` product knife · **≠** official sysroot GA.

## Primary（already landed — claim-ready）

| Item | Status |
|------|--------|
| Native SDK layout report + unit nail | [I105](adr/stage4-s4-05-doctor-native-sdk-hinge.md) · [v0](s4-05-doctor-native-sdk-v0.md) |
| full-native build script | census Primary greens |
| syx-sdk packaging tree | `packaging/syx-sdk` |
| Open inventory | [I104](adr/stage4-s4-05-open.md) |

## Deferred / supersede-only（not blockers for scoped `[x]`）

| Axis | Meaning |
|------|---------|
| `syx doctor --native-sdk` CLI wire | docs/release claim；CLI Doctor unwired |
| release.yml smoke fix | still invokes unwired flag |
| Official redistributable sysroot | `collect-sysroot` experimental |
| marketplace / 全 profile CI | absent |
| doctor UX rewrite | product surface |

Restoring any as a **hard claim gate** requires a **superseding ADR**.

## Out（Hard）

- S4-05 `[x]` this window · Stage-4 exit · Stage-4 checklist bump  
- CLI / packaging / `release.yml` product knife  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Legacy `M2-S4-05` TLS  

## Ladder

- **S3-I106:** this charter + residual-scope ADR  
- **S3-I107:** dedicated S4-05 scoped `[x]` claim — **landed**（[claim ADR](adr/stage4-s4-05-claim.md)）· Stage-4 **5/5**  
- **S3-I108:** Stage-4 exit-open  

