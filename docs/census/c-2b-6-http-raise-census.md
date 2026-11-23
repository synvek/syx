# C-2b-6 http paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `http_native_wasm_ga`；仅升格 `net.http` → `native=real`（合计 6× real）。≠ server / types / stream / timeout_tls · pool/row · 其余 io.file（**C-2b-7**）。  
**Script:** [`scripts/c2b_6_http_raise_census.sh`](../../scripts/c2b_6_http_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-5** | **exited** |
| **C-2b-6** http raise | **exited** |
| Open | 默认下一 **C-2b-7**；C-2c/d · C-0a+/e park |

## Root cause

sql/path/tcp paid 不能冒充 http 客户端产品 `native=real`；`http_native_wasm_ga` 钉子已有但未进 required job；server/types/stream/timeout_tls 需独立证据。

## Delivery

1. `full-native-linux` + http GA（`--features full-native`）  
2. 升格 `net.http` + notes `C-2b-6`；http 余债仍 partial  
3. SSOT / `02`：C-2b-6 `[x]`；默认下一 **C-2b-7**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + `data.sqlite` + `net.http` | **6** (`c2b6_native_real_count=6`) |
| http residual native | **partial** | **1** (`c2b6_http_residual_partial=1`) |
| pool/row native | **partial** | **1** (`c2b6_pool_row_partial=1`) |
| `full-native-linux` http GA | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_6_http_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b6_native_real_count=6
c2b6_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http
c2b6_http_residual_partial=1
c2b6_pool_row_partial=1
c2b6_ci_http=1
c2b6_ci_still_required=1
c2b6_census_doc=1
c2b6_ssot_updated=1
c2b6_roadmap_ok=1
c2b6_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_6_http_raise_census_ok=1
current=C-2b-6-http-raise
next=C-2b-7
```

## Non-claims

- ≠ `net.http.server` / `.types` / `.stream` / `.timeout_tls` 升格（C-2b-7）
- ≠ pool / row / io.file.raii|stream|watch（C-2b-7）
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-5-sql-raise-census`](c-2b-5-sql-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-6 → C-2b-7
