# C-2b-9 http.server paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `m2_s4_03b_graceful_shutdown_native_llvm`；仅升格 `net.http.server` → `native=real`（合计 10× real）。≠ stream / timeout_tls · io.file.raii|stream|watch（**C-2b-10**）。  
**Script:** [`scripts/c2b_9_http_server_raise_census.sh`](../../scripts/c2b_9_http_server_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-8** | **exited** |
| **C-2b-9** http.server raise | **exited** |
| Open | 默认下一 **C-2b-10**；C-2c/d · C-0a+/e park |

## Root cause

`net.http` / types paid 不能冒充 server 产品 `native=real`；`m2_s4_03b_graceful_shutdown_native_llvm` 钉子已有但未进 required job；stream/timeout_tls 需独立证据。

## Delivery

1. `full-native-linux` + server llvm（`--test stdlib` · `--features full-native`）  
2. 升格 `net.http.server` + notes `C-2b-9`；stream/timeout_tls 仍 partial  
3. SSOT / `02`：C-2b-9 `[x]`；默认下一 **C-2b-10**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http×3 | **10** (`c2b9_native_real_count=10`) |
| stream/tls/io residual native | **partial** | **1** (`c2b9_stream_tls_io_residual_partial=1`) |
| `full-native-linux` server llvm | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_9_http_server_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b9_native_real_count=10
c2b9_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server
c2b9_stream_tls_io_residual_partial=1
c2b9_ci_server=1
c2b9_ci_still_required=1
c2b9_census_doc=1
c2b9_ssot_updated=1
c2b9_roadmap_ok=1
c2b9_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_9_http_server_raise_census_ok=1
current=C-2b-9-http-server-raise
next=C-2b-10
```

## Non-claims

- ≠ `net.http.stream` / `.timeout_tls` 升格（C-2b-10）
- ≠ io.file.raii|stream|watch（C-2b-10）
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-8-http-types-raise-census`](c-2b-8-http-types-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-9 → C-2b-10
