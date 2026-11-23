# C-2b-4 tcp paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `tcp_native_wasm_ga`；仅升格 `net.tcp` + `net.tcp.listener` → `native=real`（合计 4× real）。≠ http/sql/其余 io.file（**C-2b-5**）。  
**Script:** [`scripts/c2b_4_tcp_raise_census.sh`](../../scripts/c2b_4_tcp_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-3** | **exited** |
| **C-2b-4** tcp raise | **exited** |
| Open | C-2b-5 exited → 默认下一 **C-2b-6**；C-2c/d · C-0a+/e park |

## Root cause

path_file paid 不能冒充 tcp 产品 `native=real`；`tcp_native_wasm_ga` 钉子已有但未进 required job。

## Delivery

1. `full-native-linux` + tcp GA（`--features full-native`）  
2. 升格 `net.tcp` + `net.tcp.listener` + notes `C-2b-4`  
3. SSOT / `02`：C-2b-4 `[x]`；默认下一 **C-2b-5**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | exactly path×2 + tcp×2 | **4** (`c2b4_native_real_count=4`) |
| `full-native-linux` tcp GA | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_4_tcp_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b4_native_real_count=4
c2b4_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener
c2b4_ci_tcp=1
c2b4_ci_still_required=1
c2b4_census_doc=1
c2b4_ssot_updated=1
c2b4_roadmap_ok=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_4_tcp_raise_census_ok=1
current=C-2b-4-tcp-raise
next=C-2b-5
```

## Non-claims

- ≠ http / sql / tls / io.file.raii|stream|watch / process 升格（C-2b-5）
- ≠ macOS required full-native
- ≠ C-2c JIT 文案 · C-2d job 名 · R-50

## References

- [`c-2b-3-path-file-raise-census`](c-2b-3-path-file-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-4 → C-2b-5
