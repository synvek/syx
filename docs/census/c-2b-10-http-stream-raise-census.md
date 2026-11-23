# C-2b-10 http.stream paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `m2_s4_04_http_body_stream_native_llvm`；仅升格 `net.http.stream` → `native=real`（合计 11× real）。≠ timeout_tls · io.file.raii|stream|watch（**C-2b-11**）。  
**Script:** [`scripts/c2b_10_http_stream_raise_census.sh`](../../scripts/c2b_10_http_stream_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-9** | **exited** |
| **C-2b-10** http.stream raise | **exited** |
| Open | 默认下一 **C-2b-11**；C-2c/d · C-0a+/e park |

## Root cause

http client/server/types paid 不能冒充 body stream 产品 `native=real`；`m2_s4_04_http_body_stream_native_llvm` 钉子已有但未进 required job；timeout_tls 需独立证据。

## Delivery

1. `full-native-linux` + stream llvm（`--test stdlib` · `--features full-native`）  
2. 升格 `net.http.stream` + notes `C-2b-10`；timeout_tls 仍 partial  
3. SSOT / `02`：C-2b-10 `[x]`；默认下一 **C-2b-11**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http×4 | **11** (`c2b10_native_real_count=11`) |
| timeout_tls/io residual native | **partial** | **1** (`c2b10_timeout_io_residual_partial=1`) |
| `full-native-linux` stream llvm | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_10_http_stream_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b10_native_real_count=11
c2b10_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server,net.http.stream
c2b10_timeout_io_residual_partial=1
c2b10_ci_stream=1
c2b10_ci_still_required=1
c2b10_census_doc=1
c2b10_ssot_updated=1
c2b10_roadmap_ok=1
c2b10_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_10_http_stream_raise_census_ok=1
current=C-2b-10-http-stream-raise
next=C-2b-11
```

## Non-claims

- ≠ `net.http.timeout_tls` 升格（C-2b-11）
- ≠ io.file.raii|stream|watch（C-2b-11）
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-9-http-server-raise-census`](c-2b-9-http-server-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-10 → C-2b-11
