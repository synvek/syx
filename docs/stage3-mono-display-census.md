# Stage-3 mono-display census (S3-02)

**ADR:** [stage3-s3-02-open](adr/stage3-s3-02-open.md) · [demangle-semantic](adr/stage3-s3-02-demangle-semantic.md) · [debug-stack-display](adr/stage3-s3-02-debug-stack-display.md) · [runtime-type-display](adr/stage3-s3-02-runtime-type-display.md) · [meta-type-display](adr/stage3-s3-02-meta-type-display.md) · [**claim**](adr/stage3-s3-02-claim.md) · **Window:** S3-I47 · **Current:** S3-I47 · **Claimed:** scoped `[x]`

**Goal (S3-02):** user-facing surfaces show **generic / method forms**, not internal monomorph mangled names.  
**This census:** after I47 scoped plan claim — S3-02 **`[x]`**；**≠** Stage-3 exit / **≠** S3-03 本窗.

## Command

```bash
bash scripts/s3_02_mono_display_census.sh
```

## Classification (hard)

| Class | Counts as S3-02 debt? | Notes |
|-------|----------------------|-------|
| **User diagnostic type strings** | **Claimed** | I43 demangle |
| **LSP / IDE hover & signatures** | **Claimed** | I43 附带 |
| **Debug / DAP stack fn names** | **Claimed（展示）** | I44；断点仍 mangled（OUT） |
| **Runtime `__type` / object type names** | **Claimed（展示）** | I45 identity/display split |
| **Meta / reflect type names** | **Claimed（展示）** | I46 |
| **Internal mono / codegen mangling** | **No（OUT）** | `mangle_name` + IR identity **OK** |

## KPIs（S3-I47 · Claimed · 2026-11-08）

| KPI | Value | Notes |
|-----|------:|-------|
| `type_id_to_name_named_passthrough` | **0** | claim ① |
| `user_facing_type_name_helper` | **≥1** | claim ① |
| `user_facing_mir_fn_name_helper` | **≥1** | claim ③ |
| `user_facing_runtime_type_name_helper` | **≥1** | claim ④ |
| `user_facing_meta_type_name_helper` | **≥1** | claim ⑤ |
| `debug_mangled_fn_nail` | **≥1** | intentional OUT |
| `object_ref_type_name` | **≈9–10** | identity OUT |
| `meta_type_expr_to_type_name` | **≈3–13** | identity OUT |
| `mangle_name_mono_def` / `mangle_name_codegen` | **1 / 8** | internal OK |

## Claimed (scoped)

See [stage3-s3-02-claim](adr/stage3-s3-02-claim.md)：轴①–⑤ + intentional OUT。S3-02 plan **`[x]`**；Stage-3 **`[~]`**.

## Ladder (locked)

1. **S3-I42:** S3-02 open — landed.
2. **S3-I43:** demangle-semantic — landed.
3. **S3-I44:** debug stack — landed.
4. **S3-I45:** runtime `__type` — landed.
5. **S3-I46:** meta type — landed.
6. **S3-I47:** S3-02 scoped claim — **this window**.
7. **Next:** **S3-03 open**（标准库预编译/缓存）— ≠ Stage-3 exit.

## Non-claims

- **≠** Stage-3 exit  
- **≠** S3-03 施工本窗  
- **≠** 断点 / identity demangle / 停 `mangle_name`  
- **≠** HostFn+1 / F-08 / Wave  

## References

- [stage3-s3-02-claim](adr/stage3-s3-02-claim.md) · S3-I47
- [stage3-s3-02-meta-type-display](adr/stage3-s3-02-meta-type-display.md) · S3-I46
- [plan/02](../plan/02-improvement-roadmap.md) S3-02
