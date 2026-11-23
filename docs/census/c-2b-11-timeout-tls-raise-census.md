# C-2b-11 timeout_tls paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `m2_s4_02b_timeout_hang_loopback_native_aot`；仅升格 `net.http.timeout_tls` → `native=real`（合计 12× real）。≠ io.file.raii|stream|watch · net.tls.*（**C-2b-12**）。  
**Script:** [`scripts/c2b_11_timeout_tls_raise_census.sh`](../../scripts/c2b_11_timeout_tls_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-10** | **exited** |
| **C-2b-11** timeout_tls raise | **exited** |
| Open | 默认下一 **C-2b-12**；C-2c/d · C-0a+/e park |

## Root cause

http stream/server/types paid 不能冒充 timeout_tls 产品 `native=real`；`m2_s4_02b_timeout_hang_loopback_native_aot` 钉子已有但未进 required job；≠ TLS client GA。

## Delivery

1. `full-native-linux` + timeout llvm（`--test stdlib` · `--features full-native`）  
2. 升格 `net.http.timeout_tls` + notes `C-2b-11`；io.file / tls 仍 partial  
3. SSOT / `02`：C-2b-11 `[x]`；默认下一 **C-2b-12**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http×5 | **12** (`c2b11_native_real_count=12`) |
| io/tls residual native | **partial** | **1** (`c2b11_io_tls_residual_partial=1`) |
| `full-native-linux` timeout llvm | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_11_timeout_tls_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b11_native_real_count=12
c2b11_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server,net.http.stream,net.http.timeout_tls
c2b11_io_tls_residual_partial=1
c2b11_ci_timeout=1
c2b11_ci_still_required=1
c2b11_census_doc=1
c2b11_ssot_updated=1
c2b11_roadmap_ok=1
c2b11_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_11_timeout_tls_raise_census_ok=1
current=C-2b-11-timeout-tls-raise
next=C-2b-12
```

## Non-claims

- ≠ io.file.raii|stream|watch 升格（C-2b-12）
- ≠ `net.tls.client` / `.server`（C-2b-12 / park）
- ≠ macOS required full-native · C-2c/d · R-50

## References

- [`c-2b-10-http-stream-raise-census`](c-2b-10-http-stream-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-11 → C-2b-12
