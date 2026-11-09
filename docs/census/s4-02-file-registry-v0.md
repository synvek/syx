# S4-02 file-registry / lock-semver v0（S3-I93）

**ADR:** [file-registry-hinge](../adr/stage4-s4-02-file-registry-hinge.md) · **Window:** S3-I93  
**Purpose:** Charter for Stage-4 **Primary** acceptance of S4-02「最小注册表」— elevate existing `file://` evidence to a credibility hinge.  
**≠** change [`syx-pkg`](../../crates/syx-pkg) / CLI product behavior · **≠** public registry GA · **≠** trim lock v16 schema this window.

## Primary（in scope for later scoped `[x]`）

| Axis | Meaning | Evidence anchors |
|------|---------|------------------|
| `file://` registry round-trip | Local index publish → install | `registry_install_from_file_registry`（syx-pkg）；`publish_file_registry` + `publish_install_roundtrip`（evergreen） |
| Lock exists | Project install writes / reads `syx.lock` | `SyxLockfile` / lock helpers；evergreen lock nails |
| Semver resolve | VersionReq + conflict honesty | `r001_diamond_registry_*`（syx-pkg integration） |

## Deferred / supersede-only（not blockers for Primary）

| Axis | Notes |
|------|-------|
| Public `packages.sy.dev` GA | OUT until superseding ADR |
| Lock schema v16 field trim | Honest gap；≠ I93 knife |
| CLI five-verb UX rewrite | Verbs exist；product polish deferred |
| Multi-publisher / CDN GA | Beyond minimal registry |

## Out（Hard）

- S4-02 `[x]` this window · Stage-4 exit · S4-03…05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Language semantics changes  

## Ladder

- **S3-I93:** this charter + hinge ADR + census Primary KPIs  
- **S3-I94:** residual-scope（scoped acceptance vs deferred）  
- Later: dedicated S4-02 scoped `[x]` claim  
