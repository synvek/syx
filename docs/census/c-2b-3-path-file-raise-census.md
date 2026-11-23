# C-2b-3 path_file paid raise census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux` 扩跑 `path_file_native_wasm_ga`；仅升格 `io.path` + `io.file.read_all_bytes` → `native=real`。旧称 **C-2b+++**（`-N`=N×`+`）。≠ 批量升格（**C-2b-4**）。  
**Script:** [`scripts/c2b_3_path_file_raise_census.sh`](../../scripts/c2b_3_path_file_raise_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-2** | **exited** |
| **C-2b-3** path_file raise | **exited** |
| Open | C-2b-4 exited → 默认下一 **C-2b-5**；C-2c/d · C-0a+/e park |

## Root cause

hello-only paid floor 不能诚实支撑产品 `native=real`；path_file GA 钉子已有但未进 required job。

## Delivery

1. 命名：默认下一 / Next 用 **C-2b-3** / **C-2b-4**（禁新文 `+++`）  
2. `full-native-linux` + path_file GA（`--features full-native`）  
3. 升格 2 行 + notes `C-2b-3`  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `native=real` ids | exactly `io.path`, `io.file.read_all_bytes` | **2** (`c2b3_native_real_count=2`) |
| `full-native-linux` path_file | present + required | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_3_path_file_raise_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b3_native_real_count=2
c2b3_raise_ids=io.path,io.file.read_all_bytes
c2b3_ci_path_file=1
c2b3_ci_still_required=1
c2b3_census_doc=1
c2b3_ssot_updated=1
c2b3_roadmap_ok=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2b_3_path_file_raise_census_ok=1
current=C-2b-3-path-file-raise
next=C-2b-4
```

## Non-claims

- ≠ 其余 36× / net.* / sqlite / process 升格（C-2b-4）
- ≠ macOS required full-native
- ≠ C-2c JIT 文案 · C-2d job 名 · R-50

## References

- [`c-2b-plusplus-full-native-ci-census`](c-2b-plusplus-full-native-ci-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b-3 → C-2b-4
