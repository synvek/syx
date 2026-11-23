# C-2b++ full-native CI census

**Date:** 2026-11-23  
**Scope:** required `full-native-linux`（LLVM 21.1.6 + `syx build --native examples/hello.sy`）。≠ 批量升 `native=real` · ≠ macOS LLVM（**C-2b+++** / park）。  
**Script:** [`scripts/c2b_plusplus_full_native_ci_census.sh`](../../scripts/c2b_plusplus_full_native_ci_census.sh)  
**Install:** [`scripts/ci_install_llvm.sh`](../../scripts/ci_install_llvm.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b+** | **exited** |
| **C-2b++** paid full-native floor | **exited** |
| Open | C-2b-3 exited → 默认下一 **C-2b-4**；C-2c/d · C-0a+/e park |

## Root cause

默认 CI 无 LLVM / `full-native` → 无法付 native 证据；soft `native-linux` 不能冒充 paid GA。

## Delivery

1. 抽出 `ci_install_llvm.sh`；`release.yml` 复用  
2. required job `full-native-linux`（无 `continue-on-error`；无 `full-native || release`）  
3. SSOT：Native paid floor；C-2b++ `[x]`；默认下一 **C-2b+++**  
4. census：job 契约 + 0× `native=real` 升格；硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `full-native-linux` required | no `continue-on-error` | **1** (`c2bpp_no_continue_on_error=1`) |
| uses `ci_install_llvm.sh` | yes | **1** |
| build `--features full-native` | no release fallback | **1** |
| product `native="real"` | **0** | **0** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_plusplus_full_native_ci_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2bpp_full_native_job=1
c2bpp_no_continue_on_error=1
c2bpp_install_script=1
c2bpp_release_reuses_install=1
c2bpp_native_real_count=0
c2bpp_census_doc=1
c2bpp_ssot_updated=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
roadmap_c2bpp_exit=1
roadmap_next_c2bppp=1
c2b_plusplus_full_native_ci_census_ok=1
current=C-2b++-full-native-ci
next=C-2b+++
```

## Non-claims

- ≠ 选择性/批量升产品 `native=real`（C-2b+++）
- ≠ macOS required full-native
- ≠ C-2c JIT 文案 · C-2d job 名 · R-50

## References

- [`c-2b-plus-native-real-honesty-census`](c-2b-plus-native-real-honesty-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b++ → C-2b+++
