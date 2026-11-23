# C-2b-13 net.tls.client paid raise census

**Date:** 2026-11-24  
**Scope:** required `full-native-linux` 扩跑两条 `m2_s4_05_*_native_aot`；仅升格 `net.tls.client` → `native=real`（合计 16× real）；翻锁 `m2_s4_05_docs_and_capability`。≠ `net.tls.server`（park）；默认下一 **C-2c**。  
**Script:** [`scripts/c2b_13_tls_client_raise_census.sh`](../../scripts/c2b_13_tls_client_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-12** | **exited** |
| **C-2b-13** tls.client raise | **exited** |
| Open | 默认下一 **C-2c**；tls.server · C-2d · C-0a+/e park |

## Root cause

http.timeout_tls paid 不能冒充 `net.tls.client` 产品 `native=real`；EXTRA_ROOT / self-signed native AOT 钉已有但未进 required job；docs meta 仍锁 `native=partial`。

## Delivery

1. `full-native-linux` + `m2_s4_05_https_loopback_ok_with_extra_root_native_aot` + `m2_s4_05_self_signed_fail_closed_native_aot`  
2. 升格 `net.tls.client` + notes `C-2b-13`；翻锁 docs meta；`net.tls.server` 仍 partial  
3. SSOT / `02`：C-2b-13 `[x]`；默认下一 **C-2c**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | path×2 + tcp×2 + sqlite×3 + http×5 + file×3 + tls.client | **16** (`c2b13_native_real_count=16`) |
| `net.tls.server` residual | **partial** | **1** (`c2b13_tls_server_residual_partial=1`) |
| `full-native-linux` tls aot×2 | present + required | **1** |
| docs meta | `native=real` + C-2b-13 | **1** (`c2b13_meta_flipped=1`) |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_13_tls_client_raise_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c2b13_native_real_count=16
c2b13_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server,net.http.stream,net.http.timeout_tls,io.file.raii,io.file.stream,io.file.watch,net.tls.client
c2b13_tls_server_residual_partial=1
c2b13_ci_tls_aot=1
c2b13_ci_still_required=1
c2b13_meta_flipped=1
c2b13_census_doc=1
c2b13_ssot_updated=1
c2b13_roadmap_ok=1
c2b13_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_13_tls_client_raise_census_ok=1
current=C-2b-13-tls-client-raise
next=C-2c
```

## Non-claims

- ≠ `net.tls.server` 升格 / daemon GA
- ≠ skip-verify / danger_accept_invalid_certs
- ≠ macOS required full-native · C-2c/d 本体 · R-50

## References

- [`c-2b-12-io-file-raise-census`](c-2b-12-io-file-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-13 → C-2c
