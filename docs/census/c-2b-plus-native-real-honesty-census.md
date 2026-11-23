# C-2b+ native=real honesty census

**Date:** 2026-11-23  
**Scope:** unpaid product `native="real"` → `"partial"`；禁 CI `soft_native` L006→exit/return 0。≠ CI 装 LLVM · ≠ required full-native job · ≠ 再升 `native=real`（**C-2b++**）。  
**Script:** [`scripts/c2b_plus_native_real_honesty_census.sh`](../../scripts/c2b_plus_native_real_honesty_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a** SSOT | **exited** |
| **C-2b** mirror honesty | **exited** |
| **C-2b+** native honesty | **exited** |
| Open | C-2b++ exited → 默认下一 **C-2b+++**；C-2c/d · C-0a+/e park |

## Root cause

Capability 宣称 `native=real` 而默认 CI 无 `full-native`；`soft_native` 对 L006 **return/exit 0** → 步骤假绿。

## Delivery

1. 38× `native="real"` → `"partial"` + notes `C-2b+: unpaid ≠ full-native GA`  
2. `ci.yml`：L006 → 非 0；job 仍 `continue-on-error`  
3. SSOT / `02`：C-2b+ `[x]`；默认下一 **C-2b++**  
4. census：0× unpaid real；无 L006→0；硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| product `native="real"` | **0** | **0** (`c2bp_native_real_count=0`) |
| L006 → return/exit 0 | **0** | **0** (`c2bp_l006_no_soft_success=1`) |
| `continue-on-error` kept | yes | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| mirror_vm regress | **0** | **0** (`c2bp_mirror_still_gone=1`) |
| `c2b_plus_native_real_honesty_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2bp_native_real_count=0
c2bp_demote_notes=1
c2bp_l006_no_soft_success=1
c2bp_continue_on_error_kept=1
c2bp_census_doc=1
c2bp_ssot_updated=1
c2bp_website_cap_nail=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
roadmap_c2bp_exit=1
roadmap_next_c2bpp=1
c2bp_mirror_still_gone=1
c2b_plus_native_real_honesty_census_ok=1
current=C-2b+-native-real-honesty
next=C-2b++
```

## Non-claims

- ≠ CI required full-native / 装 LLVM（C-2b++）
- ≠ 有证据后重新 `native=real`（C-2b++）
- ≠ C-2c JIT 文案 · C-2d job 名 · R-50

## References

- [`c-2b-mirror-vm-honesty-census`](c-2b-mirror-vm-honesty-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b+ → C-2b++
