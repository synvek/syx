# S4-05 doctor-native-sdk / full-native v0（S3-I105）

**ADR:** [doctor-native-sdk-hinge](../adr/stage4-s4-05-doctor-native-sdk-hinge.md) · **Window:** S3-I105  
**Purpose:** Charter for Stage-4 **Primary** acceptance of S4-05「预编译发行物」— elevate existing native SDK layout / full-native packaging evidence to a credibility hinge.  
**≠** wire `syx doctor --native-sdk` · **≠** packaging/`release.yml` product knife · **≠** official sysroot GA this window.

## Primary（in scope for later scoped `[x]`）

| Axis | Meaning | Evidence anchors |
|------|---------|------------------|
| Native SDK layout report | Library inspect of SDK layout next to `bin/syx` | [`native_sdk_layout_report`](../../crates/syx-rt/src/link.rs)；`NativeSdkLayoutReport` |
| Layout unit nail | Report runs / triple non-empty | `native_sdk_layout_report_runs` in `link.rs` |
| full-native build | Prebuilt CLI with `full-native` feature | [`scripts/build-full-native`](../../scripts/build-full-native)（+ `.ps1`） |
| syx-sdk packaging | SDK collect / layout docs | [`packaging/syx-sdk`](../../packaging/syx-sdk) |

## Deferred / residual / supersede-only（not blockers for Primary）

| Axis | Notes |
|------|-------|
| `syx doctor --native-sdk` CLI wire | docs/release claim；CLI Doctor **unwired** — residual |
| release.yml smoke fix | still invokes unwired flag — residual with CLI wire |
| Official redistributable sysroot | `collect-sysroot` experimental；not release-GA |
| marketplace / 全 profile CI | absent |
| doctor UX rewrite | product surface OUT this hinge |

## Out（Hard）

- S4-05 `[x]` this window · Stage-4 exit · Stage-4 checklist bump  
- CLI / packaging / `release.yml` product knife  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Legacy `M2-S4-05` TLS client axis  

## Ladder

- **S3-I104:** S4-05 open census — **landed**（[open ADR](../adr/stage4-s4-05-open.md)）  
- **S3-I105:** this charter + hinge ADR + census Primary KPIs  
- **S3-I106:** residual-scope — **landed**（[residual ADR](../adr/stage4-s4-05-residual-scope.md)）  
- **S3-I107:** dedicated S4-05 scoped `[x]` claim — **landed**（[claim ADR](../adr/stage4-s4-05-claim.md)）· Stage-4 **5/5**  
- **S3-I108:** Stage-4 exit-open  

