# C-2b-7 pool/row paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `hostfn_sql_pool_lifecycle_native_llvm` + `hostfn_sql_row_accessors_native_llvm`；升格 `data.sqlite.pool` + `data.sqlite.row` → `native=real`（合计 8× real）。≠ http 余债 · 其余 io.file（**C-2b-8**）。  
**Script:** [`scripts/c2b_7_pool_row_raise_census.sh`](../../scripts/c2b_7_pool_row_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-6** | **exited** |
| **C-2b-7** pool/row raise | **exited** |
| Open | 默认下一 **C-2b-8**；C-2c/d · C-0a+/e park |

## Root cause

core `data.sqlite` paid 不能冒充 pool/row 产品 `native=real`；stdlib llvm 钉子已有但未进 required job。

## Delivery

1. `full-native-linux` + pool/row llvm（`--test stdlib` · `--features full-native`）  
2. 升格 `data.sqlite.pool` + `data.sqlite.row` + notes `C-2b-7`  
3. SSOT / `02`：C-2b-7 `[x]`；默认下一 **C-2b-8**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http | **8** (`c2b7_native_real_count=8`) |
| http/io residual native | **partial** | **1** (`c2b7_http_io_residual_partial=1`) |
| `full-native-linux` pool/row llvm | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_7_pool_row_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b7_native_real_count=8
c2b7_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row
c2b7_http_io_residual_partial=1
c2b7_ci_pool_row=1
c2b7_ci_still_required=1
c2b7_census_doc=1
c2b7_ssot_updated=1
c2b7_roadmap_ok=1
c2b7_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_7_pool_row_raise_census_ok=1
current=C-2b-7-pool-row-raise
next=C-2b-8
```

## Non-claims

- ≠ `net.http.server` / `.types` / `.stream` / `.timeout_tls` 升格（C-2b-8）
- ≠ io.file.raii|stream|watch（C-2b-8）
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-6-http-raise-census`](c-2b-6-http-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-7 → C-2b-8
