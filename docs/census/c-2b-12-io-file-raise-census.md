# C-2b-12 io.file.raii·stream·watch paid raise census

**Date:** 2026-11-24  
**Scope:** required `full-native-linux` 扩跑三条 file llvm；一批升格 `io.file.raii` · `io.file.stream` · `io.file.watch` → `native=real`（合计 15× real）。≠ `net.tls.*`（**C-2b-13**）。  
**Script:** [`scripts/c2b_12_io_file_raise_census.sh`](../../scripts/c2b_12_io_file_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-11** | **exited** |
| **C-2b-12** io.file batch raise | **exited** |
| Open | 默认下一 **C-2b-13**；C-2c/d · C-0a+/e park |

## Root cause

`path_file_native_wasm_ga`（C-2b-3）不能冒充 raii/stream/watch 产品 `native=real`；三条 stdlib llvm 钉已有但未进 required job；≠ TLS client GA。

## Delivery

1. `full-native-linux` + `m2_s3_02b_with_open_native_llvm` + `m2_s3_03_file_stream_read_chunk_native_llvm` + `m2_s3_03_file_watcher_native_llvm`  
2. 升格三行 + notes `C-2b-12`；tls 仍 partial  
3. SSOT / `02`：C-2b-12 `[x]`；默认下一 **C-2b-13**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http×5 + file×3 | **15** (`c2b12_native_real_count=15`) |
| tls residual native | **partial** | **1** (`c2b12_tls_residual_partial=1`) |
| `full-native-linux` file llvm×3 | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_12_io_file_raise_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c2b12_native_real_count=15
c2b12_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server,net.http.stream,net.http.timeout_tls,io.file.raii,io.file.stream,io.file.watch
c2b12_tls_residual_partial=1
c2b12_ci_file_llvm=1
c2b12_ci_still_required=1
c2b12_census_doc=1
c2b12_ssot_updated=1
c2b12_roadmap_ok=1
c2b12_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_12_io_file_raise_census_ok=1
current=C-2b-12-io-file-raise
next=C-2b-13
```

## Non-claims

- ≠ `net.tls.client` / `.server` 升格（C-2b-13）
- ≠ recursive/async FileWatcher GA · File.watch one-shot
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-11-timeout-tls-raise-census`](c-2b-11-timeout-tls-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-12 → C-2b-13
