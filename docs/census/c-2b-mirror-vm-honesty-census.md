# C-2b mirror-VM honesty census

**Date:** 2026-11-23  
**Scope:** 删除 `run_native_strict_or_mirror_vm`；`*_native_wasm_ga` 无 AOT → **skip**（≠ VM 回放冒充 native GA）。≠ CI LLVM · ≠ demote `native=real` · ≠ 禁 soft_native exit 0（**C-2b+**）。  
**Script:** [`scripts/c2b_mirror_vm_honesty_census.sh`](../../scripts/c2b_mirror_vm_honesty_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a** SSOT | **exited** |
| **C-2b** mirror honesty | **exited** |
| Open | C-2b+ exited → 默认下一 **C-2b++**；C-2c/d · C-0a+/e park |

## Root cause

`run_native_strict_or_mirror_vm` 在无 `full-native` 时回放 VM stdout 并仍断言 native parity → soft-green。

## Delivery

1. 删除 helper；四份 GA 改 `try_run_native_strict` + early return  
2. SSOT：mirror 路径 **forbidden / exited**  
3. `02`：C-2b `[x]`；新增 **C-2b+** 为默认下一  
4. census：0× 活调用；硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `pub fn run_native_strict_or_mirror_vm` | **0** | **0** (`c2b_mirror_vm_gone=1`) |
| `use` / 调用 `run_native_strict_or_mirror_vm` | **0** | **0**（`crates/**/*.rs` 无活引用） |
| backends GA compile | green (skip ok) | **1** (`c2b_backends_http_ga=1`) |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2b_mirror_vm_honesty_census_ok` | **1** | **1** |

## Results (2026-11-23)

```
c2b_mirror_vm_gone=1
c2b_try_run_present=1
c2b_census_doc=1
c2b_ssot_updated=1
c2b_backends_http_ga=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
roadmap_c2b_exit=1
roadmap_next_c2b_plus=1
c2b_mirror_vm_honesty_census_ok=1
current=C-2b-mirror-vm-honesty
next=C-2b+
```

## Non-claims

- ≠ CI required full-native / 禁 soft_native exit 0（C-2b+）
- ≠ 批量 demote 38× `native=real`（C-2b+）
- ≠ C-2c JIT 文案 · C-2d job 名 · R-50

## References

- [`c-2a-backend-ci-matrix-census`](c-2a-backend-ci-matrix-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2b → C-2b+
