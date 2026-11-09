# Stage-3 stdlib-cache census (S3-03)

**ADR:** [stage3-s3-03-open](../adr/stage3-s3-03-open.md) · [prelude-snapshot](../adr/stage3-s3-03-prelude-snapshot.md) · [warm-program-trim](../adr/stage3-s3-03-warm-program-trim.md) · [warm-fixed-tax](../adr/stage3-s3-03-warm-fixed-tax.md) · [warm-residual](../adr/stage3-s3-03-warm-residual.md) · [cache-roundtrip](../adr/stage3-s3-03-cache-roundtrip.md) · [prelude-embed](../adr/stage3-s3-03-prelude-embed.md) · [**claim**](../adr/stage3-s3-03-claim.md) · **Window:** S3-I55 · **Current:** S3-I55 · **Claimed:** scoped `[x]`

**Goal (S3-03):** 标准库预编译/缓存；`hello` 启动 **&lt; 0.15 s**。  
**This census:** S3-I55 scoped plan claim — S3-03 **`[x]`**；Stage-3 仍 **`[~]`**；**≠** Stage-3 exit / **≠** S3-04 本窗开工。

## Command

```bash
bash scripts/s3_03_stdlib_cache_census.sh
```

## Classification (hard)

| Class | Counts as S3-03 debt? | Notes |
|-------|----------------------|-------|
| `hello` cold / warm / prelude_hit | **Claimed** | 实测均 &lt;0.15 |
| Prelude embed / roundtrip / residual / … | Hinged (I49–I54) | 保持 |
| HostFn / Wave / F-08 / S3-04…07 | **No（OUT）** | Hard OUT |

## KPIs（S3-I55 · Claim · 2026-11-08）

Machine-local（prefer release `syx`）。Claimed under scoped meaning（见 claim ADR）。

| KPI | Value | Notes |
|-----|------:|-------|
| `prelude_embed_bytes` | **≥1** | I54 |
| `prelude_embed_hit` | **≥1** | I54 |
| `prelude_embed_asset_bytes` | **≈6.6e6** | I54（regen after SyxHost CallHost map） |
| `cache_roundtrip_helper` | **≥1** | I53 |
| `cache_tag_bake` | **≥1** | I52 |
| `load_from_env_once` | **≥1** | I52 |
| `trim_skip_warm` | **≥1** | I52 |
| `cache_first_helper` | **≥1** | I51 |
| `fingerprint_once_lock` | **≥1** | I51 |
| `lazy_host_helper` | **≥1** | I51 |
| `program_trim_helper` | **≥1** | I50 |
| `entry_cache_bytes` | **≈5e3** | I50 |
| `hello_cold_s` | **≈0.08** | **&lt;0.15** claimed |
| `hello_prelude_hit_s` | **≈0.08** | **&lt;0.15** claimed |
| `hello_warm_s` | **≈0.00** | **&lt;0.15** claimed |
| `hello_nostd_s` | **≈0.00** | 护栏 |
| `dedicated_prelude_cache_hit` | **≥1** | I49 |
| `hello_target_s` | **0.15** | met (scoped) |
| `embedded_std_include_str` | **73** | residual OK（非 OUT） |
| `ensure_prelude_fn` | **1** | prelude 入口 |
| `cached_module_unit` | **≥2** | 入口 + prelude |
| `incremental_cache_path_fn` | **1** | 项目 cache 路径 |
| `syx_cache_v_tag` | **≥1** | 指纹族 |
| `stdlib_virtual_path_mentions` | **≥1** | `<stdlib>/…` |
| `core_prelude_entry_approx` | **≈16** | CORE 表项量级 |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-03 | **`[x]`** scoped claim |
| Stage-3 | 仍 **`[~]`**（余 S3-04…07） |
| Next | **S3-04 `[~]`** via [I56 open](../adr/stage3-s3-04-open.md)；下一 construct = **S3-I57** — ≠ Stage-3 exit |

## Ladder (locked)

1. **S3-I48:** S3-03 open — landed.
2. **S3-I49:** prelude disk snapshot — landed.
3. **S3-I50:** warm program trim — landed.
4. **S3-I51:** warm fixed tax — landed.
5. **S3-I52:** warm residual — landed.
6. **S3-I53:** cache roundtrip — landed.
7. **S3-I54:** prelude embed — landed.
8. **S3-I55:** S3-03 claim — landed.
9. **S3-I56:** S3-04 open — landed（[vm-hotspot census](stage3-vm-hotspot-census.md)）。
10. **Next:** **S3-I57** construct — ≠ Stage-3 exit.

## Non-claims

- **≠** Stage-3 exit  
- **≠** HostFn+1 / F-08 / Wave  
- **≠** S3-04…07 本窗开工  
- **≠** 零 `include_str!` / build.rs 编 `.sy`  

## References

- [stage3-s3-03-claim](../adr/stage3-s3-03-claim.md) · S3-I55
- [stage3-s3-03-prelude-embed](../adr/stage3-s3-03-prelude-embed.md) · S3-I54
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
