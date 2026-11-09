# Stage-4 package registry census (S4-02)

**ADR:** [open](../adr/stage4-s4-02-open.md) · [file-registry-hinge](../adr/stage4-s4-02-file-registry-hinge.md) · [residual-scope](../adr/stage4-s4-02-residual-scope.md) · [**claim**](../adr/stage4-s4-02-claim.md) · **Window:** S3-I95 · **Current:** S3-I95 · **Claimed:** scoped `[x]`

**Goal (S4-02 scoped acceptance):** Primary = I93 `file://` registry + lock/semver KPI + I94 residual scaffolding；公网 GA / lock trim / CLI UX = **deferred supersede-only** — **≠** Stage-4 exit / **≠** `syx-pkg`/CLI product knife this window.  
**This census:** S3-I95 scoped claim — Stage-4 **`[~]`** · S4-02 **`[x]`** · checklist **4/5**；下一 = **S3-I96** S4-03 open.

## Command

```bash
bash scripts/s4_02_package_registry_census.sh
```

## Classification (hard)

| Class | Counts as S4-02 debt? | Notes |
|-------|----------------------|-------|
| CLI verbs exist | **Inventory** | verbs present |
| `syx-pkg` registry + lock + semver | **Inventory** | skeleton heavy |
| lock schema v16 ≫ MVP | **Deferred / supersede-only** | ≠ claim blocker |
| `file://` registry path | **Primary hinge** | I93 |
| scoped acceptance charter | **Claimed hinge** | I94 |
| S4-02 scoped `[x]` | **Claimed** | I95 |
| Public `packages.sy.dev` GA | **Deferred / supersede-only** | ≠ claim blocker |
| S4-03…05 / Stage-4 exit | **No（OUT）** | |

## KPIs（S3-I95 · Scoped claim · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `s4_02_claim_helper` | **≥1** | I95 claim hinge |
| `residual_scope_helper` | **≥1** | I94 |
| `file_registry_hinge_helper` | **≥1** | I93 Primary |
| `file_registry_integration_test_count` | **≥1** | `registry_install_from_file_registry` |
| `file_registry_evergreen_roundtrip_count` | **≥2** | evergreen round-trip |
| `registry_semver_r001_count` | **≥1** | R001 diamond |
| `s4_02_open_helper` | **≥1** | I92 |
| `s4_02_census_helper` | **≥1** | census anchors |
| `syx_add_helper` | **≥1** | CLI Add |
| `syx_update_helper` | **≥1** | CLI Update |
| `syx_publish_helper` | **≥1** | CLI Publish |
| `syx_lock_helper` | **≥1** | lockfile |
| `registry_source_helper` | **≥1** | registry sources |
| `semver_dep_helper` | **≥1** | VersionReq |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S4-02 | **`[x]`** scoped；Primary 绿；公网 GA/lock trim deferred supersede-only |
| Stage-4 | **`[~]`** · **4/5** |
| Next | **S3-I96** S4-03 open — **landed**（[open ADR](../adr/stage4-s4-03-open.md)）；concurrency tip = **S3-I97** — ≠ Stage-4 exit |

## Ladder (locked)

1. **S3-I91:** S4-01 scoped `[x]` — landed.
2. **S3-I92:** S4-02 open — landed.
3. **S3-I93:** S4-02 file-registry hinge — landed.
4. **S3-I94:** S4-02 residual-scope — landed.
5. **S3-I95:** S4-02 scoped `[x]` claim — **this window**.
6. **S3-I96:** S4-03 open — next。

## Non-claims

- **≠** Stage-4 exit / write `stage4-exit` this window  
- **≠** `syx-pkg` / CLI 行为改  
- **≠** 公网 registry GA / lock v16 全面裁剪  
- **≠** S4-03…05 施工 / HostFn / F-08 unpark  

## References

- [stage4-s4-02-claim](../adr/stage4-s4-02-claim.md) · S3-I95
- [stage4-s4-02-residual-scope](../adr/stage4-s4-02-residual-scope.md) · S3-I94
- [s4-02-registry-residual-scope-v0](s4-02-registry-residual-scope-v0.md)
- [stage4-s4-02-file-registry-hinge](../adr/stage4-s4-02-file-registry-hinge.md) · S3-I93
- [s4-02-file-registry-v0](s4-02-file-registry-v0.md)
- [stage4-s4-02-open](../adr/stage4-s4-02-open.md) · S3-I92
- [plan/02](../../plan/02-improvement-roadmap.md) S4-02
- [`crates/syx-pkg`](../../crates/syx-pkg)
